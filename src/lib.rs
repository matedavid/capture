use std::{
    fs::File,
    io::{self, BufRead},
    path,
};

mod rules;

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<path::Path>,
{
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lines = reader.lines();
    Ok(lines)
}

pub struct Capture {
    rule: rules::Rule,
    path_str: String,
    result: Vec<String>,
}

impl Capture {
    pub fn new(path: &path::Path) -> io::Result<Self> {
        let rule = match rules::Rule::new(&path) {
            Some(rule) => rule,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    String::from(format!(
                        "Error creating Rule for {}",
                        path.to_str().unwrap()
                    )),
                ))
            }
        };

        Ok(Capture {
            rule,
            path_str: String::from(path.to_str().unwrap()),
            result: Vec::new(),
        })
    }

    pub fn from_function(&mut self, name: &String) -> io::Result<()> {
        if !self.result.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                String::from(format!("Snippet has already been created")),
            ));
        }

        let mut start_line: usize = 0;
        let mut end_line: usize = 0;

        let mut num_delimiters: usize = 0;

        let lines = read_lines(&self.path_str)?;
        for (idx, line) in lines.enumerate() {
            let line = line?;
            if self.rule.contains_function(&line, &name) {
                start_line = idx + 1;
            }

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

        self.from_interval(start_line, end_line)
    }

    pub fn from_interval(&mut self, start: usize, end: usize) -> io::Result<()> {
        if !self.result.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                String::from(format!("Snippet has already been created")),
            ));
        }

        let lines = read_lines(&self.path_str)?;

        let mut min_leading_spaces = -1;

        let mut result_lines = Vec::new();

        for (idx, line) in lines.enumerate() {
            let line = line?;
            let line_number = idx + 1;
            if line_number >= start && line_number <= end {
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
        }

        // Clean leading spaces in line based on the minimum number of leading spaces
        // found on the result strings, to prevent unnecessary indentation.
        for line in result_lines {
            let mut trimmed = String::new();
            let mut num = 0;
            for c in line.chars() {
                if c == ' ' && num < min_leading_spaces {
                    num += 1;
                } else if num >= min_leading_spaces {
                    trimmed.push(c);
                }
            }

            self.result.push(trimmed);
        }

        Ok(())
    }

    pub fn bookmark(&self, name: &String) {}

    pub fn snippet(&self, output_path: &path::Path) {}

    pub fn print(&self) {
        for line in &self.result {
            println!("{}", line);
        }
    }
}
