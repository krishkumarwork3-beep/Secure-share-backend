use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

use crate::error::HttpError;
pub async fn decrypt_file(
    encrypted_aes_key: Vec<u8>,
    encrypted_file_data: Vec<u8>,
    iv: Vec<u8>,
    user_private_key: &RsaPrivateKey,
) -> Result<Vec<u8>, HttpError> {
    let aes_key = user_private_key.decrypt(
    Pkcs1v15Encrypt,
    &encrypted_aes_key
)
.map_err(|e| HttpError::server_error(e.to_string()))?;
let iv = iv;