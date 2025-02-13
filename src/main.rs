mod cli;
mod error;
mod printer;
mod snmp;
mod utils;

fn main() -> Result<(), error::AppError> {
    // Get the command line parameters.
    let params = match cli::parse_args() {
        Ok(params) => params,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    // Fetch the values and store them in a `Printer`.
    let printer = match snmp::get_printer_values(&params.snmp) {
        Ok(printer) => printer,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    // Display the formatted values.
    cli::show_printer_values(printer, params.app.theme);

    Ok(())
}
