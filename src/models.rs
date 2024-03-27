use crate::common::datetime_serializer::serialize;
use chrono::{DateTime, Local};
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
    pub created_at: Option<DateTime<Local>>,
    #[serde(serialize_with = "serialize")]
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub phone: Option<String>,
    pub energy: Option<i32>,
    pub available_energy: Option<i32>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}
