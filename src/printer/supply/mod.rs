pub mod drum;
pub mod fuser;
pub mod reservoir;
pub mod toner;

/// Categorizes the different types of replaceable printer consumables.
pub enum PrinterSupply {
    Toner,
    Drum,
    Fuser,
    Reservoir,
}

impl std::fmt::Display for PrinterSupply {
    /// Formats the supply type as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Toner => write!(f, "Toner"),
            Self::Drum => write!(f, "Drum"),
            Self::Fuser => write!(f, "Fuser"),
            Self::Reservoir => write!(f, "Reservoir"),
        }
    }
}

/// Defines a common interface for calculating the remaining life percentage of a supply component.
///
/// Implementors should use their internal current and maximum levels to update their percentage field.
pub trait CalculateLevel {
    fn calculate_level_percent(&mut self);
}
