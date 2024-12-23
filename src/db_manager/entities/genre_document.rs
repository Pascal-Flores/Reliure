use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, Connection, ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::db_manager::entities::schema::document_genre::dsl::*;

#[derive(Queryable, PartialEq, Debug, new, Clone)]
pub struct DocumentGenre {
    pub document_ : i32,
    pub genre_ : i32
}

pub fn link_genre_to_document(connection : &mut SqliteConnection, document_id: &i32, genre_id : &i32) -> Result<DocumentGenre, String> {
    let added_rows = insert_into(document_genre)
        .values((document.eq(document_id), (genre.eq(genre_id))))
        .execute(connection)
        .map_err(|e| format!("Could not link genre {} and document {} to database : {}", genre_id, document_id, e))?;
    match added_rows {
        1 => get_document_genre(connection, document_id, genre_id)
                .ok_or(format!("Could not find newly created link between document {} and genre {} in database", document_id, genre_id)),
        _ => Err(format!("Something wrong happened while linking document {} and genre {} in database", document_id, genre_id))
    }
}

pub fn unlink_genre_to_document(connection : &mut SqliteConnection, document_id: &i32, genre_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(document_genre).filter(document.eq(document_id)).filter(genre.eq(genre_id)).execute(connection)
        .map_err(|e| format!("Could not delete link between document {} and genre {} from database : {}", document_id, genre_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting the link between document {} and genre {} from database", document_id, genre_id))
}
}


pub fn get_document_genre(connection : &mut SqliteConnection, document_id : &i32, genre_id : &i32) -> Option<DocumentGenre> {
    match document_genre.filter(document.eq(document_id)).filter(genre.eq(genre_id)).first::<DocumentGenre>(connection) {
        Ok(dg) => Some(dg),
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use chrono::NaiveDate;

    use crate::db_manager::{create_database, delete_database, entities::{add_category, add_document, add_genre, get_document_genre, link_genre_to_document, unlink_genre_to_document, DocumentGenre}, get_connection};

    #[test]
    fn adding_genre_document_should_give_newly_created_document_tag() {
        let test_db_path = Path::new("./adding_genre_document_should_give_newly_created_document_tag.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let heroic_fantasy = add_genre(&mut connection, &"Heroic Fantasy".to_string()).unwrap();
        let maybe_added_genre_document = link_genre_to_document(&mut connection, &the_fellowship_of_the_ring.id_,&heroic_fantasy.id_).unwrap();
        assert_eq!(DocumentGenre::new(the_fellowship_of_the_ring.id_, heroic_fantasy.id_), maybe_added_genre_document);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn remove_genre_document_should_delete_it() {
        let test_db_path = Path::new("./remove_genre_document_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let heroic_fantasy = add_genre(&mut connection, &"Heroic Fantasy".to_string()).unwrap();
        link_genre_to_document(&mut connection, &the_fellowship_of_the_ring.id_,&heroic_fantasy.id_).unwrap();
        unlink_genre_to_document(&mut connection, &the_fellowship_of_the_ring.id_, &heroic_fantasy.id_).unwrap();
        assert!(get_document_genre(&mut connection, &the_fellowship_of_the_ring.id_, &heroic_fantasy.id_).is_none());
        delete_database(test_db_path).unwrap();
    }


}