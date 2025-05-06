use supply::{Drums, Fuser, Reservoir, Toners};

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
        supply::{Drum, Toner},
        Drums, Toners,
    };

    #[test]
    fn test_calc_and_update_toner_level_percent() {
        let mut printer = Printer::new(
            String::from("OKI B431"),
            Some(String::from("G0J671679")),
            Toners {
                black_toner: Some(Toner {
                    level: 2800,
                    max_level: 3500,
                    level_percent: None,
                }),
                cyan_toner: Some(Toner {
                    level: 1000,
                    max_level: 3000,
                    level_percent: None,
                }),
                magenta_toner: Some(Toner {
                    level: 2000,
                    max_level: 3000,
                    level_percent: None,
                }),
                yellow_toner: Some(Toner {
                    level: 300,
                    max_level: 3000,
                    level_percent: None,
                }),
            },
            Drums {
                black_drum: None,
                cyan_drum: None,
                magenta_drum: None,
                yellow_drum: None,
            },
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

    #[test]
    fn test_calc_and_update_drum_level_percent() {
        let mut printer = Printer::new(
            String::from("OKI B431"),
            Some(String::from("G0J671679")),
            Toners {
                black_toner: Some(Toner {
                    level: 2800,
                    max_level: 3500,
                    level_percent: None,
                }),
                cyan_toner: None,
                magenta_toner: None,
                yellow_toner: None,
            },
            Drums {
                black_drum: Some(Drum {
                    level: 2800,
                    max_level: 3500,
                    level_percent: None,
                }),
                cyan_drum: Some(Drum {
                    level: 1000,
                    max_level: 3000,
                    level_percent: None,
                }),
                magenta_drum: Some(Drum {
                    level: 2000,
                    max_level: 3000,
                    level_percent: None,
                }),
                yellow_drum: Some(Drum {
                    level: 300,
                    max_level: 3000,
                    level_percent: None,
                }),
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
}
