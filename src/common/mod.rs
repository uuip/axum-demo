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
        match date {
            Some(date) => serializer.collect_str(&date.format(FORMAT)),
            None => serializer.serialize_str(""),
        }
    }
}
