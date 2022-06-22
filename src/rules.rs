use regex;

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
    //comment_chars: (String, String),
}

impl Rule {
    pub fn new(file_name: &String) -> Option<Self> {
        let path = std::path::Path::new(file_name);
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
            - Javascript: (?:function|const) *([a-zA-Z0-9_]*) *=? *\(.*\) *(?:=>)? *{
            - ...
        */
        let function_syntax = match &language {
            // TODO: Learn why it throw error in '|' character
            Javascript => {
                regex::Regex::new(r"(?:function|const) *([a-zA-Z0-9_]*) *=? *\(.*\) *(?:=>)? *\{")
            }
            _ => regex::Regex::new(r".*"),
        }
        .unwrap();

        let comment_chars = match &language {
            Javascript => ("//", "/*"),
            _ => ("", ""),
        };
        //let comment_chars = (String::from(comment_chars.0), String::from(comment_chars.1));

        Some(Rule {
            language,
            function_syntax,
            //comment_chars,
        })
    }

    pub fn contains_function(&self, line: &String) -> bool {
        self.function_syntax.is_match(line)
    }
}

// fn function_syntax(keyword: &String, tail: &String, limiters: (char, char)) -> regex::Regex {
//     regex::Regex::new(r"example").unwrap()
// }

#[cfg(test)]
mod tests {
    use super::{Language, Rule};

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
            let rule = Rule::new(&lang).unwrap();
            assert_eq!(rule.language, language.1);
        }
    }

    #[test]
    fn detects_function() {
        let js_rule = Rule::new(&String::from("javascript.js")).unwrap();

        let contains = js_rule.contains_function(&String::from("function jsFunction() {"));
        assert_eq!(contains, true);

        let contains = js_rule.contains_function(&String::from("const notFunction = 5;"));
        assert_eq!(contains, false);
    }
}
