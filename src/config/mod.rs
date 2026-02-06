use crate::cli::args::Args;
use crate::snmp::security::{AuthProtocol, PrivacyProtocol, SecurityLevel};
use crate::snmp::version::SnmpVersion;
use clap::ValueEnum;
use serde::Deserialize;
use std::path::PathBuf;

const CONFIG_TEMPLATE: &str = include_str!("../../assets/config.template.toml");

/// Represents the main application configuration containing a list of printers.
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub printers: Vec<PrinterConfig>,
}

/// Holds network and SNMP configuration details for a single printer.
#[derive(Debug, Deserialize, Clone)]
pub struct PrinterConfig {
    pub alias: String,
    pub host: String,
    pub port: Option<u16>,
    pub snmp_version: Option<String>,
    pub community: Option<String>,
    pub security_level: Option<String>,
    pub context_name: Option<String>,
    pub username: Option<String>,
    pub auth_password: Option<String>,
    pub privacy_password: Option<String>,
    pub auth_protocol: Option<String>,
    pub privacy_protocol: Option<String>,
    pub extra_supplies: Option<bool>,
    pub metrics: Option<bool>,
    pub timeout: Option<u64>,
    pub retries: Option<u8>,
}

impl Config {
    /// Loads the configuration from the system's standard config directory.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = get_config_path();
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Finds a printer configuration by its alias.
    pub fn find_by_alias(&self, target: &str) -> Option<&PrinterConfig> {
        self.printers.iter().find(|p| p.alias == target)
    }

    /// Creates a default configuration file if it does not exist.
    pub fn create_default_template() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = get_config_path();

        // Check if file already exists to avoid overwriting
        if path.exists() {
            return Err(format!("Configuration file already exists at: {path:?}").into());
        }

        // Ensure the directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write file to disk
        std::fs::write(&path, CONFIG_TEMPLATE)?;

        Ok(path)
    }
}

/// Applies the configuration from the TOML file into the CLI Args struct.
/// This modifies the `args` in place, overriding CLI defaults.
pub fn apply_config_to_args(args: &mut Args, config: &PrinterConfig) {
    // Host is required in the config
    args.host = Some(config.host.clone());

    if let Some(port) = config.port {
        args.port = port;
    }

    if let Some(v_str) = &config.snmp_version {
        if let Ok(v) = SnmpVersion::from_str(v_str, true) {
            args.snmp_version = v;
        }
    }

    if let Some(comm) = &config.community {
        args.community = comm.clone();
    }

    if let Some(sec_str) = &config.security_level {
        if let Ok(security_level) = SecurityLevel::from_str(sec_str, true) {
            args.security_level = security_level;
        }
    }

    if let Some(username) = &config.username {
        args.username = Some(username.clone());
    }

    if let Some(context_name) = &config.context_name {
        args.context_name = context_name.clone();
    }

    if let Some(auth_password) = &config.auth_password {
        args.auth_password = Some(auth_password.clone());
    }

    if let Some(privacy_password) = &config.privacy_password {
        args.privacy_password = Some(privacy_password.clone());
    }

    if let Some(auth_str) = &config.auth_protocol {
        if let Ok(auth_protocol) = AuthProtocol::from_str(auth_str, true) {
            args.auth_protocol = auth_protocol;
        }
    }

    if let Some(priv_str) = &config.privacy_protocol {
        if let Ok(privacy_protocol) = PrivacyProtocol::from_str(priv_str, true) {
            args.privacy_protocol = privacy_protocol;
        }
    }

    if let Some(extra) = config.extra_supplies {
        args.extra_supplies = extra;
    }

    if let Some(metrics) = config.metrics {
        args.metrics = metrics;
    }

    if let Some(timeout) = config.timeout {
        args.timeout = timeout;
    }

    if let Some(retries) = config.retries {
        args.retries = retries;
    }
}

/// Resolves the configuration path based on the OS standard.
fn get_config_path() -> PathBuf {
    use directories::ProjectDirs;

    // Windows: %APPDATA%\allansomensi\inkcheck\config\inkcheck.toml
    // Linux: ~/.config/inkcheck/inkcheck.toml
    // macOS: ~/Library/Application Support/com.allansomensi.inkcheck/inkcheck.toml
    if let Some(proj_dirs) = ProjectDirs::from("com", "allansomensi", "inkcheck") {
        return proj_dirs.config_dir().join("inkcheck.toml");
    }

    // Fallback to current directory
    PathBuf::from("inkcheck.toml")
}
