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
use async_trait::async_trait;
use serde_json::Value;

/// A catch-all driver that uses external JSON configuration files to map SNMP OIDs to printer supplies.
pub struct GenericDriver;

impl GenericDriver {
    /// Generic helper to fetch specific supply levels.
    ///
    /// Parses the JSON config for OIDs and queries the SNMP agent.
    async fn fetch_single_supply<T>(
        &self,
        oids: &Value,
        params: &SnmpClientParams,
        supply_type: PrinterSupply,
        color: Option<TonerColor>,
        constructor: impl Fn(i64, i64) -> T,
    ) -> Result<Option<T>, AppError> {
        let get_oid = |key: &str| -> Result<Vec<u64>, AppError> {
            let section = oids.get(supply_type.to_string().to_lowercase());

            let target = if let Some(c) = &color {
                section.and_then(|t| t.get(c.to_string().to_lowercase()))
            } else {
                section
            };

            let oid_str = target.and_then(|b| b.get(key)).and_then(|l| l.as_str());

            match oid_str {
                Some(s) if !s.is_empty() => parse_oid_to_vec(s),
                _ => Ok(Vec::new()),
            }
        };

        let level_oid = get_oid("level")?;
        let max_level_oid = get_oid("max_level")?;

        if !level_oid.is_empty() && !max_level_oid.is_empty() {
            let level = get_snmp_value(&level_oid, params).await?;
            let max_level = get_snmp_value(&max_level_oid, params).await?;
            Ok(Some(constructor(level, max_level)))
        } else {
            Ok(None)
        }
    }

    /// Fetches all toner cartridges (Black, Cyan, Magenta, Yellow).
    async fn fetch_toners(
        &self,
        oids: &Value,
        params: &SnmpClientParams,
    ) -> Result<Toners, AppError> {
        let black_toner = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Black),
                |l, m| Toner::new(l, m, None),
            )
            .await?;

        let cyan_toner = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Cyan),
                |l, m| Toner::new(l, m, None),
            )
            .await?;

        let magenta_toner = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Magenta),
                |l, m| Toner::new(l, m, None),
            )
            .await?;

        let yellow_toner = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Toner,
                Some(TonerColor::Yellow),
                |l, m| Toner::new(l, m, None),
            )
            .await?;

        Ok(Toners {
            black_toner,
            cyan_toner,
            magenta_toner,
            yellow_toner,
        })
    }

    /// Fetches all drum units if `extra_supplies` is enabled.
    async fn fetch_drums(
        &self,
        oids: &Value,
        params: &SnmpClientParams,
    ) -> Result<Drums, AppError> {
        if !params.extra_supplies {
            return Ok(Drums::default());
        }

        let black_drum = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Drum,
                Some(TonerColor::Black),
                |l, m| Drum::new(l, m, None),
            )
            .await?;

        let cyan_drum = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Drum,
                Some(TonerColor::Cyan),
                |l, m| Drum::new(l, m, None),
            )
            .await?;

        let magenta_drum = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Drum,
                Some(TonerColor::Magenta),
                |l, m| Drum::new(l, m, None),
            )
            .await?;

        let yellow_drum = self
            .fetch_single_supply(
                oids,
                params,
                PrinterSupply::Drum,
                Some(TonerColor::Yellow),
                |l, m| Drum::new(l, m, None),
            )
            .await?;

        Ok(Drums {
            black_drum,
            cyan_drum,
            magenta_drum,
            yellow_drum,
        })
    }

    /// Fetches auxiliary components (Serial Number, Fuser, Reservoir).
    async fn fetch_extras(
        &self,
        oids: &Value,
        params: &SnmpClientParams,
    ) -> Result<(Option<String>, Option<Fuser>, Option<Reservoir>), AppError> {
        if !params.extra_supplies {
            return Ok((None, None, None));
        }

        // Fetch Serial Number
        let serial_number = if let Some(oid_str) = oids
            .get("info")
            .and_then(|i| i.get("serial_number"))
            .and_then(|s| s.as_str())
        {
            if !oid_str.is_empty() {
                let oid = parse_oid_to_vec(oid_str)?;
                Some(get_snmp_value::<String>(&oid, params).await?)
            } else {
                None
            }
        } else {
            None
        };

        // Fetch Fuser & Reservoir
        let fuser = self
            .fetch_single_supply(oids, params, PrinterSupply::Fuser, None, |l, m| {
                Fuser::new(l, m, None)
            })
            .await?;

        let reservoir = self
            .fetch_single_supply(oids, params, PrinterSupply::Reservoir, None, |l, m| {
                Reservoir::new(l, m, None)
            })
            .await?;

        Ok((serial_number, fuser, reservoir))
    }

    /// Fetches usage metrics (total, mono, and color impressions).
    async fn fetch_metrics(
        &self,
        oids: &Value,
        params: &SnmpClientParams,
    ) -> Result<Option<Metrics>, AppError> {
        if !params.metrics {
            return Ok(None);
        }

        let fetch_metric_oid = |key: &str| -> Option<Vec<u64>> {
            oids.get("metrics")
                .and_then(|m| m.get(key))
                .and_then(|s| s.as_str())
                .filter(|s| !s.is_empty())
                .and_then(|s| parse_oid_to_vec(s).ok())
        };

        let mut total = None;
        if let Some(oid) = fetch_metric_oid("total_impressions") {
            total = Some(get_snmp_value::<i64>(&oid, params).await?);
        }

        let mut mono = None;
        if let Some(oid) = fetch_metric_oid("mono_impressions") {
            mono = Some(get_snmp_value::<i64>(&oid, params).await?);
        }

        let mut color = None;
        if let Some(oid) = fetch_metric_oid("color_impressions") {
            color = Some(get_snmp_value::<i64>(&oid, params).await?);
        }

        Ok(Some(Metrics {
            total_impressions: total,
            mono_impressions: mono,
            color_impressions: color,
        }))
    }
}

#[async_trait]
impl PrinterDriver for GenericDriver {
    fn is_compatible(&self, _printer_name: &str) -> bool {
        true
    }

    async fn get_supplies(
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
        let toners = self.fetch_toners(&oids, params).await?;
        let drums = self.fetch_drums(&oids, params).await?;
        let (serial_number, fuser, reservoir) = self.fetch_extras(&oids, params).await?;
        let metrics = self.fetch_metrics(&oids, params).await?;

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
