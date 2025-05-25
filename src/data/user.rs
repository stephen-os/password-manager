use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use crate::utils::crypt;

const USERS_ENCRYPTION_KEY: &str = "very-secure-hardcoded-key";
const USERS_FILENAME: &str = "users.dat";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub last_accessed: String,
}

impl User {
    pub fn new(name: String) -> Self {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        Self {
            name,
            last_accessed: now,
        }
    }
}

pub fn get_user_data_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("password-manager");

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create data directory");
    }

    path.push(USERS_FILENAME);
    path
}

pub fn load_users() -> Vec<User> {
    let path = get_user_data_path();

    // No users yet, first-time setup
    if !path.exists() {
        return vec![];
    }

    // Open user.dat
    let mut file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open user data file: {}", e);
            return vec![];
        }
    };

    // Load contents of user.dat
    let mut encrypted_data = Vec::new();
    if let Err(e) = file.read_to_end(&mut encrypted_data) {
        eprintln!("Failed to read user data file: {}", e);
        return vec![];
    }

    // Decrypt contents of user.dat
    let decrypted_data = match crypt::decrypt(&encrypted_data, USERS_ENCRYPTION_KEY) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to decrypt user data: {}", e);
            return vec![];
        }
    };

    // Deserialize users
    match serde_json::from_slice::<Vec<User>>(&decrypted_data) {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Failed to parse user data (possibly corrupted): {}", e);
            vec![]
        }
    }
}

pub fn save_users(users: &[User]) {
    // Path to user.dat
    let path = get_user_data_path();

    // Convert users to JSON
    let data = match serde_json::to_vec(users) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to serialize users: {}", e);
            return;
        }
    };

    // Encrypt users
    let encrypted_data = match crypt::encrypt(&data, USERS_ENCRYPTION_KEY) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to encrypt user data: {}", e);
            return;
        }
    };

    // Write encrypted data to file
    if let Err(e) = fs::write(path, encrypted_data) {
        eprintln!("Failed to write encrypted data to file: {}", e);
    }
}
