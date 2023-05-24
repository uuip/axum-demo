use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::tokio_pg_mapper_derive::PostgresMapper;

use crate::common::datetime_serializer::serialize;

#[derive(Clone, Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "trees")]
pub struct Trees {
    pub id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub energy: Option<i32>,
    #[serde(serialize_with = "serialize")]
    pub create_at: Option<NaiveDateTime>,
    #[serde(serialize_with = "serialize")]
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub phone: Option<String>,
    pub energy: Option<i32>,
    pub available_energy: Option<i32>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}
