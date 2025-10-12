use domain::models::token::TokenPair;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "TokenPair")]
pub struct TokenPairDto {
        pub access: String,
        pub refresh: String
}

impl From<TokenPairDto> for TokenPair {
        fn from(value: TokenPairDto) -> Self {
                TokenPair {
                        access_token: value.access,
                        refresh_token: value.refresh
                }
        }
}

impl From<TokenPair> for TokenPairDto {
        fn from(value: TokenPair) -> Self {
                Self {
                        access: value.access_token,
                        refresh: value.refresh_token
                }
        }
}