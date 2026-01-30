use crate::printer::supply::CalculateLevel;
use serde::Serialize;

/// Represents the printer's fuser unit, responsible for bonding toner to paper.
///
/// Tracks the current usage level and maximum capacity to determine the remaining lifespan.
#[derive(Clone, Serialize)]
pub struct Fuser {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

impl Fuser {
    /// Creates a new [`Fuser`] instance.
    pub fn new(level: i64, max_level: i64, level_percent: Option<i64>) -> Self {
        Self {
            level,
            max_level,
            level_percent,
        }
    }
}

impl CalculateLevel for Option<Fuser> {
    /// Computes the remaining life percentage based on current and maximum levels.
    ///
    /// Updates `level_percent` only if the fuser exists and `max_level` is valid (greater than zero).
    fn calculate_level_percent(&mut self) {
        if let Some(fuser) = self {
            if fuser.max_level > 0 {
                fuser.level_percent = Some((fuser.level * 100) / fuser.max_level);
            } else {
                fuser.level_percent = None;
            }
        }
    }
}
