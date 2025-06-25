use crate::{
    error::AppError,
    printer::{
        driver::PrinterDriver,
        load::load_printer,
        supply::{Drum, Drums, Fuser, PrinterSupply, Reservoir, Toner, TonerColor, Toners},
        Printer,
    },
    snmp::{value::get_snmp_value, SnmpClientParams},
    utils::parse_oid_to_vec,
};
use serde_json::Value;

/// Implementation of the generic driver.
pub struct GenericDriver;

/// Fetches a specific supply component (like a Toner or Drum) if its OIDs are defined.
///
/// This generic function abstracts the repetitive logic of:
/// 1. Looking up 'level' and 'max_level' OIDs.
/// 2. Fetching their values via SNMP if they exist.
/// 3. Constructing the final supply struct (Toner, Drum, Fuser, etc.).
///
/// ## Arguments
/// * `oids`: The loaded JSON value with all OID mappings.
/// * `params`: SNMP client parameters.
/// * `supply_type`: The category of the supply (e.g., PrinterSupply::Toner).
/// * `color`: An optional color for the supply.
/// * `constructor`: A closure that takes `level` and `max_level` and returns a new supply struct.
///
/// ## Returns
/// An `Option` containing the constructed supply struct if successful, or `None`.
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
    fn is_compatible(&self, _printer_name: &str) -> bool {
        // This driver is a "catch-all", so it considers itself compatible
        // with anything, since the logic in `load_printer` will determine
        // whether there is a JSON file for the model.
        true
    }

    /// Retrieves printer supply information via SNMP.
    ///
    /// Queries toner levels and other printer parameters using SNMP based on model-specific OID mappings
    /// defined in a JSON configuration file. If the configuration for the specified printer is missing,
    /// an error is returned.
    ///
    /// ## Parameters
    /// - `params`: Reference to [`SnmpClientParams`] containing SNMP connection details.
    /// - `printer_name`: Identifier for the printer driver to load the corresponding configuration.
    ///
    /// ## Returns
    /// - `Ok(Printer)`: A [`Printer`] struct with name, brand, model, toner levels, and usage percentages.
    /// - `Err(AppError)`: If the configuration or SNMP query fails.
    fn get_supplies(
        &self,
        params: &SnmpClientParams,
        printer_name: &str,
    ) -> Result<Printer, AppError> {
        let brand = printer_name
            .split_whitespace()
            .next()
            .ok_or_else(|| AppError::ParseError("Could not determine printer brand".to_string()))?;

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
            serial_number = fetch_supply(&oids, params, PrinterSupply::Info, None, |_, _| {
                String::new()
            })?
            .map(|_| {
                get_snmp_value(
                    &parse_oid_to_vec(oids["info"]["serial_number"].as_str().unwrap_or_default())?,
                    params,
                )
            })
            .transpose()?;

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

        let mut printer = Printer::new(
            printer_name.to_string(),
            serial_number,
            toners,
            drums,
            fuser,
            reservoir,
        );

        printer.calculate_all_levels();

        Ok(printer)
    }
}
