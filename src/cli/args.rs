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
    #[arg(long)]
    pub init: bool,

    /// SNMP Protocol Version.
    #[arg(long, default_value_t = SnmpVersion::V2c, help_heading = "SNMP General")]
    pub snmp_version: SnmpVersion,

    /// SNMP Service Port.
    #[arg(long, default_value_t = 161, help_heading = "SNMP General")]
    pub port: u16,

    /// SNMP Community String.
    #[arg(short, long, default_value = "public", help_heading = "SNMP v1/v2c")]
    pub community: String,

    /// SNMPv3 Username.
    #[arg(short = 'u', long, help_heading = "SNMPv3")]
    pub username: Option<String>,

    /// SNMPv3 Security Level.
    #[arg(short = 'l', long, default_value_t = SecurityLevel::AuthPriv, help_heading = "SNMPv3")]
    pub security_level: SecurityLevel,

    /// SNMPv3 Authentication Protocol.
    #[arg(short = 'a', long, default_value_t = AuthProtocol::Sha1, help_heading = "SNMPv3")]
    pub auth_protocol: AuthProtocol,

    /// SNMPv3 Authentication Password.
    #[arg(short = 'A', long, help_heading = "SNMPv3")]
    pub auth_password: Option<String>,

    /// SNMPv3 Privacy Protocol.
    #[arg(short = 'x', long, default_value_t = PrivacyProtocol::Aes128, help_heading = "SNMPv3")]
    pub privacy_protocol: PrivacyProtocol,

    /// SNMPv3 Privacy Password.
    #[arg(short = 'X', long, help_heading = "SNMPv3")]
    pub privacy_password: Option<String>,

    /// Path to a custom data directory.
    #[arg(short = 'd', long)]
    pub data_dir: Option<PathBuf>,

    /// Show extended supply levels (drums, fusers, etc).
    #[arg(short = 'e', long)]
    pub extra_supplies: bool,

    /// Show printer usage metrics (page counts).
    #[arg(short = 'm', long)]
    pub metrics: bool,

    /// Select the visual theme for the CLI output.
    #[arg(long, default_value_t = CliTheme::Solid)]
    pub theme: CliTheme,

    /// Select the output format.
    #[arg(long, short = 'o', default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,
}
