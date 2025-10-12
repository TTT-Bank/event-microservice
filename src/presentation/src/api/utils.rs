use domain::models::utils::Offset;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use utoipa::ToSchema;

use super::{Result, HandlerError};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct OffsetDto {
        #[serde_as(as = "DisplayFromStr")]
        pub page: i32,
        #[serde_as(as = "DisplayFromStr")]
        pub limit: i32
}

impl TryFrom<OffsetDto> for Offset {
        type Error = HandlerError;

        fn try_from(value: OffsetDto) -> Result<Self> {
                Ok(Offset {
                        page: value.page.try_into()?,
                        limit: value.limit.try_into()?
                })
        }
}

impl Default for OffsetDto {
        fn default() -> Self {
                Self {
                        page: 1,
                        limit: 50
                }
        }
}
