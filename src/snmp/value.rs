use super::{SnmpClientParams, create_snmp_session};
use crate::error::{AppError, ErrorKind};
use snmp2::{Oid, Value};
use tokio::time::timeout;

/// Retrieves a single SNMP value for the specified OID, handling session initialization and retries.
///
/// On each network timeout, the session is recreated to recover from dead or stale
/// connections. SNMPv3 handshake errors (time sync, engine discovery) trigger targeted
/// recovery without consuming a retry attempt.
pub async fn get_snmp_value<T>(oid: &[u64], ctx: &SnmpClientParams) -> Result<T, AppError>
where
    T: FromSnmpValue,
{
    let mut session = create_snmp_session(ctx).await?;
    let oid_obj = Oid::from(oid).map_err(|_| AppError::new(ErrorKind::OidConversion))?;

    for attempt in 1..=ctx.retries {
        let result = timeout(ctx.timeout, session.get(&oid_obj)).await;

        match result {
            // 1. Network timeout — recreate the session to recover from dead connections.
            //    The recreation is itself guarded by the same timeout. Any failure is
            //    intentionally ignored: the next iteration will timeout again and
            //    exhaust the retry budget naturally.
            Err(_) => {
                if attempt < ctx.retries
                    && let Ok(Ok(new_session)) =
                        timeout(ctx.timeout, create_snmp_session(ctx)).await
                {
                    session = new_session
                }
                continue;
            }

            // 2. SNMP library errors (protocol level)
            Ok(Err(error)) => match error {
                // Auto-recovery: time synchronization updated internally by the library
                snmp2::Error::AuthUpdated => continue,

                // Auto-recovery: missing engine boots requires re-discovery
                snmp2::Error::AuthFailure(snmp2::v3::AuthErrorKind::EngineBootsNotProvided) => {
                    let _ = timeout(ctx.timeout, session.init()).await;
                    continue;
                }

                // Fatal protocol errors — no point retrying
                _ => return Err(AppError::new(ErrorKind::SnmpRequest(error.to_string()))),
            },

            // 3. Successful response
            Ok(Ok(mut response)) => {
                if response.error_status != 0 {
                    return Err(AppError::new(ErrorKind::SnmpRequest(format!(
                        "SNMP logical error: code {}",
                        response.error_status
                    ))));
                }

                return match response.varbinds.next() {
                    Some((_, value)) => T::from_snmp_value(value),
                    None => Err(AppError::new(ErrorKind::OidNotFound)),
                };
            }
        }
    }

    Err(AppError::new(ErrorKind::SnmpRequest(
        "Max retries exceeded or connection timed out".to_string(),
    )))
}

/// A trait for converting SNMP [`Value`] types into Rust types.
///
/// This trait defines a method to consume a given SNMP [`Value`] and convert it to a specific Rust type.
/// It allows SNMP data to be easily mapped to the appropriate types in the application.
///
/// The `from_snmp_value` method takes ownership of the [`Value`].
pub trait FromSnmpValue: Sized {
    fn from_snmp_value(val: Value) -> Result<Self, AppError>;
}

impl FromSnmpValue for i64 {
    fn from_snmp_value(value: Value) -> Result<Self, AppError> {
        match value {
            Value::Integer(v) => Ok(v),
            Value::Counter32(v) | Value::Timeticks(v) => Ok(v as i64),
            Value::Counter64(v) => Ok(v as i64),
            Value::OctetString(bytes) => {
                let s = String::from_utf8(bytes.to_vec()).map_err(|_| {
                    AppError::new(ErrorKind::TypeMismatch(
                        "Invalid UTF-8 in OctetString for numeric parsing".to_string(),
                    ))
                })?;

                s.trim().parse::<i64>().map_err(|_| {
                    AppError::new(ErrorKind::TypeMismatch(format!(
                        "Expected numeric string, but got: '{s}'"
                    )))
                })
            }
            _ => Err(AppError::new(ErrorKind::TypeMismatch(format!(
                "Unsupported type for i64 conversion: {value:?}"
            )))),
        }
    }
}

impl FromSnmpValue for String {
    fn from_snmp_value(value: Value) -> Result<Self, AppError> {
        match value {
            Value::OctetString(bytes) => Ok(String::from_utf8_lossy(bytes).to_string()),
            Value::Integer(i) => Ok(i.to_string()),
            _ => Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected OctetString, but received a different type".to_string(),
            ))),
        }
    }
}

