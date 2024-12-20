use derive_new::new;
use rusqlite::{params, Connection};
use std::path::Path;

use crate::db_manager::get_connection;

#[derive(new, PartialEq, Debug, Clone)]
pub struct Genre {
    id : i32,
    name : String
}

pub fn add_genre(db_path: &Path, name: String) -> Result<Genre, String> {
    let connection = get_connection(db_path)?;
    connection.execute("INSERT INTO genre (name) VALUES (?1)", params![name])
        .map_err(|e| format!("Could not add genre {} to database", e))?;

    match get_genre_by_name(db_path, name.clone()) {
        Some(genre) => Ok(genre),
        None => Err(format!("Newly created genre {} could not be found in database", name))
    }
}

pub fn remove_genre(db_path : &Path, id : i32) -> Result<(), String> {
    let connection = get_connection(db_path)?;
    connection.execute("DELETE FROM genre WHERE id=?1", params![id])
        .map_err(|e| format!("Could not delete author {} from database", e))?;
    return Ok(());
}

pub fn get_genres(db_path : &Path) -> Result<Vec<Genre>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare("SELECT * FROM genre")
        .map_err(|e| format!("An error occured while getting all genres : {}", e))?;
    let genre_result = stmt.query_map([], |row| {Ok(Genre::new(row.get(0)?, row.get(1)?))})
        .map_err(|e| format!("An error occured while getting all genres : {}", e));
    let genres : Result<Vec<Genre>, rusqlite::Error> = genre_result?.collect();
    return genres.map_err(|e| format!("Failed to get genres : {}", e));
}

pub fn get_genre_by_id(db_path : &Path, id: i32) -> Option<Genre> {
    let connection = get_connection(db_path).ok()?;
    let genre = connection.query_row(
        "SELECT * FROM genre WHERE id = ?1",
        params![id],
        |row| Ok(Genre::new(row.get(0)?, row.get(1)?))
    ).ok()?;
    return Some(genre);
}

pub fn get_genre_by_name(db_path : &Path, name: String) -> Option<Genre> {
    let connection = get_connection(db_path).ok()?;
    let genre = connection.query_row(
        "SELECT * FROM genre WHERE name = ?1",
        params![name],
        |row| Ok(Genre::new(row.get(0)?, row.get(1)?))
    ).ok()?;
    return Some(genre);
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use crate::db_manager::{create_database, entities::{add_author, get_authors, Author}};

    use super::{add_genre, get_genres, remove_genre, get_genre_by_id, get_genre_by_name, Genre};

    #[test]
    fn adding_genre_should_give_newly_created_genre() {
        let test_db_path = Path::new("./adding_genre_should_give_newly_created_genre.db");
        create_database(test_db_path).unwrap();
        let maybe_added_genre = add_genre(test_db_path, "Sci-Fi".to_string()).unwrap();
        assert_eq!(Genre::new(1, "Sci-Fi".to_string()), maybe_added_genre);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_genres_should_give_all_genres() {
        let test_db_path = Path::new("./getting_all_genres_should_give_all_genres.db");
        create_database(test_db_path).unwrap();
        let sci_fi = add_genre(test_db_path, "Sci-Fi".to_string()).unwrap();
        let heroic_fantsay = add_genre(test_db_path, "Heroic-Fantasy".to_string()).unwrap();
        let thriller = add_genre(test_db_path, "Thriller".to_string()).unwrap();
        let genres = [sci_fi, heroic_fantsay, thriller].to_vec();
        let queried_genres = get_genres(test_db_path).unwrap();
        assert_eq!(genres, queried_genres);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_genre_by_id_should_give_genre() {
        let test_db_path = Path::new("./getting_genre_by_id_should_give_genre.db");
        create_database(test_db_path).unwrap();
        let sci_fi = add_genre(test_db_path, "Sci-Fi".to_string()).unwrap();
        let maybe_sci_fi = get_genre_by_id(test_db_path, sci_fi.id).unwrap();
        assert_eq!(sci_fi, maybe_sci_fi);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_genre_should_delete_it() {
        let test_db_path = Path::new("./remove_genre_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let sci_fi = add_genre(test_db_path, "Sci-Fi".to_string()).unwrap();
        remove_genre(test_db_path, sci_fi.id).unwrap();
        assert!(get_genre_by_id(test_db_path, sci_fi.id).is_none());
        remove_file(test_db_path).unwrap();
    }
}