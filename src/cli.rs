use crate::{
    error::AppError,
    printer::{Printer, TonerColor},
    snmp::{SnmpClientParams, SnmpVersion},
};
use clap::{Parser, ValueEnum};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fmt::Display, net::Ipv4Addr, path::PathBuf, thread, time::Duration};

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

/// Enum representing different CLI themes.
///
/// This enum defines the available themes that can be used in the CLI interface,
/// affecting the visual presentation.
#[derive(Debug, Clone, ValueEnum)]
pub enum CliTheme {
    Solid,
    Blocks,
    Circles,
    Diamonds,
    Shades,
    Vintage,
    Stars,
    Emoji,
    Moon,
}

impl Display for CliTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid => write!(f, "solid"),
            Self::Blocks => write!(f, "blocks"),
            Self::Circles => write!(f, "circles"),
            Self::Diamonds => write!(f, "diamonds"),
            Self::Shades => write!(f, "shades"),
            Self::Vintage => write!(f, "vintage"),
            Self::Stars => write!(f, "stars"),
            Self::Emoji => write!(f, "emoji"),
            Self::Moon => write!(f, "moon"),
        }
    }
}

impl Default for CliTheme {
    fn default() -> Self {
        Self::Solid
    }
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
        },
    };

    Ok(params)
}

/// Displays a progress bar representing the toner level.
///
/// # Arguments
/// * `level` - The toner level as a percentage (0-100).
/// * `toner_color` - The color of the toner from the [TonerColor] enum.
/// * `theme` - The [CliTheme] selected by the user.
fn show_toner_progress(level: u8, toner_color: TonerColor, theme: &CliTheme) {
    let theme_chars = match theme {
        CliTheme::Solid => "█ ",
        CliTheme::Blocks => "█▓▒░",
        CliTheme::Circles => "●○",
        CliTheme::Diamonds => "◆◇",
        CliTheme::Shades => "▉▇▆▅▄▃▂▁",
        CliTheme::Vintage => "#-",
        CliTheme::Stars => "★☆",
        CliTheme::Emoji => "😊🙂😐🙁😞",
        CliTheme::Moon => "🌕🌖🌗🌘🌑",
    };

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
            .progress_chars(theme_chars),
    );

    for _ in 0..level {
        thread::sleep(Duration::from_millis(1));
        pb.inc(1);
    }

    pb.abandon();
}

/// Display the formatted values.
pub fn show_printer_values(printer: Printer, theme: CliTheme) {
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
        show_toner_progress(level as u8, TonerColor::Black, &theme);
    }

    if let Some(level) = printer.cyan_toner.as_ref().and_then(|t| t.level_percent) {
        show_toner_progress(level as u8, TonerColor::Cyan, &theme);
    }

    if let Some(level) = printer.magenta_toner.as_ref().and_then(|t| t.level_percent) {
        show_toner_progress(level as u8, TonerColor::Magenta, &theme);
    }

    if let Some(level) = printer.yellow_toner.as_ref().and_then(|t| t.level_percent) {
        show_toner_progress(level as u8, TonerColor::Yellow, &theme);
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
