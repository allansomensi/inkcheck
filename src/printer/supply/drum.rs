use crate::printer::supply::CalculateLevel;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Drum {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

impl Drum {
    pub fn new(level: i64, max_level: i64, level_percent: Option<i64>) -> Self {
        Self {
            level,
            max_level,
            level_percent,
        }
    }
}

#[derive(Default, Clone, Serialize)]
pub struct Drums {
    pub black_drum: Option<Drum>,
    pub cyan_drum: Option<Drum>,
    pub magenta_drum: Option<Drum>,
    pub yellow_drum: Option<Drum>,
}

impl CalculateLevel for Option<Drum> {
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
