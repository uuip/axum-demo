use axum::{Json, Router};
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Result;
use axum::routing::{get, post};
use sea_orm::{QueryOrder, Set};
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::auth::Claims;
use crate::common::*;
use crate::models::prelude::*;

pub fn tree_route() -> Router<AppState, Body> {
    Router::new()
        .route("/:id", get(query_single_tree))
        .route("/q", get(query_some_tree))
        .route("/update", post(update_tree))
        .route("/testtoken", post(update_one))
}

pub async fn query_single_tree(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let obj = Trees::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(json!(obj)))
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    state: State<AppState>,
    pagination: Pagination,
    params: Query<SomeTrees>,
) -> Result<Json<Value>, ApiError> {
    let page = pagination.page;
    let page_size = pagination.size.unwrap();

    let paginator = Trees::find()
        .filter(trees::Column::Energy.gte(params.energy))
        .order_by_asc(trees::Column::Id)
        .paginate(&state.conn, page_size);
    // let num_pages = paginator.num_pages().await?;
    let objs = paginator.fetch_page(page - 1).await?;
    Ok(Json(json!(objs)))
    // .map(|items| Json(json!(items)))
    // .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()));
    // match objs {
    //     Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    //     Ok(objs) => Ok(Json(json!(objs))),
    // }
}

#[derive(Deserialize)]
pub struct Item {
    id: i32,
    energy: i32,
}

pub async fn update_one(
    State(state): State<AppState>,
    user: Claims,
    Json(payload): Json<Item>,
) -> Result<Json<Value>, ApiError> {
    println!("{}", user.id);
    let obj = Trees::find_by_id(payload.id)
        .one(&state.conn)
        .await?
        .ok_or(ApiError::NotFound)?;
    let mut obj: trees::ActiveModel = obj.into();
    obj.energy = Set(Option::from(payload.energy.to_owned()));
    let obj = obj.update(&state.conn).await?;
    Ok(Json(json!(obj)))
}

pub async fn update_tree(state: State<AppState>, payload: Json<Item>) -> StatusCode {
    let _ = Trees::update_many()
        .col_expr(trees::Column::Energy, Expr::value(payload.energy))
        .filter(trees::Column::Id.eq(payload.id))
        .exec(&state.conn)
        .await;
    StatusCode::OK
}
