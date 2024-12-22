use derive_new::new;
use rusqlite::params;
use std::path::Path;

use crate::db_manager::get_connection;

#[derive(new, PartialEq, Debug, Clone)]
pub struct Tag {
    pub id : i32,
    pub name : String
}

pub fn add_tag(db_path: &Path, name: String) -> Result<Tag, String> {
    let connection = get_connection(db_path)?;
    connection.execute("INSERT INTO tag (name) VALUES (?1)", params![name])
        .map_err(|e| format!("Could not add tag {} to database : {}", name, e))?;
    match get_tag_by_name(db_path, name.clone()) {
        Some(tag) => Ok(tag),
        None => Err(format!("Newly created tag {} could not be found in database", name))
    }
}

pub fn remove_tag(db_path : &Path, id : i32) -> Result<(), String> {
    let connection = get_connection(db_path)?;
    connection.execute("DELETE FROM tag WHERE id=?1", params![id])
        .map_err(|e| format!("Could not delete tag {} from database : {}", id, e))?;
    return Ok(());
}

pub fn get_tags(db_path : &Path) -> Result<Vec<Tag>, String> {
    let connection = get_connection(db_path)?;
    let mut stmt = connection.prepare("SELECT id, name FROM tag")
        .map_err(|e| format!("An error occurred while getting all tags: {}", e))?;
    let tags_result = stmt.query_map([], |row| 
        Ok(Tag::new(row.get(0)?, row.get(1)?))
        ).map_err(|e| format!("An error occurred while getting all tags: {}", e))?;
    let tags: Result<Vec<Tag>, rusqlite::Error> = tags_result.collect();
    tags.map_err(|e| format!("Failed to get tags: {}", e))
}

pub fn get_tag_by_id(db_path : &Path, id: i32) -> Option<Tag> {
    let connection = get_connection(db_path).ok()?;
    let tag = connection.query_row(
        "SELECT id, name FROM tag WHERE id = ?1",
        params![id],
        |row| Ok(Tag::new(row.get(0)?, row.get(1)?))
    ).ok()?;
    return Some(tag);
}

pub fn get_tag_by_name(db_path : &Path, name: String) -> Option<Tag> {
    let connection = get_connection(db_path).ok()?;
    let tag = connection.query_row(
        "SELECT id, name FROM tag WHERE name = ?1",
        params![name],
        |row| Ok(Tag::new(row.get(0)?, row.get(1)?))
    ).ok()?;
    return Some(tag);
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    use crate::db_manager::{create_database, entities::{add_tag, get_tag_by_id, get_tags, remove_tag, Tag}};

    #[test]
    fn adding_tag_should_give_newly_created_tag() {
        let test_db_path = Path::new("./adding_tag_should_give_newly_created_tag.db");
        create_database(test_db_path).unwrap();
        let maybe_added_tag = add_tag(test_db_path, "favorites".to_string()).unwrap();
        assert_eq!(Tag::new(1, "favorites".to_string()), maybe_added_tag);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_all_tags_should_give_all_tags() {
        let test_db_path = Path::new("./getting_all_tags_should_give_all_tags.db");
        create_database(test_db_path).unwrap();
        let favorites = add_tag(test_db_path, "favorites".to_string()).unwrap();
        let didnt_like = add_tag(test_db_path, "didn't like".to_string()).unwrap();
        let manga_chapter = add_tag(test_db_path, "chapter".to_string()).unwrap();
        let tags = [favorites, didnt_like, manga_chapter].to_vec();
        let queried_tags = get_tags(test_db_path).unwrap();
        assert_eq!(tags, queried_tags);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn getting_tag_by_id_should_give_tag() {
        let test_db_path = Path::new("./getting_tag_by_id_should_give_tag.db");
        create_database(test_db_path).unwrap();
        let favorites = add_tag(test_db_path, "favorites".to_string()).unwrap();
        let maybe_favorites = get_tag_by_id(test_db_path, favorites.id).unwrap();
        assert_eq!(favorites, maybe_favorites);
        remove_file(test_db_path).unwrap();
    }

    #[test]
    fn remove_tag_should_delete_it() {
        let test_db_path = Path::new("./remove_tag_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let favorites = add_tag(test_db_path, "favorites".to_string()).unwrap();
        remove_tag(test_db_path, favorites.id).unwrap();
        assert!(get_tag_by_id(test_db_path, favorites.id).is_none());
        remove_file(test_db_path).unwrap();
    }
}