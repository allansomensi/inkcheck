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

/// Implementation of the generic driver.
pub struct GenericDriver;

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
        // A lógica que antes estava em `snmp::get_printer_values` agora vive aqui.
        let brand = printer_name
            .split_whitespace()
            .next()
            .ok_or_else(|| AppError::ParseError("Could not determine printer brand".to_string()))?;

        let oids = load_printer(brand, printer_name, params.data_dir.clone())?;

        // Function to retrieve the OID for a specific printer supply and color.
        // It constructs the key based on the supply type and color, and then looks for the
        // specified OID key within the OID mappings.
        //
        // ## Parameters:
        // - `supply`: The type of supply to search for.
        // - `color`: The color of the toner.
        // - `key`: The specific key to retrieve.
        //
        // ## Returns:
        // A Result containing the parsed OID as a `Vec<u64>` if found, or an `OidNotFound` error if not found.
        let get_supply_oid = |supply: PrinterSupply, color: Option<TonerColor>, key: &str| {
            if color.is_some() {
                oids.get(supply.to_string().to_lowercase())
                    .and_then(|t| t.get(color.unwrap().to_string().to_lowercase()))
                    .and_then(|b| b.get(key))
                    .and_then(|l| l.as_str())
                    .map(parse_oid_to_vec)
                    .ok_or_else(|| AppError::OidNotFound)?
            } else {
                oids.get(supply.to_string().to_lowercase())
                    .and_then(|b| b.get(key))
                    .and_then(|l| l.as_str())
                    .map(parse_oid_to_vec)
                    .ok_or_else(|| AppError::OidNotFound)?
            }
        };

        //
        // OIDs
        //

        // Info

        let serial_number_oid = get_supply_oid(PrinterSupply::Info, None, "serial_number")?;

        // Toners
        let black_toner_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Black), "level")?;
        let black_toner_max_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Black), "max_level")?;

        let cyan_toner_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Cyan), "level")?;
        let cyan_toner_max_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Cyan), "max_level")?;

        let magenta_toner_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Magenta), "level")?;
        let magenta_toner_max_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Magenta), "max_level")?;

        let yellow_toner_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Yellow), "level")?;
        let yellow_toner_max_level_oid =
            get_supply_oid(PrinterSupply::Toner, Some(TonerColor::Yellow), "max_level")?;

        // Drums

        let mut black_drum_level_oid: Vec<u64> = Vec::new();
        let mut black_drum_max_level_oid: Vec<u64> = Vec::new();

        let mut cyan_drum_level_oid: Vec<u64> = Vec::new();
        let mut cyan_drum_max_level_oid: Vec<u64> = Vec::new();

        let mut magenta_drum_level_oid: Vec<u64> = Vec::new();
        let mut magenta_drum_max_level_oid: Vec<u64> = Vec::new();

        let mut yellow_drum_level_oid: Vec<u64> = Vec::new();
        let mut yellow_drum_max_level_oid: Vec<u64> = Vec::new();

        let mut fuser_level_oid: Vec<u64> = Vec::new();
        let mut fuser_max_level_oid: Vec<u64> = Vec::new();

        let mut reservoir_level_oid: Vec<u64> = Vec::new();
        let mut reservoir_max_level_oid: Vec<u64> = Vec::new();

        // Info

        let mut serial_number: Option<String> = Some(String::new());

        if params.extra_supplies {
            serial_number = if !serial_number_oid.is_empty() {
                Some(get_snmp_value(&serial_number_oid, params)?)
            } else {
                None
            };

            black_drum_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Black), "level")?;
            black_drum_max_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Black), "max_level")?;

            cyan_drum_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Cyan), "level")?;
            cyan_drum_max_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Cyan), "max_level")?;

            magenta_drum_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Magenta), "level")?;
            magenta_drum_max_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Magenta), "max_level")?;

            yellow_drum_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Yellow), "level")?;
            yellow_drum_max_level_oid =
                get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Yellow), "max_level")?;

            fuser_level_oid = get_supply_oid(PrinterSupply::Fuser, None, "level")?;
            fuser_max_level_oid = get_supply_oid(PrinterSupply::Fuser, None, "max_level")?;

            reservoir_level_oid = get_supply_oid(PrinterSupply::Reservoir, None, "level")?;
            reservoir_max_level_oid = get_supply_oid(PrinterSupply::Reservoir, None, "max_level")?;
        }

        //
        // Values
        //

        let black_toner =
            if !black_toner_level_oid.is_empty() && !black_toner_max_level_oid.is_empty() {
                Some(Toner {
                    level: get_snmp_value(&black_toner_level_oid, params)?,
                    max_level: get_snmp_value(&black_toner_max_level_oid, params)?,
                    level_percent: None,
                })
            } else {
                None
            };

        let cyan_toner = if !cyan_toner_level_oid.is_empty() && !cyan_toner_max_level_oid.is_empty()
        {
            Some(Toner {
                level: get_snmp_value(&cyan_toner_level_oid, params)?,
                max_level: get_snmp_value(&cyan_toner_max_level_oid, params)?,
                level_percent: None,
            })
        } else {
            None
        };

        let magenta_toner =
            if !magenta_toner_level_oid.is_empty() && !magenta_toner_max_level_oid.is_empty() {
                Some(Toner {
                    level: get_snmp_value(&magenta_toner_level_oid, params)?,
                    max_level: get_snmp_value(&magenta_toner_max_level_oid, params)?,
                    level_percent: None,
                })
            } else {
                None
            };

        let yellow_toner =
            if !yellow_toner_level_oid.is_empty() && !yellow_toner_max_level_oid.is_empty() {
                Some(Toner {
                    level: get_snmp_value(&yellow_toner_level_oid, params)?,
                    max_level: get_snmp_value(&yellow_toner_max_level_oid, params)?,
                    level_percent: None,
                })
            } else {
                None
            };

        let black_drum = if !black_drum_level_oid.is_empty() && !black_drum_max_level_oid.is_empty()
        {
            Some(Drum {
                level: get_snmp_value(&black_drum_level_oid, params)?,
                max_level: get_snmp_value(&black_drum_max_level_oid, params)?,
                level_percent: None,
            })
        } else {
            None
        };

        let cyan_drum = if !cyan_drum_level_oid.is_empty() && !cyan_drum_max_level_oid.is_empty() {
            Some(Drum {
                level: get_snmp_value(&cyan_drum_level_oid, params)?,
                max_level: get_snmp_value(&cyan_drum_max_level_oid, params)?,
                level_percent: None,
            })
        } else {
            None
        };

        let magenta_drum =
            if !magenta_drum_level_oid.is_empty() && !magenta_drum_max_level_oid.is_empty() {
                Some(Drum {
                    level: get_snmp_value(&magenta_drum_level_oid, params)?,
                    max_level: get_snmp_value(&magenta_drum_max_level_oid, params)?,
                    level_percent: None,
                })
            } else {
                None
            };

        let yellow_drum =
            if !yellow_drum_level_oid.is_empty() && !yellow_drum_max_level_oid.is_empty() {
                Some(Drum {
                    level: get_snmp_value(&yellow_drum_level_oid, params)?,
                    max_level: get_snmp_value(&yellow_drum_max_level_oid, params)?,
                    level_percent: None,
                })
            } else {
                None
            };

        let fuser = if !fuser_level_oid.is_empty() && !fuser_max_level_oid.is_empty() {
            Some(Fuser {
                level: get_snmp_value(&fuser_level_oid, params)?,
                max_level: get_snmp_value(&fuser_max_level_oid, params)?,
                level_percent: None,
            })
        } else {
            None
        };

        let reservoir = if !reservoir_level_oid.is_empty() && !reservoir_max_level_oid.is_empty() {
            Some(Reservoir {
                level: get_snmp_value(&reservoir_level_oid, params)?,
                max_level: get_snmp_value(&reservoir_max_level_oid, params)?,
                level_percent: None,
            })
        } else {
            None
        };

        let toners = Toners {
            black_toner,
            cyan_toner,
            magenta_toner,
            yellow_toner,
        };

        let drums = Drums {
            black_drum,
            cyan_drum,
            magenta_drum,
            yellow_drum,
        };

        let mut printer = Printer::new(
            printer_name.to_string().clone(),
            serial_number,
            toners,
            drums,
            fuser,
            reservoir,
        );

        printer.calc_and_update_toners_level_percent();
        printer.calc_and_update_drums_level_percent();
        printer.calc_and_update_fuser_level_percent();
        printer.calc_and_update_reservoir_level_percent();

        Ok(printer)
    }
}
