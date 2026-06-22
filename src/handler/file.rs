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

pub async fn upload_file(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    mut multipart: Multipart
) -> Result<impl IntoResponse, HttpError> {

    let mut file_data = Vec::new();
    let mut file_name = String::new();
    let mut file_size: i64 = 0;

    let mut form_data = FileUploadDtos {
        recipient_email: String::new(),
        password: String::new(),
        expiration_date: String::new(),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "fileUpload" => {
                file_name = field.file_name()
                    .unwrap_or("unknow_file")
                    .to_string();

                file_data = field.bytes().await.unwrap().to_vec();
                file_size = file_data.len() as i64;
            }
            "recipient_email" => {
                form_data.recipient_email =
                    field.text().await.unwrap();
            }
            "password" => {
                form_data.password =
                    field.text().await.unwrap();
            }
            "expiration_date" => {
                form_data.expiration_date =
                    field.text().await.unwrap();
            }
            _ => {}
        }
    }

    form_data.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;