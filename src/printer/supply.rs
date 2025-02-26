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

#[derive(Clone)]
pub struct Toners {
    pub black_toner: Toner,
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

#[derive(Clone)]
pub struct Drums {
    pub black_drum: Option<Drum>,
    pub cyan_drum: Option<Drum>,
    pub magenta_drum: Option<Drum>,
    pub yellow_drum: Option<Drum>,
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

#[derive(Clone)]
pub struct Reservoir {
    pub level: i64,
    pub max_level: i64,
    pub level_percent: Option<i64>,
}
