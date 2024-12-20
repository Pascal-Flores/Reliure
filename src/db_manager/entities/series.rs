use derive_new::new;
use rusqlite::params;
use std::path::Path;

use crate::db_manager::get_connection;

use super::{author, get_author_by_id, Author};

#[derive(new, PartialEq, Debug, Clone)]

pub struct Series {
    id : i32,
    author : Author,
    name : String
}

pub fn add_series(db_path: &Path, name: String, author : Author) -> Result<Series, String> {
    let connection = get_connection(db_path)?;
    let changed_row = connection.execute("INSERT INTO series (author, name) VALUES (?1, ?2)", params![author.id, name])
        .map_err(|e| format!("Could not add series {} to database", e))?;
    match get_series_by_name(db_path, name.clone()) {
        Some(series) => Ok(series),
        None => Err(format!("Newly created series {} could not be found in database", name))
    }
}

pub fn remove_series(db_path : &Path, id : i32) -> Result<(), String> {
    let connection = get_connection(db_path)?;
    connection.execute("DELETE FROM series WHERE id=?1", params![id])
        .map_err(|e| format!("Could not delete series {} from database", e))?;
    return Ok(());
}

pub fn get_series(db_path : &Path) -> Result<Vec<Series>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare("SELECT id, author, name FROM series")
        .map_err(|e| format!("An error occurred while getting all series: {}", e))?;
    let series_result = stmt.query_map([], |row| 
        Ok(make_series(db_path, row.get(0)?, row.get(1)?, row.get(2)?).unwrap())
        ).map_err(|e| format!("An error occurred while getting all series: {}", e))?;
    let series: Result<Vec<Series>, rusqlite::Error> = series_result.collect();
    series.map_err(|e| format!("Failed to get series: {}", e))
}

pub fn get_series_by_id(db_path : &Path, id: i32) -> Option<Series> {
    let connection = get_connection(db_path).ok()?;
    let series = connection.query_row(
        "SELECT id, author, name FROM series WHERE id = ?1",
        params![id],
        |row| Ok(make_series(db_path, row.get(0)?, row.get(1)?, row.get(2)?).unwrap())
    ).ok()?;
    return Some(series);
}

pub fn get_series_by_name(db_path : &Path, name: String) -> Option<Series> {
    let connection = get_connection(db_path).ok()?;
    let series = connection.query_row(
        "SELECT id, author, name FROM series WHERE name = ?1",
        params![name],
        |row| Ok(make_series(db_path, row.get(0)?, row.get(1)?, row.get(2)?).unwrap())
    ).ok()?;
    return Some(series);
}

fn make_series(db_path : &Path, id : i32, author_id : i32, name : String) -> Result<Series, String> {
    let author = get_author_by_id(db_path, author_id).unwrap();
    return Ok(Series::new(id, author, name));
}
#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use crate::db_manager::{create_database, entities::{add_author, series}};

    use super::{Series, add_series, remove_series, get_series, get_series_by_id, get_series_by_name};

    #[test]
    fn adding_series_should_give_newly_created_series() {
        let test_db_path = Path::new("./adding_series_should_give_newly_created_series.db");
        create_database(test_db_path).unwrap();
        let jk_rowling = add_author(test_db_path, "J.K Rowling".to_string()).unwrap();
        let maybe_added_series = add_series(test_db_path, "Harry Potter".to_string(), jk_rowling.clone()).unwrap();
        assert_eq!(Series::new(1, jk_rowling,"Harry Potter".to_string()), maybe_added_series);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_series_should_give_all_series() {
        let test_db_path = Path::new("./getting_all_series_should_give_all_series.db");
        create_database(test_db_path).unwrap();
        let rr_martin = add_author(test_db_path, "George R.R Martin".to_string()).unwrap();
        let jk_rowling = add_author(test_db_path, "J.K Rowling".to_string()).unwrap();
        let jrr_tolkien = add_author(test_db_path, "J.R.R Tolkien".to_string()).unwrap();
        let harry_potter = add_series(test_db_path, "Harry Potter".to_string(), rr_martin.clone()).unwrap();
        let a_song_of_ice_and_fire = add_series(test_db_path, "A song of ice and fire".to_string(), jk_rowling.clone()).unwrap();
        let the_lord_of_the_ring = add_series(test_db_path, "The lord of the rings".to_string(), jrr_tolkien.clone()).unwrap();
        let series = [harry_potter, a_song_of_ice_and_fire, the_lord_of_the_ring].to_vec();
        let queried_series = get_series(test_db_path).unwrap();
        assert_eq!(series, queried_series);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_series_by_id_should_give_series() {
        let test_db_path = Path::new("./getting_series_by_id_should_give_series.db");
        create_database(test_db_path).unwrap();
        let jk_rowling = add_author(test_db_path, "J.K Rowling".to_string()).unwrap();
        let harry_potter = add_series(test_db_path, "Harry Potter".to_string(), jk_rowling).unwrap();
        let maybe_harry_potter = get_series_by_id(test_db_path, harry_potter.id).unwrap();
        assert_eq!(harry_potter, maybe_harry_potter);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_series_should_delete_it() {
        let test_db_path = Path::new("./remove_series_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let jk_rowling = add_author(test_db_path, "J.K Rowling".to_string()).unwrap();
        let harry_potter = add_series(test_db_path, "Harry Potter".to_string(), jk_rowling.clone()).unwrap();
        remove_series(test_db_path, harry_potter.id).unwrap();
        assert!(get_series_by_id(test_db_path, harry_potter.id).is_none());
        remove_file(test_db_path).unwrap();
    }
}