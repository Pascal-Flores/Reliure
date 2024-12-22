// use derive_new::new;
// use rusqlite::params;
// use std::path::Path;

// use crate::db_manager::get_connection;

use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, ExpressionMethods, QueryResult, RunQueryDsl, Selectable, SqliteConnection};
use crate::db_manager::entities::schema::author::dsl::*;
// #[derive(new, PartialEq, Debug, Clone)]
#[derive(Queryable, PartialEq, Debug, new, Clone)]
pub struct Author {
    pub id_ : i32,
    pub name_ : String
}

pub fn add_author(connection : &mut SqliteConnection, author_name : String) -> Result<Author, String> {
    let added_rows = insert_into(author)
        .values(name.eq(&author_name))
        .execute(connection)
        .map_err(|e| format!("Could not add author {} to database : {}", &author_name, e))?;
    match added_rows {
        1 => get_author_by_name(connection, &author_name)
                .ok_or(format!("Could not find newly created author {} in database", &author_name)),
        _ => Err(format!("Something wrong happened while adding author {} in database", &author_name))
    }
}

pub fn remove_author(connection : &mut SqliteConnection, author_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(author).filter(id.eq(&author_id)).execute(connection)
        .map_err(|e| format!("Could not delete author {} from database : {}", &author_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting author {} from database", &author_id))
    }
}

pub fn get_authors(connection : &mut SqliteConnection) -> Result<Vec<Author>, String> {
    return author.load::<Author>(connection)
    .map_err(|e| format!("An errror occured while getting all authors : {}", e));
}

pub fn get_author_by_id(connection : &mut SqliteConnection, author_id : &i32) -> Option<Author> {
    match author.filter(id.eq(author_id)).first::<Author>(connection) {
        Ok(a) => Some(a),
        Err(_) => None
    }
}

pub fn get_author_by_name(connection : &mut SqliteConnection, author_name : &String) -> Option<Author> {
    match author.filter(name.eq(author_name)).first::<Author>(connection) {
        Ok(a) => Some(a),
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::db_manager::{create_database, delete_database, entities::{add_author, get_author_by_id, get_authors, remove_author, Author}, get_connection};

    #[test]
    fn adding_author_should_give_newly_created_author() {
        let test_db_path = Path::new("./adding_author_should_give_newly_created_author.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let maybe_added_author = add_author(&mut connection, "George R.R Martin".to_string()).unwrap();
        assert_eq!(Author::new(1, "George R.R Martin".to_string()), maybe_added_author);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_authors_should_give_all_authors() {
        let test_db_path = Path::new("./getting_all_authors_should_give_all_authors.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let rr_martin = add_author(&mut connection, "George R.R Martin".to_string()).unwrap();
        let jk_rowling = add_author(&mut connection, "J.K Rowling".to_string()).unwrap();
        let jrr_tolkien = add_author(&mut connection, "J.R.R Tolkien".to_string()).unwrap();
        let authors = [rr_martin, jk_rowling, jrr_tolkien].to_vec();
        let queried_authors = get_authors(&mut connection).unwrap();
        assert_eq!(authors, queried_authors);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_author_by_id_should_give_author() {
        let test_db_path = Path::new("./getting_author_by_id_should_give_author.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let rr_martin = add_author(&mut connection, "George R.R Martin".to_string()).unwrap();
        let maybe_rr_martin = get_author_by_id(&mut connection, &rr_martin.id_).unwrap();
        assert_eq!(rr_martin, maybe_rr_martin);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn remove_author_should_delete_it() {
        let test_db_path = Path::new("./remove_author_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let rr_martin = add_author(&mut connection, "George R.R Martin".to_string()).unwrap();
        remove_author(&mut connection, &rr_martin.id_).unwrap();
        assert!(get_author_by_id(&mut connection, &rr_martin.id_).is_none());
        delete_database(test_db_path).unwrap();
    }
}