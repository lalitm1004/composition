#[derive(Debug, Clone)]
pub struct Tracked {
    pub display: &'static str,
    pub extensions: Vec<&'static str>,
    pub color: (u8, u8, u8)
}

impl Tracked {
    pub fn new(
        display: &'static str,
        extensions: Vec<&'static str>,
        color_hex: &'static str
    ) -> Self {
        let color = Self::hex_to_rgb(color_hex);
        Tracked { display, extensions, color }
    }

    fn hex_to_rgb(hex_str: &str) -> (u8, u8, u8) {
        let hex_str = hex_str.trim_start_matches("#");
        if hex_str.len() != 6 {
            panic!("ERROR: invalid hex code {}", hex_str)
        }

        let r = u8::from_str_radix(&hex_str[0..2], 16).expect("ERROR: invalid hex code");
        let g = u8::from_str_radix(&hex_str[2..4], 16).expect("ERROR: invalid hex code");
        let b = u8::from_str_radix(&hex_str[4..6], 16).expect("ERROR: invalid hex code");
        
        (r, g, b)
    }
}

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
        Tracked::new("Bash", vec!["bash", "bashrc"], "#087608"),
        Tracked::new("Java", vec!["java"], "#0B9F97"),
        Tracked::new("Python", vec!["py", "ipynb"], "#3772A3"),
        Tracked::new("Assembly", vec!["asm", "mips"], "#093332"),
        Tracked::new("Go", vec!["go"], "#34BEB2"),
    ]
}