use std::env;
use std::path::Path;
use crate::db_manager::entities::user::{User, get_users, add_user, get_user_by_username};
use crate::db_manager::get_connection;

pub fn get_all_users() -> Result<Vec<User>, String> {
    let db_path_str = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data.db".to_string());
    let db_path = Path::new(&db_path_str);

    let connection = get_connection(db_path);

    if connection.is_err() {
        return Err(connection.err().unwrap());
    }

    get_users(&mut connection?)
}

pub fn get_by_username(username: String) -> Result<Vec<User>, String> {
    let db_path_str = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data.db".to_string());
    let db_path = Path::new(&db_path_str);

    let connection = get_connection(db_path);

    if connection.is_err() {
        return Err(connection.err().unwrap());
    }

    // let users = get_users(&mut connection?);
    //
    // if users.is_ok() {
    //     for user in users? {
    //         if user.username_ == username {
    //             return Ok(user);
    //         }
    //     }
    // }

    let user = get_user_by_username(&mut connection?, &username);

    if user.is_ok() {
        return Ok(user?);
    }

    Err("User not found".to_string())
}

pub fn create_user(new_user: User) -> Result<User, String> {
    let db_path_str = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data.db".to_string());
    let db_path = Path::new(&db_path_str);

    let connection = get_connection(db_path);

    if connection.is_err() {
        return Err(connection.err().unwrap());
    }

    let user = add_user(&mut connection?, new_user);

    if user.is_ok() {
        return Ok(user?);
    }

    Err("Failed to create user".to_string())
}