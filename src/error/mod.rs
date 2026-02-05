use std::fmt;

/// A unified error type for the application.
#[derive(Debug)]
pub struct AppError {
    kind: ErrorKind,
}

impl AppError {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Cli(String),
    Io(String),
    DnsResolution(String),
    OidConversion,
    TypeMismatch(String),
    Parse(String),
    SnmpRequest(String),
    OidNotFound,
    InvalidDirectory,
    DirectoryRead,
    InvalidOidFormat,
    UnsupportedPrinter(String),
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Cli(s) => write!(f, "Configuration error: {s}"),
            ErrorKind::Io(s) => write!(f, "I/O error: {s}"),
            ErrorKind::DnsResolution(s) => write!(f, "Could not resolve hostname '{s}'"),
            ErrorKind::OidConversion => write!(f, "OID conversion failed (check numeric format)"),
            ErrorKind::TypeMismatch(s) => write!(f, "Data type mismatch: {s}"),
            ErrorKind::Parse(s) => write!(f, "Failed to parse input: {s}"),
            ErrorKind::SnmpRequest(s) => write!(f, "SNMP communication failed: {s}"),
            ErrorKind::OidNotFound => write!(f, "Requested OID not found on device"),
            ErrorKind::InvalidDirectory => write!(f, "Invalid or inaccessible directory path"),
            ErrorKind::DirectoryRead => write!(f, "Unable to read directory contents"),
            ErrorKind::InvalidOidFormat => {
                write!(f, "Malformed OID (must be numeric/dot-separated)")
            }
            ErrorKind::UnsupportedPrinter(s) => {
                write!(f, "Printer model '{s}' is not officially supported")
            }
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::new(ErrorKind::Io(err.to_string()))
    }
}

impl From<clap::Error> for AppError {
    fn from(err: clap::Error) -> Self {
        AppError::new(ErrorKind::Cli(err.to_string()))
    }
}

impl From<snmp2::Error> for AppError {
    fn from(err: snmp2::Error) -> Self {
        AppError::new(ErrorKind::SnmpRequest(format!("{err:?}")))
    }
}
