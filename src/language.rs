use regex;

#[derive(Debug, PartialEq)]
pub enum Language {
    Rust,
    Python,
    Javascript,
    Typescript,
    Golang,
    C,

    Unknown,
}

impl Language {
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" => Language::Javascript,
            "ts" => Language::Typescript,
            "go" => Language::Golang,
            "c" | "cpp" | "cc" | "h" | "hpp" => Language::C,
            _ => Language::Unknown,
        }
    }

    pub fn to_extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::Javascript => "js",
            Language::Typescript => "ts",
            Language::Golang => "go",
            Language::C => "cpp",
            Language::Unknown => "",
        }
    }

    pub fn get_function_syntax(&self) -> regex::Regex {
        match self {
            Language::Rust => regex::Regex::new(r"^ *(?:pub)? *fn *([a-zA-Z0-9_]+).*\(.*\) *(?:-> *[a-zA-Z0-9_]+ *)?\{? *$"),
            Language::Python => regex::Regex::new(r"^ *def *([a-zA-Z0-9_]+) *\([.]*\) *: *$"),
            Language::Javascript => regex::Regex::new(
                r"^ *(?:function|const|let) *([a-zA-Z0-9_]+) *=? *\(.*\) *(?:: *[a-zA-Z0-9_]+)? *(?:=>)? *\{? *$",
            ),
            Language::Typescript => regex::Regex::new(r"^ *(?:function|const|let) *([a-zA-Z0-9_]+) *=? *\(.*\) *(?:: *[a-zA-Z0-9_]*)? *(?:=>)? *\{? *$"),
            Language::Golang => regex::Regex::new(r"^ *func *([a-zA-Z0-9_]+) *\(.*\) *(?:.*)? *\{? *$"),
            // In C++, when creating snippet of class function, you only need to input the 'function_name',
            // not the complete 'Class::function_name'
            Language::C => regex::Regex::new(r"^ *[a-zA-Z0-9_*& ]+(?: |::)([a-zA-Z0-9_]+)\(.*\) *\{? *$"),
            _ => regex::Regex::new(r".*"),
        }
        .unwrap()
    }

    pub fn get_comment_delimiters(&self) -> (String, String, String) {
        let comment_delimiters = match self {
            Language::Rust
            | Language::Javascript
            | Language::Typescript
            | Language::Golang
            | Language::C => ("//", "/*", "*/"),
            Language::Python => ("#", r#"""""#, r#"""""#),
            _ => ("", "", ""),
        };

        (comment_delimiters.0.to_string(), comment_delimiters.1.to_string(), comment_delimiters.2.to_string())
    }
}
