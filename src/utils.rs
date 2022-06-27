use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufRead};
use std::path;

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<path::Path>,
{
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lines = reader.lines();
    Ok(lines)
}

pub fn merkle_tree_hash(lines: &Vec<String>) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let mut tmp = lines.clone();

    while tmp.len() > 1 {
        let first = &tmp[tmp.len() - 1];
        let second = &tmp[tmp.len() - 2];

        let combined = format!("{} {}", first, second);

        let mut hasher = Sha256::new();
        hasher.update(&combined);
        let hash = format!("{:x}", hasher.finalize());

        tmp.pop().unwrap();
        tmp.pop().unwrap();

        tmp.push(hash);
    }

    tmp[0].clone()
}
