use clap::Parser;
use colored::Colorize;

use composition::{
    context::{AppContext, cli::Cli},
    core::{get_all_entries, get_composition},
    display::{display_composition, spinner},
};

fn main() {
    let cli = Cli::parse();
    let app_context = AppContext::from_cli(cli);
    if !app_context.config_loaded {
        println!(
            "{}",
            "Warning: Missing or invalid config.toml file. Using default settings."
                .yellow()
                .bold()
        );
    }

    spinner::start("Walking directory");
    let entries = get_all_entries(&app_context);
    spinner::end();

    spinner::start("Calculating composition");
    let mut composition = get_composition(&app_context, entries);
    spinner::end();

    display_composition(&app_context, &mut composition);
}
