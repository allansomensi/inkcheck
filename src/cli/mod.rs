use crate::{
    error::AppError,
    printer::Printer,
    snmp::{version::SnmpVersion, SnmpClientParams},
};
use clap::Parser;
use colored::Colorize;
use progress::show_progress;
use std::{net::Ipv4Addr, path::PathBuf};
use theme::CliTheme;

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
}

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// IP of the printer
    ip: Ipv4Addr,

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

    /// Cli theme
    #[arg(long, default_value_t = CliTheme::Solid)]
    theme: CliTheme,
}

/// Capture and return the command line arguments.
pub fn parse_args() -> Result<AppParams, AppError> {
    let args = Args::parse();

    let params = AppParams {
        app: CliParams { theme: args.theme },
        snmp: SnmpClientParams {
            ip: args.ip,
            port: args.port,
            community: args.community,
            version: args.snmp_version,
            timeout: args.timeout,
            data_dir: args.data_dir,
            extra_supplies: args.extra_supplies,
        },
    };

    Ok(params)
}

/// Display the formatted values.
pub fn show_printer_values(printer: Printer, extra_supplies: bool, theme: &CliTheme) {
    println!("{} {}\n", "Printer:".bright_cyan().bold(), printer.name);

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

    println!();
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
