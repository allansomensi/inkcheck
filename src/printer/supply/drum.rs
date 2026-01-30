use crate::printer::supply::CalculateLevel;
use serde::Serialize;

/// Represents a single imaging drum unit, tracking its current usage and maximum capacity.
#[derive(Clone, Serialize)]
pub struct Drum {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

impl Drum {
    /// Creates a new [`Drum`] instance.
    pub fn new(level: i64, max_level: i64, level_percent: Option<i64>) -> Self {
        Self {
            level,
            max_level,
            level_percent,
        }
    }
}

/// Aggregates the imaging drums for standard CMYK colors.
///
/// Each slot is optional to support monochrome printers (which only use `black_drum`)
/// or devices where specific color data is unavailable.
#[derive(Default, Clone, Serialize)]
pub struct Drums {
    pub black_drum: Option<Drum>,
    pub cyan_drum: Option<Drum>,
    pub magenta_drum: Option<Drum>,
    pub yellow_drum: Option<Drum>,
}

impl CalculateLevel for Option<Drum> {
    /// Computes the remaining life percentage based on current and maximum levels.
    ///
    /// Updates `level_percent` only if the drum exists and `max_level` is greater than zero
    /// to avoid division by zero errors.
    fn calculate_level_percent(&mut self) {
        if let Some(drum) = self {
            if drum.max_level > 0 {
                drum.level_percent = Some((drum.level * 100) / drum.max_level);
            } else {
                drum.level_percent = None;
            }
        }
    }
}
