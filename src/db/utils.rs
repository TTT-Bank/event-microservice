use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema)]
pub struct Offset {
        pub page: i32,
        pub limit: i32
}

impl Default for Offset {
        fn default() -> Self {
            Self {
                page: 1,
                limit: 50
            }
        }
}
