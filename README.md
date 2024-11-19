# composition 📊
Composition is a simple CLI tool that helps you understand your codebase better by calculating the total lines of code per language, as well as each language's share in the overall codebase.

## Features
- **Total Lines by Language:** Know exactly how many lines you've written per language.
- **Easy Visualization:** Easily wrap your head around the percentage contribution of each language via the handy contribution bar.
- **Fully customizable:** Add file extensions you want to track, ignore directories like `.git` and `node_modules` and whatever you choose.

## Example
```bash
$ composition exampleEnv/
Python     | 2446 lines | 24.42% | ████████████████████████
Svelte     | 2254 lines | 22.50% | ███████████████████████
TypeScript | 1850 lines | 18.47% | ██████████████████
CSS        | 1594 lines | 15.91% | ████████████████
JavaScript | 1010 lines | 10.08% | ██████████
HTML       | 626 lines  |  6.25% | ██████
Rust       | 225 lines  |  2.25% | ██
Go         | 11 lines   |  0.11% |
C          | 1 lines    |  0.01% |
```

## Build
1. Clone this repository and navigate into it:
   ```bash
   git clone https://github.com/lalitm1004/composition.git
   cd composition
   ```
2. **[Optional]** Edit `src/settings.rs` to track other extensions / ignore directories & files.
    - **Modify tracked extensions:**
        Edit the `public fn get_tracked_extensions()` to add/remove extensions.
        ```rs
        fn new(
            display: &'static str, // The display name [must be unique]
            extensions: Vec<&'static str>, // Vector of all extensions to track
            color: &'static str // Hex code of color value for contribution bar
        ) -> Self {}

        pub fn get_tracked_extensions() -> Vec<Tracked> {
            vec![
                Tracked::new("Rust", vec!["rs"], "#F85009"),
                Tracked::new("Python", vec!["py", "ipynb"], "#3772A3"),
                // Tracked::new("DisplayName", vec!["Ext1", "Ext2"], "#ColorHex"),
            ]
        }
        ```
    - **Modify ignored directories/files:**
        Edit the `pub fn get_ignored_directories()` or `pub fn get_ignored_files()` as per your requirements.
        ```rs
        pub fn get_ignored_directories() -> Vec<&'static str> {
            vec![
                ".git", "__venv__", ".venv", "venv", "__pycache__", "target", "node_modules",
                ".next", ".expo", ".idea", ".svelte-kit",
                // append more
            ]
        }

        // the same for files you want to ignore, just edit get_ignored_files()
        ```
    - **Modify preference for colored composition bar:**
        Edit the `color` argument under `pub struct Cli` to either `true/false` as per your requirements.

3. Build using `cargo build --release`. You can now place your built `.exe` into your PATH like i did or just do whatever 🤷‍♀️.

## Usage
- Run composition with a target directory path:
```bash
$ composition GitRepos/composition
Rust | 282 lines | 100.00% | ██████████████████████████████████████████████████
```

- Scale bar at runtime using `--scale-bar` argument 
```bash
$ compsition GitRepos/composition --scale-bar 0.5
Rust | 282 lines | 100.00% | █████████████████████████
```

- Enable color at runtime using `-c` or `--color` flag.