use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::config::META_KEY;

pub fn encrypt(value: String) -> String {
    encrypt_by_key(value, &META_KEY.load())
}

pub fn decrypt(value: String) -> String {
    decrypt_by_key(value, &META_KEY.load())
}

/// return encrypted string in base64
pub fn encrypt_by_key(value: String, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.encrypt_str_to_base64(value)
}

/// return decrypted string from base64
pub fn decrypt_by_key(value: String, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.decrypt_base64_to_string(&value).unwrap()
}

pub mod test {
    #[test]
    fn encrypt_test() {
        let msg = "https?";
        let encrypted = crate::encrypt::encrypt(msg.to_string());
        let decrypted = crate::encrypt::decrypt(encrypted);

        assert_eq!(msg, decrypted);
    }
}
