use crate::{
    cli::args::Args,
    error::{AppError, ErrorKind},
    printer::{driver::DriverManager, Printer},
    snmp::security::{AuthProtocol, PrivacyProtocol, SecurityLevel},
};
use snmp2::{
    v3::{self},
    AsyncSession,
};
use std::{net::Ipv4Addr, path::PathBuf};
use value::get_snmp_value;
use version::SnmpVersion;

pub mod security;
pub mod value;
pub mod version;

/// Encapsulates all necessary parameters to establish an SNMP connection.
#[derive(Clone, Debug)]
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
    pub context_name: String,
    pub version: SnmpVersion,
    pub data_dir: Option<PathBuf>,
    pub extra_supplies: bool,
    pub metrics: bool,
}

impl SnmpClientParams {
    /// Constructs client parameters from parsed CLI arguments and resolved IP.
    pub fn from_args(args: &Args, ip: Ipv4Addr) -> Self {
        Self {
            ip,
            port: args.port,
            community: args.community.clone(),
            username: args.username.clone(),
            auth_password: args.auth_password.clone(),
            auth_protocol: args.auth_protocol,
            privacy_password: args.privacy_password.clone(),
            privacy_protocol: args.privacy_protocol,
            security_level: args.security_level,
            context_name: args.context_name.clone(),
            version: args.snmp_version,
            data_dir: args.data_dir.clone(),
            extra_supplies: args.extra_supplies,
            metrics: args.metrics,
        }
    }
}

/// Establishes an asynchronous SNMP session based on version.
pub async fn create_snmp_session(ctx: &SnmpClientParams) -> Result<AsyncSession, AppError> {
    let agent_address = format!("{}:{}", ctx.ip, ctx.port);

    match ctx.version {
        SnmpVersion::V1 => AsyncSession::new_v1(agent_address, ctx.community.as_bytes(), 0)
            .await
            .map_err(AppError::from),

        SnmpVersion::V2c => AsyncSession::new_v2c(agent_address, ctx.community.as_bytes(), 0)
            .await
            .map_err(AppError::from),

        SnmpVersion::V3 => build_v3_session(ctx, agent_address).await,
    }
}

/// Helper function to construct and initialize an SNMPv3 session.
async fn build_v3_session(
    ctx: &SnmpClientParams,
    address: String,
) -> Result<AsyncSession, AppError> {
    let username = ctx
        .username
        .as_deref()
        .ok_or_else(|| AppError::new(ErrorKind::Cli("SNMPv3 requires a username.".into())))?;

    if username.is_empty() {
        return Err(AppError::new(ErrorKind::Cli(
            "Username cannot be empty.".into(),
        )));
    }

    let security = match ctx.security_level {
        SecurityLevel::NoAuthNoPriv => v3::Security::new(username.as_bytes(), &[])
            .with_auth(v3::Auth::NoAuthNoPriv)
            .with_context_name(&ctx.context_name),
        SecurityLevel::AuthNoPriv => {
            let pass = ctx.auth_password.as_deref().ok_or_else(|| {
                AppError::new(ErrorKind::Cli(
                    "AuthNoPriv requires an authentication password.".into(),
                ))
            })?;
            v3::Security::new(username.as_bytes(), pass.as_bytes())
                .with_auth_protocol(ctx.auth_protocol.into())
                .with_auth(v3::Auth::AuthNoPriv)
                .with_context_name(&ctx.context_name)
        }
        SecurityLevel::AuthPriv => {
            let auth_pass = ctx.auth_password.as_deref().ok_or_else(|| {
                AppError::new(ErrorKind::Cli(
                    "AuthPriv requires an authentication password.".into(),
                ))
            })?;
            let priv_pass = ctx.privacy_password.as_deref().ok_or_else(|| {
                AppError::new(ErrorKind::Cli(
                    "AuthPriv requires a privacy password.".into(),
                ))
            })?;

            v3::Security::new(username.as_bytes(), auth_pass.as_bytes())
                .with_auth_protocol(ctx.auth_protocol.into())
                .with_auth(v3::Auth::AuthPriv {
                    cipher: ctx.privacy_protocol.into(),
                    privacy_password: priv_pass.as_bytes().to_vec(),
                })
                .with_context_name(&ctx.context_name)
        }
    };

    let mut session = AsyncSession::new_v3(address, 0, security)
        .await
        .map_err(AppError::from)?;

    // Perform Engine ID discovery
    session.init().await.map_err(|e| {
        AppError::new(ErrorKind::SnmpRequest(format!(
            "SNMPv3 discovery failed: {e:?}"
        )))
    })?;

    Ok(session)
}

/// Retrieves the printer name (hrDeviceDescr) via SNMP.
pub async fn get_printer_name(ctx: &SnmpClientParams) -> Result<String, AppError> {
    const HR_DEVICE_DESCR_OID: &[u64] = &[1, 3, 6, 1, 2, 1, 25, 3, 2, 1, 3, 1];
    get_snmp_value::<String>(HR_DEVICE_DESCR_OID, ctx).await
}

/// Orchestrates driver selection and data fetching.
pub async fn get_printer_values(params: &SnmpClientParams) -> Result<Printer, AppError> {
    let printer_name = get_printer_name(params).await?;
    let manager = DriverManager::new();

    match manager.get_driver(&printer_name) {
        Some(driver) => driver.get_supplies(params, &printer_name).await,
        None => Err(AppError::new(ErrorKind::UnsupportedPrinter(printer_name))),
    }
}
