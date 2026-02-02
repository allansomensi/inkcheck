use crate::{
    cli::{output::OutputFormat, progress::show_progress, theme::CliTheme},
    printer::Printer,
};
use colored::Colorize;

/// Display the formatted values.
pub fn show_printer_values(
    printer: Printer,
    extra_supplies: bool,
    metrics: bool,
    theme: &CliTheme,
    output: &OutputFormat,
) {
    if let OutputFormat::Json = output {
        match serde_json::to_string_pretty(&printer) {
            Ok(json) => println!("{json}"),
            Err(e) => eprintln!("Error generating JSON output: {e}"),
        }
        return;
    }

    if let OutputFormat::Csv = output {
        render_csv(&printer);
        return;
    }

    // Since Toner, Drum, and Fuser are different structs, we use a macro
    // to access the `.level_percent` field regardless of the specific type.
    macro_rules! render {
        ($label:literal, $label_color:literal, $target:expr, $bar_color:expr) => {
            // Check if the parent struct exists
            if let Some(item) = $target.as_ref() {
                // Check if the `level_percent` field exists within it
                if let Some(level) = item.level_percent {
                    show_progress($label, $label_color, level as u8, $bar_color, *theme);
                }
            }
        };
    }

    // Header
    println!("{} {}", "Printer:".bright_cyan().bold(), printer.name);

    if extra_supplies {
        if let Some(serial) = &printer.serial_number {
            println!("{} {serial}", "Serial:".bright_cyan().bold());
        }
    }
    println!();

    // Toners
    println!("--> {}\n", "Toner:".bright_white().bold());
    let t = &printer.toners;

    render!("Black", "bright_white", t.black_toner, "white");
    render!("Cyan", "bright_cyan", t.cyan_toner, "cyan");
    render!("Magenta", "bright_magenta", t.magenta_toner, "magenta");
    render!("Yellow", "bright_yellow", t.yellow_toner, "yellow");

    // Extra Supplies
    if extra_supplies {
        let d = &printer.drums;

        println!("\n\n--> {}\n", "Drum:".bright_white().bold());

        render!("Black", "bright_white", d.black_drum, "white");
        render!("Cyan", "bright_cyan", d.cyan_drum, "cyan");
        render!("Magenta", "bright_magenta", d.magenta_drum, "magenta");
        render!("Yellow", "bright_yellow", d.yellow_drum, "yellow");

        // Other parts
        if printer.fuser.is_some() || printer.reservoir.is_some() {
            println!("\n\n--> {}\n", "Other:".bright_white().bold());

            render!("Fuser", "white", printer.fuser, "white");

            let reservoir_color = printer
                .reservoir
                .as_ref()
                .and_then(|r| r.level_percent)
                .map(|l| if l == 100 { "green" } else { "red" })
                .unwrap_or("white");

            render!("Reservoir", "white", printer.reservoir, reservoir_color);
        }
    }

    // Metrics
    if metrics {
        if let Some(m) = &printer.metrics {
            println!("\n\n--> {}", "Metrics:".bright_white().bold());

            if let Some(total) = m.total_impressions {
                println!(
                    "\n{} {total} pages",
                    "Total impressions:".bright_cyan().bold()
                );
            }
            if let Some(mono) = m.mono_impressions {
                println!("{} {mono} pages", "Mono:".bright_cyan().bold());
            }
            if let Some(color) = m.color_impressions {
                println!("{} {color} pages", "Color:".bright_cyan().bold());
            }
        }
    }

    println!();
}

/// Formats and prints the printer data as a CSV record to standard output.
///
/// This output includes the printer name, serial number, toner levels,
/// and total impression metrics in a comma-separated format.
fn render_csv(printer: &Printer) {
    let printer_name = printer.name.clone();
    let header = "Name,Serial,Black_Toner,Cyan_Toner,Magenta_Toner,Yellow_Toner,Total_Impressions";

    let serial = printer.serial_number.as_deref().unwrap_or("N/A");
    let bt = printer
        .toners
        .black_toner
        .as_ref()
        .and_then(|t| t.level_percent)
        .map(|v| v.to_string())
        .unwrap_or_default();
    let ct = printer
        .toners
        .cyan_toner
        .as_ref()
        .and_then(|t| t.level_percent)
        .map(|v| v.to_string())
        .unwrap_or_default();
    let mt = printer
        .toners
        .magenta_toner
        .as_ref()
        .and_then(|t| t.level_percent)
        .map(|v| v.to_string())
        .unwrap_or_default();
    let yt = printer
        .toners
        .yellow_toner
        .as_ref()
        .and_then(|t| t.level_percent)
        .map(|v| v.to_string())
        .unwrap_or_default();

    let total_impr = printer
        .metrics
        .as_ref()
        .and_then(|m| m.total_impressions)
        .map(|v| v.to_string())
        .unwrap_or_default();

    println!("{header}");
    println!("\"{printer_name}\",\"{serial}\",{bt},{ct},{mt},{yt},{total_impr}");
}
