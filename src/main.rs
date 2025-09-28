use clap::Parser;
use composition::context::{AppContext, cli::Cli};

fn main() {
    let cli = Cli::parse();
    let app_context = AppContext::from_cli(cli);

    println!("=== App Context ===");
    println!("Path: {}", app_context.path.display());
    println!("Scale Bar: {}", app_context.scale_bar);
    println!("Config Loaded from File: {}", app_context.config_loaded);
    println!();

    println!("=== Configuration ===");
    let config = app_context.config;
    println!("Use Color: {}", config.use_color);
    println!("Respect Gitignore: {}", config.respect_gitignore);
    println!("Ignore Dot Folders: {}", config.ignore_dotfolders);
    println!("Ignore Dot Files: {}", config.ignore_dotfiles);
    println!("Ignore Empty Lines: {}", config.ignore_empty_lines);
    println!();

    println!("Ignored Directories:");
    if config.ignored_directories.is_empty() {
        println!("  (none)");
    } else {
        for dir in &config.ignored_directories {
            println!("  - {}", dir);
        }
    }
    println!();

    println!("Ignored Files:");
    if config.ignored_files.is_empty() {
        println!("  (none)");
    } else {
        for file in &config.ignored_files {
            println!("  - {}", file);
        }
    }
    println!();

    println!("Excluded Patterns:");
    if config.excluded_patterns.is_empty() {
        println!("  (none)");
    } else {
        for pattern in &config.excluded_patterns {
            println!("  - {}", pattern);
        }
    }
    println!(
        "  Compiled patterns: {} regex(es)",
        config.compiled_excluded_patterns.len()
    );
    println!();

    println!("Tracked Languages:");
    if config.tracked.is_empty() {
        println!("  (none)");
    } else {
        for (i, tracked) in config.tracked.iter().enumerate() {
            println!("  {}. {}", i + 1, tracked.display);
            println!("     Extensions: [{}]", tracked.extensions.join(", "));
            if let Some(color) = &tracked.color {
                println!("     Color: {}", color);
            } else {
                println!("     Color: (default)");
            }
            if !tracked.excluded_patterns.is_empty() {
                println!(
                    "     Excluded patterns: [{}]",
                    tracked.excluded_patterns.join(", ")
                );
                println!(
                    "     Compiled patterns: {} regex(es)",
                    tracked.compiled_excluded_patterns.len()
                );
            } else {
                println!("     Excluded patterns: (none)");
            }
            println!();
        }
    }
}
