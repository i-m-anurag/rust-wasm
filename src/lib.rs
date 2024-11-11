use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use base64::{engine::general_purpose::STANDARD, Engine};
use wasm_bindgen::prelude::*;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const FIXED_KEY: &[u8; 16] = b"//htps12345nrfbw"; // Define a fixed 16-byte key

#[wasm_bindgen]
pub struct SecurityService;

#[wasm_bindgen]
impl SecurityService {
    #[wasm_bindgen]
    pub fn set(value: &str) -> String {
        let iv = FIXED_KEY; // Use the fixed key as IV
        let cipher = Aes128Cbc::new_from_slices(FIXED_KEY, iv).unwrap();

        let encrypted_data = cipher.encrypt_vec(value.as_bytes());
        let base64_encrypted = STANDARD.encode(encrypted_data); // First Base64 encoding
        STANDARD.encode(base64_encrypted) // Second Base64 encoding
    }

    #[wasm_bindgen]
    pub fn get(encrypted_value: &str) -> String {
        let iv = FIXED_KEY; // Use the fixed key as IV
        let cipher = Aes128Cbc::new_from_slices(FIXED_KEY, iv).unwrap();

        // Decode twice to match the encryption
        let base64_decoded = STANDARD.decode(encrypted_value).unwrap();
        let encrypted_data = STANDARD.decode(base64_decoded).unwrap();

        let decrypted_data = cipher.decrypt_vec(&encrypted_data).unwrap();
        String::from_utf8(decrypted_data).unwrap()
    }
}

// fn main() {
//     let key = "//htps12345nrfbw"; // 16-byte key
//     let value = "slug=backend-signin&timestamp=1731306997920";

//     // Encrypt
//     let encrypted = SecurityService::set(key, value);
//     println!("Encrypted: {}", encrypted);

//     // Decrypt
//     let decrypted = SecurityService::get(key, &encrypted);
//     println!("Decrypted: {}", decrypted);
// }