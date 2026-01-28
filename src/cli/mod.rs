use crate::{
    cli::output::OutputFormat,
    error::{AppError, ErrorKind},
    printer::Printer,
    snmp::{version::SnmpVersion, SnmpClientParams},
};
use clap::Parser;
use colored::Colorize;
use progress::show_progress;
use std::{
    net::{Ipv4Addr, ToSocketAddrs},
    path::PathBuf,
};
use theme::CliTheme;

mod output;
mod progress;
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
    #[arg(short, long, default_value_t = 161)]
    port: u16,

    /// SNMP Version
    #[arg(short, long, default_value_t = SnmpVersion::V1)]
    snmp_version: SnmpVersion,

    /// SNMP Community
    #[arg(short, long, default_value = "public")]
    community: String,

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

    let resolved_ip = if let Ok(ip) = args.host.parse::<Ipv4Addr>() {
        ip
    } else {
        let host_with_port = format!("{}:{}", args.host, args.port);
        let mut addrs_iter = match host_with_port.to_socket_addrs() {
            Ok(addrs) => addrs,
            Err(_) => return Err(AppError::new(ErrorKind::DnsResolution(args.host))),
        };

        addrs_iter
            .find_map(|socket_addr| {
                if let std::net::IpAddr::V4(ipv4_addr) = socket_addr.ip() {
                    Some(ipv4_addr)
                } else {
                    None
                }
            })
            .ok_or_else(|| AppError::new(ErrorKind::DnsResolution(args.host)))?
    };

    let params = AppParams {
        app: CliParams {
            theme: args.theme,
            output: args.output,
        },
        snmp: SnmpClientParams {
            ip: resolved_ip,
            port: args.port,
            community: args.community,
            version: args.snmp_version,
            timeout: args.timeout,
            data_dir: args.data_dir,
            extra_supplies: args.extra_supplies,
            metrics: args.metrics,
        },
    };

    Ok(params)
}

/// Display the formatted values.
pub fn show_printer_values(
    printer: Printer,
    extra_supplies: bool,
    metrics: bool,
    theme: &CliTheme,
    output: &OutputFormat,
) {
    match output {
        OutputFormat::Json => match serde_json::to_string_pretty(&printer) {
            Ok(json) => println!("{json}"),
            Err(e) => eprintln!("Error generating JSON output: {e}"),
        },

        OutputFormat::Text => {
            if extra_supplies {
                if let Some(serial_number) = printer.serial_number {
                    println!("{} {}", "Printer:".bright_cyan().bold(), printer.name);
                    println!("{} {serial_number}\n", "Serial:".bright_cyan().bold());
                }
            } else {
                println!("{} {}\n", "Printer:".bright_cyan().bold(), printer.name);
            }

            println!("--> {}\n", "Toner:".bright_white().bold());

            if let Some(level) = printer
                .toners
                .black_toner
                .as_ref()
                .and_then(|t| t.level_percent)
            {
                show_progress("Black".bright_white(), level as u8, "white", theme);
            }

            if let Some(level) = printer
                .toners
                .cyan_toner
                .as_ref()
                .and_then(|t| t.level_percent)
            {
                show_progress("Cyan".bright_cyan(), level as u8, "cyan", theme);
            }

            if let Some(level) = printer
                .toners
                .magenta_toner
                .as_ref()
                .and_then(|t| t.level_percent)
            {
                show_progress("Magenta".bright_magenta(), level as u8, "magenta", theme);
            }

            if let Some(level) = printer
                .toners
                .yellow_toner
                .as_ref()
                .and_then(|t| t.level_percent)
            {
                show_progress("Yellow".bright_yellow(), level as u8, "yellow", theme);
            }

            if extra_supplies {
                if let Some(level) = printer
                    .drums
                    .black_drum
                    .as_ref()
                    .and_then(|t| t.level_percent)
                {
                    println!("\n\n--> {}\n", "Drum:".bright_white().bold());
                    show_progress("Black".bright_white(), level as u8, "white", theme);
                }

                if let Some(level) = printer
                    .drums
                    .cyan_drum
                    .as_ref()
                    .and_then(|t| t.level_percent)
                {
                    show_progress("Cyan".bright_cyan(), level as u8, "cyan", theme);
                }

                if let Some(level) = printer
                    .drums
                    .magenta_drum
                    .as_ref()
                    .and_then(|t| t.level_percent)
                {
                    show_progress("Magenta".bright_magenta(), level as u8, "magenta", theme);
                }

                if let Some(level) = printer
                    .drums
                    .yellow_drum
                    .as_ref()
                    .and_then(|t| t.level_percent)
                {
                    show_progress("Yellow".bright_yellow(), level as u8, "yellow", theme);
                }

                if let Some(level) = printer.fuser.as_ref().and_then(|t| t.level_percent) {
                    println!("\n\n--> {}\n", "Other:".bright_white().bold());
                    show_progress("Fuser".white(), level as u8, "white", theme);
                }

                if let Some(level) = printer.reservoir.as_ref().and_then(|t| t.level_percent) {
                    let color = if level as u8 == 100 { "green" } else { "red" };
                    show_progress("Reservoir".white(), level as u8, color, theme);
                }
            }

            if metrics {
                if let Some(total_impressions) =
                    printer.metrics.as_ref().and_then(|m| m.total_impressions)
                {
                    println!("\n\n--> {}", "Metrics:".bright_white().bold());
                    print!(
                        "\n{} {total_impressions} pages",
                        "Total impressions:".bright_cyan().bold()
                    );
                }
                if let Some(mono_impressions) =
                    printer.metrics.as_ref().and_then(|m| m.mono_impressions)
                {
                    print!(
                        "\n{} {mono_impressions} pages",
                        "Mono: ".bright_cyan().bold()
                    );
                }
                if let Some(color_impressions) =
                    printer.metrics.as_ref().and_then(|m| m.color_impressions)
                {
                    print!(
                        "\n{} {color_impressions} pages",
                        "Color:".bright_cyan().bold()
                    );
                }
            }

            println!();
        }
    }
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
