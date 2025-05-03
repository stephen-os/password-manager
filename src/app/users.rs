use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub vault_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    pub users: Vec<User>,
    pub path: PathBuf,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: String::new(),
            vault_path: String::new(),
        }
    }
}

impl Users {
    pub fn new() -> Self {
        // Get the platform-specific local data directory
        // e.g., %APPDATA% on Windows, ~/.local/share on Linux
        let base_dir = dirs_next::data_local_dir().unwrap_or_else(|| PathBuf::from("."));

        // Define the full path to the users file
        let path = base_dir.join("vault_manager.users");

        // Ensure the parent directory exists; create it if necessary
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("Failed to create user data directory: {}", e);
                }
            }
        }

        // Try to read existing users from the file if it exists
        let users = if let Ok(data) = std::fs::read_to_string(&path) {
            match serde_json::from_str::<Users>(&data) {
                Ok(parsed_users) => parsed_users.users, // Extract the Vec<User> from the Users struct
                Err(e) => {
                    eprintln!("Failed to deserialize users: {}", e);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        Self { users, path }
    }

    fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(&self) {
            let _ = std::fs::write(&self.path, data);
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
        self.save();
    }
}
