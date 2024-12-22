use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::db_manager::entities::schema::tag::dsl::*;

#[derive(Queryable, PartialEq, Debug, new, Clone)]
pub struct Tag {
    pub id_ : i32,
    pub name_ : String
}

pub fn add_tag(connection : &mut SqliteConnection, tag_name : &String) -> Result<Tag, String> {
    let added_rows = insert_into(tag)
        .values(name.eq(tag_name))
        .execute(connection)
        .map_err(|e| format!("Could not add tag {} to database : {}", tag_name, e))?;
    match added_rows {
        1 => get_tag_by_name(connection, tag_name)
                .ok_or(format!("Could not find newly created tag {} in database", tag_name)),
        _ => Err(format!("Something wrong happened while adding tag {} in database", tag_name))
    }
}

pub fn remove_tag(connection : &mut SqliteConnection, tag_id : &i32) -> Result<(), String> {
    let deleted_rows = delete(tag).filter(id.eq(&tag_id)).execute(connection)
        .map_err(|e| format!("Could not delete tag {} from database : {}", tag_id, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting tag {} from database", tag_id))
    }
}

pub fn get_tags(connection : &mut SqliteConnection) -> Result<Vec<Tag>, String> {
    return tag.load::<Tag>(connection)
    .map_err(|e| format!("An errror occured while getting all tags : {}", e));
}

pub fn get_tag_by_id(connection : &mut SqliteConnection, tag_id : &i32) -> Option<Tag> {
    match tag.filter(id.eq(tag_id)).first::<Tag>(connection) {
        Ok(t) => Some(t),
        Err(_) => None
    }
}

pub fn get_tag_by_name(connection : &mut SqliteConnection, tag_name : &String) -> Option<Tag> {
    match tag.filter(name.eq(tag_name)).first::<Tag>(connection) {
        Ok(t) => Some(t),
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::db_manager::{create_database, delete_database, entities::{add_tag, get_tag_by_id, get_tags, remove_tag, Tag}, get_connection};

    #[test]
    fn adding_tag_should_give_newly_created_tag() {
        let test_db_path = Path::new("./adding_tag_should_give_newly_created_tag.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let maybe_added_tag = add_tag(&mut connection, &"favorites".to_string()).unwrap();
        assert_eq!(Tag::new(1, "favorites".to_string()), maybe_added_tag);
        delete_database(test_db_path).unwrap();

    }

    #[test]
    fn getting_all_tags_should_give_all_tags() {
        let test_db_path = Path::new("./getting_all_tags_should_give_all_tags.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let favorites = add_tag(&mut connection, &"favorites".to_string()).unwrap();
        let didnt_like = add_tag(&mut connection, &"didn't like".to_string()).unwrap();
        let manga_chapter = add_tag(&mut connection, &"chapter".to_string()).unwrap();
        let tags = [favorites, didnt_like, manga_chapter].to_vec();
        let queried_tags = get_tags(&mut connection).unwrap();
        assert_eq!(tags, queried_tags);
        delete_database(test_db_path).unwrap();

    }

    #[test]
    fn getting_tag_by_id_should_give_tag() {
        let test_db_path = Path::new("./getting_tag_by_id_should_give_tag.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let favorites = add_tag(&mut connection, &"favorites".to_string()).unwrap();
        let maybe_favorites = get_tag_by_id(&mut connection, &favorites.id_).unwrap();
        assert_eq!(favorites, maybe_favorites);
        delete_database(test_db_path).unwrap();
    }

    #[test]
    fn remove_tag_should_delete_it() {
        let test_db_path = Path::new("./remove_tag_should_delete_it.db");
        create_database(test_db_path).unwrap();
        let mut connection = get_connection(test_db_path).unwrap();
        let favorites = add_tag(&mut connection, &"favorites".to_string()).unwrap();
        remove_tag(&mut connection, &favorites.id_).unwrap();
        assert!(get_tag_by_id(&mut connection, &favorites.id_).is_none());
        delete_database(test_db_path).unwrap();
    }
}