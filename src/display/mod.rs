use colored::Colorize;

use crate::{context::AppContext, core::CompositionEntry};

pub mod spinner;

pub fn display_composition(
    app_context: &AppContext,
    composition_entries: &mut Vec<CompositionEntry>,
) {
    // reorder based on line count
    composition_entries.sort_by_key(|e| std::cmp::Reverse(e.line_count));

    let total_lines: usize = composition_entries.iter().map(|e| e.line_count).sum();
    composition_entries.iter_mut().for_each(|e| {
        let percentage = (e.line_count as f32 / total_lines as f32) * 100.0;
        e.percentage = percentage;
    });

    let max_display_width = composition_entries
        .iter()
        .map(|e| e.tracked.display.len())
        .max()
        .unwrap_or(20);

    let max_lines_width = composition_entries
        .iter()
        .map(|e| e.line_count.ilog10() + 1)
        .max()
        .unwrap_or(10) as usize;

    for entry in composition_entries {
        if entry.line_count == 0 {
            continue;
        }

        // set bar width and color
        let bar_width = (entry.percentage * app_context.scale_bar).round() as usize;
        let bar = "█".repeat(bar_width);
        let bar = match (app_context.config.use_color, &entry.tracked.color) {
            (true, Some(hex)) => {
                if let Some((r, g, b)) = hex_to_rgb(hex) {
                    bar.truecolor(r, g, b)
                } else {
                    bar.normal()
                }
            }
            (true, None) | (false, _) => bar.normal(),
        };

        println!(
            "{:>width_display$} | {:>width_lines$} lines | {:>5.2}% | {}",
            entry.tracked.display,
            entry.line_count,
            entry.percentage,
            bar,
            width_display = max_display_width + 1,
            width_lines = max_lines_width,
        );
    }
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}
