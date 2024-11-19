use std::{
    fs,
    thread,
    path::Path,
    error::Error,
    time::Duration,
    io::{self, BufRead, Write},
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use colored::*;
use settings::Tracked;

pub mod settings;

pub fn run(args: settings::Cli) -> Result<(), Box<dyn Error>> {
    let mut composition_hashmap: HashMap<String, usize> = HashMap::new();
    let tracked_extensions = settings::get_tracked_extensions();
    let ignored_directories = settings::get_ignored_directories();
    let ignored_files = settings::get_ignored_files();

    let spinner_running = Arc::new(AtomicBool::new(true));
    let spinner_handle = {
        let spinner_running = Arc::clone(&spinner_running);
        thread::spawn(move || {
            let spinner_chars = ['|', '/', '-', '\\'];
            let mut index = 0;

            while spinner_running.load(Ordering::SeqCst) {
                print!("\rwalking directory... {}", spinner_chars[index]);
                io::stdout().flush().unwrap();
                index = (index + 1) % spinner_chars.len();
                thread::sleep(Duration::from_millis(100));
            }

            // Clear spinner after completion
            print!("\r{} \r", " ".repeat(20));
            io::stdout().flush().unwrap();
        })
    };

    walk_directory(
        &args.path,
        &tracked_extensions,
        &mut composition_hashmap,
        &ignored_directories,
        &ignored_files,
    )?;

    spinner_running.store(false, Ordering::SeqCst);
    spinner_handle.join().unwrap();

    display_composition(
        &tracked_extensions,
        &composition_hashmap,
        args,
    );
    Ok(())
}

fn walk_directory(
    path: &Path,
    tracked_extensions: &Vec<Tracked>,
    composition_hashmap: &mut HashMap<String, usize>,
    ignored_directories: &Vec<&'static str>,
    ignored_files: &Vec<&'static str>,
) -> Result<(), Box<dyn Error>> {

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let dir_name = entry_path.file_name().unwrap().to_str().unwrap();
                if ignored_directories.contains(&dir_name) {
                    continue;
                }
                walk_directory(&entry_path, tracked_extensions, composition_hashmap, ignored_directories, ignored_files)?;
            } else if entry_path.is_file() {
                tally_file_lines(&entry_path, tracked_extensions, composition_hashmap, ignored_files)?;
            }
        }
    }

    Ok(())
}

fn tally_file_lines(
    path: &Path,
    tracked_extensions: &Vec<Tracked>,
    composition_hashmap: &mut HashMap<String, usize>,
    ignored_files: &Vec<&'static str>,
) -> Result<(), Box<dyn Error>> {

    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        if ignored_files.contains(&file_name) {
            return Ok(());
        }
    }

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    for tracked in tracked_extensions {
        if tracked.extensions.contains(&extension) {
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);

            let line_count = reader.lines().count();
            let count = composition_hashmap.entry(tracked.display.to_string()).or_insert(0);
            *count += line_count;
        }
    }

    Ok(())
}

fn display_composition(
    tracked_extensions: &Vec<Tracked>,
    composition_hashmap: &HashMap<String, usize>,
    args: settings::Cli
) {
    let total_lines: usize = composition_hashmap.values().sum();

    let mut sorted_entries: Vec<(&Tracked, usize, f32)> = composition_hashmap
        .iter()
        .filter_map(|(entry, count)| {
            tracked_extensions
                .iter()
                .find(|tracked| tracked.display == *entry)
                .map(|tracked| {
                    let percentage = (*count as f32 / total_lines as f32) * 100.0;
                    (tracked, *count, percentage)
                })
        })
        .collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));


    let max_display_width = sorted_entries
        .iter()
        .map(|(tracked,_,_)| tracked.display.len())
        .max()
        .unwrap_or(10);

    let max_lines_width = sorted_entries
        .iter()
        .map(|(_, line_count, _)| line_count.to_string().len())
        .max()
        .unwrap_or(10);

    for (tracked, line_count, percentage) in sorted_entries {
        // let bar_width = "█".repeat(bar)
        let bar_width = (percentage * args.scale_bar).round() as usize;
        let mut bar = "█".repeat(bar_width);

        if args.color {
            let color = Color::TrueColor {
                r: tracked.color.0,
                g: tracked.color.1,
                b: tracked.color.2,
            };
            bar = bar.color(color).to_string();
        }

        println!(
            "{:<width_display$} | {:>width_lines$} lines | {:>5.2}% | {}",
            tracked.display,
            line_count,
            percentage,
            bar,
            width_display = max_display_width,
            width_lines = max_lines_width,
        )
    }
}