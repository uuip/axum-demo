use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use deadpool_postgres::Pool;
use postgres_from_row::FromRow;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::auth::Claims;
use crate::common::{ApiError, Pagination};
use crate::models::Trees;

pub fn tree_route() -> Router<Pool> {
    Router::new()
        .route("/{id}", get(query_single_tree))
        .route("/q", get(query_some_tree))
        .route("/update", post(update_tree))
}

pub async fn test_token(
    State(_pool): State<Pool>,
    user: Claims,
    Json(_payload): Json<Item>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({ "user": user.id })))
}

pub async fn query_single_tree(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<Trees>, ApiError> {
    let client = pool.get().await?;
    let row = client
        .query_one("select * from trees where id=$1", &[&id])
        .await?;
    Ok(Json(Trees::try_from_row(&row)?))
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    State(pool): State<Pool>,
    pagination: Pagination,
    Query(params): Query<SomeTrees>,
) -> Result<Json<Vec<Trees>>, ApiError> {
    let page_size = pagination.size;
    let offset = (pagination.page - 1)
        .checked_mul(page_size)
        .ok_or(ApiError::PageError)?;
    let client = pool.get().await?;

    let trees = client
        .query(
            "select * from trees where energy>=$1 order by id desc limit $2 offset $3",
            &[&params.energy, &page_size, &offset],
        )
        .await?
        .iter()
        .map(Trees::try_from_row)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(trees))
}

#[derive(Deserialize)]
pub struct Item {
    id: i32,
    energy: i32,
}

pub async fn update_tree(
    State(pool): State<Pool>,
    Json(payload): Json<Item>,
) -> Result<Json<Value>, ApiError> {
    let client = pool.get().await?;
    let rows_affected = client
        .execute(
            "UPDATE trees SET energy=$1 WHERE id=$2",
            &[&payload.energy, &payload.id],
        )
        .await?;
    Ok(Json(
        json!({ "id": payload.id, "rows_affected": rows_affected }),
    ))
}
