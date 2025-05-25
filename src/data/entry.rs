use serde::{Deserialize, Serialize};

use crate::utils::crypt;

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub service: String,
    pub email: String,
    pub password: String,
    pub description: String,
    pub show_password: bool,
    pub edit_mode: bool,
    pub should_delete: bool,
}

// Default entry
impl Default for Entry {
    fn default() -> Self {
        Entry {
            service: "Example Service".to_string(),
            email: "example@gmail.com".to_string(),
            password: "password123".to_string(),
            description: "This is a sample entry.".to_string(),
            show_password: false,
            edit_mode: false,
            should_delete: false,
        }
    }
}

/// Save vault entries to an encrypted file
pub fn save_entries(entries: &Vec<Entry>, vault_name: &str, password: &str) -> Result<(), String> {
    // Get path to the vault file
    let path = get_vault_path(vault_name);

    // Ensure parent directories exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create vault directory: {}", e))?;
    }

    // Create a plaintext buffer with a magic header and JSON body
    let mut plaintext = Vec::new();

    // Write magic header to buffer
    writeln!(&mut plaintext, "MAGIC")
        .map_err(|e| format!("Failed to write MAGIC header: {}", e))?;

    // Serialize entries to JSON and append to buffer
    serde_json::to_writer(&mut plaintext, entries)
        .map_err(|e| format!("Failed to serialize vault entries: {}", e))?;

    // Encrypt the buffer
    let encrypted_data = crypt::encrypt(&plaintext, password)
        .map_err(|e| format!("Failed to encrypt vault data: {}", e))?;

    // Write encrypted data to file
    fs::write(path, encrypted_data)
        .map_err(|e| format!("Failed to write encrypted vault file: {}", e))?;

    Ok(())
}

pub fn load_entries(vault_name: &str, password: &str) -> Result<Vec<Entry>, String> {
    // Get path to vault file
    let path = get_vault_path(vault_name);

    // Read encrypted contents
    let encrypted_data =
        fs::read(&path).map_err(|e| format!("Failed to read vault file: {}", e))?;

    // Attempt to decrypt the contents
    let decrypted_data = crypt::decrypt(&encrypted_data, password)
        .map_err(|_| "Invalid password or corrupted vault.".to_string())?;

    // Use BufReader for reading decrypted content
    let mut reader = BufReader::new(&decrypted_data[..]);

    // Check for MAGIC header to validate password
    let mut magic_line = String::new();
    reader
        .read_line(&mut magic_line)
        .map_err(|_| "Vault is corrupted (failed to read header).".to_string())?;

    if magic_line.trim() != "MAGIC" {
        return Err("Invalid password.".to_string());
    }

    // Deserialize JSON to entries
    let entries: Vec<Entry> = serde_json::from_reader(reader)
        .map_err(|_| "Vault is corrupted (invalid entry data).".to_string())?;

    Ok(entries)
}

/// Compute vault file path based on vault name
fn get_vault_path(vault_name: &str) -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("password-manager/vaults");
    path.push(format!("{}.vault", vault_name));
    path
}
