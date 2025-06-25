use supply::{Drums, Fuser, Reservoir, Toners};

pub mod driver;
pub mod load;
pub mod supply;

/// Represents a printer with toner levels and other relevant details.
///
/// This struct stores information about a printer, including its name, brand,
/// model, and the current levels of the toner cartridges (Black, Cyan, Magenta, and Yellow).
/// It also includes the maximum toner levels and the percentage of toner remaining for each color.
pub struct Printer {
    pub name: String,
    pub serial_number: Option<String>,
    pub toners: Toners,
    pub drums: Drums,
    pub fuser: Option<Fuser>,
    pub reservoir: Option<Reservoir>,
}

impl Printer {
    pub fn new(
        name: String,
        serial_number: Option<String>,
        toners: Toners,
        drums: Drums,
        fuser: Option<Fuser>,
        reservoir: Option<Reservoir>,
    ) -> Self {
        Self {
            name,
            serial_number,
            toners,
            drums,
            fuser,
            reservoir,
        }
    }

    pub fn calculate_all_levels(&mut self) {
        self.calc_and_update_toners_level_percent();
        self.calc_and_update_drums_level_percent();
        self.calc_and_update_fuser_level_percent();
        self.calc_and_update_reservoir_level_percent();
    }

    /// Calculates and updates the toner level percentage for each toner color in the struct.
    ///
    /// This function computes the percentage of toner remaining for each color (black, cyan, magenta, and yellow)
    /// by dividing the current toner level by the maximum toner level and multiplying by 100.
    /// If the maximum toner level is zero, the function safely sets the corresponding toner percentage to `None`
    /// to avoid division by zero.
    ///
    /// The calculated percentage is assigned directly to the `level_percent` field of the respective toner color
    /// within the struct.
    pub fn calc_and_update_toners_level_percent(&mut self) {
        let calculate_level_percent = |level: i64, max_level: i64| {
            if max_level == 0 {
                None
            } else {
                Some(
                    (level * 100)
                        .checked_div(max_level)
                        .expect("Error calculating toner level"),
                )
            }
        };

        if let Some(black_toner) = &mut self.toners.black_toner {
            black_toner.level_percent =
                calculate_level_percent(black_toner.level, black_toner.max_level);
        }

        if let Some(cyan_toner) = &mut self.toners.cyan_toner {
            cyan_toner.level_percent =
                calculate_level_percent(cyan_toner.level, cyan_toner.max_level);
        }

        if let Some(magenta_toner) = &mut self.toners.magenta_toner {
            magenta_toner.level_percent =
                calculate_level_percent(magenta_toner.level, magenta_toner.max_level);
        }

        if let Some(yellow_toner) = &mut self.toners.yellow_toner {
            yellow_toner.level_percent =
                calculate_level_percent(yellow_toner.level, yellow_toner.max_level);
        }
    }

    /// Calculates and updates the drum level percentage for each toner color in the struct.
    ///
    /// This function computes the percentage of drum remaining for each color (black, cyan, magenta, and yellow)
    /// by dividing the current drum level by the maximum drum level and multiplying by 100.
    /// If the maximum drum level is zero, the function safely sets the corresponding drum percentage to `None`
    /// to avoid division by zero.
    ///
    /// The calculated percentage is assigned directly to the `level_percent` field of the respective toner color
    /// within the struct.
    pub fn calc_and_update_drums_level_percent(&mut self) {
        let calculate_level_percent = |level: i64, max_level: i64| {
            if max_level == 0 {
                None
            } else {
                Some(
                    (level * 100)
                        .checked_div(max_level)
                        .expect("Error calculating drum level"),
                )
            }
        };

        if let Some(black_drum) = &mut self.drums.black_drum {
            black_drum.level_percent =
                calculate_level_percent(black_drum.level, black_drum.max_level);
        }

        if let Some(cyan_drum) = &mut self.drums.cyan_drum {
            cyan_drum.level_percent = calculate_level_percent(cyan_drum.level, cyan_drum.max_level);
        }

        if let Some(magenta_drum) = &mut self.drums.magenta_drum {
            magenta_drum.level_percent =
                calculate_level_percent(magenta_drum.level, magenta_drum.max_level);
        }

        if let Some(yellow_drum) = &mut self.drums.yellow_drum {
            yellow_drum.level_percent =
                calculate_level_percent(yellow_drum.level, yellow_drum.max_level);
        }
    }

