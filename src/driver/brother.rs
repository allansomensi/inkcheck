use crate::{
    error::AppError,
    printer::{
        driver::PrinterDriver,
        supply::{
            drum::{Drum, Drums},
            fuser::Fuser,
            toner::{Toner, Toners},
        },
        Printer,
    },
    snmp::{value::get_snmp_value, SnmpClientParams},
};
use async_trait::async_trait;

const BLACK_TONER_CODE: u8 = 0x6F;
const CYAN_TONER_CODE: u8 = 0x70;
const MAGENTA_TONER_CODE: u8 = 0x71;
const YELLOW_TONER_CODE: u8 = 0x72;

const BLACK_DRUM_CODE: u8 = 0x41;
const CYAN_DRUM_CODE: u8 = 0x79;
const MAGENTA_DRUM_CODE: u8 = 0x7a;
const YELLOW_DRUM_CODE: u8 = 0x7b;

const FUSER_CODE: u8 = 0x6a;

/// Scans a raw byte slice for a specific supply code pattern to extract its remaining percentage.
fn find_value_in_brother_bytes(bytes: &[u8], code: u8) -> Option<i64> {
    let pattern = [code, 0x01, 0x04];

    if let Some(pos) = bytes.windows(3).position(|window| window == pattern) {
        let start = pos + 3;
        if start + 4 <= bytes.len() {
            let result_bytes: [u8; 4] = bytes[start..start + 4].try_into().ok()?;
            let value = u32::from_be_bytes(result_bytes);
            return Some((value as f32 / 100.0) as i64);
        }
    }
    None
}

/// Driver implementation for Brother printers.
pub struct BrotherDriver;

impl BrotherDriver {
    /// Fetches the serial number via SNMP.
    async fn fetch_serial(&self, params: &SnmpClientParams) -> Result<Option<String>, AppError> {
        let oid = &[1, 3, 6, 1, 2, 1, 43, 5, 1, 1, 17, 1];
        Ok(Some(get_snmp_value(oid, params).await?))
    }

    /// Determines the correct OID based on the model and fetches the maintenance binary blob.
    async fn fetch_maintenance_data(
        &self,
        params: &SnmpClientParams,
        printer_name: &str,
    ) -> Result<Vec<u8>, AppError> {
        let old_model = printer_name.contains("HL-5350DN");

        // Select OID based on model generation
        let oid = if old_model {
            &[1, 3, 6, 1, 4, 1, 2435, 2, 3, 9, 4, 2, 1, 5, 5, 11, 0] // brInfoNextCare (Legacy)
        } else {
            &[1, 3, 6, 1, 4, 1, 2435, 2, 3, 9, 4, 2, 1, 5, 5, 8, 0] // Standard
        };

        get_snmp_value::<Vec<u8>>(oid, params).await
    }

    /// Parses the binary blob to extract Toner levels.
    fn extract_toners(&self, bytes: &[u8]) -> Toners {
        let get_toner =
            |code| find_value_in_brother_bytes(bytes, code).map(|p| Toner::new(0, 0, Some(p)));

        Toners {
            black_toner: get_toner(BLACK_TONER_CODE),
            cyan_toner: get_toner(CYAN_TONER_CODE),
            magenta_toner: get_toner(MAGENTA_TONER_CODE),
            yellow_toner: get_toner(YELLOW_TONER_CODE),
        }
    }

    /// Parses the binary blob to extract Drum levels.
    fn extract_drums(&self, bytes: &[u8]) -> Drums {
        let get_drum =
            |code| find_value_in_brother_bytes(bytes, code).map(|p| Drum::new(0, 0, Some(p)));

        Drums {
            black_drum: get_drum(BLACK_DRUM_CODE),
            cyan_drum: get_drum(CYAN_DRUM_CODE),
            magenta_drum: get_drum(MAGENTA_DRUM_CODE),
            yellow_drum: get_drum(YELLOW_DRUM_CODE),
        }
    }

    /// Parses the binary blob to extract Fuser level.
    fn extract_fuser(&self, bytes: &[u8]) -> Option<Fuser> {
        find_value_in_brother_bytes(bytes, FUSER_CODE).map(|p| Fuser::new(0, 0, Some(p)))
    }
}

#[async_trait]
impl PrinterDriver for BrotherDriver {
    fn is_compatible(&self, printer_name: &str) -> bool {
        printer_name.to_lowercase().contains("brother")
    }

