use sqlite;
use std::{fs::File, io::Write};

const INDEX_FILE_PATH: &str = ".capture/index.sql";

fn get_connection() -> Result<sqlite::Connection, sqlite::Error> {
    sqlite::open(INDEX_FILE_PATH)
}

pub fn create(name: &String, lines: &Vec<String>) {
    // TODO: Instead of saving file with bookmark name, hash it
    // and use the identifier as the name.
    // Also, save the relevant information in INDEX_FILE_PATH 

    let path = format!(".capture/{}", name);
    let mut file = File::create(path).unwrap();

    for line in lines {
        let line = format!("{}\n", line);
        file.write(line.as_bytes()).unwrap();
    }
}

pub fn exists(name: &String) -> bool {
    todo!();
}

/*
struct Bookmark {
    id: String,
    name: String,
    content: Vec<String>,
    path: String,
}
*/