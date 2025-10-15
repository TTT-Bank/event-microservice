use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpMessage, dev::ServiceRequest};
use actix_web_grants::authorities::AttachAuthorities;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use domain::models::token::Claims;

pub async fn validator(
        req: ServiceRequest,
        jwt: BearerAuth
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
        let claims = Claims::decode_from(jwt.token());
        match claims {
                Ok(claims) => {
                        if !claims.is_access() {
                                return Err((actix_web::error::ErrorUnauthorized("Use of refresh token"), req))
                        }
                        req.extensions_mut().insert(claims.clone());
                        req.attach([claims.role]);
                        Ok(req)
                }
                Err(_) => {
                        Err((actix_web::error::ErrorUnauthorized("Could not parse token"), req))
                }
        }
}

pub struct ClaimsExtractor(pub Claims);

impl ClaimsExtractor {
        pub fn into_inner(self) -> Claims {
                self.0
        }
}

impl FromRequest for ClaimsExtractor {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;

        fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                match req.extensions().get::<Claims>() {
                        Some(claims) => ready(Ok(Self(claims.clone()))),
                        None => ready(Err(actix_web::error::ErrorUnauthorized("Missing claims")))
                }
        }
}