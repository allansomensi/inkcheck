use crate::printer::supply::CalculateLevel;
use serde::Serialize;
use std::fmt::{Display, Formatter};

/// Aggregates the four standard CMYK toner cartridges.
///
/// Each field is optional, allowing this structure to represent both monochrome printers and full-color devices.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Toners {
    pub black_toner: Option<Toner>,
    pub cyan_toner: Option<Toner>,
    pub magenta_toner: Option<Toner>,
    pub yellow_toner: Option<Toner>,
}

/// Represents a single toner cartridge, tracking its current fill level and maximum capacity.
#[derive(Debug, Clone, Serialize)]
pub struct Toner {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

impl Toner {
    /// Creates a new [`Toner`] instance.
    pub fn new(level: i64, max_level: i64, level_percent: Option<i64>) -> Self {
        Self {
            level,
            max_level,
            level_percent,
        }
    }
}

impl CalculateLevel for Option<Toner> {
    /// Computes the remaining life percentage based on current and maximum levels.
    ///
    /// Updates `level_percent` only if the toner exists and `max_level` is greater than zero.
    fn calculate_level_percent(&mut self) {
        if let Some(toner) = self {
            if toner.max_level > 0 {
                toner.level_percent = Some((toner.level * 100) / toner.max_level);
            } else {
                toner.level_percent = None;
            }
        }
    }
}

/// Enumerates the standard CMYK color model used for printer toner cartridges.
pub enum TonerColor {
    Black,
    Cyan,
    Magenta,
    Yellow,
}

impl Display for TonerColor {
    /// Formats the toner color as a capitalized human-readable string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::Cyan => write!(f, "Cyan"),
            Self::Magenta => write!(f, "Magenta"),
            Self::Yellow => write!(f, "Yellow"),
        }
    }
}
