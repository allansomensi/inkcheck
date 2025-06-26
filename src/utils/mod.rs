use crate::error::{AppError, ErrorKind};

/// Parses a string representing an OID into a vector of `u64` values.
///
/// This function takes an OID string.splits it by the dot (`.`) separator, and converts each segment into a `u64`
/// value. If any segment cannot be parsed into a `u64`, it returns a [AppError::InvalidOidFormat] error.
///
/// ## Arguments:
///
/// - `oid`: The OID string to be parsed.
///
/// ## Returns:
///
/// - A `Result<Vec<u64>, AppError>` containing the vector of `u64` values if successful, or an error if parsing fails.
pub fn parse_oid_to_vec(oid: &str) -> Result<Vec<u64>, AppError> {
    if oid.is_empty() {
        return Ok(vec![]);
    }

    oid.split('.')
        .map(|segment| {
            segment
                .parse::<u64>()
                .map_err(|_| AppError::new(ErrorKind::InvalidOidFormat))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_oid_to_vec;

    #[test]
    fn test_parse_oid_to_vec() {
        let oids = [
            (
                "1.3.6.1.2.1.25.3.2.1.3.1",
                vec![1, 3, 6, 1, 2, 1, 25, 3, 2, 1, 3, 1],
            ),
            (
                "1.3.6.1.2.1.4.21.1.11.169.254.0.0",
                vec![1, 3, 6, 1, 2, 1, 4, 21, 1, 11, 169, 254, 0, 0],
            ),
            (
                "1.3.6.1.4.1.2699.1.2.1.2.1.1.7.1",
                vec![1, 3, 6, 1, 4, 1, 2699, 1, 2, 1, 2, 1, 1, 7, 1],
            ),
            (
                "1.3.6.1.2.1.43.11.1.1.9.1.4",
                vec![1, 3, 6, 1, 2, 1, 43, 11, 1, 1, 9, 1, 4],
            ),
            (
                "1.3.6.1.2.1.4.21.1.13.224.0.0.0",
                vec![1, 3, 6, 1, 2, 1, 4, 21, 1, 13, 224, 0, 0, 0],
            ),
        ];

        for (oid, expected_oid_vec) in oids.iter() {
            assert_eq!(parse_oid_to_vec(oid).unwrap(), *expected_oid_vec);
        }
    }
}
