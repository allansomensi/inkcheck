use crate::{
    error::AppError,
    printer::{supply::TonerColor, Printer},
    snmp::{version::SnmpVersion, SnmpClientParams},
};
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::{net::Ipv4Addr, path::PathBuf, thread, time::Duration};
use theme::{get_theme_chars, CliTheme};

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

/// Displays a progress bar representing the toner level.
///
/// ## Arguments
/// * `level` - The toner level as a percentage (0-100).
/// * `toner_color` - The color of the toner from the [TonerColor] enum.
/// * `theme` - The [CliTheme] selected by the user.
fn show_toner_progress(level: u8, toner_color: TonerColor, theme: &CliTheme) {
    let theme_chars = get_theme_chars(theme);

    let color: &str = match toner_color {
        TonerColor::Black => "white",
        TonerColor::Cyan => "cyan",
        TonerColor::Magenta => "magenta",
        TonerColor::Yellow => "yellow",
    };

    let template = format!("{{prefix:9.{color}.bold}} [{{bar:25.{color}}}] {{percent:3}}%");

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

/// Displays a progress bar representing the drum level.
///
/// ## Arguments
/// * `level` - The drum level as a percentage (0-100).
/// * `toner_color` - The color of the drum from the [TonerColor] enum.
/// * `theme` - The [CliTheme] selected by the user.
fn show_drum_progress(level: u8, toner_color: TonerColor, theme: &CliTheme) {
    let theme_chars = get_theme_chars(theme);

    let color: &str = match toner_color {
        TonerColor::Black => "white",
        TonerColor::Cyan => "cyan",
        TonerColor::Magenta => "magenta",
        TonerColor::Yellow => "yellow",
    };

    let template = format!("{{prefix:9.{color}.bold}} [{{bar:25.{color}}}] {{percent:3}}%");

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

/// Displays a progress bar representing the fuser level.
///
/// ## Arguments
/// * `level` - The fuser level as a percentage (0-100).
/// * `theme` - The [CliTheme] selected by the user.
fn show_fuser_progress(level: u8, theme: &CliTheme) {
    let theme_chars = get_theme_chars(theme);

    let template = "{prefix:9.gray.bold} [{bar:25.white}] {percent:3}%";

    let pb = ProgressBar::new(100);
    pb.set_prefix("Fuser");
    pb.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .unwrap()
            .progress_chars(theme_chars),
    );

    for _ in 0..level {
        thread::sleep(Duration::from_millis(1));
        pb.inc(1);
    }

    pb.abandon();
}

/// Displays a progress bar representing the waste toner container level.
///
/// ## Arguments
/// * `level` - The waste toner container level as a percentage (0-100).
/// * `theme` - The [CliTheme] selected by the user.
fn show_reservoir_progress(level: u8, theme: &CliTheme) {
    let theme_chars = get_theme_chars(theme);

    let template = "{prefix:9.gray.bold} [{bar:25.white}] {percent:3}%";

    let pb = ProgressBar::new(100);
    pb.set_prefix("Reservoir");
    pb.set_style(
        ProgressStyle::default_bar()
            .template(template)
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
pub fn show_printer_values(printer: Printer, extra_supplies: bool, theme: &CliTheme) {
    let app_version = env!("CARGO_PKG_VERSION");

    println!(
        "{}  {} {}  {}\n",
        "-=-=-=-".cyan(),
        "InkCheck".white().bold(),
        app_version.bright_yellow(),
        "-=-=-=-".cyan()
    );

    println!("{} {}\n", "Printer:".bright_cyan().bold(), printer.name);

    println!("--> {}\n", "Toner:".bright_white().bold());

    if let Some(level) = printer.toners.black_toner.level_percent {
        show_toner_progress(level as u8, TonerColor::Black, theme);
    }

    if let Some(level) = printer
        .toners
        .cyan_toner
        .as_ref()
        .and_then(|t| t.level_percent)
    {
        show_toner_progress(level as u8, TonerColor::Cyan, theme);
    }

    if let Some(level) = printer
        .toners
        .magenta_toner
        .as_ref()
        .and_then(|t| t.level_percent)
    {
        show_toner_progress(level as u8, TonerColor::Magenta, theme);
    }

    if let Some(level) = printer
        .toners
        .yellow_toner
        .as_ref()
        .and_then(|t| t.level_percent)
    {
        show_toner_progress(level as u8, TonerColor::Yellow, theme);
    }

    if extra_supplies {
        if let Some(level) = printer
            .drums
            .black_drum
            .as_ref()
            .and_then(|t| t.level_percent)
        {
            println!("\n\n--> {}\n", "Drum:".bright_white().bold());
            show_drum_progress(level as u8, TonerColor::Black, theme);
        }

        if let Some(level) = printer
            .drums
            .cyan_drum
            .as_ref()
            .and_then(|t| t.level_percent)
        {
            show_drum_progress(level as u8, TonerColor::Cyan, theme);
        }

        if let Some(level) = printer
            .drums
            .magenta_drum
            .as_ref()
            .and_then(|t| t.level_percent)
        {
            show_drum_progress(level as u8, TonerColor::Magenta, theme);
        }

        if let Some(level) = printer
            .drums
            .yellow_drum
            .as_ref()
            .and_then(|t| t.level_percent)
        {
            show_drum_progress(level as u8, TonerColor::Yellow, theme);
        }

        if let Some(level) = printer.fuser.as_ref().and_then(|t| t.level_percent) {
            println!("\n\n--> {}\n", "Other:".bright_white().bold());
            show_fuser_progress(level as u8, theme);
        }

        if let Some(level) = printer.reservoir.as_ref().and_then(|t| t.level_percent) {
            show_reservoir_progress(level as u8, theme);
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
