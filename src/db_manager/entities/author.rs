use axum::Error;
use derive_new::new;
use rusqlite::{params, Connection};
use std::path::Path;

#[derive(new, PartialEq, Debug, Clone)]
pub struct Author {
    id : i32,
    name : String
}

pub fn add_author(db_path: &Path, name: String) -> Result<Author, Error> {
    let connection = Connection::open(db_path).map_err(|e| Error::new(e))?;

    connection.execute("INSERT INTO author (name) VALUES (?1)", params![name])
        .map_err(|e| Error::new(e))?;

    match get_author_by_name(db_path, name.clone()) {
        Some(author) => Ok(author),
        None => Err(Error::new(format!("Newly created author {} could not be found in database", name)))
    }
}

pub fn remove_author(db_path : &Path, id : i32) -> Result<(), Error> {
    let connection = Connection::open(db_path).map_err(|e| Error::new(e))?;
    connection.execute("DELETE FROM author WHERE id=?1", params![id]).map_err(|e| Error::new(e))?;
    return Ok(());
}

pub fn get_authors(db_path : &Path) -> Result<Vec<Author>, String> {
    let connection = Connection::open(db_path).map_err(|e| format!("An error occurred while trying to access database: {}", e))?;

    let mut stmt = connection.prepare("SELECT * FROM author").map_err(|e| format!("An error occured while getting all authors : {}", e))?;
    let authors_result = stmt.query_map([], |row| {Ok(Author::new(row.get(0)?, row.get(1)?))})
        .map_err(|e| format!("An error occured while getting all authors : {}", e));
    let authors : Result<Vec<Author>, rusqlite::Error> = authors_result?.collect();
    
    return authors.map_err(|e| format!("Failed to get authors : {}", e));
}

pub fn get_author_by_id(db_path : &Path, id: i32) -> Option<Author> {
    let connection = Connection::open(db_path).ok()?;
    let author = connection.query_row(
        "SELECT * FROM author WHERE id = ?1",
        params![id],
        |row| Ok(Author::new(row.get(0)?, row.get(1)?))
    ).ok()?;
    return Some(author);
}

pub fn get_author_by_name(db_path : &Path, name: String) -> Option<Author> {
    let connection = Connection::open(db_path).ok()?;
    let author = connection.query_row(
        "SELECT * FROM author WHERE name = ?1",
        params![name],
        |row| Ok(Author::new(row.get(0)?, row.get(1)?))
    ).ok()?;
    return Some(author);
}


#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use crate::db_manager::{create_database, entities::{add_author, get_authors, Author}};

    use super::{get_author_by_id, remove_author};

    #[test]
    fn adding_author_should_give_newly_created_author() {
        let test_db_path = Path::new("./adding_author_should_give_newly_created_author.db");
        create_database(test_db_path).unwrap();
        let maybe_added_author = add_author(test_db_path, "George R.R Martin".to_string()).unwrap();
        assert_eq!(Author::new(1, "George R.R Martin".to_string()), maybe_added_author);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_authors_should_give_all_authors() {
        let test_db_path = Path::new("./getting_all_authors_should_give_all_authors.db");
        create_database(test_db_path).unwrap();
        let rr_martin = add_author(test_db_path, "George R.R Martin".to_string()).unwrap();
        let jk_rowling = add_author(test_db_path, "J.K Rowling".to_string()).unwrap();
        let jrr_tolkien = add_author(test_db_path, "J.R.R Tolkien".to_string()).unwrap();
        let authors = [rr_martin, jk_rowling, jrr_tolkien].to_vec();
        let queried_authors = get_authors(test_db_path).unwrap();
        assert_eq!(authors, queried_authors);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_author_by_id_should_give_author() {
        let test_db_path = Path::new("./getting_author_by_id_should_give_author.db");
        create_database(test_db_path).unwrap();
        let rr_martin = add_author(test_db_path, "George R.R Martin".to_string()).unwrap();
        let maybe_rr_martin = get_author_by_id(test_db_path, rr_martin.id).unwrap();
        assert_eq!(rr_martin, maybe_rr_martin);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_author_should_delete_it() {
        let test_db_path = Path::new("./remove_author_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let rr_martin = add_author(test_db_path, "George R.R Martin".to_string()).unwrap();
        remove_author(test_db_path, rr_martin.id).unwrap();
        assert!(get_author_by_id(test_db_path, rr_martin.id).is_none());
        remove_file(test_db_path).unwrap();
    }
}