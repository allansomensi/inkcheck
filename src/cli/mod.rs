use crate::{
    cli::output::OutputFormat,
    error::{AppError, ErrorKind},
    snmp::{
        auth::{cipher::AuthCipher, protocol::AuthProtocol},
        version::SnmpVersion,
        SnmpClientParams,
    },
};
use clap::Parser;
use std::{
    net::{IpAddr, Ipv4Addr, ToSocketAddrs},
    path::PathBuf,
};
use theme::CliTheme;

pub mod display;
mod output;
pub mod progress;
mod theme;

/// Structure that holds general parameters for the application.
///
/// This structure groups together the settings related to the application configuration,
/// including CLI and SNMP settings.
pub struct AppParams {
    pub app: CliParams,
    pub snmp: SnmpClientParams,
}

/// Structure that holds parameters for the command-line interface (CLI).
///
/// This structure defines the settings specific to the CLI, such as the theme to be used.
pub struct CliParams {
    pub theme: CliTheme,
    pub output: OutputFormat,
}

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// IP or hostname of the printer
    host: String,

    /// SNMP Service Port
    #[arg(long, default_value_t = 161)]
    port: u16,

    /// SNMP Version
    #[arg(short, long, default_value_t = SnmpVersion::V1)]
    snmp_version: SnmpVersion,

    /// SNMP Community
    #[arg(short, long, default_value = "public")]
    community: String,

    /// Username (v3)
    #[arg(short, long)]
    username: Option<String>,

    /// Password (v3)
    #[arg(short, long)]
    password: Option<String>,

    /// Auth Protocol (v3)
    #[arg(long, default_value_t = AuthProtocol::Sha1)]
    auth_protocol: AuthProtocol,

    /// Auth Cipher (v3)
    #[arg(long, default_value_t = AuthCipher::Aes128)]
    auth_cipher: AuthCipher,

    /// Timeout in seconds
    #[arg(short, long, default_value_t = 5)]
    timeout: u64,

    /// Data directory
    #[arg(short, long)]
    data_dir: Option<PathBuf>,

    /// Display levels of other supplies (drum, paper, etc.)
    #[arg(short, long)]
    extra_supplies: bool,

    /// Display metrics
    #[arg(short, long)]
    metrics: bool,

    /// Cli theme
    #[arg(long, default_value_t = CliTheme::Solid)]
    theme: CliTheme,

    /// Output format
    #[arg(long, short, default_value_t = OutputFormat::Text)]
    output: OutputFormat,
}

/// Capture and return the command line arguments.
pub fn parse_args() -> Result<AppParams, AppError> {
    let args = Args::parse();

    let resolved_ip = resolve_host(&args.host, args.port)?;

    Ok(AppParams {
        app: CliParams {
            theme: args.theme,
            output: args.output,
        },
        snmp: SnmpClientParams {
            ip: resolved_ip,
            port: args.port,
            username: args.username,
            password: args.password,
            auth_protocol: args.auth_protocol,
            auth_cipher: args.auth_cipher,
            community: args.community,
            version: args.snmp_version,
            timeout: args.timeout,
            data_dir: args.data_dir,
            extra_supplies: args.extra_supplies,
            metrics: args.metrics,
        },
    })
}

/// DNS resolver
fn resolve_host(host: &str, port: u16) -> Result<Ipv4Addr, AppError> {
    if let Ok(ip) = host.parse::<Ipv4Addr>() {
        return Ok(ip);
    }

    let host_with_port = format!("{host}:{port}");

    host_with_port
        .to_socket_addrs()
        .map_err(|_| AppError::new(ErrorKind::DnsResolution(host.to_string())))?
        .find_map(|addr| match addr.ip() {
            IpAddr::V4(ipv4) => Some(ipv4),
            _ => None,
        })
        .ok_or_else(|| AppError::new(ErrorKind::DnsResolution(host.to_string())))
}

#[cfg(test)]
mod tests {
    use crate::cli::Args;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
