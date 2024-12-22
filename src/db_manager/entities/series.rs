use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::db_manager::entities::schema::series::dsl::*;

#[derive(Queryable, PartialEq, Debug, new, Clone)]

pub struct Series {
    pub id_ : i32,
    pub author_ : i32,
    pub name_ : String
}

pub fn add_series(connection : &mut SqliteConnection, series_name : &String, series_author : &i32) -> Result<Series, String> {
    let added_rows = insert_into(series)
        .values((name.eq(series_name), author.eq(series_author)))
        .execute(connection)
        .map_err(|e| format!("Could not add series {} to database : {}", series_name, e))?;
    match added_rows {
        1 => get_series_by_name(connection, series_name)
                .ok_or(format!("Could not find newly created series {} in database", series_name)),
        _ => Err(format!("Something wrong happened while adding series {} in database", series_name))
    }
}

pub fn remove_series(connection : &mut SqliteConnection, series_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(series).filter(id.eq(&series_id)).execute(connection)
        .map_err(|e| format!("Could not delete series {} from database : {}", series_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting series {} from database", series_id))
    }
}

pub fn get_series(connection : &mut SqliteConnection) -> Result<Vec<Series>, String> {
    return series.load::<Series>(connection)
    .map_err(|e| format!("An errror occured while getting all series : {}", e));
}

pub fn get_series_by_id(connection : &mut SqliteConnection, series_id : &i32) -> Option<Series> {
    match series.filter(id.eq(series_id)).first::<Series>(connection) {
        Ok(s) => Some(s),
        Err(_) => None
    }
}

pub fn get_series_by_name(connection : &mut SqliteConnection, series_name : &String) -> Option<Series> {
    match series.filter(name.eq(series_name)).first::<Series>(connection) {
        Ok(s) => Some(s),
        Err(_) => None
    }
}

pub fn get_series_from_author(connection : &mut SqliteConnection, author_id : &i32) -> Option<Vec<Series>> {
    match series.filter(author.eq(author_id)).load::<Series>(connection) {
        Ok(s) => match s.len() {
            0 => None,
            _ => Some(s)
        },
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use crate::db_manager::{create_database, delete_database, entities::{add_author, get_series_from_author, series}, get_connection};

    use super::{Series, add_series, remove_series, get_series, get_series_by_id, get_series_by_name};

    #[test]
    fn adding_series_should_give_newly_created_series() {
        let test_db_path = Path::new("./adding_series_should_give_newly_created_series.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let jk_rowling = add_author(&mut connection, &"J.K Rowling".to_string()).unwrap();
        let maybe_added_series = add_series(&mut connection, &"Harry Potter".to_string(), &jk_rowling.id_).unwrap();
        assert_eq!(Series::new(1, jk_rowling.id_,"Harry Potter".to_string()), maybe_added_series);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_series_should_give_all_series() {
        let test_db_path = Path::new("./getting_all_series_should_give_all_series.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let rr_martin = add_author(&mut connection, &"George R.R Martin".to_string()).unwrap();
        let jk_rowling = add_author(&mut connection, &"J.K Rowling".to_string()).unwrap();
        let jrr_tolkien = add_author(&mut connection, &"J.R.R Tolkien".to_string()).unwrap();
        let harry_potter = add_series(&mut connection, &"Harry Potter".to_string(), &rr_martin.id_).unwrap();
        let a_song_of_ice_and_fire = add_series(&mut connection, &"A song of ice and fire".to_string(), &jk_rowling.id_).unwrap();
        let the_lord_of_the_ring = add_series(&mut connection, &"The lord of the rings".to_string(), &jrr_tolkien.id_).unwrap();
        let series = [harry_potter, a_song_of_ice_and_fire, the_lord_of_the_ring].to_vec();
        let queried_series = get_series(&mut connection).unwrap();
        assert_eq!(series, queried_series);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_series_by_id_should_give_series() {
        let test_db_path = Path::new("./getting_series_by_id_should_give_series.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let jk_rowling = add_author(&mut connection, &"J.K Rowling".to_string()).unwrap();
        let harry_potter = add_series(&mut connection, &"Harry Potter".to_string(), &jk_rowling.id_).unwrap();
        let maybe_harry_potter = get_series_by_id(&mut connection, &harry_potter.id_).unwrap();
        assert_eq!(harry_potter, maybe_harry_potter);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn remove_series_should_delete_it() {
        let test_db_path = Path::new("./remove_series_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let jk_rowling = add_author(&mut connection, &"J.K Rowling".to_string()).unwrap();
        let harry_potter = add_series(&mut connection, &"Harry Potter".to_string(), &jk_rowling.id_).unwrap();
        remove_series(&mut connection, &harry_potter.id_).unwrap();
        assert!(get_series_by_id(&mut connection, &harry_potter.id_).is_none());
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn get_series_from_author_should_only_give_series_from_author() {
        let test_db_path = Path::new("./get_series_from_author_should_only_give_series_from_author.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let rr_martin = add_author(&mut connection, &"George R.R Martin".to_string()).unwrap();
        let jk_rowling = add_author(&mut connection, &"J.K Rowling".to_string()).unwrap();
        let jrr_tolkien = add_author(&mut connection, &"J.R.R Tolkien".to_string()).unwrap();
        let harry_potter = add_series(&mut connection, &"Harry Potter".to_string(), &rr_martin.id_).unwrap();
        let a_song_of_ice_and_fire = add_series(&mut connection, &"A song of ice and fire".to_string(), &jk_rowling.id_).unwrap();
        let the_lord_of_the_ring = add_series(&mut connection, &"The lord of the rings".to_string(), &jrr_tolkien.id_).unwrap();
        let the_hobbit = add_series(&mut connection, &"The Hobbit".to_string(), &jrr_tolkien.id_).unwrap();
        let from_tolkien = vec![the_lord_of_the_ring, the_hobbit];
        assert_eq!(from_tolkien, get_series_from_author(&mut connection, &jrr_tolkien.id_).unwrap());
        delete_database(test_db_path).unwrap();
    }
}