# composition 📊
Composition is a simple CLI tool that helps you understand your codebase better by calculating the total lines of code per language, as well as each language's share in the overall codebase.

## Features
- Total Lines by Language: Know exactly how many lines you've written per language.
- Fully customizable: Add file extensions you want to track, ignore directories like `.git` and `node_module` and whatever you choose

## Build
1. Clone this repository and navigate into it:
   ```bash
   git clone https://github.com/lalitm1004/composition.git
   cd composition
   ```
2. Edit `src/config.rs` to track other extensions / ignore directories & files.
    - **Modify tracked extensions** 
        Edit the `static TRACKED_EXTENSIONS` constant to add/remove extensions.
        ```rs
        pub struct Tracked {
            pub ext: &'static str, // the extension of the file you want to track (currently doesnt support aliases)
            pub display: &'static str, // the display name
            pub color: &'static (u8, u8, u8), // the color of the contribution bar
        }
        
        static TRACKED_EXTENSIONS: Lazy<Vec<Tracked>> = Lazy::new(|| {
            vec![
                Tracked { ext: "rs", display: "Rust", color: &(246, 82, 9) },
                Tracked { ext: "html", display: "HTML", color: &(241, 106, 48) },
                //...
                // Add a Tracked struct here or remove previous entries.
            ].into();
        });        
        ```
    - **Modify ignored directories/files**
        Edit the `static IGNORED_DIRECTORIES` or `static IGNORED_FILES` as per your requirements.
        ```rs
        static IGNORED_DIRECTORIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
            [
                ".git", "__venv__", "node_modules", "target", "__pycache__", 
                ".next", "build", ".expo", ".idea", "venv", ".svelete-kit",
                // add directory to ignore here or remove previous entries
            ].into()
        });
        ```
3. Build using `cargo build --release`. You can now place your built `.exe` into your PATH like i did or just do whatever 🤷‍♀️.