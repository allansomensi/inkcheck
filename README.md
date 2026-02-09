# Inkcheck 🖨️

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.3.0-blue.svg)](https://github.com/allansomensi/inkcheck/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Inkcheck** is a powerful and fast CLI tool built in Rust to monitor printer supply levels via **SNMP**.

It supports both **SNMP v1/v2c/v3**, allowing you to check toners, drums, fusers, and waste reservoirs for a wide range of printers. It works with both IP addresses and Hostnames.

<img width="427" height="493" alt="demo" src="https://github.com/user-attachments/assets/94596f00-497d-4e0d-acb8-71fbcfd11006" />

## 🚀 Features

- **Async Core:** Built on the Tokio runtime for non-blocking, high-performance SNMP requests.
- **Protocol Support:** Full support for SNMP v1, v2c, and v3.
- **Inventory:** Manage your printers via a configuration file and query them by alias.
- **Detailed Supplies:** Checks Toner, Drum, Fuser, and Waste Reservoir levels.
- **Metrics:** Optional display of total, mono, and color impression counts.
- **Automation Ready:** Output data in **JSON** or **CSV** formats for easy integration with spreadsheets and monitoring tools.
- **Robustness:** Configurable timeout and retry logic for unreliable networks.
- **Security:** Granular control over SNMPv3 security levels (AuthPriv, AuthNoPriv, etc.) and context names.
- **Theming:** Multiple visual themes for the terminal.

## 🚀 Getting Started

### Prerequisites
- **Rust** (latest stable version).

### Cloning the Repository
```bash
git clone https://github.com/allansomensi/inkcheck.git
cd inkcheck
```

### Development Setup
To use the development scripts, install `just`:

```elixir
cargo install just
```

## 📦 Installation
To install from the source:

```elixir
cargo install --path .
```

## ⚙️ Configuration (Inventory)
Inkcheck features an inventory system that allows you to save frequently accessed printers.

### 1. **Initialize the configuration:**
Run the following command to create the default configuration file:

```elixir
inkcheck --init
```

This will create an `inkcheck.toml` file in your system's default configuration directory (e.g., `%APPDATA%\inkcheck\config\` on Windows or `~/.config/inkcheck/` on Linux).

### 2. **Add Printers:**
Edit the generated file to add your printers using aliases.

#### Example:

```toml
[[printers]]
alias = "reception"
host = "192.168.1.50"
community = "public"
version = "v2c"
```

### 3. Use the Alias:

```elixir
inkcheck reception
```

## 🔧 Usage
Run with the following command:

```elixir
inkcheck [HOST OR ALIAS] [OPTIONS]
```

### General Options
- `--init`                           - Initialize the configuration file
- `-p, --port [PORT]`                - SNMP service port **(default: 161)**
- `-t, --timeout [SECONDS]`          - Timeout in seconds **(default: 5)**
- `-r, --retries [COUNT]`            - Number of retries for failed requests
- `-m, --metrics`                    - Show impression counts (Total/Mono/Color)
- `-e, --extra_supplies`             - Show extra supplies informations
- `-d, --data-dir [DIR]`             - Data directory
- `-o, --output [FORMAT]`            - Output format **(default: text)**
- `--theme [THEME]`                  - CLI theme **(default: solid)**
- `-h, --help`                       - Display help information
- `-V, --version`                    - Display version information

### SNMP Connection Options
- `-v, --snmp-version [VERSION]`     - SNMP version **(default: v2c)**
- `-c, --community [STRING]`         - SNMP community **(default: public)**

### SNMPv3 Security Options
- `-u, --username`                   - Username
- `-l, --security-level [LEVEL]`     - Security Level **(default: auth-priv)**
- `-n, --context-name`               - Context Name
- `-p, --password`                   - Password
- `-a, --auth-protocol [PROTOCOL]`   - Auth Protocol **(default: SHA1)**
- `-A, --auth-password [PASS]`       - Auth Password
- `-x, --privacy-protocol [PROTOCOL]`- Privacy Protocol **(default: AES128)**
- `-X, --privacy-password [PASS]`    - Privacy Password

### Examples:
To check the supply levels of a printer at `192.168.1.10`, using the `moon` theme, displaying `extra supplies`, and setting a `timeout` of 10 seconds:

```elixir
inkcheck 192.168.1.10 --theme moon -e -t 10
```

Using an Inventory Alias with Metrics:

```elixir
inkcheck hr-printer -m
```

SNMPv3 with **AuthPriv**:

```elixir
inkcheck 10.0.0.5 -v v3 -u admin -l auth-priv -a sha1 -A pass123 -x aes128 -X pass321 -n context123 -t 6 -r 4 -m -e
```

## 🎨 Themes
Supports multiple visual themes for better readability and personalization. Below are the available themes:

- Solid
- Shades
- Moon
- Circles
- Stars
- Vintage
- Diamonds
- Blocks
- Emoji

---

## 🖨️ Tested Printers
Below is a list of printers that have been tested:

- Brother MFC-L6702DW
- Brother DCP-8157DN
- Brother DCP-8152DN
- Brother MFC-7460DN
- Brother MFC-L8900CDW
- Brother DCP-L5652DN
- Brother MFC-J6935DW
- Brother HL-L2360DW
- Brother HL-5350DN
- Brother L2540
- OKI B431
- Xerox C8030
- Epson WF-C5790

If you've tested with another printer model, feel free to contribute by adding it to the list!

---

## 📬 Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.
