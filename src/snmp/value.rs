use super::{create_snmp_session, SnmpClientParams};
use crate::error::{AppError, ErrorKind};
use snmp2::{Oid, Value};

/// Retrieves and converts a single SNMP value for a specific OID.
///
/// Handles session initialization and includes automatic retry logic (up to 3 times)
/// for transient SNMPv3 USM/Security context errors.
pub fn get_snmp_value<T>(oid: &[u64], ctx: &SnmpClientParams) -> Result<T, AppError>
where
    T: for<'a> FromSnmpValue<'a>,
{
    let mut session = match create_snmp_session(ctx) {
        Ok(session) => session,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let oid = Oid::from(oid).map_err(|_| AppError::new(ErrorKind::OidConversion))?;

    let mut retries = 0;
    loop {
        match session.get(&oid) {
            Ok(mut response) => {
                if let Some((_oid, value)) = response.varbinds.next() {
                    return T::from_snmp_value(&value);
                } else {
                    return Err(AppError::new(ErrorKind::OidNotFound));
                }
            }
            Err(e) => {
                let msg = e.to_string();

                if (msg.contains("Security context") || msg.contains("USM")) && retries < 3 {
                    retries += 1;
                    continue;
                }

                return Err(AppError::new(ErrorKind::SnmpRequest(msg)));
            }
        }
    }
}

/// A trait for converting SNMP `Value` types into Rust types.
///
/// This trait defines a method to convert a given SNMP `Value` to a specific Rust type. It allows SNMP data
/// to be easily mapped to the appropriate types in the application.
///
/// The `from_snmp_value` method converts the SNMP `Value` to the implementing type. If the conversion is not possible,
/// it returns an error message.
///
/// ## Implementations
/// The following types implement [FromSnmpValue]:
/// - [i64]: Converts from various integer types ([Value::Integer], [Value::Counter32], [Value::Counter64], [Value::Timeticks]) and attempts to parse numeric strings from [Value::OctetString].
/// - [String]: Converts from an SNMP [Value::OctetString] value.
/// - [Vec<u64>]: Converts from an SNMP [Value::ObjectIdentifier] value, splitting the OID string into individual components.
/// - [Vec<u8>]: Converts from an SNMP [Value::OctetString] bytes.
/// - [u32]: Converts from an SNMP [Value::Unsigned32], [Value::Counter32], or [Value::Timeticks] value.
/// - [u64]: Converts from an SNMP [Value::Counter64] value.
/// - [bool]: Converts from an SNMP [Value::Boolean] value.
pub trait FromSnmpValue<'a>: Sized {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError>;
}

impl<'a> FromSnmpValue<'a> for i64 {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        match value {
            Value::Integer(v) => Ok(*v),
            Value::Counter32(v) | Value::Timeticks(v) => Ok(*v as i64),
            Value::Counter64(v) => Ok(*v as i64),
            Value::OctetString(bytes) => {
                let s = std::str::from_utf8(bytes).map_err(|_| {
                    AppError::new(ErrorKind::TypeMismatch(
                        "Invalid UTF-8 in OctetString".to_string(),
                    ))
                })?;

                s.trim().parse::<i64>().map_err(|_| {
                    AppError::new(ErrorKind::TypeMismatch(format!(
                        "Expected numeric string, but got: '{s}'",
                    )))
                })
            }
            _ => Err(AppError::new(ErrorKind::TypeMismatch(format!(
                "Unsupported type for i64 conversion: {value:?}"
            )))),
        }
    }
}

impl<'a> FromSnmpValue<'a> for String {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::OctetString(v) = value {
            Ok(String::from_utf8_lossy(v).to_string())
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected OctetString, but received a different type".to_string(),
            )))
        }
    }
}

impl<'a> FromSnmpValue<'a> for Vec<u64> {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::ObjectIdentifier(v) = value {
            let oid_string = v.to_string();
            oid_string
                .split('.')
                .map(|s| {
                    s.parse::<u64>().map_err(|_| {
                        AppError::new(ErrorKind::Parse(format!("Failed to parse '{s}' as u64")))
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

impl<'a> FromSnmpValue<'a> for Vec<u8> {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::OctetString(v) = value {
            Ok(v.to_vec())
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected OctetString, but received a different type".to_string(),
            )))
        }
    }
}

impl<'a> FromSnmpValue<'a> for u32 {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        match value {
            Value::Unsigned32(v) | Value::Counter32(v) | Value::Timeticks(v) => Ok(*v),
            _ => Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected Unsigned32, Counter32, or Timeticks, but received a different type"
                    .to_string(),
            ))),
        }
    }
}

impl<'a> FromSnmpValue<'a> for u64 {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::Counter64(v) = value {
            Ok(*v)
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected Counter64, but received a different type".to_string(),
            )))
        }
    }
}

impl<'a> FromSnmpValue<'a> for bool {
    fn from_snmp_value(value: &'a Value<'a>) -> Result<Self, AppError> {
        if let Value::Boolean(v) = value {
            Ok(*v)
        } else {
            Err(AppError::new(ErrorKind::TypeMismatch(
                "Expected Boolean, but received a different type".to_string(),
            )))
        }
    }
}
