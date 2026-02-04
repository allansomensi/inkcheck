use crate::{
    cli::{output::OutputFormat, resolve_host, theme::CliTheme, AppParams, CliParams},
    config,
    error::AppError,
    snmp::{
        security::{AuthProtocol, PrivacyProtocol, SecurityLevel},
        version::SnmpVersion,
        SnmpClientParams,
    },
};
use clap::Parser;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// IP or hostname of the printer
    #[arg(required_unless_present = "init")]
    pub host: Option<String>,

    /// Create a default configuration file
    #[arg(long)]
    pub init: bool,

    /// SNMP Version
    #[arg(long, default_value_t = SnmpVersion::V2c, help_heading = "SNMP General")]
    pub snmp_version: SnmpVersion,

    /// SNMP Service Port
    #[arg(long, default_value_t = 161, help_heading = "SNMP General")]
    pub port: u16,

    /// Timeout in seconds
    #[arg(short, long, default_value_t = 5, help_heading = "SNMP General")]
    pub timeout: u64,

    /// SNMP Community (v1/v2c)
    #[arg(short, long, default_value = "public", help_heading = "SNMP v1/v2c")]
    pub community: String,

    /// Username
    #[arg(short = 'u', long, help_heading = "SNMPv3")]
    pub username: Option<String>,

    /// Security Level (noAuthNoPriv, authNoPriv, authPriv)
    #[arg(short = 'l', long, default_value_t = SecurityLevel::AuthPriv, help_heading = "SNMPv3")]
    pub security_level: SecurityLevel,

    /// Auth Protocol (MD5, SHA1, SHA224, SHA256, SHA384, SHA512)
    #[arg(short = 'a', long, default_value_t = AuthProtocol::Sha1, help_heading = "SNMPv3")]
    pub auth_protocol: AuthProtocol,

    /// Auth Password
    #[arg(short = 'A', long, help_heading = "SNMPv3")]
    pub auth_password: Option<String>,

    /// Privacy Protocol (DES, AES128, AES192, AES256)
    #[arg(short = 'x', long, default_value_t = PrivacyProtocol::Aes128, help_heading = "SNMPv3")]
    pub privacy_protocol: PrivacyProtocol,

    /// Privacy Password
    #[arg(short = 'X', long, help_heading = "SNMPv3")]
    pub privacy_password: Option<String>,

    /// Data directory
    #[arg(short = 'd', long)]
    pub data_dir: Option<PathBuf>,

    /// Display levels of other supplies (drum, paper, etc.)
    #[arg(short = 'E', long)]
    pub extra_supplies: bool,

    /// Display metrics
    #[arg(short = 'm', long)]
    pub metrics: bool,

    /// CLI theme
    #[arg(long, default_value_t = CliTheme::Solid)]
    pub theme: CliTheme,

    /// Output format
    #[arg(long, short = 'o', default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,
}

/// Capture and return the command line arguments.
pub fn parse_args() -> Result<AppParams, AppError> {
    let mut args = Args::parse();

    if args.init {
        match config::Config::create_default_template() {
            Ok(path) => {
                println!("✅ Configuration file created at: {path:?}");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("❌ Failed to create config file: {e}");
                std::process::exit(1);
            }
        }
    }

    let inventory = config::Config::load().unwrap_or_default();
    let target_input = args.host.clone().unwrap();

    if let Some(saved_printer) = inventory.find_by_alias(&target_input) {
        println!(
            "📂 Loading saved configuration for: '{}'",
            saved_printer.alias
        );
        config::apply_config_to_args(&mut args, saved_printer);
    }

    let final_host = args.host.as_ref().unwrap();
    let resolved_ip = resolve_host(final_host, args.port)?;

    Ok(AppParams {
        app: CliParams {
            theme: args.theme,
            output: args.output,
        },
        snmp: SnmpClientParams {
            ip: resolved_ip,
            port: args.port,
            community: args.community,
            username: args.username,
            auth_password: args.auth_password,
            auth_protocol: args.auth_protocol,
            privacy_password: args.privacy_password,
            privacy_protocol: args.privacy_protocol,
            security_level: args.security_level,
            version: args.snmp_version,
            timeout: args.timeout,
            data_dir: args.data_dir,
            extra_supplies: args.extra_supplies,
            metrics: args.metrics,
        },
    })
}
