use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offset {
        pub page: i32,
        pub limit: i32
}
