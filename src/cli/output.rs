use clap::ValueEnum;
use std::fmt::Display;

/// Defines the available output formats for the application's reporting.
///
/// - `Text`: Human-readable plain text (Default).
/// - `Json`: Machine-readable JSON format for integration with other tools.
#[derive(Debug, Clone, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Json => write!(f, "json"),
        }
    }
}
