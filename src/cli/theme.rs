use clap::ValueEnum;
use std::fmt::Display;

/// Enum representing different CLI themes.
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum CliTheme {
    #[default]
    Solid,
    Blocks,
    Circles,
    Diamonds,
    Shades,
    Vintage,
    Stars,
    Emoji,
    Moon,
}

impl CliTheme {
    pub fn chars(&self) -> &'static str {
        match self {
            Self::Solid => "█ ",
            Self::Blocks => "█▓▒░",
            Self::Circles => "●○",
            Self::Diamonds => "◆◇",
            Self::Shades => "▉▇▆▅▄▃▂▁",
            Self::Vintage => "#-",
            Self::Stars => "★☆",
            Self::Emoji => "😊🙂😐🙁😞",
            Self::Moon => "🌕🌖🌗🌘🌑",
        }
    }
}

impl Display for CliTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("variant not skipped")
            .get_name()
            .fmt(f)
    }
}
