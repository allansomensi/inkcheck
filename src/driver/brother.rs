use crate::{
    error::AppError,
    printer::{
        driver::PrinterDriver,
        supply::{Drum, Drums, Fuser, Toner, Toners},
        Printer,
    },
    snmp::{value::get_snmp_value, SnmpClientParams},
};

const BLACK_TONER_CODE: u8 = 0x6F;
const CYAN_TONER_CODE: u8 = 0x70;
const MAGENTA_TONER_CODE: u8 = 0x71;
const YELLOW_TONER_CODE: u8 = 0x72;

const BLACK_DRUM_CODE: u8 = 0x41;
const CYAN_DRUM_CODE: u8 = 0x79;
const MAGENTA_DRUM_CODE: u8 = 0x7a;
const YELLOW_DRUM_CODE: u8 = 0x7b;

const FUSER_CODE: u8 = 0x6a;

/// The function scans for the exact sequence `[toner_code, 0x01, 0x04]`.
/// If the sequence is found, the next **4 bytes** are extracted as a big-endian `u32` value,
/// converted to a percentage, and returned as `i64`.
///
/// ## Arguments
/// * `bytes` - A slice of bytes representing the raw printer data.
/// * `toner_code` - The specific toner code to search for.
///
/// ## Returns
/// * `Some(i64)` - The toner level as a percentage if found.
/// * `None` - If the toner code sequence is not found or the data is incomplete.
fn find_value_in_brother_bytes(bytes: &[u8], toner_code: u8) -> Option<i64> {
    let pattern = [toner_code, 0x01, 0x04];

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

/// Implementation of the specific driver for `Brother` printers.
pub struct BrotherDriver;

impl PrinterDriver for BrotherDriver {
    fn is_compatible(&self, printer_name: &str) -> bool {
        printer_name.to_lowercase().contains("brother")
    }

    fn get_supplies(
        &self,
        params: &SnmpClientParams,
        printer_name: &str,
    ) -> Result<Printer, AppError> {
        get_brother_supplies_levels(params, printer_name.to_string())
    }
}

/// This function retrieves toner levels for a Brother printer and returns a [`Printer`] object.
///
/// It attempts to read the toner levels for black, cyan, magenta, and yellow toners. If any toner
/// is not found, it will be returned as [None] in the [`Printer`] struct.
/// If the black toner is not found, an error is returned.
fn get_brother_supplies_levels(
    ctx: &SnmpClientParams,
    printer_name: String,
) -> Result<Printer, AppError> {
    let old_model = printer_name.contains("HL-5350DN");

    // This OID stores most of the supply information in hexadecimal format
    let br_info_maintenance_oid = if old_model {
        &[1, 3, 6, 1, 4, 1, 2435, 2, 3, 9, 4, 2, 1, 5, 5, 11, 0] // brInfoNextCare
    } else {
        &[1, 3, 6, 1, 4, 1, 2435, 2, 3, 9, 4, 2, 1, 5, 5, 8, 0]
    };

    let serial_number_oid = &[1, 3, 6, 1, 2, 1, 43, 5, 1, 1, 17, 1];
    let serial_number = Some(get_snmp_value(serial_number_oid, ctx)?);

    let bytes = get_snmp_value::<Vec<u8>>(br_info_maintenance_oid, ctx)?;

    let black_toner_percent = find_value_in_brother_bytes(&bytes, BLACK_TONER_CODE);
    let cyan_toner_percent = find_value_in_brother_bytes(&bytes, CYAN_TONER_CODE);
    let magenta_toner_percent = find_value_in_brother_bytes(&bytes, MAGENTA_TONER_CODE);
    let yellow_toner_percent = find_value_in_brother_bytes(&bytes, YELLOW_TONER_CODE);

    let black_drum_percent = find_value_in_brother_bytes(&bytes, BLACK_DRUM_CODE);
    let cyan_drum_percent = find_value_in_brother_bytes(&bytes, CYAN_DRUM_CODE);
    let magenta_drum_percent = find_value_in_brother_bytes(&bytes, MAGENTA_DRUM_CODE);
    let yellow_drum_percent = find_value_in_brother_bytes(&bytes, YELLOW_DRUM_CODE);

    let fuser_percent = find_value_in_brother_bytes(&bytes, FUSER_CODE);

    // Toners
    let black_toner = black_toner_percent.map(|percent| Toner::new(0, 0, Some(percent)));
    let cyan_toner = cyan_toner_percent.map(|percent| Toner::new(0, 0, Some(percent)));
    let magenta_toner = magenta_toner_percent.map(|percent| Toner::new(0, 0, Some(percent)));
    let yellow_toner = yellow_toner_percent.map(|percent| Toner::new(0, 0, Some(percent)));

    // Drums
    let black_drum = black_drum_percent.map(|percent| Drum::new(0, 0, Some(percent)));
    let cyan_drum = cyan_drum_percent.map(|percent| Drum::new(0, 0, Some(percent)));
    let magenta_drum = magenta_drum_percent.map(|percent| Drum::new(0, 0, Some(percent)));
    let yellow_drum = yellow_drum_percent.map(|percent| Drum::new(0, 0, Some(percent)));

    // Other
    let fuser = fuser_percent.map(|percent| Fuser::new(0, 0, Some(percent)));

    let toners = Toners {
        black_toner,
        cyan_toner,
        magenta_toner,
        yellow_toner,
    };

    let drums = Drums {
        black_drum,
        cyan_drum,
        magenta_drum,
        yellow_drum,
    };

    Ok(Printer::new(
        printer_name,
        serial_number,
        toners,
        drums,
        fuser,
        None,
    ))
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

        // Inexistent code in mono
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

        // Inexistent code in color
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
