use clap::ValueEnum;
use std::fmt::Display;

/// Represents the different versions of the SNMP.
#[derive(Copy, Clone, ValueEnum, Debug, Default)]
pub enum SnmpVersion {
    V1,
    #[default]
    V2c,
    V3,
}

impl Display for SnmpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("variant not skipped")
            .get_name()
            .fmt(f)
    }
}
