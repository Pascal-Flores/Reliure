use derive_new::new;
use diesel::{dsl::delete, insert_into, prelude::Queryable, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::db_manager::entities::schema::users::dsl::*;
use rusqlite::params;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::db_manager::{get_connection, Author};

#[derive(Queryable, PartialEq, Debug, new, Clone, Serialize, Deserialize)]
pub struct User {
    pub id_ : i32,
    pub username_ : String,
    pub email_ : String,
    pub password_ : String,
}

pub fn get_users(connection : &mut SqliteConnection) -> Result<Vec<User>, String> {
    // if (fields.len() == 0) {
    //     return Err("No fields specified".to_string());
    // }
    //
    // let mut query = "SELECT ".to_string();
    // for field in fields {
    //     query.push_str(field);
    //     query.push_str(",");
    // }
    // query.pop();
    // query += &*(" FROM users".to_owned() + &*where_clause);
    // println!("{}", query);

    return users.load::<User>(connection)
        .map_err(|e| format!("An errror occured while getting all users : {}", e));

    // let mut statement = connection.prepare(&*query)
    //     .map_err(|e| format!("Could not prepare statement to get users : {}", e))?;
    //
    // let users = statement.query_map(params![], |row| {
    //     Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    // }).map_err(|e| format!("Could not get users from database : {}", e))?;
    //
    // let mut result = Vec::new();
    // for user in users {
    //     result.push(user.map_err(|e| format!("Could not get user from database : {}", e))?)
    // }
    // Ok(result)
}

pub fn get_user_by_id(connection : &mut SqliteConnection, user_id : i32) -> Result<Vec<User>, String> {
    return users.filter(id.eq(user_id))
        .load::<User>(connection)
        .map_err(|e| format!("An errror occured while getting user {:?} : {}", user_id, e));
}

pub fn get_user_by_username(connection : &mut SqliteConnection, filter_username : &str) -> Result<Vec<User>, String> {
    return users.filter(username.eq(filter_username))
        .load::<User>(connection)
        .map_err(|e| format!("An errror occured while getting user {:?} : {}", filter_username, e));
}

pub fn add_user(connection : &mut SqliteConnection, user: User) -> Result<User, String> {
    let added_rows = insert_into(users)
        .values((username.eq(user.username_.clone()), email.eq(user.email_), password.eq(user.password_)))
        .execute(connection)
        .map_err(|e| format!("Could not add user {} to database : {}", "username", e))?;
    match added_rows {
        1 => Ok(get_user_by_username(connection, &user.username_.clone())?.pop().unwrap()),
        _ => Err(format!("Something wrong happened while adding user {} to database", "username"))
    }
}

pub fn remove_users(connection : &mut SqliteConnection) -> Result<(), String> {
    let deleted_rows = delete(users).filter(id.eq(1)).execute(connection)
        .map_err(|e| format!("Could not delete user {} from database : {}", 1, e))?;
    match deleted_rows {
        1 => Ok(()),
        _ => Err(format!("Something wrong happened while deleting user {} from database", 1))
    }
}