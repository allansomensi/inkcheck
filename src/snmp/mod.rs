use crate::{
    error::{AppError, ErrorKind},
    printer::{driver::DriverManager, Printer},
    snmp::security::{AuthProtocol, PrivacyProtocol, SecurityLevel},
};
use snmp2::{
    v3::{self},
    SyncSession,
};
use std::{
    net::{Ipv4Addr, UdpSocket},
    path::PathBuf,
    time::Duration,
};
use value::get_snmp_value;
use version::SnmpVersion;

pub mod security;
pub mod value;
pub mod version;

/// Represents the parameters for an SNMP client connection.
///
/// This struct contains the necessary information for configuring an SNMP client to communicate with an device.
#[derive(Clone)]
pub struct SnmpClientParams {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub community: String,
    pub username: Option<String>,
    pub auth_password: Option<String>,
    pub auth_protocol: AuthProtocol,
    pub privacy_password: Option<String>,
    pub privacy_protocol: PrivacyProtocol,
    pub security_level: SecurityLevel,
    pub version: SnmpVersion,
    pub timeout: u64,
    pub data_dir: Option<PathBuf>,
    pub extra_supplies: bool,
    pub metrics: bool,
}

/// Creates a synchronous SNMP session based on the provided client parameters.
///
/// Handles version-specific initialization for SNMPv1, v2c, and v3. For v3, it performs
/// automatic Engine ID discovery and validates required credentials.
pub fn create_snmp_session(ctx: &SnmpClientParams) -> Result<SyncSession, AppError> {
    let agent_address = format!("{}:{}", ctx.ip, ctx.port);
    let timeout = Duration::from_secs(ctx.timeout);

    match ctx.version {
        SnmpVersion::V1 => {
            let community = ctx.community.as_bytes();
            SyncSession::new_v1(agent_address, community, Some(timeout), 0).map_err(AppError::from)
        }
        SnmpVersion::V2c => {
            let community = ctx.community.as_bytes();
            SyncSession::new_v2c(agent_address, community, Some(timeout), 0).map_err(AppError::from)
        }
        SnmpVersion::V3 => {
            let username = ctx.username.as_deref().ok_or_else(|| {
                AppError::new(ErrorKind::SnmpRequest(
                    "SNMPv3 requires a username (security name).".into(),
                ))
            })?;

            if username.is_empty() {
                return Err(AppError::new(ErrorKind::SnmpRequest(
                    "The username cannot be empty.".into(),
                )));
            }

            // Discover the Engine ID
            let engine_id = discover_engine_id(&ctx.ip.to_string(), ctx.port).map_err(|e| {
                AppError::new(ErrorKind::SnmpRequest(format!(
                    "Engine ID discovery failed: {e:?}"
                )))
            })?;

            // Build security context based on the user-selected security level
            let security = match ctx.security_level {
                SecurityLevel::NoAuthNoPriv => {
                    // Username only, no passwords.
                    v3::Security::new(username.as_bytes(), &[])
                        .with_auth(v3::Auth::NoAuthNoPriv)
                        .with_engine_id(&engine_id)
                        .map_err(|e| {
                            AppError::new(ErrorKind::SnmpRequest(format!(
                                "Error configuring NoAuthNoPriv: {e:?}"
                            )))
                        })?
                }
                SecurityLevel::AuthNoPriv => {
                    // Requires Username and Auth Password
                    let auth_password = ctx.auth_password.as_deref().ok_or_else(|| {
                        AppError::new(ErrorKind::SnmpRequest(
                            "The AuthNoPriv level requires an authentication password (auth_password)."
                                .into(),
                        ))
                    })?;

                    v3::Security::new(username.as_bytes(), auth_password.as_bytes())
                        .with_auth_protocol(ctx.auth_protocol.into())
                        .with_auth(v3::Auth::AuthNoPriv)
                        .with_engine_id(&engine_id)
                        .map_err(|e| {
                            AppError::new(ErrorKind::SnmpRequest(format!(
                                "Error configuring AuthNoPriv: {e:?}"
                            )))
                        })?
                }
                SecurityLevel::AuthPriv => {
                    // Requires everything: Username, Auth Password, and Privacy Password
                    let auth_password = ctx.auth_password.as_deref().ok_or_else(|| {
                        AppError::new(ErrorKind::SnmpRequest(
                            "The AuthPriv level requires an authentication password (auth_password)."
                                .into(),
                        ))
                    })?;

                    let privacy_password = ctx.privacy_password.as_deref().ok_or_else(|| {
                        AppError::new(ErrorKind::SnmpRequest(
                            "The AuthPriv level requires a privacy password (privacy_password)."
                                .into(),
                        ))
                    })?;

                    v3::Security::new(username.as_bytes(), auth_password.as_bytes())
                        .with_auth_protocol(ctx.auth_protocol.into())
                        .with_auth(v3::Auth::AuthPriv {
                            cipher: ctx.privacy_protocol.into(),
                            privacy_password: privacy_password.as_bytes().to_vec(),
                        })
                        .with_engine_id(&engine_id)
                        .map_err(|e| {
                            AppError::new(ErrorKind::SnmpRequest(format!(
                                "Error configuring AuthPriv: {e:?}"
                            )))
                        })?
                }
            };

            // Create the session
            SyncSession::new_v3(agent_address, Some(timeout), 0, security).map_err(AppError::from)
        }
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

/// Identifies the printer via SNMP and retrieves supply levels using the corresponding driver.
///
/// Returns [`AppError`] with [`ErrorKind::UnsupportedPrinter`] if no driver matches the retrieved device name.
pub fn get_printer_values(params: &SnmpClientParams) -> Result<Printer, AppError> {
    let printer_name = get_printer_name(params)?;

    let driver_manager = DriverManager::new();

    if let Some(driver) = driver_manager.get_driver(&printer_name) {
        driver.get_supplies(params, &printer_name)
    } else {
        Err(AppError::new(ErrorKind::UnsupportedPrinter(printer_name)))
    }
}

/// Sends a raw SNMPv3 probe packet to the target to discover the authoritative Engine ID.
///
/// This function handles the low-level UDP communication and parses the response to extract
/// the Engine ID, which is required for initializing an encrypted SNMPv3 session.
pub fn discover_engine_id(ip: &str, port: u16) -> Result<Vec<u8>, AppError> {
    let target = format!("{ip}:{port}");

    let probe_packet: [u8; 64] = [
        0x30, 0x3e, 0x02, 0x01, 0x03, 0x30, 0x11, 0x02, 0x04, 0x7b, 0x00, 0x00, 0x01, 0x02, 0x03,
        0x00, 0xff, 0xe0, 0x04, 0x01, 0x04, 0x02, 0x01, 0x03, 0x04, 0x10, 0x30, 0x0e, 0x04, 0x00,
        0x02, 0x01, 0x00, 0x02, 0x01, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x30, 0x14, 0x04,
        0x00, 0x04, 0x00, 0xa0, 0x0c, 0x02, 0x04, 0x7b, 0x00, 0x00, 0x01, 0x02, 0x01, 0x00, 0x02,
        0x01, 0x00, 0x30, 0x00,
    ];

    let socket = UdpSocket::bind("0.0.0.0:0")
        .map_err(|e| AppError::new(ErrorKind::SnmpRequest(format!("Socket bind error: {e}"))))?;
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    socket.send_to(&probe_packet, &target).map_err(|e| {
        AppError::new(ErrorKind::SnmpRequest(format!(
            "Failed to send discovery probe: {e}"
        )))
    })?;

    let mut buf = [0u8; 1024];
    let (amt, _) = socket.recv_from(&mut buf).map_err(|_| {
        AppError::new(ErrorKind::SnmpRequest(
            "Timeout: Target did not respond to SNMPv3 discovery probe".to_string(),
        ))
    })?;

    let response = &buf[..amt];

    // Heuristic parser to find the EngineID in the raw response
    let mut i = 6;
    while i < response.len() - 5 {
        if response[i] == 0x04 && response[i + 2] == 0x30 && response[i + 4] == 0x04 {
            let id_len = response[i + 5] as usize;
            if i + 6 + id_len <= response.len() {
                let engine_id = response[i + 6..i + 6 + id_len].to_vec();
                if engine_id.len() > 4 && engine_id[0] == 0x80 {
                    return Ok(engine_id);
                }
            }
        }
        i += 1;
    }

    Err(AppError::new(ErrorKind::SnmpRequest(
        "Failed to extract Engine ID from the response payload.".to_string(),
    )))
}
