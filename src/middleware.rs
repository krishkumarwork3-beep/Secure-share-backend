use std::sync::Arc;

use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};
use axum_extra::extract::cookie::CookieJar;

use crate::{db::UserExt, error::{ErrorMessage, HttpError}, models::User, utils::token, AppState};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddeware {
    pub user: User,
}