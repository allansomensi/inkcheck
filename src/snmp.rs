use crate::{
    error::AppError,
    printer::{load_printer, Drum, Drums, Printer, PrinterSupply, Toner, TonerColor, Toners},
    special,
    utils::parse_oid_to_vec,
};
use clap::ValueEnum;
use snmp2::{Oid, SyncSession, Value};
use std::{fmt::Display, net::Ipv4Addr, path::PathBuf, time::Duration};

/// Represents the different versions of the SNMP.
#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum SnmpVersion {
    V1,
    V2c,
    V3,
}

impl Display for SnmpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1 => write!(f, "v1"),
            Self::V2c => write!(f, "v2c"),
            Self::V3 => write!(f, "v3"),
        }
    }
}

impl Default for SnmpVersion {
    fn default() -> Self {
        Self::V2c
    }
}

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

/// A trait for converting SNMP `Value` types into Rust types.
///
/// This trait defines a method to convert a given SNMP `Value` to a specific Rust type. It allows SNMP data
/// to be easily mapped to the appropriate types in the application.
///
/// The `from_snmp_value` method converts the SNMP `Value` to the implementing type. If the conversion is not possible,
/// it returns an error message.
///
/// ## Implementations
/// The following types implement [FromSnmpValue]:
/// - [i64]: Converts from an SNMP [Value::Integer] value.
/// - [String]: Converts from an SNMP [Value::OctetString] value.
/// - [Vec<u64>]: Converts from an SNMP [Value::ObjectIdentifier] value, splitting the OID string into individual components.
/// - [Vec<u8>]: Converts from an SNMP [Value::OctetString] bytes.
/// - [u32]: Converts from an SNMP [Value::Unsigned32], [Value::Counter32], or [Value::Timeticks] value.
/// - [u64]: Converts from an SNMP [Value::Counter64] value.
/// - [bool]: Converts from an SNMP [Value::Boolean] value.
pub trait FromSnmpValue<'a>: Sized {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError>;
}

impl<'a> FromSnmpValue<'a> for i64 {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::Integer(v) = value {
            Ok(*v)
        } else {
            Err(AppError::TypeMismatch(
                "Expected Integer, but received a different type".to_string(),
            ))
        }
    }
}

impl<'a> FromSnmpValue<'a> for String {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::OctetString(v) = value {
            Ok(String::from_utf8_lossy(v).to_string())
        } else {
            Err(AppError::TypeMismatch(
                "Expected OctetString, but received a different type".to_string(),
            ))
        }
    }
}

impl<'a> FromSnmpValue<'a> for Vec<u64> {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::ObjectIdentifier(v) = value {
            let oid_string = v.to_string();
            oid_string
                .split('.')
                .map(|s| {
                    s.parse::<u64>()
                        .map_err(|_| AppError::ParseError(format!("Failed to parse '{s}' as u64")))
                })
                .collect()
        } else {
            Err(AppError::TypeMismatch(
                "Expected ObjectIdentifier, but received a different type".to_string(),
            ))
        }
    }
}

impl<'a> FromSnmpValue<'a> for Vec<u8> {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::OctetString(v) = value {
            Ok(v.to_vec())
        } else {
            Err(AppError::TypeMismatch(
                "Expected OctetString, but received a different type".to_string(),
            ))
        }
    }
}

impl<'a> FromSnmpValue<'a> for u32 {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        match value {
            Value::Unsigned32(v) | Value::Counter32(v) | Value::Timeticks(v) => Ok(*v),
            _ => Err(AppError::TypeMismatch(
                "Expected Unsigned32, Counter32, or Timeticks, but received a different type"
                    .to_string(),
            )),
        }
    }
}

impl<'a> FromSnmpValue<'a> for u64 {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::Counter64(v) = value {
            Ok(*v)
        } else {
            Err(AppError::TypeMismatch(
                "Expected Counter64, but received a different type".to_string(),
            ))
        }
    }
}

impl<'a> FromSnmpValue<'a> for bool {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::Boolean(v) = value {
            Ok(*v)
        } else {
            Err(AppError::TypeMismatch(
                "Expected Boolean, but received a different type".to_string(),
            ))
        }
    }
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

