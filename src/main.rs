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
use openssl::provider::Provider;
use std::process;

#[tokio::main]
async fn main() {
    // Load legacy provider for older algorithms.
    let _legacy_guard = Provider::try_load(None, "legacy", true)
        .map(Some)
        .unwrap_or_else(|e| {
            eprintln!("Warning: Failed to load OpenSSL Legacy Provider: {e}");
            None // Continue execution without legacy support
        });

    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

/// Orchestrates the application flow: configuration loading, argument parsing,
/// and execution of the main logic.
async fn run() -> Result<(), AppError> {
    let mut args = Args::parse();

    if args.init {
        let path = config::Config::create_default_template()
            .map_err(|e| AppError::new(ErrorKind::Io(format!("Failed to create config: {e}"))))?;

        println!("✅ Configuration file created at: {path:?}");
        return Ok(());
    }

    // Load inventory/config and merge with CLI args
    let inventory = config::Config::load().unwrap_or_default();

    // If a host is provided, check if it matches an alias
    if let Some(host_input) = &args.host {
        if let Some(saved_printer) = inventory.find_by_alias(host_input) {
            println!(
                "📂 Loading saved configuration for: '{}'",
                saved_printer.alias
            );
            config::apply_config_to_args(&mut args, saved_printer);
        }
    }

    let host = args
        .host
        .as_ref()
        .ok_or_else(|| AppError::new(ErrorKind::Cli("Host is required.".to_string())))?;

    let ip = cli::resolve_host(host, args.port)?;

    let params = cli::AppParams {
        app: cli::CliParams {
            theme: args.theme,
            output: args.output.clone(),
        },
        snmp: snmp::SnmpClientParams::from_args(&args, ip),
    };

    // Execute core logic
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
