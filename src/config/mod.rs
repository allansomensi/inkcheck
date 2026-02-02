use crate::cli::Args;
use crate::snmp::auth::{cipher::AuthCipher, protocol::AuthProtocol};
use crate::snmp::version::SnmpVersion;
use clap::ValueEnum;
use serde::Deserialize;
use std::path::PathBuf;

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
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth_protocol: Option<String>,
    pub auth_cipher: Option<String>,
    pub timeout: Option<u64>,
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

        let template = r#"# InkCheck Inventory
# Define your printers below to access them via alias.

# Example 1: Standard Printer (SNMP v2c)
[[printers]]
alias = "office"
host = "192.168.1.50"
community = "public"

# Example 2: Secure Printer (SNMP v3)
[[printers]]
alias = "hr-secure"
host = "10.0.0.5"
snmp_version = "v3"
username = "admin"
auth_protocol = "sha1"
auth_cipher = "aes128"
password = "my_secret_pass"
"#;

        // Write file to disk
        std::fs::write(&path, template)?;

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

    if let Some(user) = &config.username {
        args.username = Some(user.clone());
    }

    if let Some(pass) = &config.password {
        args.password = Some(pass.clone());
    }

    if let Some(auth_str) = &config.auth_protocol {
        if let Ok(ap) = AuthProtocol::from_str(auth_str, true) {
            args.auth_protocol = ap;
        }
    }

    if let Some(priv_str) = &config.auth_cipher {
        if let Ok(ac) = AuthCipher::from_str(priv_str, true) {
            args.auth_cipher = ac;
        }
    }

    if let Some(t) = config.timeout {
        args.timeout = t;
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
