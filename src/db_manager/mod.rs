mod entities;

use std::{fs, path::Path};
use diesel::{connection::SimpleConnection, Connection, SqliteConnection};

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
        name TEXT NOT NULL,
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

fn establish_connection(db_url : &str) -> Result<SqliteConnection, String> {
    return SqliteConnection::establish(&db_url)
        .map_err(|_| format!("Error connecting to {}", db_url));
}

pub fn create_database(db_path : &Path) -> Result<(), String> {
    return match establish_connection(db_path.to_str().unwrap()) {
        Err(e) => Err(format!("An error occured while creating database {} : {}", db_path.display(), e)),
        Ok(mut connection) => {
            connection.batch_execute(DB_CREATION_QUERY)
                .map_err(|e| format!("An error occured while initiallizing database {} : {}", db_path.display(), e))?;
            return Ok(());
        }
    }
}

pub fn get_connection(db_path : &Path) -> Result<SqliteConnection, String> {
    return establish_connection(db_path.to_str().unwrap())
        .map_err(|e| format!("Could not access to databse {} : {}", db_path.display(), e));
}

pub fn delete_database(db_path: &Path) -> Result<(), String> {
    if db_path.exists() {
        fs::remove_file(db_path).map_err(|e| format!("An error occurred while deleting the database {}: {}",db_path.display(), e))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use super::*;

    #[test]
    fn database_is_successfully_created() {
        let test_db_path = Path::new("./database_is_successfully_created");
        let created_db = create_database(test_db_path);
        assert!(created_db.is_ok());
        remove_file(test_db_path).unwrap();

    }

    
}