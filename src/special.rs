use crate::{
    error::AppError,
    printer::{Printer, Toner},
    snmp::{get_snmp_value, SnmpClientParams},
};

const BLACK_TONER_CODE: u8 = 0x6F;
const CYAN_TONER_CODE: u8 = 0x70;
const MAGENTA_TONER_CODE: u8 = 0x71;
const YELLOW_TONER_CODE: u8 = 0x72;

/// This function searches for a specific toner code in the provided byte array
/// and extracts its corresponding value if found.
///
/// The toner value is expected to be a 4-byte value stored in big-endian format.
/// If the toner code is found, it returns the toner level as a percentage.
/// If not found, it returns None.
fn find_value_in_brother_bytes(bytes: Vec<u8>, toner_code: u8) -> Option<i64> {
    let mut result_bytes = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        // Check if the current byte matches the toner code
        if bytes[i] == toner_code {
            i += 1;

            // Skip the default values "01 04" if present
            if i + 1 < bytes.len() && bytes[i] == 0x01 && bytes[i + 1] == 0x04 {
                i += 2;
            }

            // Check if there are at least 4 bytes available for the toner value
            if i + 3 < bytes.len() {
                // Extract 4 bytes representing the toner level
                result_bytes = bytes[i..i + 4].to_vec();
            } else {
                // If fewer than 4 bytes remain, extract the remaining bytes
                result_bytes = bytes[i..].to_vec();
            }
            break;
        }
        i += 1;
    }

    // If no value was found, return None
    if result_bytes.is_empty() {
        return None;
    }

    let value = u32::from_be_bytes(result_bytes.try_into().unwrap());

    // Return the toner level as a percentage
    Some((value as f32 / 100.0) as i64)
}

/// This function retrieves toner levels for a Brother printer and returns a Printer object.
///
/// It attempts to read the toner levels for black, cyan, magenta, and yellow toners. If any toner
/// is not found, it will be returned as `None` in the `Printer` struct.
/// If the black toner is not found, an error is returned.
pub fn brother(ctx: &SnmpClientParams, printer_name: String) -> Result<Printer, AppError> {
    let br_info_maintenance_oid = &[1, 3, 6, 1, 4, 1, 2435, 2, 3, 9, 4, 2, 1, 5, 5, 8, 0];

    let bytes = get_snmp_value::<Vec<u8>>(br_info_maintenance_oid, ctx)?;

    let black_toner_percent = find_value_in_brother_bytes(bytes.clone(), BLACK_TONER_CODE)
        .ok_or(AppError::UnsupportedPrinter("Brother".to_string()))?;

    let cyan_toner_percent = find_value_in_brother_bytes(bytes.clone(), CYAN_TONER_CODE);
    let magenta_toner_percent = find_value_in_brother_bytes(bytes.clone(), MAGENTA_TONER_CODE);
    let yellow_toner_percent = find_value_in_brother_bytes(bytes, YELLOW_TONER_CODE);

    let black_toner = Toner {
        level: 0,
        max_level: 0,
        level_percent: Some(black_toner_percent),
    };

    let cyan_toner = cyan_toner_percent.map(|percent| Toner {
        level: 0,
        max_level: 0,
        level_percent: Some(percent),
    });

    let magenta_toner = magenta_toner_percent.map(|percent| Toner {
        level: 0,
        max_level: 0,
        level_percent: Some(percent),
    });

    let yellow_toner = yellow_toner_percent.map(|percent| Toner {
        level: 0,
        max_level: 0,
        level_percent: Some(percent),
    });

    Ok(Printer::new(
        printer_name,
        black_toner,
        cyan_toner,
        magenta_toner,
        yellow_toner,
    ))
}
