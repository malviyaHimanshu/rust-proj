use serde::{Deserialize, Serialize};
use std::fs;

use crate::{encrypt_password, decrypt_password};

// data file where passwords will be stored
const DATA_FILE: &str = "password.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordManager {
    pub entries: Vec<PasswordEntry>,
}

impl PasswordManager {
    pub fn new() -> Self {
        PasswordManager { entries: Vec::new() }
    }

    // function to get the data from password file to model
    pub fn load() -> Self {
        let data = fs::read_to_string(DATA_FILE).unwrap_or_else(|_| {
            String::from("{}")
        });
        serde_json::from_str(&data).unwrap_or_else(|_| PasswordManager::new())
    }

    // write data from model to password file
    pub fn save(&self) {
        let data = serde_json::to_string(&self).unwrap();
        fs::write(DATA_FILE, data).expect("unable to write file.");
    }

    pub fn add_entry(&mut self, service: String, username: String, password: String) {
        let encrypted_password = encrypt_password(&password);
        let entry = PasswordEntry { service, username, password: encrypted_password };
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, service: String) {
        self.entries.retain(|entry| entry.service != service);
    }

    pub fn list_entries(&self) {
        println!("Service\t\t\tUsername\t\t\tPassword");
        for entry in &self.entries {
            println!("{}\t\t\t{}\t\t\t{}", entry.service, entry.username, decrypt_password(&entry.password));
        }
    }
}