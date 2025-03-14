use crate::tracked;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tracked {
    pub display: String,
    pub extensions: Vec<String>,
}

pub fn get_all_tracked() -> Vec<Tracked> {
    vec![
        tracked!("Rust", ["rs"]),
        tracked!("HTML", ["html"]),
        tracked!("CSS", ["css", "scss"]),
        tracked!("JavaScript", ["js", "jsx"]),
        tracked!("TypeScript", ["ts", "tsx"]),
        tracked!("Svelte", ["svelte"]),
        tracked!("C", ["c"]),
        tracked!("C++", ["cpp"]),
        tracked!("C#", ["cs"]),
        tracked!("Bash", ["bash", "bashrc", "bashprofile"]),
        tracked!("Java", ["java"]),
        tracked!("Python", ["py", "ipynb"]),
        tracked!("Assembly", ["asm", "mips"]),
        tracked!("Go", ["go"]),
        tracked!("Prisma", ["prisma"]),
        // tracked!("Display Name", ["ext1", "ext2"]),
    ]
}

#[macro_export]
macro_rules! tracked {
    ($display:expr, [$($ext:expr), *]) => {
        Tracked {
            display: $display.to_string(),
            extensions: vec![$($ext.to_string()), *],
        }
    };
}
