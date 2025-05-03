use crate::app::crypt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::Path;

use super::entry::Entry;

const MAGIC: &str = "MAGIC";

#[derive(Serialize, Deserialize)]
struct VaultFile {
    magic: String,
    entries: Vec<Entry>,
}

pub fn load(path: &str, name: &str, password: &str) -> Result<Vec<Entry>, String> {
    let file_path = Path::new(path).join(format!("{name}.vault"));
    let file_path_str = file_path
        .to_str()
        .ok_or("Failed to construct vault file path")?;

    let decrypted_data = crypt::load_encrypted(file_path_str, password)?;

    let vault: VaultFile = serde_json::from_slice(&decrypted_data)
        .map_err(|e| format!("Failed to parse vault: {}", e))?;

    if vault.magic != MAGIC {
        return Err("Invalid vault file or incorrect password".to_string());
    }

    Ok(vault.entries)
}

/// Saves entries to a vault file, encrypting them with the given password.
pub fn save(path: &str, name: &str, password: &str, entries: &[Entry]) -> Result<(), String> {
    let file_path = Path::new(path).join(format!("{name}.vault"));
    let file_path_str = file_path
        .to_str()
        .ok_or("Failed to construct vault file path")?;

    let vault = VaultFile {
        magic: MAGIC.to_string(),
        entries: entries.to_vec(),
    };

    let json_data =
        serde_json::to_vec(&vault).map_err(|e| format!("Failed to serialize vault: {}", e))?;

    crypt::save_encrypted(&json_data, file_path_str, password)
}
