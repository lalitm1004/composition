use clap::Parser;
use composition::{
    context::{AppContext, cli::Cli},
    core::{get_all_entries, get_composition},
};

fn main() {
    let cli = Cli::parse();
    let app_context = AppContext::from_cli(cli);

    println!("=== App Context ===");
    println!("Path: {}", app_context.path.display());
    println!("Scale Bar: {}", app_context.scale_bar);
    println!("Config Loaded from File: {}", app_context.config_loaded);
    println!();

    println!("{:?}", app_context.config.tracked);

    let entries = get_all_entries(&app_context);
    println!("{}", entries.len());
    // for e in &entries {
    //     println!("{:?}", e);
    // }

    let composition = get_composition(&app_context, entries);
    println!("{:?}", composition);
}
