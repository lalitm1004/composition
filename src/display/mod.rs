use crate::settings::tracked::get_tracked_extensions;

pub fn list_tracked_extensions() {
    let tracked_extensions = get_tracked_extensions();

    let max_display_width = tracked_extensions
        .iter()
        .map(|tracked| tracked.display.len())
        .max()
        .unwrap_or(10);

    tracked_extensions
        .iter()
        .for_each(|tracked| {
            println!(
                "• {:>display_width$} - [ {} ]",
                tracked.display,
                tracked.extensions.join(", "),
                display_width = max_display_width,
            )
        });
}