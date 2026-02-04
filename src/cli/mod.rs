use crate::{
    cli::output::OutputFormat,
    error::{AppError, ErrorKind},
    snmp::SnmpClientParams,
};
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs};
use theme::CliTheme;

pub mod args;
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
    use crate::cli::args::Args;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
