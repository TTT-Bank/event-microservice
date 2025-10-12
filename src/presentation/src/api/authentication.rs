use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpMessage, dev::ServiceRequest};
use actix_web_grants::authorities::AttachAuthorities;
use actix_web_httpauth::{extractors::{AuthenticationError, bearer::{self, BearerAuth}}, headers::www_authenticate::bearer::Bearer};
use domain::models::token::Claims;

pub async fn validator(
        req: ServiceRequest,
        jwt: BearerAuth
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
        let claims = Claims::decode_from(jwt.token());
        match claims {
                Ok(claims) => {
                        if claims.aud != "access" {
                                return Err((AuthenticationError::new(Bearer::build().error(bearer::Error::InvalidToken).finish()).into(), req));
                        }
                        req.extensions_mut().insert(claims.clone());
                        req.attach([claims.role]);
                        Ok(req)
                }
                Err(_) => {
                        Err((AuthenticationError::new(Bearer::build().error(bearer::Error::InvalidToken).finish()).into(), req))
                }
        }
}

pub struct ReqClaims(pub Claims);

impl FromRequest for ReqClaims {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;

        fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                if let Some(claims) = req.extensions().get::<Claims>() {
                        ready(Ok(Self(claims.clone())))
                } else {
                        ready(Err(actix_web::error::ErrorUnauthorized("Missing credentials")))
                }
        }
}