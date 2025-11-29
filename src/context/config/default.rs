use super::Tracked;

pub fn use_color() -> bool {
    false
}

pub fn log_scale() -> bool {
    false
}

pub fn respect_gitignore() -> bool {
    true
}

pub fn ignore_dotfolders() -> bool {
    true
}

pub fn ignore_dotfiles() -> bool {
    true
}

pub fn ignore_empty_lines() -> bool {
    true
}

pub fn ignored_directories() -> Vec<String> {
    vec![
        "node_modules".into(),
        "target".into(),
        "build".into(),
        "dist".into(),
        "__venv__".into(),
        "__pycache__".into(),
    ]
}

pub fn ignored_files() -> Vec<String> {
    vec!["package-lock.json".into()]
}

pub fn excluded_patterns() -> Vec<String> {
    vec![
        r"^\s*//".into(),                    // skip lines starting with //
        r"^\s*#".into(),                     // skip lines starting with #
        r"^\s*[\{\}\[\]\(\),;]+\s*$".into(), // skip lines containing only braces, commas, or  semicolons
    ]
}

pub fn tracked() -> Vec<Tracked> {
    vec![
        Tracked {
            display: "Rust".into(),
            extensions: vec!["rs".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Python".into(),
            extensions: vec!["py".into(), "pyi".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "HTML".into(),
            extensions: vec!["html".into(), "htm".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "CSS".into(),
            extensions: vec!["css".into(), "scss".into(), "sass".into(), "less".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "JavaScript".into(),
            extensions: vec!["js".into(), "jsx".into(), "mjs".into(), "cjs".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "TypeScript".into(),
            extensions: vec!["ts".into(), "tsx".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Svelte".into(),
            extensions: vec!["svelte".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "C".into(),
            extensions: vec!["c".into(), "h".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "C++".into(),
            extensions: vec![
                "cpp".into(),
                "cxx".into(),
                "cc".into(),
                "hpp".into(),
                "hh".into(),
            ],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "C#".into(),
            extensions: vec!["cs".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Bash / Shell".into(),
            extensions: vec![
                "sh".into(),
                "bash".into(),
                "bashrc".into(),
                "bash_profile".into(),
                "zsh".into(),
                "zshrc".into(),
            ],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Java".into(),
            extensions: vec!["java".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Jupyter Notebook".into(),
            extensions: vec!["ipynb".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Assembly".into(),
            extensions: vec!["asm".into(), "s".into(), "mips".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Go".into(),
            extensions: vec!["go".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Prisma".into(),
            extensions: vec!["prisma".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Kotlin".into(),
            extensions: vec!["kt".into(), "kts".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Swift".into(),
            extensions: vec!["swift".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Objective-C".into(),
            extensions: vec!["m".into(), "mm".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "PHP".into(),
            extensions: vec!["php".into(), "phtml".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Ruby".into(),
            extensions: vec!["rb".into(), "erb".into(), "gemspec".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Perl".into(),
            extensions: vec!["pl".into(), "pm".into(), "t".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "R".into(),
            extensions: vec!["r".into(), "R".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Julia".into(),
            extensions: vec!["jl".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Scala".into(),
            extensions: vec!["scala".into(), "sc".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Haskell".into(),
            extensions: vec!["hs".into(), "lhs".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Elixir".into(),
            extensions: vec!["ex".into(), "exs".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Erlang".into(),
            extensions: vec!["erl".into(), "hrl".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "F#".into(),
            extensions: vec!["fs".into(), "fsi".into(), "fsx".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "OCaml".into(),
            extensions: vec!["ml".into(), "mli".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Clojure".into(),
            extensions: vec!["clj".into(), "cljs".into(), "cljc".into(), "edn".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Dart".into(),
            extensions: vec!["dart".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Makefile".into(),
            extensions: vec!["mk".into(), "make".into(), "Makefile".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Dockerfile".into(),
            extensions: vec!["dockerfile".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "YAML".into(),
            extensions: vec!["yml".into(), "yaml".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "JSON".into(),
            extensions: vec!["json".into(), "json5".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "TOML".into(),
            extensions: vec!["toml".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "INI / Config".into(),
            extensions: vec!["ini".into(), "cfg".into(), "conf".into(), "env".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Markdown".into(),
            extensions: vec!["md".into(), "markdown".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "LaTeX".into(),
            extensions: vec!["tex".into(), "sty".into(), "cls".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "SQL".into(),
            extensions: vec!["sql".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "GraphQL".into(),
            extensions: vec!["graphql".into(), "gql".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Vue".into(),
            extensions: vec!["vue".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
        Tracked {
            display: "Sass".into(),
            extensions: vec!["sass".into()],
            color: None,
            excluded_patterns: vec![],
            compiled_excluded_patterns: vec![],
        },
    ]
}

// tests to ensure default values are valid
#[cfg(test)]
mod default_config_tests {
    use crate::context::config::Config;

    use std::collections::HashSet;

    #[test]
    fn default_config_loads() {
        let _config = Config::default();
    }

    #[test]
    fn tracked_display_names_are_unique() {
        let config = Config::default();

        let mut seen = HashSet::new();
        for tracked in &config.tracked {
            assert!(
                seen.insert(&tracked.display),
                "duplicate display name found '{}'",
                tracked.display
            )
        }
    }

    #[test]
    fn global_regexes_compile_correctly() {
        let config = Config::default();

        // check compiled_excluded_patterns count matches raw patterns
        assert_eq!(
            config.excluded_patterns.len(),
            config.compiled_excluded_patterns.len(),
            "compiled_excluded_patterns length should match excluded_patterns"
        );

        // ensure compiled regex matches original pattern string
        for (pat, compiled) in config
            .excluded_patterns
            .iter()
            .zip(config.compiled_excluded_patterns.iter())
        {
            assert_eq!(
                compiled.as_str(),
                pat,
                "compiled regex differs from pattern {}",
                pat
            );
        }
    }

    #[test]
    fn tracked_regexes_compile_correctly() {
        let config = Config::default();

        for tracked in &config.tracked {
            assert_eq!(
                tracked.excluded_patterns.len(),
                tracked.compiled_excluded_patterns.len(),
                "compiled_excluded_patterns length mismatch for tracked '{}'",
                tracked.display
            );

            for (pat, compiled) in tracked
                .excluded_patterns
                .iter()
                .zip(tracked.compiled_excluded_patterns.iter())
            {
                assert_eq!(
                    compiled.as_str(),
                    pat,
                    "compiled regex differs from pattern {} for tracked {}",
                    pat,
                    tracked.display
                );
            }
        }
    }
}
