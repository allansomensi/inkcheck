use crate::error::AppError;
use include_dir::{include_dir, Dir};
use serde_json::Value;
use std::fmt::{self, Display, Formatter};

/// Represents the different types of supplies.
pub enum PrinterSupply {
    Toner,
    // TODO! Drum
}

impl Display for PrinterSupply {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Toner => write!(f, "Toner"),
            // Self::Drum => write!(f, "Drum"),
        }
    }
}

#[derive(Clone)]
pub struct Toner {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

/// Represents the different colors of toner cartridges.
pub enum TonerColor {
    Black,
    Cyan,
    Magenta,
    Yellow,
}

impl Display for TonerColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::Cyan => write!(f, "Cyan"),
            Self::Magenta => write!(f, "Magenta"),
            Self::Yellow => write!(f, "Yellow"),
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
    pub black_toner: Toner,
    pub cyan_toner: Toner,
    pub magenta_toner: Toner,
    pub yellow_toner: Toner,
}

impl Printer {
    pub fn new(
        name: String,
        black_toner: Toner,
        cyan_toner: Toner,
        magenta_toner: Toner,
        yellow_toner: Toner,
    ) -> Self {
        Self {
            name,
            black_toner,
            cyan_toner,
            magenta_toner,
            yellow_toner,
        }
    }

    /// Calculates and updates the toner level percentage for each toner color in the struct.
    ///
    /// This function computes the percentage of toner remaining for each color (black, cyan, magenta, and yellow)
    /// by dividing the current toner level by the maximum toner level and multiplying by 100.
    /// If the maximum toner level is zero, the function safely sets the corresponding toner percentage to `None`
    /// to avoid division by zero.
    ///
    /// The calculated percentage is assigned directly to the `level_percent` field of the respective toner color
    /// within the struct.
    pub fn calc_and_update_toners_level_percent(&mut self) {
        let calculate_level_percent = |level: i64, max_level: i64| {
            if max_level == 0 {
                None
            } else {
                Some(
                    (level * 100)
                        .checked_div(max_level)
                        .expect("Error calculating toner level"),
                )
            }
        };

        self.black_toner.level_percent =
            calculate_level_percent(self.black_toner.level, self.black_toner.max_level);
        self.cyan_toner.level_percent =
            calculate_level_percent(self.cyan_toner.level, self.cyan_toner.max_level);
        self.magenta_toner.level_percent =
            calculate_level_percent(self.magenta_toner.level, self.magenta_toner.max_level);
        self.yellow_toner.level_percent =
            calculate_level_percent(self.yellow_toner.level, self.yellow_toner.max_level);
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
