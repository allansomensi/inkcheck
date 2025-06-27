use crate::printer::supply::CalculateLevel;
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Default, Clone, Serialize)]
pub struct Toners {
    pub black_toner: Option<Toner>,
    pub cyan_toner: Option<Toner>,
    pub magenta_toner: Option<Toner>,
    pub yellow_toner: Option<Toner>,
}

#[derive(Clone, Serialize)]
pub struct Toner {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}

impl Toner {
    pub fn new(level: i64, max_level: i64, level_percent: Option<i64>) -> Self {
        Self {
            level,
            max_level,
            level_percent,
        }
    }
}

impl CalculateLevel for Option<Toner> {
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

/// Represents the different colors of toner cartridges.
pub enum TonerColor {
    Black,
    Cyan,
    Magenta,
    Yellow,
}

impl Display for TonerColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::Cyan => write!(f, "Cyan"),
            Self::Magenta => write!(f, "Magenta"),
            Self::Yellow => write!(f, "Yellow"),
        }
    }
}
