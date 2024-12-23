use std::env;
use derive_new::new;
use rusqlite::params;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::db_manager::get_connection;

#[derive(new, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id : i32,
    pub username : String,
    pub email : String,
    pub password : String
}

pub fn get_users(db_path: &Path, where_clause : String, fields : &[String]) -> Result<Vec<User>, String> {
    let connection = get_connection(db_path)?;

    if (fields.len() == 0) {
        return Err("No fields specified".to_string());
    }

    let mut query = "SELECT ".to_string();
    for field in fields {
        query.push_str(field);
        query.push_str(",");
    }
    query.pop();
    query += &*(" FROM users".to_owned() + &*where_clause);
    // println!("{}", query);

    let mut statement = connection.prepare(&*query)
        .map_err(|e| format!("Could not prepare statement to get users : {}", e))?;

    let users = statement.query_map(params![], |row| {
        Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    }).map_err(|e| format!("Could not get users from database : {}", e))?;

    let mut result = Vec::new();
    for user in users {
        result.push(user.map_err(|e| format!("Could not get user from database : {}", e))?)
    }
    Ok(result)
}

pub fn add_users(db_path: &Path, name: String, user : User) -> Result<User, String> {
    let connection = get_connection(db_path)?;

    let mut statement = connection.prepare("INSERT INTO users (username, email, password) VALUES (?1, ?2, ?3)")
        .map_err(|e| format!("Could not prepare statement to add user : {}", e))?;
    statement.execute(params![name, user.email, user.password])
        .map_err(|e| format!("Could not add user to database : {}", e))?;
    let id = connection.last_insert_rowid() as i32;
    Ok(User::new(id, name, user.email, user.password))
}

pub fn remove_users(db_path : &Path, id : i32) -> Result<(), String> {
    let connection = get_connection(db_path)?;

    let mut statement = connection.prepare("DELETE FROM users WHERE id = ?1")
        .map_err(|e| format!("Could not prepare statement to remove user : {}", e))?;
    Ok(())
}