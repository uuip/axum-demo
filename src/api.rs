use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{ErrorResponse, IntoResponse, Result};
use sea_orm::{QueryOrder, Set};
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use serde::Deserialize;
use serde_json::{json, Value};

use models::prelude::*;

use crate::AppState;

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

pub async fn query_single_tree(state: State<AppState>, Path(id): Path<i32>) -> Json<Value> {
    let obj = Trees::find_by_id(id).one(&state.conn).await.unwrap();
    if let Some(obj) = obj {
        Json(json!(obj))
    } else {
        Json(json!({"status":"find nothing"}))
    }
}

#[derive(Deserialize)]
pub struct SomeTrees {
    energy: i32,
}

pub async fn query_some_tree(
    state: State<AppState>,
    pagination: Option<Query<Pagination>>,
    params: Query<SomeTrees>,
) -> Result<Json<Value>> {
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
    let objs = paginator.fetch_page(page - 1).await;
    match objs {
        Err(e) => Err(ErrorResponse::from(e.to_string())),
        Ok(objs) => Ok(Json(json!(objs))),
    }
}

#[derive(Deserialize)]
pub struct Item {
    id: i32,
    energy: i32,
}

pub async fn update_tree_obj(state: State<AppState>, payload: Json<Item>) -> impl IntoResponse {
    let obj = Trees::find_by_id(payload.id)
        .one(&state.conn)
        .await
        .expect("can't find this id")
        .unwrap();
    let mut obj: trees::ActiveModel = obj.into();
    obj.energy = Set(Option::from(payload.energy.to_owned()));
    obj.update(&state.conn).await.expect("update failed");
    StatusCode::OK
}

pub async fn update_tree(state: State<AppState>, payload: Json<Item>) -> StatusCode {
    let _ = Trees::update_many()
        .col_expr(trees::Column::Energy, Expr::value(payload.energy))
        .filter(trees::Column::Id.eq(payload.id))
        .exec(&state.conn)
        .await;
    StatusCode::OK
}
