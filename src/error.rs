#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("CLI error: {0}. Ensure the correct arguments are provided.")]
    CliError(#[from] clap::Error),

    #[error("Failed to convert OID: {0}. Ensure the OID format is valid and numeric.")]
    OidConversion(String),

    #[error("Type conversion failed: {0}. The provided value does not match the expected type.")]
    TypeMismatch(String),

    #[error("Parsing error: {0}. Check the input format and try again.")]
    ParseError(String),

    #[error(
        "SNMP request failed: {0}. Verify the network connection and SNMP agent availability."
    )]
    SnmpRequest(String),

    #[error("Connection timed out while attempting to communicate with the SNMP agent. Ensure the agent is reachable and the network is stable.")]
    Timeout,

    #[error(
        "OID '{0}' not found. Verify that the correct OID is being used for the target device."
    )]
    OidNotFound(String),

    #[error("Invalid OID format: '{0}'. OID segments must be numeric and separated by dots.")]
    InvalidOidFormat(String),

    #[error("SNMP v3 is not supported yet. Use SNMP v1 or v2c instead.")]
    UnsupportedVersion,

    #[error("Printer model '{0}' is not registered in the application. You can manually add the printer along with its corresponding OIDs.")]
    UnsupportedPrinter(String),
}
