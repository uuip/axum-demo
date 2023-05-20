#![allow(dead_code, unused_variables)]

use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::Database;
use tower_http::cors::CorsLayer;

use crate::api::{query_single_tree, query_some_tree, update_tree};
use crate::db::AppState;

pub mod api;
pub mod db;
pub mod error;
pub mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // dotenvy::dotenv()?;
    let dsn = dotenvy::var("DATABASE_URL")?;
    let conn = Database::connect(dsn).await?;
    let state = AppState { conn };

    let app = Router::new()
        .route("/tree/:id", get(query_single_tree))
        .route("/tree/q", get(query_some_tree))
        .route("/tree/update", post(update_tree))
        .layer(CorsLayer::permissive())
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
