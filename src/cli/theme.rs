use clap::ValueEnum;
use std::fmt::Display;

/// Enum representing different CLI themes.
///
/// This enum defines the available themes that can be used in the CLI interface,
/// affecting the visual presentation.
#[derive(Debug, Clone, ValueEnum)]
pub enum CliTheme {
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

impl Display for CliTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid => write!(f, "solid"),
            Self::Blocks => write!(f, "blocks"),
            Self::Circles => write!(f, "circles"),
            Self::Diamonds => write!(f, "diamonds"),
            Self::Shades => write!(f, "shades"),
            Self::Vintage => write!(f, "vintage"),
            Self::Stars => write!(f, "stars"),
            Self::Emoji => write!(f, "emoji"),
            Self::Moon => write!(f, "moon"),
        }
    }
}

impl Default for CliTheme {
    fn default() -> Self {
        Self::Solid
    }
}

pub fn get_theme_chars(theme: &CliTheme) -> &str {
    match theme {
        CliTheme::Solid => "█ ",
        CliTheme::Blocks => "█▓▒░",
        CliTheme::Circles => "●○",
        CliTheme::Diamonds => "◆◇",
        CliTheme::Shades => "▉▇▆▅▄▃▂▁",
        CliTheme::Vintage => "#-",
        CliTheme::Stars => "★☆",
        CliTheme::Emoji => "😊🙂😐🙁😞",
        CliTheme::Moon => "🌕🌖🌗🌘🌑",
    }
}
