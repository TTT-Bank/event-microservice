use super::utils;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct EventEntry {
        id: i64,
        owner_id: i64,
        name: Box<str>,
        description: Box<str>,
        date: time::OffsetDateTime,
        place: utils::Place,
        cost: utils::Cost,
        status: utils::Status,
        created: time::OffsetDateTime,
        updated: time::OffsetDateTime
}