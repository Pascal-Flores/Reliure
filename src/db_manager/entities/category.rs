use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::db_manager::entities::schema::category::dsl::*;

#[derive(Queryable, PartialEq, Debug, new, Clone)]
pub struct Category {
    pub id_ : i32,
    pub name_ : String,
    pub path_ : String
}

pub fn add_category(connection : &mut SqliteConnection, category_name : &String, category_path : &String) -> Result<Category, String> {
    let added_rows = insert_into(category)
        .values((name.eq(category_name), path.eq(category_path)))
        .execute(connection)
        .map_err(|e| format!("Could not add category {} to database : {}", category_name, e))?;
    match added_rows {
        1 => get_category_by_name(connection, category_name)
                .ok_or(format!("Could not find newly created author {} in database", category_name)),
        _ => Err(format!("Something wrong happened while adding author {} in database", category_name))
    }
}

pub fn remove_category(connection : &mut SqliteConnection, category_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(category).filter(id.eq(category_id)).execute(connection)
        .map_err(|e| format!("Could not delete author {} from database : {}", category_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting author {} from database", category_id))
    }
}

pub fn get_categories(connection : &mut SqliteConnection) -> Result<Vec<Category>, String> {
    return category.load::<Category>(connection)
    .map_err(|e| format!("An errror occured while getting all categories : {}", e));
}

pub fn get_category_by_name(connection : &mut SqliteConnection, category_name : &String) -> Option<Category> {
    match category.filter(name.eq(category_name)).first::<Category>(connection) {
        Ok(c) => Some(c),
        Err(_) => None
    }
}

pub fn get_category_by_id(connection : &mut SqliteConnection, category_id : &i32) -> Option<Category> {
    match category.filter(id.eq(category_id)).first::<Category>(connection) {
        Ok(c) => Some(c),
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use crate::db_manager::{create_database, entities::{add_category, get_categories, get_category_by_id, remove_category, Category}, get_connection};

    #[test]
    fn adding_category_should_give_newly_created_category() {
        let test_db_path = Path::new("./adding_category_should_give_newly_created_category.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let maybe_added_category = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        assert_eq!(Category::new(1, "Books".to_string(), "~/Documents/Books".to_string()), maybe_added_category);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_categories_should_give_all_categories() {
        let test_db_path = Path::new("./getting_all_categories_should_give_all_categories.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let mangas = add_category(&mut connection, &"Mangas".to_string(), &"~/Documents/Mangas".to_string()).unwrap();
        let comics = add_category(&mut connection, &"Comics".to_string(), &"~/Documents/Comics".to_string()).unwrap();
        let categories = [books, mangas, comics].to_vec();
        let queried_categories = get_categories(&mut connection).unwrap();
        assert_eq!(categories, queried_categories);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_category_by_id_should_give_category() {
        let test_db_path = Path::new("./getting_category_by_id_should_give_category.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        let maybe_books = get_category_by_id(&mut connection, &books.id_).unwrap();
        assert_eq!(books, maybe_books);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_category_should_delete_it() {
        let test_db_path = Path::new("./remove_category_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let books = add_category(&mut connection, &"Books".to_string(), &"~/Documents/Books".to_string()).unwrap();
        remove_category(&mut connection, &books.id_).unwrap();
        assert!(get_category_by_id(&mut connection, &books.id_).is_none());
        remove_file(test_db_path).unwrap();
    }
}