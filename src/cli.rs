use crate::{
    error::AppError,
    printer::Printer,
    snmp::{SnmpClientParams, SnmpVersion},
};
use clap::Parser;
use std::net::Ipv4Addr;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// IP of the printer
    #[arg()]
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
    };

    Ok(params)
}

/// Display the formatted values.
pub fn show_printer_values(printer: Printer) {
    println!("Name: {}", printer.name);

    if let Some(level) = printer.black_toner_level_percent {
        println!("Black Toner: {level}%");
    }

    if let Some(level) = printer.cyan_toner_level_percent {
        println!("Cyan Toner: {level}%");
    }

    if let Some(level) = printer.magenta_toner_level_percent {
        println!("Magenta Toner: {level}%");
    }

    if let Some(level) = printer.yellow_toner_level_percent {
        println!("Yellow Toner: {level}%");
    }
}
