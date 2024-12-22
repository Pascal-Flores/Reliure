use std::path::Path;

use derive_new::new;
use rusqlite::params;

use super::document::Document;
use super::tag::Tag;

use crate::db_manager::get_connection;

#[derive(new, PartialEq, Debug, Clone)]
pub struct DocumentTag {
    pub document : Document,
    pub tag : Tag
}

pub fn link_tag_to_document(db_path: &Path, document : &Document, tag : &Tag) -> Result<DocumentTag, String> {
    let connection = get_connection(db_path)?;
    connection.execute("INSERT INTO document_tag (document, tag) VALUES (?1, ?2)", params![document.id, tag.id])
        .map_err(|e| format!("Could not add link between tag {} and document {} : {}", tag.name, document.name, e))?;
    match get_document_tag(db_path, document, tag) {
        Some(document_tag) => Ok(document_tag),
        None => Err(format!("Newly created link between tag {} and document {} could not be found in database", tag.name, document.name))
    }
}

pub fn unlink_tag_to_document(db_path: &Path, document : &Document, tag : &Tag) -> Result<(), String> {
    let connection = get_connection(db_path)?;
    connection.execute("DELETE FROM document_tag WHERE document = ?1 AND tag = ?2", params![document.id, tag.id])
        .map_err(|e| format!("Could not delete link between tag {} and document {} from database : {}", tag.id, document.id, e))?;
    return Ok(());
}


pub fn get_document_tag(db_path: &Path, document : &Document, tag : &Tag) -> Option<DocumentTag> {
    let connection = get_connection(db_path).ok()?;
    let document_tag = connection.query_row(
    "SELECT document, tag FROM document_tag WHERE document = ?1 AND tag = ?2",
        params![document.id, tag.id],
        |_| Ok(DocumentTag::new(document.clone(), tag.clone()))
    ).ok()?;
    return Some(document_tag);
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use chrono::NaiveDate;

    use crate::db_manager::{create_database, entities::{add_category, add_document, add_tag, get_document_tag, get_tag_by_id, get_tags, link_tag_to_document, remove_tag, unlink_tag_to_document, DocumentTag, Tag}};

    #[test]
    fn adding_document_tag_should_give_newly_created_document_tag() {
        let test_db_path = Path::new("./adding_document_tag_should_give_newly_created_document_tag.db");
        create_database(test_db_path).unwrap();
        let mangas = add_category(test_db_path, "Mangas".to_string(), "~/Documents/Mangas".to_string()).unwrap();
        let one_piece_ch_1001 = add_document(test_db_path, "Chapter 1001".to_string(), mangas, None, None, NaiveDate::from_ymd_opt(2021, 1, 18).unwrap(), "Oda/One piece/chapter-1001.cbz".to_string()).unwrap();
        let chapters = add_tag(test_db_path, "manga chapters".to_string()).unwrap();
        let maybe_added_document_tag = link_tag_to_document(test_db_path, &one_piece_ch_1001, &chapters).unwrap();
        assert_eq!(DocumentTag::new(one_piece_ch_1001.clone(), chapters.clone()), maybe_added_document_tag);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_document_tag_should_delete_it() {
        let test_db_path = Path::new("./remove_document_tag_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mangas = add_category(test_db_path, "Mangas".to_string(), "~/Documents/Mangas".to_string()).unwrap();
        let one_piece_ch_1001 = add_document(test_db_path, "Chapter 1001".to_string(), mangas, None, None, NaiveDate::from_ymd_opt(2021, 1, 18).unwrap(), "Oda/One piece/chapter-1001.cbz".to_string()).unwrap();
        let chapters = add_tag(test_db_path, "manga chapters".to_string()).unwrap();
        unlink_tag_to_document(test_db_path, &one_piece_ch_1001, &chapters).unwrap();
        assert!(get_document_tag(test_db_path, &one_piece_ch_1001, &chapters).is_none());
        remove_file(test_db_path).unwrap();
    }


}

