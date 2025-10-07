use std::{fs::File, io::Read, sync::LazyLock};
use actix_web::{HttpMessage, dev::ServiceRequest};
use actix_web_httpauth::{extractors::{AuthenticationError, bearer::{self, BearerAuth}}, headers::www_authenticate::bearer::Bearer};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::OffsetDateTime;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};

use crate::domain::user::model::{UserId, UserModel, UserRole};

static PUB_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
        let mut buffer = Vec::new();
        File::open("pkey.pub.pem")
                .expect("file pkey.pub.pem should exist")
                .read_to_end(&mut buffer)
                .expect("file pkey.pub.pem should contain pub key");

        DecodingKey::from_ed_pem(&buffer)
                .expect("file pkey.pub.pem should be valid EdDSA pub key")
});

static SEC_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
        let mut buffer = Vec::new();
        File::open("pkey.pem")
                .expect("file pkey.pem should exist")
                .read_to_end(&mut buffer)
                .expect("file pkey.pem should contain sec key");

        EncodingKey::from_ed_pem(&buffer)
                .expect("file pkey.pem should be valid EdDSA sec key")
});

static JWT_EXPIRES_AFTER: LazyLock<time::Duration> = LazyLock::new(|| {
        dotenvy::var("JWT_EXPIRES_AFTER")
                .expect("JWT_EXPIRES_AFTER var should be set")
                .parse()
                .map(time::Duration::seconds)
                .expect("JWT_EXPIRES_AFTER should be valid u64")
});

static VALIDATION: LazyLock<Validation> = LazyLock::new(|| {
        Validation::new(Algorithm::EdDSA)
});

static HEADER: LazyLock<Header> = LazyLock::new(|| {
        Header::new(Algorithm::EdDSA)
});

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credentials {
        iss: String,
        sub: UserId,
        #[serde_as(as = "TimestampSeconds")]
        exp: OffsetDateTime,
        #[serde_as(as = "TimestampSeconds")]
        nbf: OffsetDateTime,
        #[serde_as(as = "TimestampSeconds")]
        iat: OffsetDateTime,
        role: UserRole
}

impl Credentials {
        pub fn new(user: UserModel) -> Self {
                let now = time::UtcDateTime::now();
                Self {
                        iss: String::from("event-microservice"),
                        sub: user.id,
                        exp: (now + *JWT_EXPIRES_AFTER).into(),
                        nbf: now.into(),
                        iat: now.into(),
                        role: user.role
                }
        }

        pub fn encode(self) -> Result<String, jsonwebtoken::errors::Error> {
                jsonwebtoken::encode(&HEADER, &self, &SEC_KEY)
        }
}

pub async fn validator(
        req: ServiceRequest,
        jwt: BearerAuth
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
        let credentials =
                jsonwebtoken::decode::<Credentials>(jwt.token(), &PUB_KEY, &VALIDATION);
        match credentials {
                Ok(data) => {
                        req.extensions_mut().insert(data.claims);
                        Ok(req)
                }
                Err(_) => Err((AuthenticationError::new(Bearer::build().error(bearer::Error::InvalidToken).finish()).into(), req))
        }
}

#[derive(Clone)]
pub struct AuthenticatedUser {
        pub id: UserId,
        pub role: UserRole
}

impl FromRequest for AuthenticatedUser {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;

        fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                if let Some(credentials) = req.extensions().get::<Credentials>() {
                        let credentials = credentials.clone();
                        ready(Ok(AuthenticatedUser {id: credentials.sub, role: credentials.role}))
                } else {
                        ready(Err(actix_web::error::ErrorUnauthorized("Missing credentials")))
                }
        }
}
