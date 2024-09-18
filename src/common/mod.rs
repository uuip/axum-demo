pub use self::db::connection;
pub use self::error::ApiError;
pub use self::pagination::Pagination;

mod db;
mod error;
mod pagination;

pub mod datetime_serializer {
    use chrono::{DateTime, Utc};
    use serde::Serializer;

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.map_or_else(|| "".to_string(), |date| date.format(FORMAT).to_string());
        serializer.serialize_str(&s)
    }
}
