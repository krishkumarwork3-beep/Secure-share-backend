use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, routing::{get, put}, Extension, Json, Router};
use validator::Validate;

use crate::{
    db::UserExt,
    dtos::{
        EmailListResponseDto,
        FilterEmailDto,
        FilterUserDto,
        NameUpdateDto,
        Response,
        SearchQueryByEmailDTO,
        UserData,
        UserPasswordUpdateDto,
        UserResponseDto
    },
    error::{ErrorMessage, HttpError},
    middleware::JWTAuthMiddeware,
    utils::password,
    AppState
};
pub fn users_handler() -> Router {
    Router::new()
    .route(
        "/me",
        get(get_me)
    )
        .route("/name", put(update_user_name))
    .route("/password", put(update_user_password))
    .route("/search-emails", get(search_by_email))
}