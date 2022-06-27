use sqlite;
use std::{
    fs::File,
    io::{self, Write},
};

use crate::utils;

const INDEX_FILE_PATH: &str = ".capture/index.sql";
fn get_connection() -> io::Result<sqlite::Connection> {
    match sqlite::open(INDEX_FILE_PATH) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}

pub fn setup() {
    let conn = get_connection().unwrap();
    conn.execute("CREATE TABLE bookmarks (id TEXT PRIMARY KEY, name TEXT);")
        .unwrap();
}

struct Bookmark {
    id: String,
    name: String,
    content: Vec<String>,
}

fn get_bookmarks() -> io::Result<Vec<Bookmark>> {
    let conn = get_connection()?;

    let mut bookmarks = Vec::new();

    match conn.iterate("SELECT * FROM bookmarks;", |pairs| {
        let id = pairs[0].1.unwrap();
        let name = pairs[1].1.unwrap();

        let path = format!(".capture/{}", id);
        let content: Vec<String> = utils::read_lines(path)
            .unwrap()
            .map(|f| f.unwrap())
            .collect();

        let bookmark = Bookmark {
            id: String::from(id),
            name: String::from(name),
            content,
        };
        bookmarks.push(bookmark);

        true
    }) {
        Ok(()) => Ok(bookmarks),
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}

fn exists(name: &String) -> io::Result<bool> {
    let mut num_matches: usize = 0;

    let conn = get_connection()?;
    let statement = format!("SELECT * FROM bookmarks WHERE name = '{}'", name);

    match conn.iterate(statement, |_| {
        num_matches += 1;

        true
    }) {
        Ok(()) => true,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    if num_matches > 1 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "More than one bookmark with the same name",
        ));
    }

    Ok(num_matches == 1)
}

pub fn create(name: &String, lines: &Vec<String>) -> io::Result<()> {
    let id = utils::merkle_tree_hash(&lines);

    let path = format!(".capture/{}", id);
    let mut file = File::create(&path)?;


    for line in lines {
        let line = format!("{}\n", line);
        file.write(line.as_bytes())?;
    }

    let conn = get_connection()?;

    let statement = format!("INSERT INTO bookmarks VALUES ('{}', '{}');", id, name);
    match conn.execute(&statement) {
        Ok(()) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}

pub fn delete(name: &String) -> io::Result<()> {
    todo!();
}

pub fn list() {
    let bookmarks = get_bookmarks().unwrap();

    for b in bookmarks {
        println!("Bookmark: {} - {}", b.name, b.id);
        for line in b.content {
            println!("{}", line);
        }
        print!("\n");
    }
}
