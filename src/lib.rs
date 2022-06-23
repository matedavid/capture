use std::{
    fs::File,
    io::{self, BufRead},
    path,
};

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
        None => {
            return Err(std::io::Error::new(
                io::ErrorKind::Other,
                String::from(format!(
                    "Error creating Rule for {}",
                    path.to_str().unwrap()
                )),
            ))
        }
    };

    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;

    let mut num_delimiters: usize = 0;

    let mut result_lines: Vec<String> = Vec::new();

    let lines = read_lines(&path)?;
    for (idx, line) in lines.enumerate() {
        let line = line?;
        if rule.contains_function(&line, &name) {
            start_idx = idx + 1;
        }

        if start_idx != 0 {
            result_lines.push(line.clone());
        }

        if start_idx != 0 && line.contains(&rule.delimiter.0) {
            num_delimiters += 1;
        } else if start_idx != 0 && line.contains(&rule.delimiter.1) {
            num_delimiters -= 1;

            if num_delimiters == 0 {
                end_idx = idx + 1;
                break;
            }
        }
    }

    println!("Result:");
    for line in result_lines {
        println!("{}", line);
    }

    //let collected: Vec<String> = lines.map(|f| return f.unwrap()).collect();

    Ok(())
}
