use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::datetime_serializer::serialize;

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Trees {
    pub id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub energy: Option<i32>,
    #[serde(serialize_with = "serialize")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(serialize_with = "serialize")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub phone: Option<String>,
    pub energy: Option<i32>,
    pub available_energy: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
