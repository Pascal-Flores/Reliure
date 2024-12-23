use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, Connection, ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::db_manager::entities::schema::document_tag::dsl::*;

#[derive(Queryable, PartialEq, Debug, new, Clone)]
pub struct DocumentTag {
    pub document_ : i32,
    pub tag_ : i32
}

pub fn link_tag_to_document(connection : &mut SqliteConnection, document_id: &i32, tag_id : &i32) -> Result<DocumentTag, String> {
    let added_rows = insert_into(document_tag)
        .values((document.eq(document_id), (tag.eq(tag_id))))
        .execute(connection)
        .map_err(|e| format!("Could not link tag {} and document {} to database : {}", tag_id, document_id, e))?;
    match added_rows {
        1 => get_document_tag(connection, document_id, tag_id)
                .ok_or(format!("Could not find newly created link between document {} and tag {} in database", document_id, tag_id)),
        _ => Err(format!("Something wrong happened while linking document {} and tag {} in database", document_id, tag_id))
    }
}

pub fn unlink_tag_to_document(connection : &mut SqliteConnection, document_id: &i32, tag_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(document_tag).filter(document.eq(document_id)).filter(tag.eq(tag_id)).execute(connection)
        .map_err(|e| format!("Could not delete link between document {} and tag {} from database : {}", document_id, tag_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting the link between document {} and tag {} from database", document_id, tag_id))
}
}


pub fn get_document_tag(connection : &mut SqliteConnection, document_id : &i32, genre_id : &i32) -> Option<DocumentTag> {
    match document_tag.filter(document.eq(document_id)).filter(tag.eq(genre_id)).first::<DocumentTag>(connection) {
        Ok(dg) => Some(dg),
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use chrono::NaiveDate;

    use crate::db_manager::{create_database, delete_database, entities::{add_category, add_document, add_tag, get_document_tag, get_tag_by_id, get_tags, link_tag_to_document, remove_tag, unlink_tag_to_document, DocumentTag, Tag}, get_connection};

    #[test]
    fn adding_document_tag_should_give_newly_created_document_tag() {
        let test_db_path = Path::new("./adding_document_tag_should_give_newly_created_document_tag.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let mangas = add_category(&mut connection, &"Mangas".to_string(), &"~/Documents/Mangas".to_string()).unwrap();
        let one_piece_ch_1001 = add_document(&mut connection, &"Chapter 1001".to_string(), &mangas.id_, &None, &None, &NaiveDate::from_ymd_opt(2021, 1, 18).unwrap().to_string(), &"Oda/One piece/chapter-1001.cbz".to_string()).unwrap();
        let chapters = add_tag(&mut connection, &"manga chapters".to_string()).unwrap();
        let maybe_added_document_tag = link_tag_to_document(&mut connection, &one_piece_ch_1001.id_, &chapters.id_).unwrap();
        assert_eq!(DocumentTag::new(one_piece_ch_1001.id_, chapters.id_), maybe_added_document_tag);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn remove_document_tag_should_delete_it() {
        let test_db_path = Path::new("./remove_document_tag_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let mangas = add_category(&mut connection, &"Mangas".to_string(), &"~/Documents/Mangas".to_string()).unwrap();
        let one_piece_ch_1001 = add_document(&mut connection, &"Chapter 1001".to_string(), &mangas.id_, &None, &None, &NaiveDate::from_ymd_opt(2021, 1, 18).unwrap().to_string(), &"Oda/One piece/chapter-1001.cbz".to_string()).unwrap();
        let chapters = add_tag(&mut connection, &"manga chapters".to_string()).unwrap();
        link_tag_to_document(&mut connection, &one_piece_ch_1001.id_, &chapters.id_).unwrap();
        unlink_tag_to_document(&mut connection, &one_piece_ch_1001.id_, &chapters.id_).unwrap();
        assert!(get_document_tag(&mut connection, &one_piece_ch_1001.id_, &chapters.id_).is_none());
        delete_database(test_db_path).unwrap();
    }
}

