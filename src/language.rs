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
            "c" | "cpp" | "cc" => Language::C,
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
}

