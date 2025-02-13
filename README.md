# InkCheck

**InkCheck** is a CLI tool to quickly check the status of printer supplies via the command line. Written in Rust.

# Getting Started 🎯
## Prerequisites:

- **Rust** *(latest stable version)*

## Dev

``` bash
git clone https://github.com/allansomensi/inkcheck.git
cd inkcheck
```

For the scripts:
``` elixir
cargo install just
```

## Installation

``` elixir
cargo install --path .
```

## Usage
``` elixir
inkcheck [IP] [OPTIONS]
```

**Options:**
  - `--port` *[PORT]*            SNMP Service Port **[default: 161]**
  - `--snmp-version` *[VERSION]* SNMP Version **[default: v2c]**
  - `--community` *[COMMUNITY]*  SNMP Community **[default: public]**
  - `--timeout` *[TIMEOUT]*      Timeout in seconds **[default: 5]**
  - `--data-dir` *[DIR]*         Data directory
  - `--theme` *[THEME]*          CLI theme **[default: solid]**
  - `-h, --help`               Print help
  - `-V, --version`            Print version

### Example

> This will fetch the toner levels from the printer at IP **192.168.1.10**, using the moon theme, and a timeout of 10 seconds.
``` elixir
cargo run 192.168.1.10 --theme moon --timeout 10
```

## Themes
The available themes are:

### Solid:
![solid](https://github.com/user-attachments/assets/ee5dbe47-62e2-475d-8e53-b4b1626469c5)

### Shades
![shades](https://github.com/user-attachments/assets/928e1445-40e1-4b8c-96b9-a7a1c2cdecf4)

### Moon
![moon](https://github.com/user-attachments/assets/f5e96cfc-4ea4-4100-bb45-8985af1bb430)

### Circles
![circles](https://github.com/user-attachments/assets/cff648b6-50d9-42db-9e07-233c2403013a)

### Stars
![stars](https://github.com/user-attachments/assets/5ebe3510-1293-42c5-a3a0-a44661c91651)

### Vintage:
![vintage](https://github.com/user-attachments/assets/6baf98d5-4425-42de-88b0-5d956aadcdd0)

### Diamonds
![diamonds](https://github.com/user-attachments/assets/2d60d616-e729-4d1a-8daa-3f4c103cbed0)

### Blocks
![blocks](https://github.com/user-attachments/assets/b6950cb6-e365-4d20-a2b1-67a4f8c25206)

### Emoji
![emoji](https://github.com/user-attachments/assets/48e190b8-c18b-4c0b-8314-bfb36105a82a)
