use std::{path::Path, process::exit};
use std::env;
use axum::{routing::get, routing::post, Router, response::IntoResponse, Extension};
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use db_manager::create_database;
use controllers::*;
use serde::{Deserialize, Serialize};
use time::Duration;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer, SessionStore};
use tower::{ServiceBuilder};

mod db_manager;
mod controllers;
mod services;


// pub fn session_middlware (req: Request<Body>, session: Session) -> Result<Response<Body>, StatusCode> {
//     let mut parts = req.into_parts();
//     parts.extensions.insert(session);
//     let req = Request::from_parts(parts.0, parts.1);
//     Ok(req.into_response())
// }



#[tokio::main()]
async fn main() {

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)));


    let db_path_str = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data.db".to_string());
    let db_path = Path::new(&db_path_str);
    if !db_path.exists() {
        match create_database(db_path) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("{}", error);
                exit(-1);
            }
        }
    }

    let app = Router::new()
        .route("/", get( hello_world ))
        .route("/register", post( register ))
        .route("/login", post( login ))
            .layer(Extension(session_layer)

        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}