use std::env;
use std::path::Path;
use crate::db_manager::entities::user::{User, get_users, add_users};
use crate::db_manager::get_connection;

pub fn get_all_users() -> Result<Vec<User>, String> {
    let db_path_str = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data.db".to_string());
    let db_path = Path::new(&db_path_str);

    let where_clause = "".to_string();
    let fields = vec!["id".to_string(), "username".to_string(), "email".to_string(), "password".to_string()];

    get_users(db_path, where_clause, &fields)
}

pub fn get_user_by_username(username: String) -> Result<User, String> {
    let db_path_str = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data.db".to_string());
    let db_path = Path::new(&db_path_str);

    let fields = vec!["id".to_string(), "username".to_string(), "email".to_string(), "password".to_string()];

    let where_clause = " WHERE users.username = '".to_string() + &username + "'";

    let users = get_users(db_path, where_clause, &fields)?;

    if users.len() == 0 {
        return Err("No user found".to_string());
    }

    Ok(users[0].clone())
}

pub fn create_user(new_user: User) -> Result<User, String> {
    let db_path_str = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data.db".to_string());
    let db_path = Path::new(&db_path_str);

    let name = new_user.username.clone();

    add_users(db_path, name, new_user)
}