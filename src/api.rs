use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Result;
use axum::routing::{get, post};
use axum::{Json, Router};
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{DatabaseConnection, QueryOrder, Set};
use serde::Deserialize;

use crate::auth::Claims;
use crate::common::*;
use crate::models::prelude::*;

pub fn tree_route() -> Router<DatabaseConnection> {
    Router::new()
        .route("/:id", get(query_single_tree))
        .route("/q", get(query_some_tree))
        .route("/update", post(update_tree))
        .route("/testtoken", post(update_one))
}

pub async fn query_single_tree(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<trees::Model>, ApiError> {
    let obj = Trees::find_by_id(id)
        .one(&conn)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(obj))
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    State(conn): State<DatabaseConnection>,
    pagination: Pagination,
    params: Query<SomeTrees>,
) -> Result<Json<Vec<trees::Model>>, ApiError> {
    let page = pagination.page;
    let page_size = pagination.size.unwrap();

    let paginator = Trees::find()
        .filter(trees::Column::Energy.gte(params.energy))
        .order_by_asc(trees::Column::Id)
        .paginate(&conn, page_size);
    // let num_pages = paginator.num_pages().await?;
    let objs = paginator.fetch_page(page - 1).await?;
    Ok(Json(objs))
}

#[derive(Deserialize)]
pub struct Item {
    id: i32,
    energy: i32,
}

pub async fn update_one(
    State(conn): State<DatabaseConnection>,
    user: Claims,
    Json(payload): Json<Item>,
) -> Result<Json<trees::Model>, ApiError> {
    println!("{}", user.id);
    let obj = Trees::find_by_id(payload.id)
        .one(&conn)
        .await?
        .ok_or(ApiError::NotFound)?;
    let mut obj: trees::ActiveModel = obj.into();
    obj.energy = Set(Some(payload.energy));
    let obj = obj.update(&conn).await?;
    Ok(Json(obj))
}

pub async fn update_tree(
    State(conn): State<DatabaseConnection>,
    payload: Json<Item>,
) -> Result<StatusCode, ApiError> {
    Trees::update_many()
        .col_expr(trees::Column::Energy, Expr::value(payload.energy))
        .filter(trees::Column::Id.eq(payload.id))
        .exec(&conn)
        .await?;
    Ok(StatusCode::OK)
}