impl FromSnmpValue for Vec<u64> {
    fn from_snmp_value(value: Value) -> Result<Self, AppError> {
        if let Value::ObjectIdentifier(oid) = value {
            let oid_string = oid.to_string();
            oid_string
                .split('.')
                .map(|s| {
                    s.parse::<u64>().map_err(|_| {
                        AppError::new(ErrorKind::Parse(format!(
                            "Failed to parse OID component '{s}'"
                        )))
                    })
                })
                .collect()
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected ObjectIdentifier, but received a different type".to_string(),
            )))
        }
    }
}

impl FromSnmpValue for Vec<u8> {
    fn from_snmp_value(value: Value) -> Result<Self, AppError> {
        if let Value::OctetString(bytes) = value {
            Ok(bytes.to_vec())
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected OctetString, but received a different type".to_string(),
            )))
        }
    }
}

impl FromSnmpValue for u32 {
    fn from_snmp_value(value: Value) -> Result<Self, AppError> {
        match value {
            Value::Unsigned32(v) | Value::Counter32(v) | Value::Timeticks(v) => Ok(v),
            _ => Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected Unsigned32, Counter32, or Timeticks".to_string(),
            ))),
        }
    }
}

impl FromSnmpValue for u64 {
    fn from_snmp_value(value: Value) -> Result<Self, AppError> {
        if let Value::Counter64(v) = value {
            Ok(v)
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected Counter64".to_string(),
            )))
        }
    }
}

