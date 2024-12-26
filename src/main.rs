use std::{path::Path, process::exit};
use std::env;
use axum::{routing::get, routing::post, Router, response::IntoResponse, Extension};
use axum::extract::FromRequestParts;
use db_manager::create_database;
use controllers::*;
use serde::{Deserialize, Serialize};
use time::Duration;
use tower::Layer;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer, SessionStore};

mod db_manager;
mod controllers;
mod services;
mod document_scanner;


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
            .layer(session_layer.clone())
            .layer(Extension(session_layer.clone())
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}