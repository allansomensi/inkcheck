use crate::{
    error::AppError,
    printer::{Printer, TonerColor},
    snmp::{SnmpClientParams, SnmpVersion},
};
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::{net::Ipv4Addr, path::PathBuf, thread, time::Duration};

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// IP of the printer
    ip: Ipv4Addr,

    /// SNMP Service Port
    #[arg(short, long, default_value_t = 161)]
    port: u16,

    /// SNMP Version
    #[arg(short, long, default_value_t = SnmpVersion::V2c)]
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
}

/// Capture and return the command line arguments.
pub fn parse_args() -> Result<SnmpClientParams, AppError> {
    let args = Args::parse();

    let params = SnmpClientParams {
        ip: args.ip,
        port: args.port,
        community: args.community,
        version: args.snmp_version,
        timeout: args.timeout,
        data_dir: args.data_dir,
    };

    Ok(params)
}

/// Displays a progress bar representing the toner level.
///
/// # Arguments
/// * `level` - The toner level as a percentage (0-100).
/// * `toner_color` - The color of the toner from the [TonerColor] enum.
fn show_toner_progress(level: u8, toner_color: TonerColor) {
    let color: &str = match toner_color {
        TonerColor::Black => "white",
        TonerColor::Cyan => "cyan",
        TonerColor::Magenta => "magenta",
        TonerColor::Yellow => "yellow",
    };

    let template = format!("{{prefix:8.{color}.bold}} [{{bar:25.{color}}}] {{percent:3}}%");

    let pb = ProgressBar::new(100);
    pb.set_prefix(format!("{}:", toner_color));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&template)
            .unwrap()
            .progress_chars("█▓▒░"),
    );

    for _ in 0..level {
        thread::sleep(Duration::from_millis(1));
        pb.inc(1);
    }

    pb.abandon();
}

/// Display the formatted values.
pub fn show_printer_values(printer: Printer) {
    let app_version = env!("CARGO_PKG_VERSION");

    println!(
        "{}  {} {}  {}\n",
        "-=-=-=-".cyan(),
        "InkCheck".white().bold(),
        app_version.bright_yellow(),
        "-=-=-=-".cyan()
    );

    println!("{} {}\n", "Printer:".bright_cyan().bold(), printer.name);

    if let Some(level) = printer.black_toner.level_percent {
        show_toner_progress(level as u8, TonerColor::Black);
    }

    if let Some(level) = printer.cyan_toner.level_percent {
        show_toner_progress(level as u8, TonerColor::Cyan);
    }

    if let Some(level) = printer.magenta_toner.level_percent {
        show_toner_progress(level as u8, TonerColor::Magenta);
    }

    if let Some(level) = printer.yellow_toner.level_percent {
        show_toner_progress(level as u8, TonerColor::Yellow);
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
