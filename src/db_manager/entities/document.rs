use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SqliteConnection, Table};
use crate::db_manager::entities::{document_genre, schema::{self, document::dsl::*}};

#[derive(Queryable, PartialEq, Debug, new, Clone)]
pub struct Document {
    pub id_ : i32,
    pub name_ : String,
    pub category_ : i32,
    pub author_ : Option<i32>,
    pub series_ : Option<i32>,
    pub date_ : String,
    pub path_ : String
}

pub fn add_document(connection : &mut SqliteConnection, 
                    document_name : &String, 
                    document_category : &i32, 
                    document_author : &Option<i32>, 
                    document_series : &Option<i32>,
                    document_date : &String,
                    document_path : &String) -> Result<Document, String> {
    let added_rows = insert_into(document)
        .values((name.eq(document_name), 
                category.eq(document_category), 
                author.eq(document_author), 
                series.eq(document_series), 
                date.eq(document_date), 
                path.eq(document_path)))
        .execute(connection)
        .map_err(|e| format!("Could not add document {} to database : {}", document_name, e))?;
    match added_rows {
        1 => get_document_by_name(connection, document_name)
                .ok_or(format!("Could not find newly created author {} in database", document_name)),
        _ => Err(format!("Something wrong happened while adding author {} in database", document_name))
    }
}

