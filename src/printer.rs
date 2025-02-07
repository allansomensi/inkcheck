use include_dir::{include_dir, Dir};
use serde_json::Value;

use crate::error::AppError;

/// Represents the different types of supplies.
pub enum PrinterSupply {
    Toner,
    // TODO! Drum
}

impl ToString for PrinterSupply {
    fn to_string(&self) -> String {
        match self {
            Self::Toner => "Toner".to_string(),
            // Self::Drum => "Drum".to_string(),
        }
    }
}

/// Represents the different colors of toner cartridges.
pub enum TonerColor {
    Black,
    Cyan,
    Magenta,
    Yellow,
}

impl ToString for TonerColor {
    fn to_string(&self) -> String {
        match self {
            Self::Black => "Black".to_string(),
            Self::Cyan => "Cyan".to_string(),
            Self::Magenta => "Magenta".to_string(),
            Self::Yellow => "Yellow".to_string(),
        }
    }
}

/// Represents a printer with toner levels and other relevant details.
///
/// This struct stores information about a printer, including its name, brand,
/// model, and the current levels of the toner cartridges (Black, Cyan, Magenta, and Yellow).
/// It also includes the maximum toner levels and the percentage of toner remaining for each color.
pub struct Printer {
    pub name: String,
    pub black_toner_level: i64,
    pub black_toner_max_level: i64,
    pub black_toner_level_percent: Option<i64>,
    pub cyan_toner_level: i64,
    pub cyan_toner_max_level: i64,
    pub cyan_toner_level_percent: Option<i64>,
    pub magenta_toner_level: i64,
    pub magenta_toner_max_level: i64,
    pub magenta_toner_level_percent: Option<i64>,
    pub yellow_toner_level: i64,
    pub yellow_toner_max_level: i64,
    pub yellow_toner_level_percent: Option<i64>,
}

impl Printer {
    pub fn new(
        name: String,
        black_toner_level: i64,
        black_toner_max_level: i64,
        black_toner_level_percent: Option<i64>,
        cyan_toner_level: i64,
        cyan_toner_max_level: i64,
        cyan_toner_level_percent: Option<i64>,
        magenta_toner_level: i64,
        magenta_toner_max_level: i64,
        magenta_toner_level_percent: Option<i64>,
        yellow_toner_level: i64,
        yellow_toner_max_level: i64,
        yellow_toner_level_percent: Option<i64>,
    ) -> Self {
        Self {
            name,
            black_toner_level,
            black_toner_max_level,
            black_toner_level_percent,
            cyan_toner_level,
            cyan_toner_max_level,
            cyan_toner_level_percent,
            magenta_toner_level,
            magenta_toner_max_level,
            magenta_toner_level_percent,
            yellow_toner_level,
            yellow_toner_max_level,
            yellow_toner_level_percent,
        }
    }

    /// Calculates the percentage of toner remaining for a given color.
    ///
    /// This function calculates the percentage of toner left in the specified toner cartridge
    /// by dividing the current toner level by the maximum toner level, and then multiplying by 100.
    /// If the maximum toner level is zero, it returns 0 to prevent division by zero.
    pub fn calc_toner_level_percent(&self, color: TonerColor) -> i64 {
        let (level, max_level) = match color {
            TonerColor::Black => (self.black_toner_level, self.black_toner_max_level),
            TonerColor::Cyan => (self.cyan_toner_level, self.cyan_toner_max_level),
            TonerColor::Magenta => (self.magenta_toner_level, self.magenta_toner_max_level),
            TonerColor::Yellow => (self.yellow_toner_level, self.yellow_toner_max_level),
        };

        if max_level == 0 {
            return 0;
        }

        (level * 100)
            .checked_div(max_level)
            .expect("Error calculating toner level")
    }
}

/// A static directory containing printer json files.
static DATA_DIR: Dir = include_dir!("src/data");

/// Loads printer data from a JSON file based on the brand and model.
///
/// This function searches for a JSON file corresponding to the specified printer brand in the `DATA_DIR` directory.
/// If a file for the given brand is found, it loads the JSON data and returns the value associated with the specified model.
pub fn load_printer(brand: &str, model: &str) -> Result<Value, AppError> {
    let brand_lower = brand.to_lowercase();

    DATA_DIR
        .files()
        .into_iter()
        .find(|file| {
            file.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                && file
                    .path()
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|s| s.to_lowercase())
                    == Some(brand_lower.clone())
        })
        .and_then(|file| file.contents_utf8())
        .and_then(|json_str| serde_json::from_str::<Value>(json_str).ok())
        .and_then(|json| json.get(model).cloned())
        .ok_or_else(|| AppError::UnsupportedPrinter(model.to_string()))
}
