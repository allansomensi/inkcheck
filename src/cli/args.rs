use crate::{
    cli::{output::OutputFormat, theme::CliTheme},
    snmp::{
        security::{AuthProtocol, PrivacyProtocol, SecurityLevel},
        version::SnmpVersion,
    },
};
use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments structure.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// IP address, hostname, or configuration alias of the printer.
    #[arg(required_unless_present = "init")]
    pub host: Option<String>,

    /// Initialize a default configuration file.
    #[arg(long, exclusive = true)]
    pub init: bool,

    /// Protocol Version.
    #[arg(short = 'v', long, default_value_t = SnmpVersion::V2c, help_heading = "SNMP General")]
    pub snmp_version: SnmpVersion,

    /// SNMP Service Port.
    #[arg(
        short = 'p',
        long,
        default_value_t = 161,
        help_heading = "SNMP General"
    )]
    pub port: u16,

    /// Community String.
    #[arg(
        short = 'c',
        long,
        default_value = "public",
        help_heading = "SNMP v1/v2c"
    )]
    pub community: String,

    /// Username.
    #[arg(short = 'u', long, help_heading = "SNMPv3")]
    pub username: Option<String>,

    /// Security Level.
    #[arg(short = 'l', long, default_value_t = SecurityLevel::default(), help_heading = "SNMPv3")]
    pub security_level: SecurityLevel,

    /// Authentication Protocol.
    #[arg(short = 'a', long, default_value_t = AuthProtocol::default(), help_heading = "SNMPv3")]
    pub auth_protocol: AuthProtocol,

    /// Authentication Password.
    #[arg(short = 'A', long, help_heading = "SNMPv3")]
    pub auth_password: Option<String>,

    /// Privacy Protocol.
    #[arg(short = 'x', long, default_value_t = PrivacyProtocol::default(), help_heading = "SNMPv3")]
    pub privacy_protocol: PrivacyProtocol,

    /// Privacy Password.
    #[arg(short = 'X', long, help_heading = "SNMPv3")]
    pub privacy_password: Option<String>,

    /// Context Name.
    #[arg(short = 'n', long, default_value_t = String::new(), help_heading = "SNMPv3")]
    pub context_name: String,

    /// Path to a custom data directory.
    #[arg(short = 'd', long)]
    pub data_dir: Option<PathBuf>,

    /// Show extended supply levels (drums, fusers, etc).
    #[arg(short = 'e', long)]
    pub extra_supplies: bool,

    /// Show printer usage metrics (page counts).
    #[arg(short = 'm', long)]
    pub metrics: bool,

    /// Timeout in seconds.
    #[arg(short = 't', long, default_value_t = 5)]
    pub timeout: u64,

    /// Retries.
    #[arg(short = 'r', long, default_value_t = 3)]
    pub retries: u8,

    /// Select the visual theme for the CLI output.
    #[arg(long, default_value_t = CliTheme::default())]
    pub theme: CliTheme,

    /// Select the output format.
    #[arg(short = 'o', long, default_value_t = OutputFormat::default())]
    pub output: OutputFormat,
}