pub fn remove_document(connection : &mut SqliteConnection, document_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(document).filter(id.eq(&document_id)).execute(connection)
        .map_err(|e| format!("Could not delete document {} from database : {}", document_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting document {} from database", document_id))
    }
}

pub fn get_documents(connection : &mut SqliteConnection) -> Result<Vec<Document>, String> {
    return document.load::<Document>(connection)
    .map_err(|e| format!("An errror occured while getting all documents : {}", e));
}

pub fn get_document_by_id(connection : &mut SqliteConnection, document_id : &i32) -> Option<Document> {
    match document.filter(id.eq(document_id)).first::<Document>(connection) {
        Ok(d) => Some(d),
        Err(_) => None
    }
}

pub fn get_document_by_name(connection : &mut SqliteConnection, document_name : &String) -> Option<Document> {
    match document.filter(name.eq(document_name)).first::<Document>(connection) {
        Ok(d) => Some(d),
        Err(_) => None
    }
}


pub fn get_documents_from_author(connection : &mut SqliteConnection, author_id : &i32) -> Result<Vec<Document>, String> {
    return document.filter(author.eq(author_id)).load::<Document>(connection)
        .map_err(|e| format!("An error occured while trying to get all documents from author {} : {}", author_id, e));
}

pub fn get_documents_from_series(connection : &mut SqliteConnection, series_id : &i32) -> Result<Vec<Document>, String> {
    return document.filter(series.eq(series_id)).load::<Document>(connection)
    .map_err(|e| format!("An error occured while trying to get all documents from series {} : {}", series_id, e));
}

pub fn get_documents_from_category(connection : &mut SqliteConnection, category_id : &i32) -> Result<Vec<Document>, String> {
    return document.filter(category.eq(category_id)).load::<Document>(connection)
        .map_err(|e| format!("An error occured while trying to get all documents from category {} : {}", category_id, e));
}

pub fn get_documents_with_genre(connection : &mut SqliteConnection, genre_id : &i32) -> Result<Vec<Document>, String> {
    use crate::db_manager::entities::schema::document_genre::dsl::{document_genre, document as dg_document, genre as dg_genre};
    use crate::db_manager::entities::schema::genre::dsl::{genre, id as g_id};
    return document
        .inner_join(document_genre.on(id.eq(dg_document)))
        .inner_join(genre.on(dg_genre.eq(g_id)))
        .filter(g_id.eq(genre_id))
        .select(document::all_columns())
        .load::<Document>(connection)
        .map_err(|e| format!("An error occured whiletrying to get all documents with genre {} : {}", genre_id, e));

}

pub fn get_documents_with_tag(connection : &mut SqliteConnection, tag_id : &i32) -> Result<Vec<Document>, String> {
    use crate::db_manager::entities::schema::document_tag::dsl::{document_tag, document as dt_document, tag as dt_tag};
    use crate::db_manager::entities::schema::tag::dsl::{tag, id as t_id};
    return document
    .inner_join(document_tag.on(id.eq(dt_document)))
    .inner_join(tag.on(dt_tag.eq(t_id)))
    .filter(t_id.eq(tag_id))
    .select(document::all_columns())
    .load::<Document>(connection)
    .map_err(|e| format!("An error occured whiletrying to get all documents with tag {} : {}", tag_id, e));

}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use chrono::NaiveDate;
    use crate::db_manager::{create_database, delete_database, entities::{add_author, add_category, add_document, add_genre, add_series, add_tag, get_document_by_id, get_documents, get_documents_from_author, get_documents_from_series, get_documents_with_genre, get_documents_with_tag, link_genre_to_document, link_tag_to_document, remove_document, Document}, get_connection};

    #[test]
    fn adding_document_should_give_newly_created_document() {
        let test_db_path = Path::new("./adding_document_should_give_newly_created_document.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let jrr_tolkien = add_author(&mut connection, &"J.R.R Tolkien".to_string()).unwrap();
        let the_lord_of_the_rings = add_series(&mut connection, &"The Lord Of The Rings".to_string(), &jrr_tolkien.id_).unwrap();
        let maybe_the_fellowship_of_the_ring = add_document(&mut connection, &"The fellowship of the ring".to_string(), &books.id_, &Some(jrr_tolkien.id_), &Some(the_lord_of_the_rings.id_), &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        assert_eq!(Document::new(1, "The fellowship of the ring".to_string(), books.id_, Some(jrr_tolkien.id_), Some(the_lord_of_the_rings.id_), NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), "Tolkien/the_fellowship_of_the_ring.epub".to_string()), maybe_the_fellowship_of_the_ring);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_documents_should_give_all_documents() {
        let test_db_path = Path::new("./getting_all_documents_should_give_all_documents.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(&mut connection, &"The Two Towers".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 11, 11).unwrap().to_string(), &"Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(&mut connection, &"The Return of The King".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1955, 10, 20).unwrap().to_string(), &"Tolkien/the_return_of_the_king".to_string()).unwrap();
        let documents = [the_fellowship_of_the_ring, the_two_towers, the_return_of_the_king].to_vec();
        let queried_documents = get_documents(&mut connection).unwrap();
        assert_eq!(documents, queried_documents);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_document_by_id_should_give_document() {
        let test_db_path = Path::new("./getting_document_by_id_should_give_document.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let maybe_the_fellowship_of_the_ring = get_document_by_id(&mut connection, &the_fellowship_of_the_ring.id_).unwrap();
        assert_eq!(the_fellowship_of_the_ring, maybe_the_fellowship_of_the_ring);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn remove_document_should_delete_it() {
        let test_db_path = Path::new("./remove_document_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        remove_document(&mut connection, &the_fellowship_of_the_ring.id_).unwrap();
        assert!(get_document_by_id(&mut connection, &the_fellowship_of_the_ring.id_).is_none());
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_documents_from_author_should_only_return_documents_from_author() {
        let test_db_path = Path::new("./getting_documents_from_author_should_only_return_documents_from_author.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let jrr_tolkien = add_author(&mut connection, &"J.R.R Tolkien".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &Some(jrr_tolkien.id_), &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(&mut connection, &"The Two Towers".to_string(), &books.id_, &Some(jrr_tolkien.id_), &None, &NaiveDate::from_ymd_opt(1954, 11, 11).unwrap().to_string(), &"Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(&mut connection, &"The Return of The King".to_string(), &books.id_, &Some(jrr_tolkien.id_), &None, &NaiveDate::from_ymd_opt(1955, 10, 20).unwrap().to_string(), &"Tolkien/the_return_of_the_king".to_string()).unwrap();
        let from_tolkien = [the_fellowship_of_the_ring, the_two_towers, the_return_of_the_king].to_vec();
        let rr_martin = add_author(&mut connection, &"R.R Martin".to_string()).unwrap();
        let a_game_of_throne = add_document(&mut connection, &"A Game Of Thrones".to_string(), &books.id_, &Some(rr_martin.id_), &None, &NaiveDate::from_ymd_opt(1996, 8, 6).unwrap().to_string(), &"Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let queried_documents = get_documents_from_author(&mut connection, &jrr_tolkien.id_).unwrap();
        assert_eq!(from_tolkien, queried_documents);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_documents_from_series_should_only_return_documents_from_series() {
        let test_db_path = Path::new("./getting_documents_from_series_should_only_return_documents_from_series.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let jrr_tolkien = add_author(&mut connection, &"J.R.R Tolkien".to_string()).unwrap();
        let the_lord_of_the_ring = add_series(&mut connection, &"The Lord Of The Rings".to_string(), &jrr_tolkien.id_).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &Some(jrr_tolkien.id_), &Some(the_lord_of_the_ring.id_), &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(&mut connection, &"The Two Towers".to_string(), &books.id_, &Some(jrr_tolkien.id_), &Some(the_lord_of_the_ring.id_), &NaiveDate::from_ymd_opt(1954, 11, 11).unwrap().to_string(), &"Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(&mut connection, &"The Return of The King".to_string(), &books.id_, &Some(jrr_tolkien.id_), &Some(the_lord_of_the_ring.id_), &NaiveDate::from_ymd_opt(1955, 10, 20).unwrap().to_string(), &"Tolkien/the_return_of_the_king".to_string()).unwrap();
        let from_the_lord_of_the_ring = [the_fellowship_of_the_ring, the_two_towers, the_return_of_the_king].to_vec();
        let rr_martin = add_author(&mut connection, &"R.R Martin".to_string()).unwrap();
        let a_song_of_ice_and_fire = add_series(&mut connection, &"A Song Of Ice And Fire".to_string(), &rr_martin.id_).unwrap();
        let a_game_of_throne = add_document(&mut connection, &"A Game Of Thrones".to_string(), &books.id_, &Some(rr_martin.id_), &Some(a_song_of_ice_and_fire.id_), &NaiveDate::from_ymd_opt(1996, 8, 6).unwrap().to_string(), &"Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let queried_documents = get_documents_from_series(&mut connection, &the_lord_of_the_ring.id_).unwrap();
        assert_eq!(from_the_lord_of_the_ring, queried_documents);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_tagged_documents_should_only_return_documents_with_tag() {
        let test_db_path = Path::new("./getting_tagged_documents_should_only_return_documents_with_tag.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(&mut connection, &"The Two Towers".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 11, 11).unwrap().to_string(), &"Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(&mut connection, &"The Return of The King".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1955, 10, 20).unwrap().to_string(), &"Tolkien/the_return_of_the_king".to_string()).unwrap();
        let bangers = [the_fellowship_of_the_ring.clone(), the_two_towers.clone(), the_return_of_the_king.clone()].to_vec();
        let a_game_of_throne = add_document(&mut connection, &"A Game Of Thrones".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1996, 8, 6).unwrap().to_string(), &"Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let banger = add_tag(&mut connection, &"banger".to_string()).unwrap();
        link_tag_to_document(&mut connection, &the_fellowship_of_the_ring.id_, &banger.id_).unwrap();
        link_tag_to_document(&mut connection, &the_two_towers.id_, &banger.id_).unwrap();
        link_tag_to_document(&mut connection, &the_return_of_the_king.id_, &banger.id_).unwrap();
        let queried_documents = get_documents_with_tag(&mut connection, &banger.id_).unwrap();
        assert_eq!(bangers, queried_documents);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_documents_with_genre_should_only_return_documents_with_genre() {
        let test_db_path = Path::new("./getting_documents_with_genre_should_only_return_documents_with_genre.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let the_fellowship_of_the_ring = add_document(&mut connection, &"The Fellowship Of The Ring".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 6, 29).unwrap().to_string(), &"Tolkien/the_fellowship_of_the_ring.epub".to_string()).unwrap();
        let the_two_towers = add_document(&mut connection, &"The Two Towers".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1954, 11, 11).unwrap().to_string(), &"Tolkien/the_two_towers.epub".to_string()).unwrap();
        let the_return_of_the_king = add_document(&mut connection, &"The Return of The King".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1955, 10, 20).unwrap().to_string(), &"Tolkien/the_return_of_the_king".to_string()).unwrap();
        let hf = [the_fellowship_of_the_ring.clone(), the_two_towers.clone(), the_return_of_the_king.clone()].to_vec();
        let a_game_of_throne = add_document(&mut connection, &"A Game Of Thrones".to_string(), &books.id_, &None, &None, &NaiveDate::from_ymd_opt(1996, 8, 6).unwrap().to_string(), &"Martin/A_game_of_thrones.epub".to_string()).unwrap();
        let heroic_fantasy = add_genre(&mut connection, &"Heroic Fantasy".to_string()).unwrap();
        link_genre_to_document(&mut connection, &the_fellowship_of_the_ring.id_, &heroic_fantasy.id_).unwrap();
        link_genre_to_document(&mut connection, &the_two_towers.id_, &heroic_fantasy.id_).unwrap();
        link_genre_to_document(&mut connection, &the_return_of_the_king.id_, &heroic_fantasy.id_).unwrap();
        let queried_documents = get_documents_with_genre(&mut connection, &heroic_fantasy.id_).unwrap();
        assert_eq!(hf, queried_documents);
        delete_database(test_db_path).unwrap();
    }
}