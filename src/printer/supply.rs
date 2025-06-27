use core::fmt;
use std::fmt::{Display, Formatter};

/// Represents the different types of supplies.
pub enum PrinterSupply {
    Toner,
    Drum,
    Fuser,
    Reservoir,
}

impl Display for PrinterSupply {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Toner => write!(f, "Toner"),
            Self::Drum => write!(f, "Drum"),
            Self::Fuser => write!(f, "Fuser"),
            Self::Reservoir => write!(f, "Reservoir"),
        }
    }
}

pub trait CalculateLevel {
    fn calculate_level_percent(&mut self);
}

#[derive(Default, Clone)]
pub struct Toners {
    pub black_toner: Option<Toner>,
    pub cyan_toner: Option<Toner>,
    pub magenta_toner: Option<Toner>,
    pub yellow_toner: Option<Toner>,
}

#[derive(Clone)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::Cyan => write!(f, "Cyan"),
            Self::Magenta => write!(f, "Magenta"),
            Self::Yellow => write!(f, "Yellow"),
        }
    }
}

#[derive(Clone)]
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

#[derive(Default, Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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
