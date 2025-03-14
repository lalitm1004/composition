mod core;
mod display;
mod settings;

use core::{get_composition, get_investigation};
use display::{display_hashmap, spinner};
pub use settings::{Cli, Command};
use settings::{Tracked, get_all_tracked};
use std::io;

pub fn run_composition(args: &Cli) -> Result<(), io::Error> {
    spinner::start("Walking directory...");
    let composition_hashmap = get_composition(&args.path, &get_all_tracked())?;
    spinner::end();

    display_hashmap(args, composition_hashmap);

    Ok(())
}

pub fn run_investigation(args: &Cli, tracked_extension: Tracked) -> Result<(), io::Error> {
    spinner::start("Walking directory...");
    let investigation_hashmap = get_investigation(&args.path, tracked_extension)?;
    spinner::end();

    display_hashmap(args, investigation_hashmap);

    Ok(())
}
