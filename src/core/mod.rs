use std::{collections::HashMap, io};

use crate::settings::{
    cli::Cli, ignored::{
        get_ignored_directories,
        get_ignored_files
    }, tracked::{get_tracked_extensions, Tracked}
};

mod utils;

pub fn run_composition(cli: Cli) -> Result<HashMap<String, usize>, io::Error> {
    let tracked_extensions = get_tracked_extensions();
    let ignored_dirs = get_ignored_directories();
    let ignored_files = get_ignored_files();

    // get_composition()
    Ok(HashMap::new())
}

pub fn run_investigation(cli: Cli, extension: Tracked) -> Result<HashMap<String, usize>, io::Error> {
    let tracked_extensions = vec![extension];
    let ignored_dirs = get_ignored_directories();
    let ignored_files = get_ignored_files();

    Ok(HashMap::new())
}

