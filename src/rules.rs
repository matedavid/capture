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

        /* Regular expressions:
            - Rust: fn *([a-zA-Z0-9_]*) *\(.*\) *{?
            - Python: def *([a-zA-Z0-9_]*) *\([.]*\) *: *
            - Javascript: (?:function|const) *([a-zA-Z0-9_]*) *=? *\(.*\) *(?:=>)? *(?:{)?
            - ...
        */
        let function_syntax = match &language {
            Language::Rust => regex::Regex::new(r"fn *([a-zA-Z0-9_]*) *\(.*\) *\{?"),
            Language::Python => regex::Regex::new(r"def *([a-zA-Z0-9_]*) *\([.]*\) *: *"),
            Language::Javascript => regex::Regex::new(
                r"(?:function|const) *([a-zA-Z0-9_]*) *=? *\(.*\) *(?:=>)? *\{?",
            ),
            // TODO: Finish all regex
            _ => regex::Regex::new(r".*"),
        }
        .unwrap();

        // let comment_chars = match &language {
        //     Javascript => ("//", "/*"),
        //     _ => ("", ""),
        // };
        //let comment_chars = (String::from(comment_chars.0), String::from(comment_chars.1));

        Some(Rule {
            language,
            function_syntax,
            // comment_chars,
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

    #[test]
    fn detects_language() {
        let languages = vec![
            ("rust.rs", Language::Rust),
            ("python.py", Language::Python),
            ("path/to/typescript.ts", Language::Typescript),
            ("javascript.module.js", Language::Javascript),
            ("unknown.unknown", Language::Unknown),
        ];

        for language in languages {
            let lang = String::from(language.0);

            let path = path::Path::new(&lang);
            let rule = Rule::new(&path).unwrap();

            assert_eq!(rule.language, language.1);
        }
    }

    #[test]
    fn detects_function() {
        let js_rule = Rule::new(&path::Path::new("javascript.js")).unwrap();

        let contains = js_rule.contains_function(
            &String::from("function jsFunction() {"),
            &String::from("jsFunction"),
        );
        assert_eq!(contains, true);

        let contains =
            js_rule.contains_function(&String::from("const notFunction = 5;"), &String::new());
        assert_eq!(contains, false);
    }
}
