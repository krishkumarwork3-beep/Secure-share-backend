use std::{fs, path::PathBuf, sync::Arc};

use axum::{
    body::Body,
    extract::Multipart,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Extension,
    Json,
    Router,
};

use chrono::{DateTime, Utc};

use rsa::{
    pkcs1::{
        DecodeRsaPrivateKey,
        DecodeRsaPublicKey,
    },
    RsaPrivateKey,
    RsaPublicKey,
};

use validator::Validate;

use base64::{
    engine::general_purpose::STANDARD,
    Engine,
};

use crate::{
    db::UserExt,
    dtos::{
        FileUploadDtos,
        Response as ResponseDto,
        RetrieveFileDto,
    },
    error::HttpError,
    middleware::JWTAuthMiddeware,
    utils::{
        decrypt::decrypt_file,
        encrypt::encrypt_file,
        password,
    },
    AppState,
};
pub fn file_handle() -> Router {
    Router::new()
        .route("/upload", post(upload_file))
        .route("/retrieve", post(retrieve_file))
}