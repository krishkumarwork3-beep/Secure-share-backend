use std::sync::Arc;

use axum::{
    extract::Query,
    response::IntoResponse,
    routing::get,
    Extension,
    Json,
    Router
};
use validator::Validate;

use crate::{
    db::UserExt,
    dtos::{
        RequestQueryDto,
        UserReceiveFileDto,
        UserReceiveFileListResponseDto,
        UserSendFileDto,
        UserSendFileListResponseDto
    },
    error::HttpError,
    middleware::JWTAuthMiddeware,
    AppState
};

pub fn get_file_list_handler() -> Router {
    Router::new()
       .route("/send", get(get_user_shared_files))
       .route("/receive", get(get_receive_shared_files))
}