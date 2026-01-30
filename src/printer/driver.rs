use crate::{
    driver::{brother::BrotherDriver, generic::GenericDriver},
    error::AppError,
    printer::Printer,
    snmp::SnmpClientParams,
};

/// Defines the interface for printer-specific SNMP implementations.
///
/// This trait allows different printer brands or models to implement unique logic for
/// identifying themselves and retrieving supply data.
pub trait PrinterDriver {
    /// Determines if this driver supports the given printer model name.
    fn is_compatible(&self, printer_name: &str) -> bool;

    /// Executes the SNMP queries required to populate the [`Printer`] data structure for the target device.
    fn get_supplies(
        &self,
        params: &SnmpClientParams,
        printer_name: &str,
    ) -> Result<Printer, AppError>;
}

/// Registry and selector for printer drivers.
///
/// Manages the collection of available drivers and handles the logic for selecting
/// the most appropriate implementation for a discovered device.
pub struct DriverManager {
    drivers: Vec<Box<dyn PrinterDriver>>,
}

impl DriverManager {
    /// Initializes the manager with the registry of supported drivers.
    ///
    /// **Note:** The order of registration matters. Specific drivers (e.g., Brother) are checked
    /// before generic fallbacks to ensure the best possible data extraction.
    pub fn new() -> Self {
        Self {
            drivers: vec![
                Box::new(BrotherDriver),
                Box::new(GenericDriver), // Fallback for other models
            ],
        }
    }

    /// Iterates through registered drivers to find the first one compatible with the provided printer name.
    pub fn get_driver(&self, printer_name: &str) -> Option<&dyn PrinterDriver> {
        self.drivers
            .iter()
            .find(|d| d.is_compatible(printer_name))
            .map(|d| d.as_ref())
    }
}
