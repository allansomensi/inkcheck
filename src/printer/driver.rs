use crate::{
    driver::{brother::BrotherDriver, generic::GenericDriver},
    error::AppError,
    printer::Printer,
    snmp::SnmpClientParams,
};

pub trait PrinterDriver {
    /// Checks if the driver is compatible with the printer based on the model name obtained via SNMP.
    fn is_compatible(&self, printer_name: &str) -> bool;

    /// The main method that retrieves the printer's supply levels.
    fn get_supplies(
        &self,
        params: &SnmpClientParams,
        printer_name: &str,
    ) -> Result<Printer, AppError>;
}

/// The [`DriverManager`] manages all available drivers and selects
/// the most appropriate one for a given device.
pub struct DriverManager {
    drivers: Vec<Box<dyn PrinterDriver>>,
}

impl DriverManager {
    /// Creates a new instance of [`DriverManager`] and registers all drivers.
    /// More specific drivers should come before more generic ones.
    pub fn new() -> Self {
        Self {
            drivers: vec![
                Box::new(BrotherDriver),
                Box::new(GenericDriver), // Fallback for other models
            ],
        }
    }

    /// Finds and returns the first compatible driver for a printer.
    pub fn get_driver(&self, printer_name: &str) -> Option<&dyn PrinterDriver> {
        self.drivers
            .iter()
            .find(|d| d.is_compatible(printer_name))
            .map(|d| d.as_ref())
    }
}
