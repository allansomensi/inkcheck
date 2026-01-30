use crate::{
    error::{AppError, ErrorKind},
    printer::{
        driver::PrinterDriver,
        load::load_printer,
        supply::{
            drum::{Drum, Drums},
            fuser::Fuser,
            reservoir::Reservoir,
            toner::{Toner, TonerColor, Toners},
            PrinterSupply,
        },
        Metrics, Printer,
    },
    snmp::{value::get_snmp_value, SnmpClientParams},
    utils::parse_oid_to_vec,
};
use serde_json::Value;

/// A catch-all driver that uses external JSON configuration files to map SNMP OIDs to printer supplies.
///
/// This driver is used when no specific hardcoded driver matches the printer.
/// It relies on data files stored in the `data_dir` to define how to fetch levels for specific models.
pub struct GenericDriver;

/// Helper function to fetch current and maximum levels for a specific supply component.
///
/// It looks up the OID strings in the provided JSON configuration (based on supply type and color),
/// parses them, and queries the SNMP agent. Returns `None` if the OIDs are missing or empty.
fn fetch_supply<T>(
    oids: &Value,
    params: &SnmpClientParams,
    supply_type: PrinterSupply,
    color: Option<TonerColor>,
    constructor: impl Fn(i64, i64) -> T,
) -> Result<Option<T>, AppError> {
    let get_supply_oid = |key: &str| -> Result<Vec<u64>, AppError> {
        let oid_str = if let Some(c) = &color {
            oids.get(supply_type.to_string().to_lowercase())
                .and_then(|t| t.get(c.to_string().to_lowercase()))
                .and_then(|b| b.get(key))
                .and_then(|l| l.as_str())
        } else {
            oids.get(supply_type.to_string().to_lowercase())
                .and_then(|b| b.get(key))
                .and_then(|l| l.as_str())
        };

        match oid_str {
            Some(s) if !s.is_empty() => parse_oid_to_vec(s),
            _ => Ok(Vec::new()),
        }
    };

    let level_oid = get_supply_oid("level")?;
    let max_level_oid = get_supply_oid("max_level")?;

    if !level_oid.is_empty() && !max_level_oid.is_empty() {
        let level = get_snmp_value(&level_oid, params)?;
        let max_level = get_snmp_value(&max_level_oid, params)?;
        Ok(Some(constructor(level, max_level)))
    } else {
        Ok(None)
    }
}

impl PrinterDriver for GenericDriver {
    /// Always returns `true` as this is the fallback driver.
    ///
    /// Compatibility is ultimately determined by the existence of a matching JSON configuration file
    /// during the `load_printer` step.
    fn is_compatible(&self, _printer_name: &str) -> bool {
        true
    }

    /// Loads the model-specific JSON configuration and queries all mapped OIDs via SNMP.
    ///
    /// Constructs a full [`Printer`] object by fetching toners and optionally fetching
    /// extra supplies (drums, fuser, reservoir) and usage metrics if requested in `params`.
    fn get_supplies(
        &self,
        params: &SnmpClientParams,
        printer_name: &str,
    ) -> Result<Printer, AppError> {
        let brand = printer_name.split_whitespace().next().ok_or_else(|| {
            AppError::new(ErrorKind::Parse(
                "Could not determine printer brand".to_string(),
            ))
        })?;

        let oids = load_printer(brand, printer_name, params.data_dir.clone())?;

        let toners = Toners {
            black_toner: fetch_supply(
                &oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Black),
                |l, m| Toner::new(l, m, None),
            )?,
            cyan_toner: fetch_supply(
                &oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Cyan),
                |l, m| Toner::new(l, m, None),
            )?,
            magenta_toner: fetch_supply(
                &oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Magenta),
                |l, m| Toner::new(l, m, None),
            )?,
            yellow_toner: fetch_supply(
                &oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Yellow),
                |l, m| Toner::new(l, m, None),
            )?,
        };

        let mut drums = Drums::default();
        let mut fuser: Option<Fuser> = None;
        let mut reservoir: Option<Reservoir> = None;
        let mut serial_number: Option<String> = None;

        if params.extra_supplies {
            if let Some(oid_str) = oids
                .get("info")
                .and_then(|i| i.get("serial_number"))
                .and_then(|s| s.as_str())
            {
                if !oid_str.is_empty() {
                    let serial_oid = parse_oid_to_vec(oid_str)?;
                    serial_number = Some(get_snmp_value::<String>(&serial_oid, params)?);
                }
            }

            drums = Drums {
                black_drum: fetch_supply(
                    &oids,
                    params,
                    PrinterSupply::Drum,
                    Some(TonerColor::Black),
                    |l, m| Drum::new(l, m, None),
                )?,
                cyan_drum: fetch_supply(
                    &oids,
                    params,
                    PrinterSupply::Drum,
                    Some(TonerColor::Cyan),
                    |l, m| Drum::new(l, m, None),
                )?,
                magenta_drum: fetch_supply(
                    &oids,
                    params,
                    PrinterSupply::Drum,
                    Some(TonerColor::Magenta),
                    |l, m| Drum::new(l, m, None),
                )?,
                yellow_drum: fetch_supply(
                    &oids,
                    params,
                    PrinterSupply::Drum,
                    Some(TonerColor::Yellow),
                    |l, m| Drum::new(l, m, None),
                )?,
            };

            fuser = fetch_supply(&oids, params, PrinterSupply::Fuser, None, |l, m| {
                Fuser::new(l, m, None)
            })?;
            reservoir = fetch_supply(&oids, params, PrinterSupply::Reservoir, None, |l, m| {
                Reservoir::new(l, m, None)
            })?;
        }

        let mut metrics: Option<Metrics> = None;
        let mut total_impressions: Option<i64> = None;
        let mut mono_impressions: Option<i64> = None;
        let mut color_impressions: Option<i64> = None;

        if params.metrics {
            if let Some(oid_str) = oids
                .get("metrics")
                .and_then(|m| m.get("total_impressions"))
                .and_then(|s| s.as_str())
            {
                if !oid_str.is_empty() {
                    let total_impressions_oid = parse_oid_to_vec(oid_str)?;
                    total_impressions =
                        Some(get_snmp_value::<i64>(&total_impressions_oid, params)?);
                }
            }

            if let Some(oid_str) = oids
                .get("metrics")
                .and_then(|m| m.get("mono_impressions"))
                .and_then(|s| s.as_str())
            {
                if !oid_str.is_empty() {
                    let mono_impressions_oid = parse_oid_to_vec(oid_str)?;
                    mono_impressions = Some(get_snmp_value::<i64>(&mono_impressions_oid, params)?);
                }
            }

            if let Some(oid_str) = oids
                .get("metrics")
                .and_then(|m| m.get("color_impressions"))
                .and_then(|s| s.as_str())
            {
                if !oid_str.is_empty() {
                    let color_impressions_oid = parse_oid_to_vec(oid_str)?;
                    color_impressions =
                        Some(get_snmp_value::<i64>(&color_impressions_oid, params)?);
                }
            }

            metrics = Some(Metrics {
                total_impressions,
                mono_impressions,
                color_impressions,
            });
        }

        let mut printer = Printer::new(
            printer_name.to_string(),
            serial_number,
            toners,
            drums,
            fuser,
            reservoir,
            metrics,
        );

        printer.calculate_all_levels();

        Ok(printer)
    }
}
