use crate::{
    error::AppError,
    printer::{Printer, Toner},
    snmp::{get_snmp_value, SnmpClientParams},
};

const BLACK_TONER_CODE: u8 = 0x6F;
const CYAN_TONER_CODE: u8 = 0x70;
const MAGENTA_TONER_CODE: u8 = 0x71;
const YELLOW_TONER_CODE: u8 = 0x72;

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

/// This function retrieves toner levels for a Brother printer and returns a Printer object.
///
/// It attempts to read the toner levels for black, cyan, magenta, and yellow toners. If any toner
/// is not found, it will be returned as `None` in the `Printer` struct.
/// If the black toner is not found, an error is returned.
pub fn brother(ctx: &SnmpClientParams, printer_name: String) -> Result<Printer, AppError> {
    let br_info_maintenance_oid = &[1, 3, 6, 1, 4, 1, 2435, 2, 3, 9, 4, 2, 1, 5, 5, 8, 0];

    let bytes = get_snmp_value::<Vec<u8>>(br_info_maintenance_oid, ctx)?;

    let black_toner_percent = find_value_in_brother_bytes(&bytes, BLACK_TONER_CODE)
        .ok_or(AppError::UnsupportedPrinter("Brother".to_string()))?;

    let cyan_toner_percent = find_value_in_brother_bytes(&bytes, CYAN_TONER_CODE);
    let magenta_toner_percent = find_value_in_brother_bytes(&bytes, MAGENTA_TONER_CODE);
    let yellow_toner_percent = find_value_in_brother_bytes(&bytes, YELLOW_TONER_CODE);

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