    /// Calculates and updates the fuser level percentage.
    ///
    /// This function computes the percentage of fuser remaining by dividing the current level by the maximum
    /// level and multiplying by 100.
    /// If the maximum fuser level is zero, the function safely sets the percentage to `None`
    /// to avoid division by zero.
    ///
    /// The calculated percentage is assigned directly to the `level_percent` field.
    pub fn calc_and_update_fuser_level_percent(&mut self) {
        let calculate_level_percent = |level: i64, max_level: i64| {
            if max_level == 0 {
                None
            } else {
                Some(
                    (level * 100)
                        .checked_div(max_level)
                        .expect("Error calculating fuser level"),
                )
            }
        };

        if let Some(fuser) = &mut self.fuser {
            fuser.level_percent = calculate_level_percent(fuser.level, fuser.max_level);
        }
    }

    /// Calculates and updates the reservoir level percentage.
    ///
    /// This function computes the percentage of reservoir remaining by dividing the current level by the maximum
    /// level and multiplying by 100.
    /// If the maximum reservoir level is zero, the function safely sets the percentage to `None`
    /// to avoid division by zero.
    ///
    /// The calculated percentage is assigned directly to the `level_percent` field.
    pub fn calc_and_update_reservoir_level_percent(&mut self) {
        let calculate_level_percent = |level: i64, max_level: i64| {
            if max_level == 0 {
                None
            } else {
                Some(
                    (level * 100)
                        .checked_div(max_level)
                        .expect("Error calculating reservoir level"),
                )
            }
        };

        if let Some(reservoir) = &mut self.reservoir {
            reservoir.level_percent = calculate_level_percent(reservoir.level, reservoir.max_level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Printer;
    use crate::printer::{
        supply::{Drum, Fuser, Reservoir, Toner},
        Drums, Toners,
    };

    /// Tests the constructor and field assignments for the Printer struct.
    #[test]
    fn test_printer_constructor() {
        let printer = Printer::new(
            String::from("Constructor Test"),
            Some(String::from("XYZ-123")),
            Toners {
                black_toner: Some(Toner::new(50, 100, None)),
                ..Default::default()
            },
            Drums::default(),
            Some(Fuser::new(75, 100, None)),
            None,
        );

        assert_eq!(printer.name, "Constructor Test");
        assert_eq!(printer.serial_number.unwrap(), "XYZ-123");
        assert!(printer.toners.black_toner.is_some());
        assert!(printer.drums.black_drum.is_none());
        assert!(printer.fuser.is_some());
        assert!(printer.reservoir.is_none());
    }

    /// Tests the percentage calculation for all toner colors.
    #[test]
    fn test_calc_and_update_toner_level_percent() {
        let mut printer = Printer::new(
            String::from("Toner Test Printer"),
            None,
            Toners {
                black_toner: Some(Toner::new(80, 100, None)),
                cyan_toner: Some(Toner::new(33, 100, None)),
                magenta_toner: Some(Toner::new(66, 100, None)),
                yellow_toner: Some(Toner::new(10, 100, None)),
            },
            Drums::default(),
            None,
            None,
        );

        printer.calc_and_update_toners_level_percent();

        assert_eq!(printer.toners.black_toner.unwrap().level_percent, Some(80));
        assert_eq!(printer.toners.cyan_toner.unwrap().level_percent, Some(33));
        assert_eq!(
            printer.toners.magenta_toner.unwrap().level_percent,
            Some(66)
        );
        assert_eq!(printer.toners.yellow_toner.unwrap().level_percent, Some(10));
    }

    /// Tests the percentage calculation for all drum units.
    #[test]
    fn test_calc_and_update_drum_level_percent() {
        let mut printer = Printer::new(
            String::from("Drum Test Printer"),
            None,
            Toners::default(),
            Drums {
                black_drum: Some(Drum::new(80, 100, None)),
                cyan_drum: Some(Drum::new(33, 100, None)),
                magenta_drum: Some(Drum::new(66, 100, None)),
                yellow_drum: Some(Drum::new(10, 100, None)),
            },
            None,
            None,
        );

        printer.calc_and_update_drums_level_percent();

        assert_eq!(printer.drums.black_drum.unwrap().level_percent, Some(80));
        assert_eq!(printer.drums.cyan_drum.unwrap().level_percent, Some(33));
        assert_eq!(printer.drums.magenta_drum.unwrap().level_percent, Some(66));
        assert_eq!(printer.drums.yellow_drum.unwrap().level_percent, Some(10));
    }

    /// Tests the percentage calculation for the fuser unit.
    #[test]
    fn test_calc_and_update_fuser_level_percent() {
        let mut printer = Printer::new(
            String::from("Fuser Test Printer"),
            None,
            Toners::default(),
            Drums::default(),
            Some(Fuser::new(75, 100, None)),
            None,
        );

        printer.calc_and_update_fuser_level_percent();
        assert_eq!(printer.fuser.unwrap().level_percent, Some(75));
    }

    /// Tests the percentage calculation for the waste toner reservoir.
    #[test]
    fn test_calc_and_update_reservoir_level_percent() {
        let mut printer = Printer::new(
            String::from("Reservoir Test Printer"),
            None,
            Toners::default(),
            Drums::default(),
            None,
            Some(Reservoir::new(40, 50, None)),
        );

        printer.calc_and_update_reservoir_level_percent();
        assert_eq!(printer.reservoir.unwrap().level_percent, Some(80));
    }

    /// Tests that percentage calculation handles a max_level of zero gracefully, returning None.
    #[test]
    fn test_calculation_with_zero_max_level() {
        let mut printer = Printer::new(
            String::from("Edge Case Printer"),
            None,
            Toners {
                black_toner: Some(Toner::new(0, 0, None)), // Division by zero case
                ..Default::default()
            },
            Drums::default(),
            None,
            None,
        );

        printer.calc_and_update_toners_level_percent();
        assert_eq!(printer.toners.black_toner.unwrap().level_percent, None);
    }

    /// Tests the master method `calculate_all_levels` to ensure it calls all individual calculation methods.
    #[test]
    fn test_calculate_all_levels_integration() {
        let mut printer = Printer::new(
            String::from("Comprehensive Test"),
            None,
            Toners {
                black_toner: Some(Toner::new(50, 100, None)),
                ..Default::default()
            },
            Drums {
                black_drum: Some(Drum::new(40, 100, None)),
                ..Default::default()
            },
            Some(Fuser::new(30, 100, None)),
            Some(Reservoir::new(20, 100, None)),
        );

        assert!(printer
            .toners
            .black_toner
            .as_ref()
            .unwrap()
            .level_percent
            .is_none());
        assert!(printer
            .drums
            .black_drum
            .as_ref()
            .unwrap()
            .level_percent
            .is_none());

        printer.calculate_all_levels();

        assert_eq!(printer.toners.black_toner.unwrap().level_percent, Some(50));
        assert_eq!(printer.drums.black_drum.unwrap().level_percent, Some(40));
        assert_eq!(printer.fuser.unwrap().level_percent, Some(30));
        assert_eq!(printer.reservoir.unwrap().level_percent, Some(20));
    }
}
