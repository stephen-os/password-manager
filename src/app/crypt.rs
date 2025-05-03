use ring::{aead, pbkdf2, rand};
use std::fs;
use std::io::Read;
use std::num::NonZeroU32;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;
const KEY_LEN: usize = 32;
const ITERATIONS: u32 = 100_000;

// Derive an encryption key from a password and salt
fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(ITERATIONS).unwrap(),
        salt,
        password.as_bytes(),
        &mut key,
    );
    key
}

// Generate a random salt
fn generate_salt() -> [u8; SALT_LEN] {
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; SALT_LEN];
    rand::SecureRandom::fill(&rng, &mut salt).expect("Failed to generate random salt");
    salt
}

// Generate a random nonce
fn generate_nonce() -> [u8; NONCE_LEN] {
    let rng = rand::SystemRandom::new();
    let mut nonce = [0u8; NONCE_LEN];
    rand::SecureRandom::fill(&rng, &mut nonce).expect("Failed to generate random nonce");
    nonce
}

// Encrypt data with a password
pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    // Generate a random salt and nonce
    let salt = generate_salt();
    let nonce_bytes = generate_nonce();

    // Derive the encryption key from the password and salt
    let key_bytes = derive_key(password, &salt);

    // Create an UnboundKey from the derived key
    let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key_bytes)
        .map_err(|_| "Failed to create encryption key".to_string())?;

    // Bind the key with the nonce to create a SealingKey
    let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_bytes)
        .map_err(|_| "Failed to create nonce".to_string())?;
    let sealing_key = aead::LessSafeKey::new(unbound_key);

    // Create a buffer for the output: data + authentication tag
    let mut in_out = Vec::with_capacity(data.len() + TAG_LEN);
    in_out.extend_from_slice(data);

    // Seal (encrypt) the data in-place
    sealing_key
        .seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| "Encryption failed".to_string())?;

    // Combine salt, nonce, and encrypted data into a single buffer
    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + in_out.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&in_out);

    Ok(result)
}

// Decrypt data with a password
pub fn decrypt(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    // Check if the data is large enough to contain all necessary components
    if encrypted_data.len() < SALT_LEN + NONCE_LEN + TAG_LEN {
        return Err("Encrypted data is too short".to_string());
    }

    // Extract the salt, nonce, and ciphertext
    let salt = &encrypted_data[..SALT_LEN];
    let nonce_bytes = &encrypted_data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &encrypted_data[SALT_LEN + NONCE_LEN..];

    // Derive the key from the password and salt
    let key_bytes = derive_key(password, salt);

    // Create an UnboundKey from the derived key
    let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key_bytes)
        .map_err(|_| "Failed to create decryption key".to_string())?;

    // Bind the key with the nonce to create an OpeningKey
    let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes)
        .map_err(|_| "Failed to create nonce".to_string())?;
    let opening_key = aead::LessSafeKey::new(unbound_key);

    // Copy the ciphertext to a mutable buffer for in-place decryption
    let mut in_out = ciphertext.to_vec();

    // Open (decrypt) the data in-place
    let decrypted_data = opening_key
        .open_in_place(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| "Decryption failed (invalid password or corrupted data)".to_string())?;

    Ok(decrypted_data.to_vec())
}

// Save encrypted data to a file
pub fn save_encrypted(data: &[u8], path: &str, password: &str) -> Result<(), String> {
    let encrypted_data = encrypt(data, password)?;

    fs::write(path, encrypted_data)
        .map_err(|e| format!("Failed to write encrypted data to file: {}", e))
}

// Load and decrypt data from a file
pub fn load_encrypted(path: &str, password: &str) -> Result<Vec<u8>, String> {
    let mut file = fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    decrypt(&encrypted_data, password)
}
