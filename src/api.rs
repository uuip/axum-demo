use axum::extract::{Path, Query, State};
use axum::response::Result;
use axum::routing::{get, post};
use axum::{Json, Router};
use deadpool_postgres::Pool;
use postgres_from_row::FromRow;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::auth::Claims;
use crate::common::*;
use crate::models::Trees;

pub fn tree_route() -> Router<Pool> {
    Router::new()
        .route("/:id", get(query_single_tree))
        .route("/q", get(query_some_tree))
        .route("/update", post(update_tree))
}

pub async fn test_token(
    State(pool): State<Pool>,
    user: Claims,
    Json(payload): Json<Item>,
) -> Result<Json<Value>, ApiError> {
    let user_id = user.id;
    Ok(Json(json!({ "user": user_id })))
}

pub async fn query_single_tree(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let client = pool.get().await?;
    let row = client
        .query_one("select * from trees where id=$1", &[&id])
        .await?;
    let obj = Trees::from_row(&row);
    Ok(Json(json!(obj)))
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    State(pool): State<Pool>,
    pagination: Pagination,
    params: Query<SomeTrees>,
) -> Result<Json<Value>, ApiError> {
    let page = pagination.page;
    let page_size = pagination.size.unwrap();
    let offset = (page - 1) * page_size;
    let client = pool.get().await?;

    let objs = client
        .query(
            "select * from trees where energy>=$1 order by id desc limit $2 offset $3",
            &[&params.energy, &page_size, &offset],
        )
        .await?
        .iter()
        .map(Trees::from_row)
        .collect::<Vec<Trees>>();
    Ok(Json(json!(objs)))
}

#[derive(Deserialize)]
pub struct Item {
    id: i32,
    energy: i32,
}

pub async fn update_tree(
    State(pool): State<Pool>,
    payload: Json<Item>,
) -> Result<Json<Value>, ApiError> {
    let client = pool.get().await?;
    let rst = client
        .execute(
            "UPDATE trees SET energy=$1 WHERE id=$2",
            &[&payload.energy, &payload.id],
        )
        .await?;
    Ok(Json(json!({"id":payload.id, "rows_affected": rst })))
}
