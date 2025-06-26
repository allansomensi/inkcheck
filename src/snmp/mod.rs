use crate::{
    error::{AppError, ErrorKind},
    printer::{driver::DriverManager, Printer},
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

/// Creates an SNMP session based on the provided client parameters.
///
/// Supports SNMP versions v1 and v2c.
///
/// ## Parameters
/// - `ctx`: Reference to [`SnmpClientParams`] containing connection details like IP, port, version, community, and timeout.
///
/// ## Returns
/// - `Ok(SyncSession)`: A configured SNMP session ready for queries.
/// - `Err(AppError)`: If the session creation fails or the SNMP version is unsupported.
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
        SnmpVersion::V3 => Err(AppError::new(ErrorKind::UnsupportedVersion)),
    }
}

/// Retrieves the printer name using SNMP.
///
/// This function performs an SNMP `GET` operation on the OID that corresponds to the printer's name and returns it as a `String`.
/// It uses the [`get_snmp_value`] function to fetch the value and convert it to a `String`.
pub fn get_printer_name(ctx: &SnmpClientParams) -> Result<String, AppError> {
    let hr_device_descr_oid = &[1, 3, 6, 1, 2, 1, 25, 3, 2, 1, 3, 1];

    get_snmp_value::<String>(hr_device_descr_oid, ctx)
}

/// Fetches printer supply levels using the appropriate driver.
///
/// This function first retrieves the printer's model name via SNMP,
/// then uses [`DriverManager`] to select a compatible driver.
/// The selected driver handles the actual retrieval of supply data.
///
/// ## Parameters
/// - `params`: SNMP connection parameters.
///
/// ## Returns
/// - `Ok(Printer)`: A struct containing printer details and supply levels.
/// - `Err(AppError)`: If the printer is unsupported or an SNMP error occurs.
pub fn get_printer_values(params: &SnmpClientParams) -> Result<Printer, AppError> {
    let printer_name = get_printer_name(params)?;

    let driver_manager = DriverManager::new();

    if let Some(driver) = driver_manager.get_driver(&printer_name) {
        driver.get_supplies(params, &printer_name)
    } else {
        Err(AppError::new(ErrorKind::UnsupportedPrinter(printer_name)))
    }
}
