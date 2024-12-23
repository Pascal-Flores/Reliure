use std::env;
use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Json};
use axum::response::{IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use crate::services::*;
use serde::{Deserialize, Serialize};
use sha_crypt::{Sha512Params, sha512_simple, sha512_check};
use tower_sessions::{Session, SessionStore};
use crate::db_manager::entities::user::User;

#[derive(Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtToken {
    sub: String,
    exp: usize,
}

fn generate_jwt(username: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = JwtToken {
        sub: username.to_string(),
        exp: expiration,
    };

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default".to_string());

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
        .expect("Failed to encode JWT")
}

pub async fn register(Json(payload): Json<CreateUser>) -> Json<Value> {
    let params = Sha512Params::new(10_000).expect("Erreur!");

    let hashed_password = sha512_simple(&*payload.password, &params)
        .expect("Should not fail");

    let user = User::new(0, payload.username, payload.email, hashed_password);

    let result = create_user(user);

    if result.is_err() {
        return Json(json!({ "error": result.err().unwrap() }));
    }

    Json(json!({ "data": result.unwrap() }))

}

pub async fn login(Extension(session): Extension<Session>, Json(payload): Json<LoginUser>) -> impl IntoResponse {

    let users = get_by_username(payload.username);

    if users.is_err() {
        return (
            [(axum::http::header::SET_COOKIE, "".to_string())],
            Json(json!({ "data": "Wrong username or password" }))
        )
    }

    let user = users.unwrap();

    if user.len() == 1 {
        let user_check = user.get(0).unwrap();
        if !sha512_check(&*payload.password, &*user_check.password_).is_ok() {
           return (
               [(axum::http::header::SET_COOKIE, "".to_string())],
               Json(json!({ "data": "Wrong username or password" }))
           )
        }

        let jwt = generate_jwt(&user_check.username_);
        session.insert("jwt", jwt.clone()).await.unwrap();
        let cookie = format!("jwt={}; HttpOnly; Path=/", jwt);
        (
            [(axum::http::header::SET_COOKIE, cookie)],
            Json(json!({ "data": "Logged in" }))
        )
    } else {
        (
            [(axum::http::header::SET_COOKIE, "".to_string())],
            Json(json!({ "data": "Wrong username or password" }))
        )
    }

}