/// Retrieves an SNMP value for a given OID and converts it to the specified type.
///
/// This function performs an SNMP `GET` operation for a specified OID and converts the resulting SNMP value
/// into the desired Rust type using the [FromSnmpValue] trait. It creates an SNMP session using the provided
/// [SnmpClientParams] and returns the value as the requested type.
///
/// ## Arguments:
/// - `oid`: A slice of `u64` representing the OID to retrieve from the SNMP device.
/// - `ctx`: A reference to [SnmpClientParams] containing the SNMP client parameters.
///
/// ## Returns:
/// - `Result<T, String>`: Returns the SNMP value converted to type `T` if successful, or an error message if the operation fails.
///
/// ## Type Constraints:
/// - `T`: The target type, which must implement the [FromSnmpValue] trait. This allows the conversion of the SNMP value
///        into any type that supports this trait.
pub fn get_snmp_value<T>(oid: &[u64], ctx: &SnmpClientParams) -> Result<T, AppError>
where
    T: for<'a> FromSnmpValue<'a>,
{
    let mut session = match create_snmp_session(ctx) {
        Ok(session) => session,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let oid = Oid::from(oid).map_err(|_| AppError::OidConversion)?;

    let mut response = session
        .get(&oid)
        .map_err(|e| AppError::SnmpRequest(e.to_string()))?;

    if let Some((_oid, value)) = response.varbinds.next() {
        Ok(T::from_snmp_value(&value)?)
    } else {
        Err(AppError::OidNotFound)
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
        return special::brother(params, model.clone());
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
    let get_supply_oid = |supply: PrinterSupply, color: TonerColor, key: &str| {
        oids.get(supply.to_string().to_lowercase())
            .and_then(|t| t.get(color.to_string().to_lowercase()))
            .and_then(|b| b.get(key))
            .and_then(|l| l.as_str())
            .map(parse_oid_to_vec)
            .ok_or_else(|| AppError::OidNotFound)?
    };

    //
    // OIDs
    //

    // Toners
    let black_toner_level_oid = get_supply_oid(PrinterSupply::Toner, TonerColor::Black, "level")?;
    let black_toner_max_level_oid =
        get_supply_oid(PrinterSupply::Toner, TonerColor::Black, "max_level")?;

    let cyan_toner_level_oid = get_supply_oid(PrinterSupply::Toner, TonerColor::Cyan, "level")?;
    let cyan_toner_max_level_oid =
        get_supply_oid(PrinterSupply::Toner, TonerColor::Cyan, "max_level")?;

    let magenta_toner_level_oid =
        get_supply_oid(PrinterSupply::Toner, TonerColor::Magenta, "level")?;
    let magenta_toner_max_level_oid =
        get_supply_oid(PrinterSupply::Toner, TonerColor::Magenta, "max_level")?;

    let yellow_toner_level_oid = get_supply_oid(PrinterSupply::Toner, TonerColor::Yellow, "level")?;
    let yellow_toner_max_level_oid =
        get_supply_oid(PrinterSupply::Toner, TonerColor::Yellow, "max_level")?;

    // Drums
    let mut black_drum_level_oid: Vec<u64> = Vec::new();
    let mut black_drum_max_level_oid: Vec<u64> = Vec::new();

    let mut cyan_drum_level_oid: Vec<u64> = Vec::new();
    let mut cyan_drum_max_level_oid: Vec<u64> = Vec::new();

    let mut magenta_drum_level_oid: Vec<u64> = Vec::new();
    let mut magenta_drum_max_level_oid: Vec<u64> = Vec::new();

    let mut yellow_drum_level_oid: Vec<u64> = Vec::new();
    let mut yellow_drum_max_level_oid: Vec<u64> = Vec::new();

    if params.extra_supplies {
        black_drum_level_oid = get_supply_oid(PrinterSupply::Drum, TonerColor::Black, "level")?;
        black_drum_max_level_oid =
            get_supply_oid(PrinterSupply::Drum, TonerColor::Black, "max_level")?;

        cyan_drum_level_oid = get_supply_oid(PrinterSupply::Drum, TonerColor::Cyan, "level")?;
        cyan_drum_max_level_oid =
            get_supply_oid(PrinterSupply::Drum, TonerColor::Cyan, "max_level")?;

        magenta_drum_level_oid = get_supply_oid(PrinterSupply::Drum, TonerColor::Magenta, "level")?;
        magenta_drum_max_level_oid =
            get_supply_oid(PrinterSupply::Drum, TonerColor::Magenta, "max_level")?;

        yellow_drum_level_oid = get_supply_oid(PrinterSupply::Drum, TonerColor::Yellow, "level")?;
        yellow_drum_max_level_oid =
            get_supply_oid(PrinterSupply::Drum, TonerColor::Yellow, "max_level")?;
    }

    //
    // Values
    //

    let black_toner = Toner {
        level: get_snmp_value(&black_toner_level_oid, params)?,
        max_level: get_snmp_value(&black_toner_max_level_oid, params)?,
        level_percent: None,
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

    let mut printer = Printer::new(name.clone(), toners, drums);

    printer.calc_and_update_toners_level_percent();
    printer.calc_and_update_drums_level_percent();

    Ok(printer)
}
