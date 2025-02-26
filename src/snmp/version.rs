use clap::ValueEnum;
use std::fmt::Display;

/// Represents the different versions of the SNMP.
#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum SnmpVersion {
    V1,
    V2c,
    V3,
}

impl Display for SnmpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1 => write!(f, "v1"),
            Self::V2c => write!(f, "v2c"),
            Self::V3 => write!(f, "v3"),
        }
    }
}

impl Default for SnmpVersion {
    fn default() -> Self {
        Self::V2c
    }
}
