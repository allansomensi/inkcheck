use crate::error::AppError;
use include_dir::{include_dir, Dir};
use serde_json::Value;
use std::{
    fmt::{self, Display, Formatter},
    fs,
    path::PathBuf,
};

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
    pub cyan_toner: Option<Toner>,
    pub magenta_toner: Option<Toner>,
    pub yellow_toner: Option<Toner>,
}

impl Printer {
    pub fn new(
        name: String,
        black_toner: Toner,
        cyan_toner: Option<Toner>,
        magenta_toner: Option<Toner>,
        yellow_toner: Option<Toner>,
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

        if let Some(cyan_toner) = &mut self.cyan_toner {
            cyan_toner.level_percent =
                calculate_level_percent(cyan_toner.level, cyan_toner.max_level);
        }

        if let Some(magenta_toner) = &mut self.magenta_toner {
            magenta_toner.level_percent =
                calculate_level_percent(magenta_toner.level, magenta_toner.max_level);
        }

        if let Some(yellow_toner) = &mut self.yellow_toner {
            yellow_toner.level_percent =
                calculate_level_percent(yellow_toner.level, yellow_toner.max_level);
        }
    }
}

/// A static directory containing printer json files.
static INTERNAL_DATA_DIR: Dir = include_dir!("src/data");

/// Loads printer data from a JSON file based on the brand and model.
///
/// This function searches for a JSON file corresponding to the specified printer brand in the given `data_dir` directory.
/// If no directory is provided, it defaults to using the `INTERNAL_DATA_DIR` static directory.
/// Once a matching file is found, it loads the JSON data and returns the value associated with the specified model.
pub fn load_printer(
    brand: &str,
    model: &str,
    data_dir: Option<PathBuf>,
) -> Result<Value, AppError> {
    let brand_lower = brand.to_lowercase();

    let search_dir = match data_dir {
        Some(ref path) => path.clone(),
        None => PathBuf::from(INTERNAL_DATA_DIR.path()),
    };

    if data_dir.is_some() && !search_dir.is_dir() {
        return Err(AppError::InvalidDirectory);
    }

    if data_dir.is_some() {
        fs::read_dir(&search_dir)
            .map_err(|_| AppError::DirectoryReadError)?
            .filter_map(Result::ok)
            .find(|entry| {
                entry.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                    && entry
                        .path()
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .map(|s| s.to_lowercase())
                        == Some(brand_lower.clone())
            })
            .and_then(|entry| fs::read_to_string(entry.path()).ok())
            .and_then(|json_str| serde_json::from_str::<Value>(&json_str).ok())
            .and_then(|json| json.get(model).cloned())
            .ok_or_else(|| AppError::UnsupportedPrinter(model.to_string()))
    } else {
        INTERNAL_DATA_DIR
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
}

#[cfg(test)]
mod tests {
    use super::{Printer, Toner};

    #[test]
    fn test_calc_and_update_toner_level_percent() {
        let mut printer = Printer::new(
            String::from("OKI B431"),
            Toner {
                level: 2800,
                max_level: 3500,
                level_percent: None,
            },
            Some(Toner {
                level: 1000,
                max_level: 3000,
                level_percent: None,
            }),
            Some(Toner {
                level: 2000,
                max_level: 3000,
                level_percent: None,
            }),
            Some(Toner {
                level: 300,
                max_level: 3000,
                level_percent: None,
            }),
        );

        printer.calc_and_update_toners_level_percent();

        assert_eq!(printer.black_toner.level_percent, Some(80));
        assert_eq!(printer.cyan_toner.unwrap().level_percent, Some(33));
        assert_eq!(printer.magenta_toner.unwrap().level_percent, Some(66));
        assert_eq!(printer.yellow_toner.unwrap().level_percent, Some(10));
    }
}
