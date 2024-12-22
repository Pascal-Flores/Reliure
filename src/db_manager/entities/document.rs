use chrono::NaiveDate;
use derive_new::new;
use rusqlite::params;

use crate::db_manager::get_connection;

use super::category::{self, Category};
use super::author::Author;
use super::{get_author_by_id, get_category_by_id, get_series_by_id, tag, Genre, Tag};
use super::series::Series;
use std::path::Path;
use std::str::FromStr;


#[derive(new, PartialEq, Debug, Clone)]
pub struct Document {
    pub id : i32,
    pub name : String,
    pub category : Category,
    pub author : Option<Author>,
    pub series : Option<Series>,
    pub date : NaiveDate,
    pub path : String
}

pub fn add_document(db_path: &Path, name: String, category : Category, author : Option<Author>, series : Option<Series>, date : NaiveDate, path : String) -> Result<Document, String> {
    let connection = get_connection(db_path)?;
    connection.execute("INSERT INTO document (name, category, author, series, date, path) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", 
        params![name, 
                category.id, 
                author.map(|a| a.id),
                series.map(|s| s.id),
                date.to_string(),
                path
                 ])
        .map_err(|e| format!("Could not add document {} to database : {}", name, e))?;
    match get_document_by_name(db_path, name.clone()) {
        Some(document) => Ok(document),
        None => Err(format!("Newly created document {} could not be found in database", name))
    }
}

pub fn remove_document(db_path : &Path, id : i32) -> Result<(), String> {
    let connection = get_connection(db_path)?;
    connection.execute("DELETE FROM document WHERE id=?1", params![id])
        .map_err(|e| format!("Could not delete document from database : {}",e))?;
    return Ok(());
}

pub fn get_documents(db_path : &Path) -> Result<Vec<Document>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare("SELECT * FROM document")
        .map_err(|e| format!("An error occured while getting all documents : {}", e))?;
    let documents_result = stmt.query_map([], 
        |row| Ok(make_document(db_path, row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?).unwrap()))
        .map_err(|e| format!("An error occured while getting all documents : {}", e));
    let documents : Result<Vec<Document>, rusqlite::Error> = documents_result?.collect();
    return documents.map_err(|e| format!("Failed to get authors : {}", e));
}

pub fn get_documents_from_author(db_path : &Path, author : Author) -> Result<Vec<Document>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare("SELECT id, name, category, author, series, date, path FROM document WHERE author = ?1")
        .map_err(|e| format!("An error occured while getting all documents from {}: {}", author.name, e))?;
    let documents_result = stmt.query_map(params![author.id], 
        |row| Ok(make_document(db_path, row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?).unwrap()))
        .map_err(|e| format!("An error occured while getting all documents from {}: {}", author.name, e));
    let documents : Result<Vec<Document>, rusqlite::Error> = documents_result?.collect();
    return documents.map_err(|e| format!("Failed to get documents : {}", e));
}

pub fn get_documents_from_series(db_path : &Path, series : Series) -> Result<Vec<Document>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare("SELECT id, name, category, author, series, date, path FROM document WHERE series = ?1")
        .map_err(|e| format!("An error occured while getting all documents from {}: {}", series.name, e))?;
    let documents_result = stmt.query_map(params![series.id], 
        |row| Ok(make_document(db_path, row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?).unwrap()))
        .map_err(|e| format!("An error occured while getting all documents from {}: {}", series.name, e));
    let documents : Result<Vec<Document>, rusqlite::Error> = documents_result?.collect();
    return documents.map_err(|e| format!("Failed to get documents : {}", e));
}


pub fn get_documents_by_tag(db_path : &Path, tag : Tag) -> Result<Vec<Document>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare(
        "SELECT d.id, d.name, d.category, d.author, d.series, d.date, d.path FROM document d
        JOIN document_tag dt ON d.id = dt.document
        JOIN tag t ON dt.tag = t.id WHERE t.id = ?1")
        .map_err(|e| format!("An error occured while getting all documents tagged {}: {}", tag.name, e))?;
    let documents_result = stmt.query_map(params![tag.id], 
        |row| Ok(make_document(db_path, row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?).unwrap()))
        .map_err(|e| format!("An error occured while getting all documents tagged {}: {}", tag.name, e));
    let documents : Result<Vec<Document>, rusqlite::Error> = documents_result?.collect();
    return documents.map_err(|e| format!("Failed to get documents : {}", e));
}

pub fn get_documents_by_genre(db_path : &Path, genre : Genre) -> Result<Vec<Document>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare(
        "SELECT d.id, d.name, d.category, d.author, d.series, d.date, d.path FROM document d
        JOIN document_genre dg ON d.id = dg.document
        JOIN genre g ON dg.genre = g.id WHERE g.id = ?1")
        .map_err(|e| format!("An error occured while getting all documents with genre {}: {}", genre.name, e))?;
    let documents_result = stmt.query_map(params![genre.id], 
        |row| Ok(make_document(db_path, row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?).unwrap()))
        .map_err(|e| format!("An error occured while getting all documents with genre {}: {}", genre.name, e));
    let documents : Result<Vec<Document>, rusqlite::Error> = documents_result?.collect();
    return documents.map_err(|e| format!("Failed to get documents : {}", e));
}


