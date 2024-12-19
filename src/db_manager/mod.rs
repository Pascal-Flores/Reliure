mod entities;

use std::{path::Path};

use rusqlite::Connection;



pub fn create_database(path : &Path) -> Result<(), String> {
    match Connection::open(path) {
        Err(error) => return Err(format!("An error occured while creating database : {}", error.to_string())),
        Ok(connection) => {
            return match connection.execute_batch("
                CREATE TABLE IF NOT EXISTS author (
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
                );
            ") {
                Err(error) => Err(format!("An error occured while initializing the database : {}", error.to_string())),
                Ok(_) => Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use super::*;

    #[test]
    fn database_is_successfully_created() {
        let test_db_path = Path::new("./test-db.db");
        let created_db = create_database(test_db_path);
        assert!(created_db.is_ok());
        remove_file(test_db_path).unwrap();
    }
}