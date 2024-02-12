use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use rand::rngs::OsRng;
use crate::helpers::types::TextInput;

pub mod list;
pub mod traits;
pub mod types;

pub fn generate_sec_websocket_key() -> String {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    general_purpose::STANDARD.encode(&key)
}

pub fn input_to_string(text_input: &TextInput) -> String {
    text_input.iter().collect()
}
