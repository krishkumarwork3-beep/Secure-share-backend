use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::Rng;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

use crate::error::HttpError;
pub async fn encrypt_file(
    file_data: Vec<u8>,
    user_public_key: &RsaPublicKey
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), HttpError> {