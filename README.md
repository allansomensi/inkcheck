# InkCheck 🖨️

**InkCheck** is a CLI tool built in Rust that checks printer supply levels via `SNMP`. It provides a fast and efficient way to monitor the status of **toner**, **drum**, **fuser** and other consumables for both **color** and **monochrome** printers directly from the command line.

![preview](https://github.com/user-attachments/assets/97243faf-8140-40cb-b43d-a0953070f4b7)


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
- `-d, --data-dir [DIR]`         - Data directory
- `--theme [THEME]`              - CLI theme **(default: solid)**
- `-e, --extra_supplies`         - Show extra supplies informations
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
