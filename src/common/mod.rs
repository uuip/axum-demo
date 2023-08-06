pub use self::db::{connection, AppState};
pub use self::error::ApiError;
pub use self::pagination::Pagination;

mod db;
mod error;
mod pagination;

pub mod datetime_serializer {
    use sea_orm::prelude::DateTimeLocal;
    use serde::Serializer;

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(date: &Option<DateTimeLocal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = if let Some(date) = date {
            format!("{}", date.format(FORMAT))
        } else {
            "".to_string()
        };
        serializer.serialize_str(&s)
    }
}
