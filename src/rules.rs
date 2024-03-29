use crate::language::Language;
use std::path;

#[derive(Debug, PartialEq)]
pub enum CommentType {
    SingleLine,
    MultiLineStart,
    MultiLineEnd,
    MultiLineComplete,
}

pub struct Rule {
    pub language: Language,
    pub delimiter: (String, String),
}

impl Rule {
    pub fn new(path: &path::Path) -> Option<Self> {
        let extension = path.extension()?.to_str().unwrap();
        let language = Language::from_extension(&extension);

        Some(Rule {
            language,
            delimiter: (String::from("{"), String::from("}")),
        })
    }

    pub fn contains_function(&self, line: &String, function_name: &String) -> bool {
        let function_syntax = self.language.get_function_syntax();
        if !function_syntax.is_match(&line) {
            return false;
        }

        match function_syntax.captures(&line) {
            Some(cap) => cap.get(1).unwrap().as_str() == function_name.as_str(),
            None => false,
        }
    }

    pub fn contains_comment(&self, line: &String) -> Option<CommentType> {
        let (single_line, multi_line_start, multi_line_end) = self.language.get_comment_delimiters();

        let trimmed = line.trim();
        if trimmed.starts_with(&single_line) {
            return Some(CommentType::SingleLine);
        } else if trimmed.starts_with(&multi_line_start)
            && trimmed.ends_with(&multi_line_end)
        {
            return Some(CommentType::MultiLineComplete);
        } else if trimmed.starts_with(&multi_line_start) {
            return Some(CommentType::MultiLineStart);
        } else if trimmed.ends_with(&multi_line_end) {
            return Some(CommentType::MultiLineEnd);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{CommentType, Language, Rule};
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
            let lang = lang.to_string();

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
            (
                "pub fn new<N>(params: &N::Params) -> RawNotification {",
                "new",
                Language::Rust,
                true,
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
            let result = rule.contains_function(&line.to_string(), &name.to_string());
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn detects_comment() {
        let comments = vec![
            (
                "// Single line comment",
                Language::Rust,
                Some(CommentType::SingleLine),
            ),
            ("No comment", Language::Rust, None),
            (
                "/* Multiline start",
                Language::Golang,
                Some(CommentType::MultiLineStart),
            ),
            (
                "Multiline end */",
                Language::C,
                Some(CommentType::MultiLineEnd),
            ),
            (
                "/* Multiline complete */",
                Language::Typescript,
                Some(CommentType::MultiLineComplete),
            ),
            // Python
            (
                "# Single line",
                Language::Python,
                Some(CommentType::SingleLine),
            ),
            (
                r#"""" Multi line complete """"#,
                Language::Python,
                Some(CommentType::MultiLineComplete),
            ),
            (
                r#"""" Multi line start"#,
                Language::Python,
                Some(CommentType::MultiLineStart),
            ),
            (
                r#"Multi line end """"#,
                Language::Python,
                Some(CommentType::MultiLineEnd),
            ),
        ];

        for (comment, lang, expected) in comments {
            let comment = comment.to_string();

            let rule = rule_from_language(lang);
            let result = rule.contains_comment(&comment);
            assert_eq!(result, expected);
        }
    }
}
