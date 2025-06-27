use crate::printer::supply::CalculateLevel;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Reservoir {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

impl Reservoir {
    pub fn new(level: i64, max_level: i64, level_percent: Option<i64>) -> Self {
        Self {
            level,
            max_level,
            level_percent,
        }
    }
}

impl CalculateLevel for Option<Reservoir> {
    fn calculate_level_percent(&mut self) {
        if let Some(reservoir) = self {
            if reservoir.max_level > 0 {
                reservoir.level_percent = Some((reservoir.level * 100) / reservoir.max_level);
            } else {
                reservoir.level_percent = None;
            }
        }
    }
}
