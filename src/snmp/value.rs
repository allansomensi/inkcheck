use super::{create_snmp_session, SnmpClientParams};
use crate::error::{AppError, ErrorKind};
use snmp2::{Oid, Value};
use tokio::time::timeout;

/// Retrieves a single SNMP value for the specified OID, handling session initialization and retries.
///
/// This function implements automatic recovery for common SNMPv3 handshake issues
/// (Time Synchronization and Engine Discovery) and enforces strict timeouts on
/// network operations to prevent blocking.
pub async fn get_snmp_value<T>(oid: &[u64], ctx: &SnmpClientParams) -> Result<T, AppError>
where
    T: FromSnmpValue,
{
    let mut session = create_snmp_session(ctx).await?;
    let oid_obj = Oid::from(oid).map_err(|_| AppError::new(ErrorKind::OidConversion))?;

    for _ in 1..=ctx.retries {
        let result = timeout(ctx.timeout, session.get(&oid_obj)).await;

        match result {
            // 1. Network Timeout
            Err(_) => continue,

            // 2. SNMP Library Errors (Protocol level)
            Ok(Err(error)) => match error {
                // Auto-recovery: Time synchronization updated internally
                snmp2::Error::AuthUpdated => continue,

                // Auto-recovery: Missing Engine Boots (requires re-discovery)
                snmp2::Error::AuthFailure(snmp2::v3::AuthErrorKind::EngineBootsNotProvided) => {
                    let _ = timeout(ctx.timeout, session.init()).await;
                    continue;
                }

                // Fatal errors
                _ => return Err(AppError::new(ErrorKind::SnmpRequest(error.to_string()))),
            },

            // 3. Successful Response
            Ok(Ok(mut response)) => {
                // Check for logical SNMP errors (e.g., GenErr, NoAccess)
                if response.error_status != 0 {
                    return Err(AppError::new(ErrorKind::SnmpRequest(format!(
                        "SNMP logical error: code {}",
                        response.error_status
                    ))));
                }

                // Extract value
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
