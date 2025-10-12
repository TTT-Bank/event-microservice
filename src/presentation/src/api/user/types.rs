use domain::models::user::{UserFilter, UserId, UserModel, UserOrder};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_default_from_null;
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};
use utoipa::{IntoParams, ToResponse};

use super::super::{HandlerError, Result};

use super::super::utils::OffsetDto;

use super::dto::UserDto;

#[derive(Debug, Serialize, ToResponse)]
pub struct UserResponse {
        pub user: UserDto
}

impl From<UserModel> for UserResponse {
        fn from(value: UserModel) -> Self {
                Self { user: value.into() }
        }
}

#[derive(Debug, Serialize, ToResponse)]
pub struct UserVecResponse {
        pub users: Vec<UserDto>
}

impl From<Vec<UserModel>> for UserVecResponse {
        fn from(value: Vec<UserModel>) -> Self {
                Self {
                        users: value.into_iter().map(Into::into).collect()
                }
        }
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(names("user_id"), parameter_in = Path)]
pub struct UserIdParam(pub i64);

impl TryFrom<UserIdParam> for UserId {
        type Error = HandlerError;

        fn try_from(value: UserIdParam) -> Result<Self> {
                value.0.try_into().map_err(Into::into)
        }
}

#[serde_as]
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct ListUsersQuery {
        #[param(required = false)]
        #[serde(flatten, deserialize_with = "deserialize_default_from_null")]
        pub offset: OffsetDto,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, UserFilter>")]
        pub filter: Vec<UserFilter>,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, UserOrder>")]
        pub order_by: Vec<UserOrder>
}
