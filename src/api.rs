use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Result;
use axum::Json;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{QueryOrder, Set};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::db::AppState;
use crate::error::{ApiError, Error};
use crate::models::prelude::*;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u64>,
    size: Option<u64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: Some(1),
            size: Some(10),
        }
    }
}

pub async fn query_single_tree(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let obj = Trees::find_by_id(id)
        .one(&state.conn)
        .await
        .map_err(Error::DbError)?
        .ok_or(Error::NotFound)?;
    Ok(Json(json!(obj)))
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    state: State<AppState>,
    pagination: Option<Query<Pagination>>,
    params: Query<SomeTrees>,
) -> Result<Json<Value>, ApiError> {
    let p = pagination.unwrap_or_default();
    let mut page = p.page.unwrap_or(1);
    if page < 1 {
        page = 1
    }
    let page_size = p.size.unwrap_or(10);

    let paginator = Trees::find()
        .filter(trees::Column::Energy.gte(params.energy))
        .order_by_asc(trees::Column::Id)
        .paginate(&state.conn, page_size);
    // let num_pages = paginator.num_pages().await?;
    let objs = paginator
        .fetch_page(page - 1)
        .await
        .map_err(Error::DbError)?;
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
    state: State<AppState>,
    payload: Json<Item>,
) -> Result<Json<Value>, ApiError> {
    let obj = Trees::find_by_id(payload.id)
        .one(&state.conn)
        .await
        .map_err(Error::DbError)?
        .ok_or(Error::NotFound)?;
    let mut obj: trees::ActiveModel = obj.into();
    obj.energy = Set(Option::from(payload.energy.to_owned()));
    let obj = obj.update(&state.conn).await.map_err(Error::DbError)?;
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