impl FromSnmpValue for bool {
    fn from_snmp_value(value: Value) -> Result<Self, AppError> {
        if let Value::Boolean(v) = value {
            Ok(v)
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected Boolean".to_string(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snmp2::{Oid, Value};

    mod i64_conversion {
        use super::*;

        #[test]
        fn from_integer_positive() {
            assert_eq!(i64::from_snmp_value(Value::Integer(42)).unwrap(), 42);
        }

        #[test]
        fn from_integer_negative() {
            assert_eq!(i64::from_snmp_value(Value::Integer(-1)).unwrap(), -1);
        }

        #[test]
        fn from_integer_zero() {
            assert_eq!(i64::from_snmp_value(Value::Integer(0)).unwrap(), 0);
        }

        #[test]
        fn from_counter32() {
            assert_eq!(
                i64::from_snmp_value(Value::Counter32(1_000)).unwrap(),
                1_000
            );
        }

        #[test]
        fn from_counter32_max() {
            assert_eq!(
                i64::from_snmp_value(Value::Counter32(u32::MAX)).unwrap(),
                u32::MAX as i64
            );
        }

        #[test]
        fn from_timeticks() {
            assert_eq!(i64::from_snmp_value(Value::Timeticks(500)).unwrap(), 500);
        }

        #[test]
        fn from_counter64() {
            assert_eq!(
                i64::from_snmp_value(Value::Counter64(999_999)).unwrap(),
                999_999
            );
        }

        #[test]
        fn from_counter64_wraps_on_overflow() {
            assert_eq!(
                i64::from_snmp_value(Value::Counter64(u64::MAX)).unwrap(),
                -1
            );
        }

        #[test]
        fn from_octet_string_valid() {
            let val = Value::OctetString(b"1234");
            assert_eq!(i64::from_snmp_value(val).unwrap(), 1234);
        }

        #[test]
        fn from_octet_string_with_surrounding_whitespace() {
            let val = Value::OctetString(b"  42  ");
            assert_eq!(i64::from_snmp_value(val).unwrap(), 42);
        }

        #[test]
        fn from_octet_string_negative_number() {
            let val = Value::OctetString(b"-7");
            assert_eq!(i64::from_snmp_value(val).unwrap(), -7);
        }

        #[test]
        fn from_octet_string_non_numeric_returns_error() {
            let val = Value::OctetString(b"not_a_number");
            assert!(i64::from_snmp_value(val).is_err());
        }

        #[test]
        fn from_octet_string_empty_returns_error() {
            let val = Value::OctetString(b"");
            assert!(i64::from_snmp_value(val).is_err());
        }

        #[test]
        fn from_octet_string_invalid_utf8_returns_error() {
            let val = Value::OctetString(&[0xFF, 0xFE]);
            assert!(i64::from_snmp_value(val).is_err());
        }

        #[test]
        fn type_mismatch_returns_error() {
            assert!(i64::from_snmp_value(Value::Boolean(true)).is_err());
        }
    }

    mod string_conversion {
        use super::*;

        #[test]
        fn from_octet_string_ascii() {
            let val = Value::OctetString(b"Brother MFC-L8900CDW");
            assert_eq!(
                String::from_snmp_value(val).unwrap(),
                "Brother MFC-L8900CDW"
            );
        }

        #[test]
        fn from_octet_string_empty() {
            let val = Value::OctetString(b"");
            assert_eq!(String::from_snmp_value(val).unwrap(), "");
        }

        #[test]
        fn from_octet_string_lossy_on_invalid_utf8() {
            // Non-UTF-8 bytes must not return an error.
            let val = Value::OctetString(&[0xFF, 0xFE, b'X']);
            assert!(String::from_snmp_value(val).is_ok());
        }

        #[test]
        fn from_octet_string_with_null_terminator() {
            // Some SNMP devices return strings with a trailing null byte.
            let val = Value::OctetString(b"Brother MFC\x00");
            let result = String::from_snmp_value(val).unwrap();
            assert!(result.contains("Brother MFC"));
        }

        #[test]
        fn from_integer_positive() {
            assert_eq!(String::from_snmp_value(Value::Integer(42)).unwrap(), "42");
        }

        #[test]
        fn from_integer_negative() {
            assert_eq!(String::from_snmp_value(Value::Integer(-5)).unwrap(), "-5");
        }

        #[test]
        fn type_mismatch_returns_error() {
            assert!(String::from_snmp_value(Value::Boolean(true)).is_err());
        }
    }

    mod vec_u8_conversion {
        use super::*;

        #[test]
        fn from_octet_string() {
            let data: &[u8] = &[0x41, 0x01, 0x04, 0x00, 0xFF];
            let val = Value::OctetString(data);
            assert_eq!(Vec::<u8>::from_snmp_value(val).unwrap(), data);
        }

        #[test]
        fn from_octet_string_empty() {
            let val = Value::OctetString(&[]);
            assert_eq!(Vec::<u8>::from_snmp_value(val).unwrap(), Vec::<u8>::new());
        }

        #[test]
        fn type_mismatch_returns_error() {
            assert!(Vec::<u8>::from_snmp_value(Value::Integer(1)).is_err());
        }
    }

    mod vec_u64_conversion {
        use super::*;

        #[test]
        fn from_object_identifier_round_trip() {
            let components: &[u64] = &[1, 3, 6, 1, 2, 1, 25, 3, 2, 1, 3, 1];
            let oid = Oid::from(components).unwrap();
            let result = Vec::<u64>::from_snmp_value(Value::ObjectIdentifier(oid)).unwrap();
            assert_eq!(
                result, components,
                "OID round-trip must preserve all components exactly"
            );
        }

        #[test]
        fn type_mismatch_returns_error() {
            assert!(Vec::<u64>::from_snmp_value(Value::Integer(1)).is_err());
        }
    }

    mod u32_conversion {
        use super::*;

        #[test]
        fn from_unsigned32() {
            assert_eq!(u32::from_snmp_value(Value::Unsigned32(42)).unwrap(), 42);
        }

        #[test]
        fn from_counter32() {
            assert_eq!(u32::from_snmp_value(Value::Counter32(100)).unwrap(), 100);
        }

        #[test]
        fn from_timeticks() {
            assert_eq!(u32::from_snmp_value(Value::Timeticks(200)).unwrap(), 200);
        }

        #[test]
        fn type_mismatch_returns_error() {
            assert!(u32::from_snmp_value(Value::Integer(1)).is_err());
        }
    }

    mod u64_conversion {
        use super::*;

        #[test]
        fn from_counter64() {
            assert_eq!(
                u64::from_snmp_value(Value::Counter64(999_999)).unwrap(),
                999_999
            );
        }

        #[test]
        fn from_counter64_max() {
            assert_eq!(
                u64::from_snmp_value(Value::Counter64(u64::MAX)).unwrap(),
                u64::MAX
            );
        }

        #[test]
        fn type_mismatch_returns_error() {
            assert!(u64::from_snmp_value(Value::Integer(1)).is_err());
        }
    }

    mod bool_conversion {
        use super::*;

        #[test]
        fn from_boolean_true() {
            assert_eq!(bool::from_snmp_value(Value::Boolean(true)).unwrap(), true);
        }

        #[test]
        fn from_boolean_false() {
            assert_eq!(bool::from_snmp_value(Value::Boolean(false)).unwrap(), false);
        }

        #[test]
        fn type_mismatch_returns_error() {
            assert!(bool::from_snmp_value(Value::Integer(1)).is_err());
        }
    }
}
