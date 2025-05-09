use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub service: String,
    pub email: String,
    pub password: String,
    pub description: String,
    pub show_password: bool,
    pub edit_mode: bool,
}

pub fn get_test_entries() -> Vec<Entry> {
    vec![
        Entry {
            service: String::from("Google"),
            email: String::from("user@example.com"),
            password: String::from("P@ssw0rd123"),
            description: String::from("Personal Google account"),
            show_password: false,
            edit_mode: false,
        },
        Entry {
            service: String::from("GitHub"),
            email: String::from("dev@example.com"),
            password: String::from("C0d3r#2023"),
            description: String::from("Work GitHub account"),
            show_password: false,
            edit_mode: false,
        },
        Entry {
            service: String::from("Netflix"),
            email: String::from("movies@example.com"),
            password: String::from("Str3@ming!"),
            description: String::from("Family Netflix subscription"),
            show_password: false,
            edit_mode: false,
        },
        Entry {
            service: String::from("Amazon"),
            email: String::from("shop@example.com"),
            password: String::from("Buy$tuff2023"),
            description: String::from(""),
            show_password: false,
            edit_mode: false,
        },
    ]
}
