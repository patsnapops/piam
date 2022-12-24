use busylib::crypto::{decrypt_by_key, encrypt_by_key};

use crate::config::META_KEY;

pub fn encrypt(value: String) -> String {
    encrypt_by_key(value, &META_KEY.load())
}

pub fn decrypt(value: String) -> String {
    decrypt_by_key(value, &META_KEY.load())
}
