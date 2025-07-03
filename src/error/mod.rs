/// An error that can occur in this application.
#[derive(Debug, Clone)]
pub struct AppError {
    kind: ErrorKind,
}

impl AppError {
    /// Creates a new error from an `ErrorKind`.
    pub(crate) fn new(kind: ErrorKind) -> AppError {
        AppError { kind }
    }
}

/// The kind of an error that can occur.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error that occurred as a result of parsing CLI arguments.
    Cli(String),
    /// An I/O error that occurred.
    Io(String),
    /// An DNS resolution error that occurred.
    DnsResolution(String),
    /// An error that occurs when failing to convert an OID.
    OidConversion,
    /// An error that occurs when a type conversion fails.
    TypeMismatch(String),
    /// An error that occurred during a parsing operation.
    Parse(String),
    /// An error that occurred during an SNMP request.
    SnmpRequest(String),
    /// An error for when a requested OID is not found on the device.
    OidNotFound,
    /// An error for when a specified directory is invalid or does not exist.
    InvalidDirectory,
    /// An error for when the contents of a directory cannot be read.
    DirectoryRead,
    /// An error for when an OID string has an invalid format.
    InvalidOidFormat,
    /// An error for when an unsupported SNMP version is used (e.g., v3).
    UnsupportedVersion,
    /// An error for when a printer model is not supported.
    UnsupportedPrinter(String),
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Cli(s) => write!(f, "CLI error: {s}. Please check the provided arguments."),
            ErrorKind::Io(s) => write!(f, "I/O error: {s}."),
            ErrorKind::DnsResolution(s) => write!(f, "DNS resolution failed for '{s}'."),
            ErrorKind::OidConversion => write!(f, "Failed to convert OID. Ensure the format is valid and numeric."),
            ErrorKind::TypeMismatch(s) => write!(f, "Type conversion failed: {s}."),
            ErrorKind::Parse(s) => write!(f, "Parsing error: {s}. Please check the input format."),
            ErrorKind::SnmpRequest(s) => write!(f, "SNMP request failed: {s}. Please verify the network connection and SNMP agent availability."),
            ErrorKind::OidNotFound => write!(f, "OID not found. Verify that the correct OID is being used for the target device."),
            ErrorKind::InvalidDirectory => write!(f, "The specified directory is invalid or does not exist."),
            ErrorKind::DirectoryRead => write!(f, "Failed to read the contents of the specified directory."),
            ErrorKind::InvalidOidFormat => write!(f, "Invalid OID format. Segments must be numeric and separated by dots."),
            ErrorKind::UnsupportedVersion => write!(f, "SNMP v3 is not supported yet. Please use v1 or v2c instead."),
            ErrorKind::UnsupportedPrinter(s) => write!(f, "Printer model '{s}' is not registered. You can manually add the printer and its corresponding OIDs."),
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
