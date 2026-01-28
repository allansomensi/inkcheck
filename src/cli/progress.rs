use super::theme::CliTheme;
use indicatif::{ProgressBar, ProgressStyle};

pub fn show_progress(label: &str, label_color: &str, level: u8, bar_color: &str, theme: CliTheme) {
    let template =
        format!("{{prefix:>12.{label_color}.bold}} [{{bar:25.{bar_color}}}] {{pos:>3}}%");

    let pb = ProgressBar::new(100);

    let style = ProgressStyle::with_template(&template)
        .expect("Failed to create progress bar template")
        .progress_chars(theme.chars());

    pb.set_style(style);
    pb.set_prefix(label.to_string());
    pb.set_position(level as u64);

    pb.abandon();
}
