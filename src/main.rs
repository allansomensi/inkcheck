mod cli;
mod config;
mod driver;
mod error;
mod printer;
mod snmp;
mod utils;

use clap::Parser;
use cli::args::Args;
use error::{AppError, ErrorKind};
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

/// Orchestrates the application flow: configuration loading, argument parsing,
/// and execution of the main logic.
async fn run() -> Result<(), AppError> {
    let mut args = Args::parse();

    if let Some(cmd) = &args.command {
        match cmd {
            cli::commands::Commands::Scan { timeout } => {
                printer::scan::run_mdns_scan(*timeout).await;
                return Ok(());
            }
        }
    }

    if args.init {
        let path = config::Config::create_default_template()
            .map_err(|e| AppError::new(ErrorKind::Io(format!("Failed to create config: {e}"))))?;

        println!("✅ Configuration file created at: {path:?}");
        return Ok(());
    }

    let inventory = config::Config::load().unwrap_or_default();

    if let Some(host_input) = &args.host
        && let Some(saved_printer) = inventory.find_by_alias(host_input)
    {
        println!(
            "📂 Loading saved configuration for: '{}'",
            saved_printer.alias
        );
        config::apply_config_to_args(&mut args, saved_printer);
    }

    let host = args.host.as_ref().ok_or_else(|| {
        AppError::new(ErrorKind::Cli(
            "Host is required unless using a subcommand (like 'scan') or '--init'.".to_string(),
        ))
    })?;

    let ip = cli::resolve_host(host, args.port)?;

    let params = cli::AppParams {
        app: cli::CliParams {
            theme: args.theme,
            output: args.output.clone(),
        },
        snmp: snmp::SnmpClientParams::from_args(&args, ip),
    };

    let printer = snmp::get_printer_values(&params.snmp).await?;

    cli::display::show_printer_values(
        printer,
        params.snmp.extra_supplies,
        params.snmp.metrics,
        &params.app.theme,
        &params.app.output,
    );

    Ok(())
}
