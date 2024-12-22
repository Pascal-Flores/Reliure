use derive_new::new;
use rusqlite::params;
use std::path::Path;

use crate::db_manager::get_connection;

#[derive(new, PartialEq, Debug, Clone)]
pub struct Category {
    pub id : i32,
    pub name : String,
    pub path : String
}

pub fn add_category(db_path: &Path, name: String, path : String) -> Result<Category, String> {
    let connection = get_connection(db_path)?;
    connection.execute("INSERT INTO category (name, path) VALUES (?1, ?2)", params![name, path])
        .map_err(|e| format!("Could not add category {} to database : {}", name, e))?;
    match get_category_by_name(db_path, name.clone()) {
        Some(category) => Ok(category),
        None => Err(format!("Newly created category {} could not be found in database", name))
    }
}

pub fn remove_category(db_path : &Path, id : i32) -> Result<(), String> {
    let connection = get_connection(db_path)?;
    connection.execute("DELETE FROM category WHERE id=?1", params![id])
        .map_err(|e| format!("Could not delete category {} from database : {}", id, e))?;
    return Ok(());
}

pub fn get_categories(db_path : &Path) -> Result<Vec<Category>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare("SELECT id, name, path FROM category")
        .map_err(|e| format!("An error occurred while getting all tags: {}", e))?;
    let tags_result = stmt.query_map([], |row| 
        Ok(Category::new(row.get(0)?, row.get(1)?, row.get(2)?))
        ).map_err(|e| format!("An error occurred while getting all tags: {}", e))?;
    let tags: Result<Vec<Category>, rusqlite::Error> = tags_result.collect();
    tags.map_err(|e| format!("Failed to get categories: {}", e))
}

pub fn get_category_by_id(db_path : &Path, id: i32) -> Option<Category> {
    let connection = get_connection(db_path).ok()?;
    let category = connection.query_row(
        "SELECT id, name, path FROM category WHERE id = ?1",
        params![id],
        |row| Ok(Category::new(row.get(0)?, row.get(1)?, row.get(2)?))
    ).ok()?;
    return Some(category);
}

pub fn get_category_by_name(db_path : &Path, name: String) -> Option<Category> {
    let connection = get_connection(db_path).ok()?;
    let category = connection.query_row(
        "SELECT id, name, path FROM category WHERE name = ?1",
        params![name],
        |row| Ok(Category::new(row.get(0)?, row.get(1)?, row.get(2)?))
    ).ok()?;
    return Some(category);
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use crate::db_manager::{create_database, entities::{add_category, add_tag, get_categories, get_category_by_id, get_tag_by_id, get_tags, remove_tag, Category, Tag}};

    #[test]
    fn adding_category_should_give_newly_created_category() {
        let test_db_path = Path::new("./adding_category_should_give_newly_created_category.db");
        create_database(test_db_path).unwrap();
        let maybe_added_category = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        assert_eq!(Category::new(1, "Books".to_string(), "~/Documents/Books".to_string()), maybe_added_category);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_categories_should_give_all_categories() {
        let test_db_path = Path::new("./getting_all_categories_should_give_all_categories.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let mangas = add_category(test_db_path, "Mangas".to_string(), "~/Documents/Mangas".to_string()).unwrap();
        let comics = add_category(test_db_path, "Comics".to_string(), "~/Documents/Comics".to_string()).unwrap();
        let categories = [books, mangas, comics].to_vec();
        let queried_categories = get_categories(test_db_path).unwrap();
        assert_eq!(categories, queried_categories);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_category_by_id_should_give_category() {
        let test_db_path = Path::new("./getting_category_by_id_should_give_category.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        let maybe_books = get_category_by_id(test_db_path, books.id).unwrap();
        assert_eq!(books, maybe_books);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_category_should_delete_it() {
        let test_db_path = Path::new("./remove_category_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let books = add_category(test_db_path, "Books".to_string(), "~/Documents/Books".to_string()).unwrap();
        remove_tag(test_db_path, books.id).unwrap();
        assert!(get_tag_by_id(test_db_path, books.id).is_none());
        remove_file(test_db_path).unwrap();
    }
}