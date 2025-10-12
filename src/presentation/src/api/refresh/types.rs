use domain::models::token::TokenPair;
use serde::Serialize;
use utoipa::ToResponse;

use super::dto::TokenPairDto;

#[derive(Debug, Serialize, ToResponse)]
pub struct TokenResponse {
        pub token_pair: TokenPairDto
}

impl From<TokenPair> for TokenResponse {
        fn from(value: TokenPair) -> Self {
                Self {
                        token_pair: value.into()
                }
        }
}