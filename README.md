# composition 📊
Composition is a simple CLI tool that helps you understand your codebase better by calculating the total lines of code per language, as well as each language's share in the overall codebase.

## Features
- **Total Lines by Language:** Know exactly how many lines you've written per language.
- **Easy Visualization:** Easily wrap your head around the percentage contribution of each language via the handy contribution bar.
- **Fully customizable:** Add file extensions you want to track, ignore directories like `.git` and `node_module` and whatever you choose.

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
2. **[Optional]** Edit `src/config.rs` to track other extensions / ignore directories & files.
    - **Modify tracked extensions:**
        Edit the `static TRACKED_EXTENSIONS` constant to add/remove extensions.
        ```rs
        pub struct Tracked {
            pub ext: &'static str, // the extension of the file you want to track (currently doesnt support aliases)
            pub display: &'static str, // the display name
            pub color: &'static (u8, u8, u8), // the RGB color of the contribution bar
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
    - **Modify ignored directories/files:**
        Edit the `static IGNORED_DIRECTORIES` or `static IGNORED_FILES` as per your requirements.
        ```rs
        static IGNORED_DIRECTORIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
            [
                ".git", "__venv__", "node_modules", "target", "__pycache__",
                ".next", "build", ".expo", ".idea", "venv", ".svelete-kit",
                // add directory to ignore here or remove previous entries
            ].into()
        });

        // the same for files you want to ignore, just edit IGNORED_FILES
        ```
    - **Modify preference for colored composition bar:**
        Edit the `static COLORED_COMPOSITION_BAR` to either `true/false` as per your requirements.
3. Build using `cargo build --release`. You can now place your built `.exe` into your PATH like i did or just do whatever 🤷‍♀️.

## Usage
Run composition with a target directory path:
```bash
$ composition GitRepos/composition
Rust | 202 lines | 100.00% | █████████████████████████████████████████████
```
Bar too big? just pass in a minify value like so:
```bash
$ compsition GitRepos/composition --minify 2
Rust | 202 lines | 100.00% | █████████████████████████
```
(you can use this to magnify aswell)
