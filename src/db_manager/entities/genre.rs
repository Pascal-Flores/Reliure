use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::db_manager::entities::schema::genre::dsl::*;

#[derive(Queryable, PartialEq, Debug, new, Clone)]
pub struct Genre {
    pub id_ : i32,
    pub name_ : String
}
pub fn add_genre(connection : &mut SqliteConnection, genre_name : &String) -> Result<Genre, String> {
    let added_rows = insert_into(genre)
        .values(name.eq(genre_name))
        .execute(connection)
        .map_err(|e| format!("Could not add genre {} to database : {}", genre_name, e))?;
    match added_rows {
        1 => get_genre_by_name(connection, genre_name)
                .ok_or(format!("Could not find newly created author {} in database", genre_name)),
        _ => Err(format!("Something wrong happened while adding author {} in database", genre_name))
    }
}

pub fn remove_genre(connection : &mut SqliteConnection, genre_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(genre).filter(id.eq(&genre_id)).execute(connection)
        .map_err(|e| format!("Could not delete author {} from database : {}", genre_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting author {} from database", genre_id))
    }
}

pub fn get_genres(connection : &mut SqliteConnection) -> Result<Vec<Genre>, String> {
    return genre.load::<Genre>(connection)
    .map_err(|e| format!("An errror occured while getting all genres : {}", e));
}

pub fn get_genre_by_id(connection : &mut SqliteConnection, genre_id : &i32) -> Option<Genre> {
    match genre.filter(id.eq(genre_id)).first::<Genre>(connection) {
        Ok(g) => Some(g),
        Err(_) => None
    }
}

pub fn get_genre_by_name(connection : &mut SqliteConnection, genre_name : &String) -> Option<Genre> {
    match genre.filter(name.eq(genre_name)).first::<Genre>(connection) {
        Ok(g) => Some(g),
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::db_manager::{create_database, delete_database, entities::{add_author, get_authors, Author}, get_connection};

    use super::{add_genre, get_genres, remove_genre, get_genre_by_id, get_genre_by_name, Genre};

    #[test]
    fn adding_genre_should_give_newly_created_genre() {
        let test_db_path = Path::new("./adding_genre_should_give_newly_created_genre.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let maybe_added_genre = add_genre(&mut connection, &"Sci-Fi".to_string()).unwrap();
        assert_eq!(Genre::new(1, "Sci-Fi".to_string()), maybe_added_genre);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_genres_should_give_all_genres() {
        let test_db_path = Path::new("./getting_all_genres_should_give_all_genres.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let sci_fi = add_genre(&mut connection, &"Sci-Fi".to_string()).unwrap();
        let heroic_fantsay = add_genre(&mut connection, &"Heroic-Fantasy".to_string()).unwrap();
        let thriller = add_genre(&mut connection, &"Thriller".to_string()).unwrap();
        let genres = [sci_fi, heroic_fantsay, thriller].to_vec();
        let queried_genres = get_genres(&mut connection).unwrap();
        assert_eq!(genres, queried_genres);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn getting_genre_by_id_should_give_genre() {
        let test_db_path = Path::new("./getting_genre_by_id_should_give_genre.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let sci_fi = add_genre(&mut connection, &"Sci-Fi".to_string()).unwrap();
        let maybe_sci_fi = get_genre_by_id(&mut connection, &sci_fi.id_).unwrap();
        assert_eq!(sci_fi, maybe_sci_fi);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn remove_genre_should_delete_it() {
        let test_db_path = Path::new("./remove_genre_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let sci_fi = add_genre(&mut connection, &"Sci-Fi".to_string()).unwrap();
        remove_genre(&mut connection, &sci_fi.id_).unwrap();
        assert!(get_genre_by_id(&mut connection, &sci_fi.id_).is_none());
        delete_database(test_db_path).unwrap();
    }
}