pub fn get_document_by_id(db_path : &Path, id: i32) -> Option<Document> {
    let connection = get_connection(db_path).ok()?;
    let document = connection.query_row(
        "SELECT id, name, category, author, series, date, path FROM document WHERE id = ?1",
        params![id],
        |row| Ok(make_document(db_path, row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?).unwrap())
    ).ok()?;
    return Some(document);
}

pub fn get_document_by_name(db_path : &Path, name: String) -> Option<Document> {
    let connection = get_connection(db_path).ok()?;
    let document = connection.query_row(
        "SELECT id, name, category, author, series, date, path FROM document WHERE name = ?1",
        params![name],
        |row| Ok(make_document(db_path, row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?).unwrap())
    ).ok()?;
    return Some(document);
}

fn make_document(db_path : &Path, id : i32, name : String, category_id : i32, author_id : Option<i32>, series_id : Option<i32>, date : String, path : String) -> Result<Document, String> {
    let category = get_category_by_id(db_path, category_id).unwrap();
    let author = match author_id {
        Some(id) => get_author_by_id(db_path, id),
        None => None
    };
    let series = match series_id {
        Some(id) => get_series_by_id(db_path, id),
        None => None
    };
    return Ok(Document::new(id, name, category, author, series, NaiveDate::from_str(&date).unwrap(), path));

}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use chrono::NaiveDate;

    use crate::db_manager::{create_database, entities::{add_author, add_category, add_document, add_genre, add_series, add_tag, author, get_document_by_id, get_documents, get_documents_by_genre, get_documents_by_tag, get_documents_from_author, get_documents_from_series, link_genre_to_document, link_tag_to_document, remove_document, series, Document}};

    #[test]
    fn adding_document_should_give_newly_created_document() {
        let test_db_path = Path::new("./adding_document_should_give_newly_created_document.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let jrr_tolkien = add_author(test_db_path, "J.R.R Tolkien".to_string()).unwrap();
        let the_lord_of_the_rings = add_series(test_db_path, "The Lord Of The Rings".to_string(), jrr_tolkien.clone()).unwrap();
        let maybe_the_fellowship_of_the_ring = add_document(test_db_path, "The fellowship of the ring".to_string(), books.clone(), Some(jrr_tolkien.clone()), Some(the_lord_of_the_rings.clone()), NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        assert_eq!(Document::new(1, "The fellowship of the ring".to_string(), books, Some(jrr_tolkien), Some(the_lord_of_the_rings),NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()), maybe_the_fellowship_of_the_ring);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_documents_should_give_all_documents() {
        let test_db_path = Path::new("./getting_all_documents_should_give_all_documents.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(test_db_path, "The Two Towers".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 11, 11).unwrap(), "Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(test_db_path, "The Return of The King".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1955, 10, 20).unwrap(), "Tolkien/the_return_of_the_king".to_string()).unwrap();
        let documents = [the_fellowship_of_the_ring, the_two_towers, the_return_of_the_king].to_vec();
        let queried_documents = get_documents(test_db_path).unwrap();
        assert_eq!(documents, queried_documents);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_document_by_id_should_give_document() {
        let test_db_path = Path::new("./getting_document_by_id_should_give_document.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let maybe_the_fellowship_of_the_ring = get_document_by_id(test_db_path, the_fellowship_of_the_ring.id).unwrap();
        assert_eq!(the_fellowship_of_the_ring, maybe_the_fellowship_of_the_ring);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_document_should_delete_it() {
        let test_db_path = Path::new("./remove_document_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        remove_document(test_db_path, the_fellowship_of_the_ring.id).unwrap();
        assert!(get_document_by_id(test_db_path, the_fellowship_of_the_ring.id).is_none());
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_documents_from_author_should_only_return_documents_from_author() {
        let test_db_path = Path::new("./getting_documents_from_author_should_only_return_documents_from_author.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let jrr_tolkien = add_author(test_db_path, "J.R.R Tolkien".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), Some(jrr_tolkien.clone()), None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(test_db_path, "The Two Towers".to_string(), books.clone(), Some(jrr_tolkien.clone()), None, NaiveDate::from_ymd_opt(1954, 11, 11).unwrap(), "Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(test_db_path, "The Return of The King".to_string(), books.clone(), Some(jrr_tolkien.clone()), None, NaiveDate::from_ymd_opt(1955, 10, 20).unwrap(), "Tolkien/the_return_of_the_king".to_string()).unwrap();
        let from_tolkien = [the_fellowship_of_the_ring, the_two_towers, the_return_of_the_king].to_vec();
        let rr_martin = add_author(test_db_path, "R.R Martin".to_string()).unwrap();
        let a_game_of_throne = add_document(test_db_path, "A Game Of Thrones".to_string(), books.clone(), Some(rr_martin), None, NaiveDate::from_ymd_opt(1996, 8, 6).unwrap(), "Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let queried_documents = get_documents_from_author(test_db_path, jrr_tolkien.clone()).unwrap();
        assert_eq!(from_tolkien, queried_documents);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_documents_from_series_should_only_return_documents_from_series() {
        let test_db_path = Path::new("./getting_documents_from_series_should_only_return_documents_from_series.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let jrr_tolkien = add_author(test_db_path, "J.R.R Tolkien".to_string()).unwrap();
        let the_lord_of_the_ring = add_series(test_db_path, "The Lord Of The Rings".to_string(), jrr_tolkien.clone()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), Some(jrr_tolkien.clone()), Some(the_lord_of_the_ring.clone()), NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(test_db_path, "The Two Towers".to_string(), books.clone(), Some(jrr_tolkien.clone()), Some(the_lord_of_the_ring.clone()), NaiveDate::from_ymd_opt(1954, 11, 11).unwrap(), "Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(test_db_path, "The Return of The King".to_string(), books.clone(), Some(jrr_tolkien.clone()), Some(the_lord_of_the_ring.clone()), NaiveDate::from_ymd_opt(1955, 10, 20).unwrap(), "Tolkien/the_return_of_the_king".to_string()).unwrap();
        let from_the_lord_of_the_ring = [the_fellowship_of_the_ring, the_two_towers, the_return_of_the_king].to_vec();
        let rr_martin = add_author(test_db_path, "R.R Martin".to_string()).unwrap();
        let a_song_of_ice_and_fire = add_series(test_db_path, "A Song Of Ice And Fire".to_string(), rr_martin.clone()).unwrap();
        let a_game_of_throne = add_document(test_db_path, "A Game Of Thrones".to_string(), books.clone(), Some(rr_martin), Some(a_song_of_ice_and_fire.clone()), NaiveDate::from_ymd_opt(1996, 8, 6).unwrap(), "Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let queried_documents = get_documents_from_series(test_db_path, the_lord_of_the_ring.clone()).unwrap();
        assert_eq!(from_the_lord_of_the_ring, queried_documents);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_tagged_documents_should_only_return_documents_with_tag() {
        let test_db_path = Path::new("./getting_tagged_documents_should_only_return_documents_with_tag.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(test_db_path, "The Two Towers".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 11, 11).unwrap(), "Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(test_db_path, "The Return of The King".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1955, 10, 20).unwrap(), "Tolkien/the_return_of_the_king".to_string()).unwrap();
        let bangers = [the_fellowship_of_the_ring.clone(), the_two_towers.clone(), the_return_of_the_king.clone()].to_vec();
        let a_game_of_throne = add_document(test_db_path, "A Game Of Thrones".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1996, 8, 6).unwrap(), "Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let banger = add_tag(test_db_path, "banger".to_string()).unwrap();
        link_tag_to_document(test_db_path, &the_fellowship_of_the_ring, &banger).unwrap();
        link_tag_to_document(test_db_path, &the_two_towers, &banger).unwrap();
        link_tag_to_document(test_db_path, &the_return_of_the_king, &banger).unwrap();
        let queried_documents = get_documents_by_tag(test_db_path, banger).unwrap();
        assert_eq!(bangers, queried_documents);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_documents_with_genre_should_only_return_documents_with_genre() {
        let test_db_path = Path::new("./getting_documents_with_genre_should_only_return_documents_with_genre.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(test_db_path, "The Two Towers".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 11, 11).unwrap(), "Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(test_db_path, "The Return of The King".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1955, 10, 20).unwrap(), "Tolkien/the_return_of_the_king".to_string()).unwrap();
        let hf = [the_fellowship_of_the_ring.clone(), the_two_towers.clone(), the_return_of_the_king.clone()].to_vec();
        let a_game_of_throne = add_document(test_db_path, "A Game Of Thrones".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1996, 8, 6).unwrap(), "Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let heroic_fantasy = add_genre(test_db_path, "Heroic Fantasy".to_string()).unwrap();
        link_genre_to_document(test_db_path, &the_fellowship_of_the_ring, &heroic_fantasy).unwrap();
        link_genre_to_document(test_db_path, &the_two_towers, &heroic_fantasy).unwrap();
        link_genre_to_document(test_db_path, &the_return_of_the_king, &heroic_fantasy).unwrap();
        let queried_documents = get_documents_by_genre(test_db_path, heroic_fantasy).unwrap();
        assert_eq!(hf, queried_documents);
        remove_file(test_db_path).unwrap();
    }
}