use std::{path, io::{self, BufRead}, fs::File};

mod rules;

fn read_lines(path: &path::Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(&path)?;
    let reader = std::io::BufReader::new(file);

    let lines = reader.lines();
    Ok(lines)
}

pub fn from_function(path: &path::Path, name: &String) -> std::io::Result<()> {
    let rule = match rules::Rule::new(&path) {
        Some(rule) => rule,
        None => return Err(std::io::Error::new(io::ErrorKind::Other, String::from("Error creating rules")))
    };

    let lines = read_lines(&path)?;
    for (idx, line) in lines.enumerate() {
        let line = line?;
        if rule.contains_function(&line, &name) {
            println!("Contains function: {} at line: {}", name, idx+1);
        }
    }

    Ok(())
}