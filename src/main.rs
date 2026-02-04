mod cli;
mod config;
mod driver;
mod error;
mod printer;
mod snmp;
mod utils;

use error::AppError;
use std::process;
use std::thread;

/// Stack size for the SNMP thread (4MB).
/// Increased size is required to prevent stack overflow during SNMP parsing.
const WORKER_STACK_SIZE: usize = 4 * 1024 * 1024;

/// Application entry point.
fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/// Orchestrates the application flow: parses CLI arguments, executes SNMP operations in a dedicated thread with increased stack size, and displays the results.
fn run() -> Result<(), AppError> {
    let params = cli::args::parse_args()
        .map_err(|e| AppError::new(error::ErrorKind::Parse(format!("{e}"))))?;

    let snmp_params = params.snmp.clone();

    let handler = thread::Builder::new()
        .name("snmp-worker".into())
        .stack_size(WORKER_STACK_SIZE)
        .spawn(move || snmp::get_printer_values(&snmp_params))
        .map_err(|e| {
            AppError::new(error::ErrorKind::SnmpRequest(format!(
                "Failed to spawn SNMP worker thread: {e}"
            )))
        })?;

    let printer = handler.join().map_err(|_| {
        AppError::new(error::ErrorKind::SnmpRequest(
            "Critical error: The SNMP worker thread crashed.".to_string(),
        ))
    })??;

    cli::display::show_printer_values(
        printer,
        params.snmp.extra_supplies,
        params.snmp.metrics,
        &params.app.theme,
        &params.app.output,
    );

    Ok(())
}
