use std::sync::Arc;

use axum::{http::{header, HeaderMap, StatusCode}, response::IntoResponse, routing::post, Extension, Json, Router};
use axum_extra::extract::cookie::Cookie;
use validator::Validate;

use crate::{db::UserExt, dtos::{LoginUserDto, RegisterUserDto, Response, UserLoginResponseDto}, error::{ErrorMessage, HttpError}, utils::{keys::generate_key, password, token}, AppState};
pub fn auth_handler() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
    .map_err(|e| HttpError::bad_request(e.to_string()))?;
   let hash_password = password::hash(&body.password)
    .map_err(|e| HttpError::server_error(e.to_string()))?;