    async fn get_supplies(
        &self,
        params: &SnmpClientParams,
        printer_name: &str,
    ) -> Result<Printer, AppError> {
        // Fetch Serial Number
        let serial_number = self.fetch_serial(params).await?;

        // Fetch Maintenance Data
        let bytes = self.fetch_maintenance_data(params, printer_name).await?;

        // Parse Supplies
        let toners = self.extract_toners(&bytes);
        let drums = self.extract_drums(&bytes);
        let fuser = self.extract_fuser(&bytes);

        Ok(Printer::new(
            printer_name.to_string(),
            serial_number,
            toners,
            drums,
            fuser,
            None, // Reservoir not supported
            None, // Metrics not supported in this driver version
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_value_in_brother_bytes() {
        let bytes_mono = vec![
            0x63, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x11, 0x01, 0x04, 0x00, 0x00, 0x08, 0x62,
            0x41, 0x01, 0x04, 0x00, 0x00, 0x25, 0x80, 0x31, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x6F, 0x01, 0x04, 0x00, 0x00, 0x21, 0xFC, 0x81, 0x01, 0x04, 0x00, 0x00, 0x00, 0x5A,
            0x86, 0x01, 0x04, 0x00, 0x00, 0x00, 0x0A, 0x67, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x6B, 0x01, 0x04, 0x00, 0x00, 0x19, 0x64, 0x54, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x66, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x35, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x6A, 0x01, 0x04, 0x00, 0x00, 0x19, 0x64, 0x6C, 0x01, 0x04, 0x00, 0x00, 0x27, 0x10,
            0x6D, 0x01, 0x04, 0x00, 0x00, 0x1B, 0xBC, 0xFF,
        ];

        let bytes_color = vec![
            0x63, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x41, 0x01, 0x04, 0x00, 0x00, 0x25, 0x1C,
            0x11, 0x01, 0x04, 0x00, 0x00, 0x06, 0x7B, 0x68, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x55, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x32, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x33, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x34, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x31, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x70, 0x01, 0x04, 0x00, 0x00, 0x10, 0x68,
            0x82, 0x01, 0x04, 0x00, 0x00, 0x00, 0x32, 0x71, 0x01, 0x04, 0x00, 0x00, 0x11, 0x30,
            0x83, 0x01, 0x04, 0x00, 0x00, 0x00, 0x32, 0x72, 0x01, 0x04, 0x00, 0x00, 0x1E, 0xDC,
            0x84, 0x01, 0x04, 0x00, 0x00, 0x00, 0x50, 0x6F, 0x01, 0x04, 0x00, 0x00, 0x10, 0xCC,
            0x81, 0x01, 0x04, 0x00, 0x00, 0x00, 0x32, 0x69, 0x01, 0x04, 0x00, 0x00, 0x26, 0xAC,
            0x67, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x6B, 0x01, 0x04, 0x00, 0x00, 0x26, 0xAC,
            0x54, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x6A, 0x01, 0x04, 0x00, 0x00, 0x26, 0xAC,
            0x66, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x6C, 0x01, 0x04, 0x00, 0x00, 0x27, 0x10,
            0x35, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x6D, 0x01, 0x04, 0x00, 0x00, 0x26, 0xAC,
            0xFF,
        ];

        let empty_bytes: Vec<u8> = Vec::new();

        assert_eq!(
            find_value_in_brother_bytes(&empty_bytes, BLACK_TONER_CODE),
            None
        );

        // Mono
        assert_eq!(
            find_value_in_brother_bytes(&bytes_mono, BLACK_TONER_CODE),
            Some(87)
        );

        assert_eq!(
            find_value_in_brother_bytes(&bytes_mono, YELLOW_TONER_CODE),
            None
        );

        assert_eq!(find_value_in_brother_bytes(&bytes_mono, 0x99), None);

        // Color
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, BLACK_TONER_CODE),
            Some(43)
        );
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, CYAN_TONER_CODE),
            Some(42)
        );
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, MAGENTA_TONER_CODE),
            Some(44)
        );
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, YELLOW_TONER_CODE),
            Some(79)
        );

        // Drums
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, BLACK_DRUM_CODE),
            Some(95)
        );
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, CYAN_DRUM_CODE),
            None
        );
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, MAGENTA_DRUM_CODE),
            None
        );
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, YELLOW_DRUM_CODE),
            None
        );

        // Fuser
        assert_eq!(
            find_value_in_brother_bytes(&bytes_color, FUSER_CODE),
            Some(99)
        );

        assert_eq!(find_value_in_brother_bytes(&bytes_color, 0x99), None);
    }

    #[test]
    fn test_find_value_in_old_models() {
        let bytes = vec![
            0x41, 0x01, 0x04, 0x00, 0x00, 0x17, 0x53, 0x82, 0x01, 0x04, 0x00, 0x00, 0x3A, 0x50,
            0x73, 0x01, 0x04, 0x00, 0x00, 0xE1, 0x07, 0x86, 0x01, 0x04, 0x00, 0x00, 0xC3, 0x14,
            0x77, 0x01, 0x04, 0x00, 0x00, 0xE6, 0x86, 0x81, 0x01, 0x04, 0x00, 0x00, 0xE1, 0x07,
            0x89, 0x01, 0x04, 0x00, 0x00, 0xE1, 0x07, 0xFF,
        ];

        let empty_bytes: Vec<u8> = Vec::new();

        assert_eq!(
            find_value_in_brother_bytes(&empty_bytes, BLACK_DRUM_CODE),
            None
        );

        assert_eq!(
            find_value_in_brother_bytes(&bytes, BLACK_DRUM_CODE),
            Some(59)
        );
    }
}
