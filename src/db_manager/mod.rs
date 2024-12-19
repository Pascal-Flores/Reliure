mod entities;

use core::error;
use std::path::Path;

use entities::Author;
use rusqlite::{Connection, params};

const DB_CREATION_QUERY : &str = 
    "CREATE TABLE IF NOT EXISTS author (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS series (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        author INTEGER,
        name TEXT NOT NULL,
        FOREIGN KEY (author) REFERENCES author(id)
    );

    CREATE TABLE IF NOT EXISTS tag (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS genre (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS category (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        path TEXT NOT NULL    
    );

    CREATE TABLE IF NOT EXISTS document (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        category INTEGER,
        author INTEGER,
        series INTEGER,
        date TEXT,
        path TEXT NOT NULL,
    FOREIGN KEY (category) REFERENCES category(id),
    FOREIGN KEY (author) REFERENCES author(id),
    FOREIGN KEY (series) REFERENCES series(id)
    );
        
    CREATE TABLE IF NOT EXISTS document_category (
        document INTEGER,
        category INTEGER,
        PRIMARY KEY (document, category),
        FOREIGN KEY (document) REFERENCES document(id),
        FOREIGN KEY (category) REFERENCES category(id)
    );

    CREATE TABLE IF NOT EXISTS author_series (
        author INTEGER,
        series INTEGER,
        PRIMARY KEY (author, series),
        FOREIGN KEY (author) REFERENCES author(id),
        FOREIGN KEY (series) REFERENCES series(id)
    );

    CREATE TABLE IF NOT EXISTS document_genre (
        document INTEGER,
        genre INTEGER,
        PRIMARY KEY (document, genre),
        FOREIGN KEY (document) REFERENCES document(id),
        FOREIGN KEY (genre) REFERENCES genre(id)
    );

    CREATE TABLE IF NOT EXISTS document_tag (
        document INTEGER,
        tag INTEGER,
        PRIMARY KEY (document, tag),
        FOREIGN KEY (document) REFERENCES document(id),
        FOREIGN KEY (tag) REFERENCES tag(id)
    );";

pub fn create_database(path : &Path) -> Result<(), String> {
    match Connection::open(path) {
        Err(error) => return Err(format!("An error occured while creating database : {}", error.to_string())),
        Ok(connection) => {
            return match connection.execute_batch(DB_CREATION_QUERY) {
                Err(error) => Err(format!("An error occured while initializing the database : {}", error.to_string())),
                Ok(_) => Ok(())
            }
        }
    }
}

pub fn add_author(db_path : &Path, author_name : String) -> Result<Author, String> {
    match Connection::open(db_path) {
        Err(error) => Err(format!("An error occured while trying to access database : {}", error.to_string())),
        Ok(connection) => {
            match connection.execute("INSERT INTO author (name) VALUES (?1)", params![author_name]) {
                Err(error) => Err(format!("An error occured while trying to add author {} : {}", author_name, error.to_string())),
                Ok(_) => match connection.query_row("SELECT * FROM 'author' WHERE name=:name", &[(":name", author_name.as_str())], |row| Ok(Author::new(row.get(0)?, row.get(1)?))) {
                    Err(error) => Err(format!("An error occured while fetching newly created author {} : {}", author_name, error.to_string())),
                    Ok(author) => return Ok(author)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, thread, time::Duration};

    use super::*;

    #[test]
    fn database_is_successfully_created() {
        let test_db_path = Path::new("./test-db.db");
        let created_db = create_database(test_db_path);
        assert!(created_db.is_ok());
        remove_file(test_db_path).unwrap();
        thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn adding_author_should_give_newly_created_author() {
        let test_db_path = Path::new("./test-db.db");
        create_database(test_db_path).unwrap();
        let maybe_added_author = add_author(test_db_path, "George R.R Martin".to_string()).unwrap();
        assert_eq!(Author::new(1, "George R.R Martin".to_string()), maybe_added_author);
        remove_file(test_db_path).unwrap();
        thread::sleep(Duration::from_secs(2));
    }
}