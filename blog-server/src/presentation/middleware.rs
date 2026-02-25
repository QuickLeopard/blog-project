use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest, error::ErrorUnauthorized, web};
use std::future::Future;
use std::pin::Pin;

use crate::infrastructure::app_state::AppState;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub user_name: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            // 1. Extract and parse Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .ok_or_else(|| ErrorUnauthorized("Missing Authorization header"))?
                .to_str()
                .map_err(|_| ErrorUnauthorized("Invalid Authorization header"))?
                .to_owned();

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or_else(|| ErrorUnauthorized("Invalid token format"))?;

            // 2. Verify token using AppState
            let state = req
                .app_data::<web::Data<AppState>>()
                .ok_or_else(|| ErrorUnauthorized("App state missing"))?;

            let claims = state
                .auth_service
                .verify_token(token)
                .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

            Ok(AuthenticatedUser {
                user_id: claims.user_id,
                user_name: claims.user_name,
            })
        })
    }
}

/*use actix_web::dev::{Payload, ServiceRequest};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, error::ErrorUnauthorized, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::{Ready, ready};

use crate::infrastructure::app_state::AppState;
use crate::infrastructure::jwt::JwtService;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    //#[allow(dead_code)]
    pub user_name: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = req
            .extensions()
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or_else(|| ErrorUnauthorized("Missing authenticated user context"));

        ready(result)
    }
}

fn auth_error(msg: &'static str, req: ServiceRequest) -> (Error, ServiceRequest) {
    (ErrorUnauthorized(msg), req)
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // 1. Extract Application State safely
    // Use match to avoid borrowing issues with closures
    let state = match req.app_data::<web::Data<AppState>>() {
        Some(state) => state,
        None => return Err((ErrorUnauthorized("Application state missing"), req)),
    };

    // 2. Verify JWT Token
    // Assumes JwtService::verify_token returns Result<Claims, JwtError>
    let claims = match state.auth_service.verify_token(credentials.token()) {
        Ok(claims) => claims,
        Err(_) => return Err((ErrorUnauthorized("Invalid or expired token"), req)),
    };

    // 3. Construct Authenticated User
    let user = AuthenticatedUser {
        user_id: claims.user_id,
        user_name: claims.user_name,
    };

    // 4. Inject User into Request Extensions
    req.extensions_mut().insert(user);

    Ok(req)
}*/
