use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Offset {
        pub page: i32,
        pub limit: i32
}
