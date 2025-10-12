use domain::models::event::{EventFilter, EventId, EventModel, EventOrder};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_default_from_null;
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};
use utoipa::{IntoParams, ToResponse};

use super::super::{HandlerError, Result};

use super::super::utils::OffsetDto;

use super::dto::EventDto;

#[derive(Debug, Serialize, ToResponse)]
pub struct EventResponse {
        pub event: EventDto
}

impl From<EventModel> for EventResponse {
        fn from(value: EventModel) -> Self {
                Self { event: value.into() }
        }
}

#[derive(Debug, Serialize, ToResponse)]
pub struct EventVecResponse {
        pub events: Vec<EventDto>
}

impl From<Vec<EventModel>> for EventVecResponse {
        fn from(value: Vec<EventModel>) -> Self {
                Self {
                        events: value.into_iter().map(Into::into).collect()
                }
        }
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(names("event_id"), parameter_in = Path)]
pub struct EventIdParam(pub i64);

impl TryFrom<EventIdParam> for EventId {
        type Error = HandlerError;

        fn try_from(value: EventIdParam) -> Result<Self> {
                value.0.try_into().map_err(Into::into)
        }
}

#[serde_as]
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct ListEventsQuery {
        #[param(required = false)]
        #[serde(flatten, deserialize_with = "deserialize_default_from_null")]
        pub offset: OffsetDto,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, EventFilter>")]
        pub filter: Vec<EventFilter>,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, EventOrder>")]
        pub order_by: Vec<EventOrder>
}
