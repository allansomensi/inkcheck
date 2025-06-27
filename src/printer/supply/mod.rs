pub mod drum;
pub mod fuser;
pub mod reservoir;
pub mod toner;

/// Represents the different types of supplies.
pub enum PrinterSupply {
    Toner,
    Drum,
    Fuser,
    Reservoir,
}

impl std::fmt::Display for PrinterSupply {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
