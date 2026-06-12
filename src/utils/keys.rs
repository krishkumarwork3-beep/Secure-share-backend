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