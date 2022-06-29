use std::{
    io::{self},
    path,
};

pub mod bookmark;
mod rules;
mod utils;

pub struct Capture {
    rule: rules::Rule,
    path_str: String,
    pub result: Vec<String>,
}

impl Capture {
    pub fn new(path: &path::Path) -> Result<Self, String> {
        let rule = match rules::Rule::new(&path) {
            Some(rule) => rule,
            None => {
                let error_msg = format!("Error creating Rule for: {}", path.to_str().unwrap());
                return Err(error_msg);
            }
        };

        Ok(Capture {
            rule,
            path_str: String::from(path.to_str().unwrap()),
            result: Vec::new(),
        })
    }

    pub fn from_function(&mut self, name: &String, include_comments: bool) -> io::Result<()> {
        if !self.result.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                String::from(format!("Snippet has already been created")),
            ));
        }

        let mut start_line: usize = 0;
        let mut end_line: usize = 0;

        let mut num_delimiters: usize = 0;

        let lines = utils::read_lines(&self.path_str)?;
        for (idx, line) in lines.enumerate() {
            let line = line?;
            if self.rule.contains_function(&line, &name) {
                start_line = idx + 1;
            }

            // TODO: Revisit this implementation, and should test for programming languages
            // that do not use an explicit delimiter such as python
            if start_line != 0 && line.contains(&self.rule.delimiter.0) {
                num_delimiters += 1;
            } else if start_line != 0 && line.contains(&self.rule.delimiter.1) {
                num_delimiters -= 1;

                if num_delimiters == 0 {
                    end_line = idx + 1;
                    break;
                }
            }
        }

        if start_line == 0 && end_line == 0 {
            let err_msg = format!("Function '{}' not found in {}", name, self.path_str);
            return Err(io::Error::new(io::ErrorKind::NotFound, err_msg));
        }

        self.from_interval(start_line, end_line, include_comments)
    }

    pub fn from_interval(
        &mut self,
        start: usize,
        end: usize,
        include_comments: bool,
    ) -> io::Result<()> {
        if !self.result.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                String::from(format!("Snippet has already been created")),
            ));
        }

        let lines = utils::read_lines(&self.path_str)?;
        let mut result_lines = Vec::new();

        let mut min_leading_spaces = -1;
        let mut number_multiline_comment: usize = 0;

        for (idx, line) in lines.enumerate() {
            let line_number = idx + 1;
            if line_number < start || line_number > end {
                continue;
            }

            let line = line?;

            if !include_comments {
                match self.rule.contains_comment(&line) {
                    Some(rules::CommentType::SingleLine) => continue,
                    Some(rules::CommentType::MultiLineStart) => {
                        number_multiline_comment += 1;
                        continue;
                    }
                    Some(rules::CommentType::MultiLineEnd) => {
                        number_multiline_comment -= 1;
                        continue;
                    }
                    Some(rules::CommentType::MultiLineComplete) => continue,
                    None => (),
                }
            }

            if number_multiline_comment > 0 {
                continue;
            }

            // Compute number of leading spaces for later cleaning
            let leading_spaces = {
                let mut number = 0;
                for c in line.chars() {
                    if c == ' ' {
                        number += 1;
                    } else if c != ' ' {
                        break;
                    }
                }

                if line.is_empty() {
                    min_leading_spaces
                } else {
                    number
                }
            };

            if min_leading_spaces == -1
                || std::cmp::min(min_leading_spaces, leading_spaces) == leading_spaces
            {
                min_leading_spaces = leading_spaces;
            }

            result_lines.push(line.clone());
        }

        // Clean leading spaces based on the minimum number of leading spaces
        // found on the result strings, to prevent unnecessary indentation.
        for mut line in result_lines {
            let mut num = 0;
            while !line.is_empty()
                && line.chars().next().unwrap() == ' '
                && num < min_leading_spaces
            {
                line.remove(0);
                num += 1;
            }

            self.result.push(line);
        }

        Ok(())
    }

    pub fn bookmark(&self, name: &String) -> io::Result<()> {
        bookmark::create(&name, &self.result)
    }

    pub fn print(&self) {
        for line in &self.result {
            println!("{}", line);
        }
    }
}
