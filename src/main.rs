fn main() {
    if let Err(err) = composition::run() {
        eprintln!("Error: {err}");
    }
}
