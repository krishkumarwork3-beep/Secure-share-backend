use std::sync::Arc;

use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};
use axum_extra::extract::cookie::CookieJar;

use crate::{db::UserExt, error::{ErrorMessage, HttpError}, models::User, utils::token, AppState};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddeware {
    pub user: User,
}
pub async fn auth(
    cookie_jar: CookieJar,
    Extension(app_state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, HttpError> {
    let cookies = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
});
let token = cookies.ok_or_else(|| {
    HttpError::unauthorized(
        ErrorMessage::TokenNotProvided.to_string()
    )
})?;