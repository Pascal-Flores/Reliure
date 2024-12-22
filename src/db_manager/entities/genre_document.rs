use std::path::Path;

use derive_new::new;
use rusqlite::params;

use super::document::Document;
use super::genre::Genre;

use crate::db_manager::get_connection;


#[derive(new, PartialEq, Debug, Clone)]
pub struct DocumentGenre {
    pub document : Document,
    pub genre : Genre
}

pub fn link_genre_to_document(db_path: &Path, document : &Document, genre : &Genre) -> Result<DocumentGenre, String> {
    let connection = get_connection(db_path)?;
    connection.execute("INSERT INTO document_genre (document, genre) VALUES (?1, ?2)", params![document.id, genre.id])
        .map_err(|e| format!("Could not add link between genre {} and document {} : {}", genre.name, document.name, e))?;
    match get_genre_document(db_path, document, genre) {
        Some(genre_document) => Ok(genre_document),
        None => Err(format!("Newly created link between genre {} and document {} could not be found in database", genre.name, document.name))
    }
}

pub fn unlink_genre_to_document(db_path: &Path, document : &Document, genre : &Genre) -> Result<(), String> {
    let connection = get_connection(db_path)?;
    connection.execute("DELETE FROM document_genre WHERE document = ?1 AND genre = ?2", params![document.id, genre.id])
        .map_err(|e| format!("Could not delete link between genre {} and document {} from database : {}", genre.id, document.id, e))?;
    return Ok(());
}


pub fn get_genre_document(db_path: &Path, document : &Document, genre : &Genre) -> Option<DocumentGenre> {
    let connection = get_connection(db_path).ok()?;
    let genre_document = connection.query_row(
    "SELECT document, genre FROM document_genre WHERE document = ?1 AND genre = ?2",
        params![document.id, genre.id],
        |_| Ok(DocumentGenre::new(document.clone(), genre.clone()))
    ).ok()?;
    return Some(genre_document);
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use chrono::NaiveDate;

    use crate::db_manager::{create_database, entities::{add_category, add_document, add_genre, add_tag, get_document_tag, get_genre_document, get_tag_by_id, get_tags, link_genre_to_document, link_tag_to_document, remove_tag, unlink_genre_to_document, unlink_tag_to_document, DocumentTag, DocumentGenre, Tag}};

    #[test]
    fn adding_genre_document_should_give_newly_created_document_tag() {
        let test_db_path = Path::new("./adding_genre_document_should_give_newly_created_document_tag.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let heroic_fantasy = add_genre(test_db_path, "Heroic Fantasy".to_string()).unwrap();
        let maybe_added_genre_document = link_genre_to_document(test_db_path, &the_fellowship_of_the_ring,&heroic_fantasy).unwrap();
        assert_eq!(DocumentGenre::new(the_fellowship_of_the_ring.clone(), heroic_fantasy.clone()), maybe_added_genre_document);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_genre_document_should_delete_it() {
        let test_db_path = Path::new("./remove_genre_document_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(test_db_path, "The Fellowship Of The Ring".to_string(), books.clone(), None, None, NaiveDate::from_ymd_opt(1954, 6, 29).unwrap(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let heroic_fantasy = add_genre(test_db_path, "Heroic Fantasy".to_string()).unwrap();
        link_genre_to_document(test_db_path, &the_fellowship_of_the_ring,&heroic_fantasy).unwrap();
        unlink_genre_to_document(test_db_path, &the_fellowship_of_the_ring, &heroic_fantasy).unwrap();
        assert!(get_genre_document(test_db_path, &the_fellowship_of_the_ring, &heroic_fantasy).is_none());
        remove_file(test_db_path).unwrap();
    }


}