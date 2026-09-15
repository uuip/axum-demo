use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::auth::Claims;
use crate::common::{ApiError, Pagination};
use crate::models::Trees;

pub fn tree_route() -> Router<PgPool> {
    Router::new()
        .route("/{id}", get(query_single_tree))
        .route("/q", get(query_some_tree))
        .route("/update", post(update_tree))
}

pub async fn test_token(
    State(_pool): State<PgPool>,
    user: Claims,
    Json(_payload): Json<Item>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({ "user": user.id })))
}

pub async fn query_single_tree(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Trees>, ApiError> {
    let tree = sqlx::query_as("select * from trees where id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(tree))
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    State(pool): State<PgPool>,
    pagination: Pagination,
    Query(params): Query<SomeTrees>,
) -> Result<Json<Vec<Trees>>, ApiError> {
    let page_size = pagination.size;
    let offset = (pagination.page - 1)
        .checked_mul(page_size)
        .ok_or(ApiError::PageError)?;

    let trees =
        sqlx::query_as("select * from trees where energy>=$1 order by id desc limit $2 offset $3")
            .bind(params.energy)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&pool)
            .await?;
    Ok(Json(trees))
}

#[derive(Deserialize)]
pub struct Item {
    id: i32,
    energy: i32,
}

pub async fn update_tree(
    State(pool): State<PgPool>,
    Json(payload): Json<Item>,
) -> Result<Json<Value>, ApiError> {
    // let id: i32 = rand::thread_rng().gen_range(1..=9999999);
    let rows_affected = sqlx::query("UPDATE trees SET energy=$1 WHERE id=$2")
        .bind(payload.energy)
        .bind(payload.id)
        .execute(&pool)
        .await?
        .rows_affected();
    Ok(Json(
        json!({ "id": payload.id, "rows_affected": rows_affected }),
    ))
}
