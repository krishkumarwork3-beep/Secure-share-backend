use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::Rng;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

use crate::error::HttpError;