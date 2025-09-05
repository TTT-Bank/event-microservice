use super::utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventInsert {
        name: String,
        description: String,
        date: time::OffsetDateTime,
        place: utils::Place,
        cost: utils::Cost
}