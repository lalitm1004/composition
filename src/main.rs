use std::process;
use composition::settings;

fn main() {
    let args = settings::build_args();

    if let Err(err) = composition::run(args) {
        eprintln!("ERROR: {err}");
        process::exit(1);
    }
}