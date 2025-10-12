use std::{fs::File, io::Read, sync::LazyLock};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds, DisplayFromStr};
use time::{Duration, OffsetDateTime};

use super::user::{UserRole, UserId};

static PUBLIC_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
        let mut buffer = Vec::new();
        File::open("pkey.pub.pem")
                .expect("file pkey.pub.pem should exist")
                .read_to_end(&mut buffer)
                .expect("file pkey.pub.pem should contain pub key");

        DecodingKey::from_ed_pem(&buffer)
                .expect("file pkey.pub.pem should be valid EdDSA pub key")
});

static SECRET_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
        let mut buffer = Vec::new();
        File::open("pkey.pem")
                .expect("file pkey.pem should exist")
                .read_to_end(&mut buffer)
                .expect("file pkey.pem should contain sec key");

        EncodingKey::from_ed_pem(&buffer)
                .expect("file pkey.pem should be valid EdDSA sec key")
});

static ACCESS_EXPIRES_AFTER: LazyLock<Duration> = LazyLock::new(|| {
        dotenvy::var("ACCESS_EXPIRES_AFTER")
                .expect("ACCESS_EXPIRES_AFTER var should be set")
                .parse()
                .map(time::Duration::seconds)
                .expect("ACCESS_EXPIRES_AFTER should be valid u64")
});

static REFRESH_EXPIRES_AFTER: LazyLock<Duration> = LazyLock::new(|| {
        dotenvy::var("REFRESH_EXPIRES_AFTER")
                .expect("REFRESH_EXPIRES_AFTER var should be set")
                .parse()
                .map(time::Duration::seconds)
                .expect("REFRESH_EXPIRES_AFTER should be valid u64")
});

static VALIDATION: LazyLock<Validation> = LazyLock::new(|| {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_issuer(&["event_microservice"]);
        validation.set_audience(&["access", "refresh"]);
        validation
});

static HEADER: LazyLock<Header> = LazyLock::new(|| {
        Header::new(Algorithm::EdDSA)
});

pub type AccessToken = String;
pub type RefreshToken = String;

#[derive(Debug)]
pub struct TokenPair {
        pub access_token: AccessToken,
        pub refresh_token: RefreshToken
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
        pub iss: String,
        pub sub: UserId,
        pub aud: String,
        #[serde_as(as = "TimestampSeconds")]
        pub exp: OffsetDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub nbf: OffsetDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub iat: OffsetDateTime,
        #[serde_as(as = "DisplayFromStr")]
        pub role: UserRole
}

impl Claims {
        pub fn new_access(user_id: UserId, current_timestamp: OffsetDateTime, role: UserRole) -> Self {
                Self {
                        iss: String::from("event_microservice"),
                        sub: user_id,
                        aud: String::from("access"),
                        exp: current_timestamp + *ACCESS_EXPIRES_AFTER,
                        nbf: current_timestamp,
                        iat: current_timestamp,
                        role
                }
        }

        pub fn new_refresh(user_id: UserId, current_timestamp: OffsetDateTime, role: UserRole) -> Self {
                Self {
                        iss: String::from("event_microservice"),
                        sub: user_id,
                        aud: String::from("refresh"),
                        exp: current_timestamp + *REFRESH_EXPIRES_AFTER,
                        nbf: current_timestamp,
                        iat: current_timestamp,
                        role
                }
        }

        pub fn encode(self) -> Result<String, jsonwebtoken::errors::Error> {
                jsonwebtoken::encode(&HEADER, &self, &SECRET_KEY)
        }

        pub fn decode_from(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
                Ok(jsonwebtoken::decode(&token, &PUBLIC_KEY, &VALIDATION)?.claims)
        }
}