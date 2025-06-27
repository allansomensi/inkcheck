use crate::printer::supply::CalculateLevel;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Fuser {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

impl Fuser {
    pub fn new(level: i64, max_level: i64, level_percent: Option<i64>) -> Self {
        Self {
            level,
            max_level,
            level_percent,
        }
    }
}

impl CalculateLevel for Option<Fuser> {
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
