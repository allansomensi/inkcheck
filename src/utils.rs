use crate::error::AppError;

/// Parses a string representing an OID into a vector of `u64` values.
///
/// This function takes an OID string, splits it by the dot (`.`) separator, and converts each segment into a `u64`
/// value. If any segment cannot be parsed into a `u64`, it returns a `SnmpError::InvalidOidFormat` error.
///
/// # Arguments:
///
/// - `oid`: The OID string to be parsed (e.g., "1.3.6.1.2.1.25").
///
/// # Returns:
///
/// - A `Result<Vec<u64>, SnmpError>` containing the vector of `u64` values if successful, or an error if parsing fails.
pub fn parse_oid_to_vec(oid: &str) -> Result<Vec<u64>, AppError> {
    oid.split('.')
        .map(|segment| {
            segment
                .parse::<u64>()
                .map_err(|e| AppError::InvalidOidFormat(e.to_string()))
        })
        .collect()
}
