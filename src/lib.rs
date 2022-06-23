use std::{
    fs::File,
    io::{self, BufRead},
    path,
};

mod rules;

fn read_lines(path: &path::Path) -> io::Result<io::Lines<io::BufReader<File>>> {
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
        let path = path::Path::new(&self.path_str);

        let mut found = false;
        let mut num_delimiters: usize = 0;

        let mut result_lines = Vec::new();

        let lines = read_lines(&path)?;
        for line in lines {
            let line = line?;
            if self.rule.contains_function(&line, &name) {
                found = true;
            }

            if found {
                result_lines.push(line.clone());
            }

            if found && line.contains(&self.rule.delimiter.0) {
                num_delimiters += 1;
            } else if found && line.contains(&self.rule.delimiter.1) {
                num_delimiters -= 1;

                if num_delimiters == 0 {
                    break;
                }
            }
        }

        self.result = result_lines;

        Ok(())
    }

    pub fn from_interval(&mut self, start: usize, end: usize) {}

    pub fn bookmark(&self, name: &String) {}

    pub fn snippet(&self, output_path: &path::Path) {}

    pub fn print(&self) {
        for line in &self.result {
            println!("{}", line);
        }
    }
}
