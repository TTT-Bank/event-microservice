use domain::models::favorite::{FavoriteEventModel, FavoriteFilter, FavoriteId, FavoriteModel, FavoriteOrder};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_default_from_null;
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};
use utoipa::{IntoParams, ToResponse};

use super::super::{HandlerError, Result};

use super::super::utils::OffsetDto;

use super::dto::{FavoriteDto, FavoriteEventDto};

#[derive(Debug, Serialize, ToResponse)]
pub struct FavoriteResponse {
        pub favorite: FavoriteDto
}

impl From<FavoriteModel> for FavoriteResponse {
        fn from(value: FavoriteModel) -> Self {
                Self { favorite: value.into() }
        }
}

#[derive(Debug, Serialize, ToResponse)]
pub struct FavoriteEventResponse {
        pub favorite_event: FavoriteEventDto
}

impl From<FavoriteEventModel> for FavoriteEventResponse {
        fn from(value: FavoriteEventModel) -> Self {
                Self { favorite_event: value.into() }
        }
}

#[derive(Debug, Serialize, ToResponse)]
pub struct FavoriteEventVecResponse {
        pub favorite_events: Vec<FavoriteEventDto>
}

impl From<Vec<FavoriteEventModel>> for FavoriteEventVecResponse {
        fn from(value: Vec<FavoriteEventModel>) -> Self {
                Self {
                        favorite_events: value.into_iter().map(Into::into).collect()
                }
        }
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(names("user_id", "event_id"), parameter_in = Path)]
pub struct FavoriteIdParam(pub i64, pub i64);

impl TryFrom<FavoriteIdParam> for FavoriteId {
        type Error = HandlerError;

        fn try_from(value: FavoriteIdParam) -> Result<Self> {
                Ok(FavoriteId {
                        user_id: value.0.try_into()?,
                        event_id: value.1.try_into()?
                })
        }
}

#[serde_as]
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct ListFavoriteEventsQuery {
        #[param(required = false)]
        #[serde(flatten, deserialize_with = "deserialize_default_from_null")]
        pub offset: OffsetDto,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, FavoriteFilter>")]
        pub filter: Vec<FavoriteFilter>,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, FavoriteOrder>")]
        pub order_by: Vec<FavoriteOrder>
}
