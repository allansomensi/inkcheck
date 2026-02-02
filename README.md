# Inkcheck 🖨️

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://github.com/allansomensi/inkcheck/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Inkcheck** is a powerful and fast CLI tool built in Rust to monitor printer supply levels via **SNMP**.

It supports both **SNMP v1/v2c** and the secure **SNMP v3**, allowing you to check toners, drums, fusers, and waste reservoirs for a wide range of printers. It works with both IP addresses and Hostnames.

<img width="427" height="493" alt="demo" src="https://github.com/user-attachments/assets/94596f00-497d-4e0d-acb8-71fbcfd11006" />

## 🚀 Features

- **Protocol Support:** SNMP v1, v2c, and **v3**.
- **Hostname Resolution:** Target printers by IP (`192.168.1.10`) or Hostname (`printer_name`).
- **Detailed Supplies:** Checks Toner, Drum, Fuser, and Waste Reservoir levels.
- **Metrics:** Optional display of total, mono, and color impression counts.
- **Extensible:** Uses JSON definitions for generic printer drivers.
- **Automation Ready:** Supports **JSON output** for integration with other monitoring tools.
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

## 🔧 Usage
Run with the following command:

```elixir
inkcheck [IP] [OPTIONS]
```

### Options:

- `-p, --port [PORT]`            - SNMP service port **(default: 161)**
- `-s, --snmp-version [VERSION]` - SNMP version **(default: v2c)**
- `-c, --community [COMMUNITY]`  - SNMP community **(default: public)**
- `-t, --timeout [TIMEOUT]`      - Timeout in seconds **(default: 5)**
- `-u, --username`               - Username **(v3)**
- `-p, --password`               - Password **(v3)**
- `--auth-protocol`              - Auth Protocol **(v3)**
- `--auth-cipher`                - Auth Cipher **(v3)**
- `-m, --metrics`                - Show impression counts (Total/Mono/Color)
- `-e, --extra_supplies`         - Show extra supplies informations
- `-d, --data-dir [DIR]`         - Data directory
- `-o, --output`                 - Output format (**text** or **json**)
- `--theme [THEME]`              - CLI theme **(default: solid)**
- `-h, --help`                   - Display help information
- `-V, --version`                - Display version information

### Example:
To check the supply levels of a printer at `192.168.1.10`, using the `moon` theme, displaying `extra supplies`, and setting a `timeout` of 10 seconds:

```elixir
inkcheck 192.168.1.10 --theme moon -e -t 10
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
