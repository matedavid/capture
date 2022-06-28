use sqlite;
use std::{
    fmt::{self, write},
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

pub struct Bookmark {
    pub id: String,
    pub name: String,
    pub content: Vec<String>,
}

impl Bookmark {
    fn load(pair: &[(&str, Option<&str>)]) -> Self {
        let id = pair[0].1.unwrap();
        let name = pair[1].1.unwrap();

        let path = format!(".capture/{}", id);
        let content: Vec<String> = utils::read_lines(path)
            .unwrap()
            .map(|f| f.unwrap())
            .collect();

        Bookmark {
            id: String::from(id),
            name: String::from(name),
            content,
        }
    }

    pub fn print(&self, display_content: bool) {
        println!("Bookmark: {} - {}", self.name, self.id);
        if display_content {
            for line in &self.content {
                println!("{}", line);
            }
        }
    }
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bookmark: {} - {}\n", self.name, self.id)?;

        for line in &self.content {
            write!(f, "{}\n", line)?;
        }

        Ok(())
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

pub fn get(name: &String) -> io::Result<Option<Bookmark>> {
    let conn = get_connection()?;

    let mut bookmark: Option<Bookmark> = None;

    let statement = format!("SELECT * FROM bookmarks WHERE name = '{}'", name);
    match conn.iterate(statement, |pairs| {
        bookmark = Some(Bookmark::load(&pairs));
        true
    }) {
        Ok(()) => true,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    Ok(bookmark)
}

pub fn get_bookmarks() -> io::Result<Vec<Bookmark>> {
    let conn = get_connection()?;

    let mut bookmarks = Vec::new();

    match conn.iterate("SELECT * FROM bookmarks;", |pairs| {
        let bookmark = Bookmark::load(&pairs);
        bookmarks.push(bookmark);

        true
    }) {
        Ok(()) => Ok(bookmarks),
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}
