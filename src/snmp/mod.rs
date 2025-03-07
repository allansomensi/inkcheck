use crate::{
    error::AppError,
    printer::{
        load::load_printer,
        supply::{Drum, Drums, Fuser, PrinterSupply, Reservoir, Toner, TonerColor, Toners},
        Printer,
    },
    special,
    utils::parse_oid_to_vec,
};
use snmp2::SyncSession;
use std::{net::Ipv4Addr, path::PathBuf, time::Duration};
use value::get_snmp_value;
use version::SnmpVersion;

pub mod value;
pub mod version;

/// Represents the parameters for an SNMP client connection.
///
/// This struct contains the necessary information for configuring an SNMP client to communicate with an device.
pub struct SnmpClientParams {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub community: String,
    pub version: SnmpVersion,
    pub timeout: u64,
    pub data_dir: Option<PathBuf>,
    pub extra_supplies: bool,
}

pub fn create_snmp_session(ctx: &SnmpClientParams) -> Result<SyncSession, AppError> {
    let agent_address = format!("{}:{}", ctx.ip, ctx.port);
    let community = ctx.community.as_bytes();
    let timeout = Duration::from_secs(ctx.timeout);

    match ctx.version {
        SnmpVersion::V1 => {
            SyncSession::new_v1(agent_address, community, Some(timeout), 0).map_err(AppError::from)
        }
        SnmpVersion::V2c => {
            SyncSession::new_v2c(agent_address, community, Some(timeout), 0).map_err(AppError::from)
        }
        SnmpVersion::V3 => Err(AppError::UnsupportedVersion),
    }
}

/// Retrieves the printer name using SNMP.
///
/// This function performs an SNMP `GET` operation on the OID that corresponds to the printer's name and returns it as a `String`.
/// It uses the `get_snmp_value` function to fetch the value and convert it to a `String`.
pub fn get_printer_name(ctx: &SnmpClientParams) -> Result<String, AppError> {
    let hr_device_descr_oid = &[1, 3, 6, 1, 2, 1, 25, 3, 2, 1, 3, 1];

    get_snmp_value::<String>(hr_device_descr_oid, ctx)
}

/// Retrieves and computes the toner levels and other printer details via SNMP.
///
/// This function fetches printer information, including toner levels and other parameters, using SNMP. It queries
/// specific OIDs for each toner color (black, cyan, magenta, and yellow) and returns a [Printer] struct populated
/// with the printer's name, brand, model, toner levels, and toner percentages.
///
/// The function assumes the presence of a JSON configuration file with printer-specific OID mappings for toners and other values.
/// If the configuration for the printer model is not found, an error is returned.
///
/// ## Arguments:
/// - `params`: A reference to [SnmpClientParams] containing the SNMP client parameters.
///
/// ## Returns:
/// - A `Printer` struct containing the printer's details such as name, brand, model, toner levels, and toner percentages.
pub fn get_printer_values(params: &SnmpClientParams) -> Result<Printer, AppError> {
    let name = get_printer_name(params)?;

    let brand = name
        .split_whitespace()
        .next()
        .expect("Error fetching printer brand");

    let model = &name;

    // Special handling for Brother printers
    if model.to_lowercase().contains("brother") {
        // Special handling for old models
        let old_model = model.contains("HL-5350DN");

        return special::brother::get_supplies_levels(params, model.clone(), old_model);
    }

    let data_dir = params.data_dir.clone();

    let oids = match load_printer(brand, model, data_dir) {
        Ok(oids) => Ok(oids),
        Err(e) => Err(e),
    }?;

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

    if params.extra_supplies {
        black_drum_level_oid =
            get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Black), "level")?;
        black_drum_max_level_oid =
            get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Black), "max_level")?;

        cyan_drum_level_oid = get_supply_oid(PrinterSupply::Drum, Some(TonerColor::Cyan), "level")?;
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

    let black_toner = if !black_toner_level_oid.is_empty() && !black_toner_max_level_oid.is_empty()
    {
        Some(Toner {
            level: get_snmp_value(&black_toner_level_oid, params)?,
            max_level: get_snmp_value(&black_toner_max_level_oid, params)?,
            level_percent: None,
        })
    } else {
        None
    };

    let cyan_toner = if !cyan_toner_level_oid.is_empty() && !cyan_toner_max_level_oid.is_empty() {
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

    let black_drum = if !black_drum_level_oid.is_empty() && !black_drum_max_level_oid.is_empty() {
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

    let yellow_drum = if !yellow_drum_level_oid.is_empty() && !yellow_drum_max_level_oid.is_empty()
    {
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

    let mut printer = Printer::new(name.clone(), toners, drums, fuser, reservoir);

    printer.calc_and_update_toners_level_percent();
    printer.calc_and_update_drums_level_percent();
    printer.calc_and_update_fuser_level_percent();
    printer.calc_and_update_reservoir_level_percent();

    Ok(printer)
}
