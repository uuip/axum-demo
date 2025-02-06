use axum::routing::post;
use axum::Router;
use tower_http::cors::CorsLayer;

use crate::api::tree_route;
use crate::auth::login;
use crate::common::*;

pub mod api;
pub mod auth;
pub mod common;
pub mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/auth/login", post(login))
        .nest("/tree", tree_route())
        .layer(CorsLayer::permissive())
        .with_state(connection().await?);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
