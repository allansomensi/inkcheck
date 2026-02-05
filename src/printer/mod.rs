use crate::printer::supply::{
    drum::Drums, fuser::Fuser, reservoir::Reservoir, toner::Toners, CalculateLevel,
};
use serde::Serialize;

pub mod driver;
pub mod load;
pub mod supply;

/// Tracks printing usage statistics including total, monochrome, and color impressions.
#[derive(Serialize, Debug, Default, Clone, Copy)]
pub struct Metrics {
    pub total_impressions: Option<i64>,
    pub mono_impressions: Option<i64>,
    pub color_impressions: Option<i64>,
}

/// Represents the comprehensive state of a printer.
///
/// Aggregates identity, consumable supplies, and usage metrics.
#[derive(Serialize, Debug)]
pub struct Printer {
    pub name: String,
    pub serial_number: Option<String>,
    pub toners: Toners,
    pub drums: Drums,
    pub fuser: Option<Fuser>,
    pub reservoir: Option<Reservoir>,
    pub metrics: Option<Metrics>,
}

impl Printer {
    /// Creates a new [`Printer`] instance.
    pub fn new(
        name: String,
        serial_number: Option<String>,
        toners: Toners,
        drums: Drums,
        fuser: Option<Fuser>,
        reservoir: Option<Reservoir>,
        metrics: Option<Metrics>,
    ) -> Self {
        Self {
            name,
            serial_number,
            toners,
            drums,
            fuser,
            reservoir,
            metrics,
        }
    }

    /// Iterates over all attached supply components and calculates their remaining life percentage.
    pub fn calculate_all_levels(&mut self) {
        // Toners
        self.toners.black_toner.calculate_level_percent();
        self.toners.cyan_toner.calculate_level_percent();
        self.toners.magenta_toner.calculate_level_percent();
        self.toners.yellow_toner.calculate_level_percent();

        // Drums
        self.drums.black_drum.calculate_level_percent();
        self.drums.cyan_drum.calculate_level_percent();
        self.drums.magenta_drum.calculate_level_percent();
        self.drums.yellow_drum.calculate_level_percent();

        // Maintenance
        self.fuser.calculate_level_percent();
        self.reservoir.calculate_level_percent();
    }
}

#[cfg(test)]
mod tests {
    use super::Printer;
    use crate::printer::{
        supply::{drum::Drum, fuser::Fuser, reservoir::Reservoir, toner::Toner},
        Drums, Toners,
    };

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
            None,
        );

        assert_eq!(printer.name, "Constructor Test");
        assert_eq!(printer.serial_number.unwrap(), "XYZ-123");
        assert!(printer.toners.black_toner.is_some());
        assert!(printer.drums.black_drum.is_none());
        assert!(printer.fuser.is_some());
        assert!(printer.reservoir.is_none());
    }

    #[test]
    fn test_toner_level_calculation() {
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
            None,
        );

        printer.calculate_all_levels();

        assert_eq!(printer.toners.black_toner.unwrap().level_percent, Some(80));
        assert_eq!(printer.toners.cyan_toner.unwrap().level_percent, Some(33));
        assert_eq!(
            printer.toners.magenta_toner.unwrap().level_percent,
            Some(66)
        );
        assert_eq!(printer.toners.yellow_toner.unwrap().level_percent, Some(10));
    }

    #[test]
    fn test_drum_level_calculation() {
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
            None,
        );

        printer.calculate_all_levels();

        assert_eq!(printer.drums.black_drum.unwrap().level_percent, Some(80));
        assert_eq!(printer.drums.cyan_drum.unwrap().level_percent, Some(33));
        assert_eq!(printer.drums.magenta_drum.unwrap().level_percent, Some(66));
        assert_eq!(printer.drums.yellow_drum.unwrap().level_percent, Some(10));
    }

    #[test]
    fn test_fuser_level_calculation() {
        let mut printer = Printer::new(
            String::from("Fuser Test Printer"),
            None,
            Toners::default(),
            Drums::default(),
            Some(Fuser::new(75, 100, None)),
            None,
            None,
        );

        printer.calculate_all_levels();

        assert_eq!(printer.fuser.unwrap().level_percent, Some(75));
    }

    #[test]
    fn test_reservoir_level_calculation() {
        let mut printer = Printer::new(
            String::from("Reservoir Test Printer"),
            None,
            Toners::default(),
            Drums::default(),
            None,
            Some(Reservoir::new(40, 50, None)),
            None,
        );

        printer.calculate_all_levels();

        assert_eq!(printer.reservoir.unwrap().level_percent, Some(80));
    }

    #[test]
    fn test_calculate_all_levels() {
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
            None,
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
