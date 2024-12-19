use std::{path::Path, process::exit};
use std::env;
use axum::{routing::get, Router,};
use db_manager::create_database;
mod db_manager;


#[tokio::main()]
async fn main() {
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
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}