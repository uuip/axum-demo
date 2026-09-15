use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::Deserialize;

use crate::auth::Claims;
use crate::common::{ApiError, Pagination};
use crate::models::prelude::{trees, Trees};

pub fn tree_route() -> Router<DatabaseConnection> {
    Router::new()
        .route("/{id}", get(query_single_tree))
        .route("/q", get(query_some_tree))
        .route("/update", post(update_tree))
        .route("/testtoken", post(update_one))
}

pub async fn query_single_tree(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<trees::Model>, ApiError> {
    let tree = Trees::find_by_id(id)
        .one(&conn)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(tree))
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    State(conn): State<DatabaseConnection>,
    pagination: Pagination,
    Query(params): Query<SomeTrees>,
) -> Result<Json<Vec<trees::Model>>, ApiError> {
    let page_size = pagination.size;
    let offset = (pagination.page - 1)
        .checked_mul(page_size)
        .ok_or(ApiError::PageError)?;

    let trees = Trees::find()
        .filter(trees::Column::Energy.gte(params.energy))
        .order_by_asc(trees::Column::Id)
        .limit(page_size)
        .offset(offset)
        .all(&conn)
        .await?;
    Ok(Json(trees))
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
    let tree = Trees::find_by_id(payload.id)
        .one(&conn)
        .await?
        .ok_or(ApiError::NotFound)?;
    let mut tree: trees::ActiveModel = tree.into();
    tree.energy = Set(Some(payload.energy));
    Ok(Json(tree.update(&conn).await?))
}

pub async fn update_tree(
    State(conn): State<DatabaseConnection>,
    Json(payload): Json<Item>,
) -> Result<StatusCode, ApiError> {
    Trees::update_many()
        .col_expr(trees::Column::Energy, Expr::value(payload.energy))
        .filter(trees::Column::Id.eq(payload.id))
        .exec(&conn)
        .await?;
    Ok(StatusCode::OK)
}
