pub mod spinner;

use super::Cli;
use std::collections::HashMap;

pub fn display_hashmap(args: &Cli, hashmap: HashMap<String, usize>) {
    let total_lines: usize = hashmap.values().sum();

    let mut sorted_entries: Vec<(&String, usize, f32)> = hashmap
        .iter()
        .map(|(display, count)| {
            let percentage = (*count as f32 / total_lines as f32) * 100.0;
            (display, *count, percentage)
        })
        .collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));

    let max_display_width = sorted_entries
        .iter()
        .map(|(display, _, _)| display.len())
        .max()
        .unwrap_or(10);

    let max_lines_width = sorted_entries
        .iter()
        .map(|(_, count, _)| count.to_string().len())
        .max()
        .unwrap_or(10);

    for (display, count, percentage) in sorted_entries {
        if count == 0 {
            continue;
        }

        let bar_width = (percentage * args.scale_bar).round() as usize;
        let bar = "█".repeat(bar_width);

        println!(
            "{:<width_display$} | {:>width_lines$} lines | {:>5.2}% | {}",
            display,
            count,
            percentage,
            bar,
            width_display = max_display_width,
            width_lines = max_lines_width,
        );
    }
}
