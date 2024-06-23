mod models;
pub use models::{PasswordEntry, PasswordManager};

use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit}, Aes128};
use base64::{engine::general_purpose, Engine};

// key for the encryption and decryption of the passwords
const KEY: &[u8; 16] = b"maal--o--escobar";

// function to encrypt the password using AES128 algo
pub fn encrypt_password(password: &str) -> String {
    let key = GenericArray::from_slice(KEY);
    let cipher = Aes128::new(key);

    // pad the password to 16 bytes
    let mut block = [0u8; 16];
    let bytes = password.as_bytes();
    let len = bytes.len().min(16);
    block[..len].copy_from_slice(&bytes[..len]);

    let mut block = GenericArray::clone_from_slice(password.as_bytes());
    cipher.encrypt_block(&mut block);

    general_purpose::STANDARD.encode(&block[..])
}

// function for decryption
pub fn decrypt_password(encrypted_password: &str) -> String {
    let key = GenericArray::from_slice(KEY);
    let cipher = Aes128::new(key);

    let decoded = general_purpose::STANDARD.decode(encrypted_password).expect("Failed to decode base64");
    let mut block: GenericArray<u8, _> = GenericArray::clone_from_slice(&decoded);
    cipher.decrypt_block(&mut block);

    // convert to string and trim the null padding
    let decrypted = String::from_utf8(block.to_vec()).expect("failed to convert to string");
    decrypted.trim_matches(char::from(0)).to_string()
}