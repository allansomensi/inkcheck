/// Enum representing the various errors that can occur in the application.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("CLI error: {0}. Ensure the correct arguments are provided.")]
    CliError(#[from] clap::Error),

    #[error("IO error: {0}.")]
    IoError(#[from] std::io::Error),

    #[error("Failed to convert OID. Ensure the OID format is valid and numeric.")]
    OidConversion,

    #[error("Type conversion failed: {0}. The provided value does not match the expected type.")]
    TypeMismatch(String),

    #[error("Parsing error: {0}. Check the input format and try again.")]
    ParseError(String),

    #[error(
        "SNMP request failed: {0}. Verify the network connection and SNMP agent availability."
    )]
    SnmpRequest(String),

    #[error("OID not found. Verify that the correct OID is being used for the target device.")]
    OidNotFound,

    #[error("The specified directory is invalid or does not exist.")]
    InvalidDirectory,

    #[error("Failed to read the contents of the specified directory.")]
    DirectoryReadError,

    #[error("Invalid OID format. OID segments must be numeric and separated by dots.")]
    InvalidOidFormat,

    #[error("SNMP v3 is not supported yet. Use SNMP v1 or v2c instead.")]
    UnsupportedVersion,

    #[error("Printer model '{0}' is not registered in the application. You can manually add the printer along with its corresponding OIDs.")]
    UnsupportedPrinter(String),
}
