use super::theme::{get_theme_chars, CliTheme};
use colored::ColoredString;
use indicatif::{ProgressBar, ProgressStyle};

pub fn show_progress(prefix: ColoredString, level: u8, color: &str, theme: &CliTheme) {
    let theme_chars = get_theme_chars(theme);
    let template = format!("{{prefix:10.{color}.bold}} [{{bar:25.{color}}}] {{percent:3}}%");

    let pb = ProgressBar::new(100);
    pb.set_prefix(format!("{prefix}:"));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&template)
            .expect("Failed to create progress bar template")
            .progress_chars(theme_chars),
    );

    pb.set_position(level as u64);
    pb.abandon();
}
