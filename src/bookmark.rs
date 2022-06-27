use sqlite;
use std::{fs::File, io::Write};

const INDEX_FILE_PATH: &str = ".capture/index.sql";

fn get_connection() -> Result<sqlite::Connection, sqlite::Error> {
    sqlite::open(INDEX_FILE_PATH)
}

pub fn setup() {
    let conn = get_connection().unwrap();
    conn.execute("CREATE TABLE bookmarks (id TEXT PRIMARY KEY, name TEXT);").unwrap();
}

pub fn create(name: &String, lines: &Vec<String>) {
    // TODO: Instead of saving file with bookmark name, hash it
    // and use the identifier as id and file name. 

    let path = format!(".capture/{}", name);
    let mut file = File::create(&path).unwrap();

    for line in lines {
        let line = format!("{}\n", line);
        file.write(line.as_bytes()).unwrap();
    }

    let conn = get_connection().unwrap();

    let statement = format!("INSERT INTO bookmarks VALUES ('{}', '{}');", name, name);
    conn.execute(&statement).unwrap();
}

pub fn exists(name: &String) -> bool {
    todo!();
}

pub fn list() {
    let conn = get_connection().unwrap();

    conn.iterate("SELECT * FROM bookmarks;", |pairs| {
        let id = pairs[0].1.unwrap();
        let name = pairs[1].1.unwrap();

        println!("{} {}", id, name);
        true
    }).unwrap();
}

/*
struct Bookmark {
    id: String,
    name: String,
    content: Vec<String>,
    path: String,
}
*/