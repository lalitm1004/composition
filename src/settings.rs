use clap::Parser;
use std::{path::{Path, PathBuf}, process};

pub fn get_tracked_extensions() -> Vec<Tracked> {
    vec![
        Tracked::new("Rust", vec!["rs"], "#F85009"),
        Tracked::new("HTML", vec!["html"], "#E8622C"),
        Tracked::new("CSS", vec!["css", "scss"], "#3BABDC"),
        Tracked::new("JavaScript", vec!["js", "jsx"], "#FDDB44"),
        Tracked::new("TypeScript", vec!["ts", "tsx"], "#2D79C7"),
        Tracked::new("Svelte", vec!["svelte"], "#F73C00"),
        Tracked::new("C", vec!["c"], "#084A86"),
        Tracked::new("C++", vec!["cpp"], "#085E9F"),
        Tracked::new("C#", vec!["cs"], "#3F0893"),
        Tracked::new("Bash", vec!["bash"], "#087608"),
        Tracked::new("Java", vec!["java"], "#0B9F97"),
        Tracked::new("Python", vec!["py", "ipynb"], "#3772A3"),
        Tracked::new("Assembly", vec!["asm", "mips"], "#093332"),
        Tracked::new("Go", vec!["go"], "#34BEB2"),
        // Tracked::new("DisplayName", vec!["Ext1", "Ext2"], "ColorHex"),
        // append more
    ]
}

pub fn get_ignored_directories() -> Vec<&'static str> {
    vec![
        ".git", "__venv__", ".venv", "venv", "__pycache__", "target", "node_modules",
        ".next", ".expo", ".idea", ".svelte-kit", ".github", ".rustup", ".config"
        // append more
    ]
}

pub fn get_ignored_files() -> Vec<&'static str> {
    vec![
        "package-lock.json",
        // append more
    ]
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Tracked {
    pub display: &'static str,
    pub extensions: Vec<&'static str>,
    pub color: (u8, u8, u8),
}

impl Tracked {
    fn new(
        display: &'static str,
        extensions: Vec<&'static str>,
        color: &'static str
    ) -> Self {
        let color = Self::hex_to_rgb(color);
        Tracked { display, extensions, color }
    }

    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            panic!("ERROR: invalid hex code: {}", hex);
        }

        let r = u8::from_str_radix(&hex[0..2], 16).expect("ERROR: invalid hex code");
        let g = u8::from_str_radix(&hex[2..4], 16).expect("ERROR: invalid hex code");
        let b = u8::from_str_radix(&hex[4..6], 16).expect("ERROR: invalid hex code");

        (r, g, b)
    }
}

pub fn build_args() -> Cli {
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };
    args
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(
        default_value = ".",
        value_parser=parse_path,
        help = "Specify the path to a directory."
    )]
    pub path: PathBuf,

    #[arg(
        long,
        default_value_t = 1.0,
        value_parser = parse_scale_bar,
        help = "Set the scale factor for the bars."
    )]
    pub scale_bar: f32,

    #[arg(
        short = 'c',
        long = "color",
        default_value_t = false,
        help = "Switch color output. [default = false]"
    )]
    pub color: bool,
}

fn parse_path(arg: &str) -> Result<PathBuf, String> {
    let path = Path::new(arg);
    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err(format!("is not a valid path or does not exist"))
    }
}

fn parse_scale_bar(arg: &str) -> Result<f32, String> {
    arg.parse::<f32>().map_err(|_| {
        format!("must be a valid floating-point value")
    })
}
