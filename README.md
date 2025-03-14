# composition 📊
Composition is a simple CLI tool that helps you understand your codebase better by calculating the total lines of code per language, as well as each language's share in the overall codebase.

## Features
- **Total Lines by Language:** Know exactly how many lines you've written per language.
- **Easy Visualization:** Easily wrap your head around the percentage contribution of each language via the handy contribution bar.
- **Fully customizable:** Add file extensions you want to track, ignore directories like `.git` and `node_modules` and whatever you choose.
- **Invesitgate Languages:** Know exactly where the files of a certain language lie using `composition investigate`

## Example
- composition
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

- investigate
```bash
$ composition src investigate rust
settings/ | 173 lines | 38.88% | ███████████████████
core/     | 122 lines | 27.42% | ██████████████
display/  | 101 lines | 22.70% | ███████████
./        |  49 lines | 11.01% | ██████
```

## Build
1. Clone this repository and navigate into it:
   ```bash
   git clone https://github.com/lalitm1004/composition.git
   cd composition
   ```
2. **[Optional]** Edit `src/settings/` to track other extensions / ignore directories & files.
    - **Modify tracked extensions:**
        Edit the `public fn get_all-tracked()` in `src/settings/tracked.rs` to add/remove extensions.
        ```rs
        pub fn get_all_tracked() -> Vec<Tracked> {
            vec![
                tracked!("Rust", ["rs"]),
                tracked!("Assembly", ["asm", "mips"]),
                //tracked!("Display Name", ["ext1", "ext2",]),
            ]
        }
        ```
    - **Modify ignored directories/files:**
        Edit the `const IGNORED_DIRECTORIES` and `const IGNORED_FILES` in `src/settings/ignore.rs` as per your requirements.
        ```rs
        const IGNORE_DOTFOLDERS: bool = true; // ignores all folders that start with a . by default
        const IGNORED_DIRECTORIES: &[&str] = &[
            "node_modules",
            "target",
            ".git",
            ".venv",
            "__venv__",
            "__pycache__",
            // append more
        ];

        // exact same for ignored files
        ```

3. Build using `cargo build --release`. You can now place your built binary in your PATH like i did or just do whatever 🤷‍♀️.

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

- Run an investigation with a target language
```bash
$ comspotion GitRepos/composition/src investigate rust --scale-bar 0.5
settings/ | 173 lines | 38.88% | ███████████████████
core/     | 122 lines | 27.42% | ██████████████
display/  | 101 lines | 22.70% | ███████████
./        |  49 lines | 11.01% | ██████
```