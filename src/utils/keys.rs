use std::{fs::{self, File}, io::Write, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse};
use rand::rngs::OsRng;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    RsaPrivateKey,
    RsaPublicKey
};
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::{db::UserExt, error::HttpError, models::User, AppState};

pub async fn generate_key(
    app_state: Arc<AppState>,
    user: User,
) -> Result<impl IntoResponse, HttpError> {
        let mut rng = OsRng;

        let private_key = RsaPrivateKey::new(&mut rng, 2048)
    .map_err(|e| {
        HttpError::server_error(e.to_string())
    })?;

        let public_key = RsaPublicKey::from(&private_key);