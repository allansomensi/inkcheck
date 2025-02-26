use crate::error::AppError;
use include_dir::{include_dir, Dir};
use serde_json::Value;
use std::{fs, path::PathBuf};

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
