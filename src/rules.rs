use regex;
use std::path;

#[derive(Debug, PartialEq, Eq)]
enum Language {
    Rust,
    Python,
    Javascript,
    Typescript,
    Golang,
    C,

    Unknown,
}

pub struct Rule {
    language: Language,
    function_syntax: regex::Regex,
    // comment_syntax: regex::Regex,
    pub delimiter: (String, String),
}

impl Rule {
    pub fn new(path: &path::Path) -> Option<Self> {
        let extension = path.extension()?.to_str().unwrap();

        let language = match extension {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" => Language::Javascript,
            "ts" => Language::Typescript,
            "go" => Language::Golang,
            "c" | "cpp" | "cc" => Language::C,
            _ => Language::Unknown,
        };

        let function_syntax = match &language {
            Language::Rust => regex::Regex::new(r"^ *fn *([a-zA-Z0-9_]+) *\(.*\) *\{? *$"),
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
        }.unwrap();

        Some(Rule {
            language,
            function_syntax,
            delimiter: (String::from("{"), String::from("}")),
        })
    }

    pub fn contains_function(&self, line: &String, function_name: &String) -> bool {
        if !self.function_syntax.is_match(&line) {
            return false;
        }

        match self.function_syntax.captures(&line) {
            Some(cap) => cap.get(1).unwrap().as_str() == function_name.as_str(),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Language, Rule};
    use std::path;

    fn rule_from_language(lang: Language) -> Rule {
        let rust_path = path::Path::new("rust.rs");
        let python_path = path::Path::new("python.py");
        let javascript_path = path::Path::new("javascript.js");
        let typescript_path = path::Path::new("typescript.ts");
        let golang_path = path::Path::new("golang.go");
        let c_path = path::Path::new("c.c");

        match lang {
            Language::Rust => Rule::new(&rust_path),
            Language::Python => Rule::new(&python_path),
            Language::Javascript => Rule::new(&javascript_path),
            Language::Typescript => Rule::new(&typescript_path),
            Language::Golang => Rule::new(&golang_path),
            Language::C => Rule::new(&c_path),
            _ => unreachable!(),
        }
        .unwrap()
    }

    #[test]
    fn detects_language() {
        let languages = vec![
            ("rust.rs", Language::Rust),
            ("python.py", Language::Python),
            ("path/to/typescript.ts", Language::Typescript),
            ("javascript.module.js", Language::Javascript),
            ("golang-file.go", Language::Golang),
            ("c_cpp.cc", Language::C),
            ("multiple.py.js", Language::Javascript),
            ("unknown.unknown", Language::Unknown),
        ];

        for (lang, expected) in languages {
            let lang = String::from(lang);

            let path = path::Path::new(&lang);
            let rule = Rule::new(&path).unwrap();

            assert_eq!(rule.language, expected);
        }
    }

    #[test]
    fn detects_function() {
        let functions = vec![
            // Rust
            (
                "fn rust_function() {",
                "rust_function",
                Language::Rust,
                true,
            ),
            (
                "func rust_function() {",
                "rust_function",
                Language::Rust,
                false,
            ),
            // Python
            ("def py_function():", "py_function", Language::Python, true),
            ("def py_function()", "py_function", Language::Python, false),
            // Javascript/Typescript
            ("function jsFunc() {", "jsFunc", Language::Javascript, true),
            ("function js_func(){", "jsFunc", Language::Javascript, false),
            ("function js_func", "js_func", Language::Javascript, false),
            ("let tsFunc = () => {", "tsFunc", Language::Typescript, true),
            (
                "let func = () number => {",
                "func",
                Language::Typescript,
                false,
            ),
            // Golang
            ("func goFunc() int {", "goFunc", Language::Golang, true),
            (
                "func go_func(func(int,int) int, int) int",
                "go_func",
                Language::Golang,
                true,
            ),
            (
                "func f(func(int,int) int, int) func(int, int) int { ",
                "f",
                Language::Golang,
                true,
            ),
            ("func () int {", "", Language::Golang, false),
            // C/C++
            ("static void* func() {", "func", Language::C, true),
            ("CustomClass func()", "func", Language::C, true),
            ("void () {", "", Language::C, false),
        ];

        for (line, name, lang, expected) in functions {
            let rule = rule_from_language(lang);
            let result = rule.contains_function(&String::from(line), &String::from(name));
            assert_eq!(result, expected);
        }
    }
}
