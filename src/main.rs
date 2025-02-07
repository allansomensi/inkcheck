mod cli;
mod error;
mod printer;
mod snmp;
mod utils;

fn main() -> Result<(), error::AppError> {
    // Get the command line parameters.
    let params = cli::parse_args()?;

    // Fetch the values and store them in a `Printer`.
    let printer = snmp::get_printer_values(&params)?;

    // Display the formatted values.
    cli::show_printer_values(printer);

    Ok(())
}
