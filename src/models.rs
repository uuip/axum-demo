use crate::common::datetime_serializer::serialize;
use chrono::NaiveDateTime;
use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
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

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
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
