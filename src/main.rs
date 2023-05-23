#![allow(dead_code, unused_variables)]

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
        .with_state(connection().await);

    axum::Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
