use std::collections::HashMap;
use std::{env, fs};

pub struct Config {
    pub db_url: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}

impl Config {
    pub fn new() -> Self {
        let local_secrets = read_local_secrets();
        let db_url = env::var("KADEPAY_DB_URL")
            .ok()
            .or_else(|| local_secrets.get("kadepay_db_url").cloned())
            .expect("Missing KADEPAY_DB_URL environment variable or kadepay_db_url in secrets");
        let db_user = env::var("KADEPAY_DB_USER")
            .ok()
            .or_else(|| local_secrets.get("kadepay_db_user").cloned())
            .expect("Missing KADEPAY_DB_USER environment variable or kadepay_db_user in secrets");
        let db_password = env::var("KADEPAY_DB_PASSWORD")
            .ok()
            .or_else(|| local_secrets.get("kadepay_db_password").cloned())
            .expect("Missing KADEPAY_DB_PASSWORD environment variable or kadepay_db_password in secrets");
        let db_name = env::var("KADEPAY_DB_NAME")
            .ok()
            .or_else(|| local_secrets.get("kadepay_db_name").cloned())
            .expect("Missing KADEPAY_DB_NAME environment variable or kadepy_db_name in secrets");
        Self {
            db_url,
            db_user,
            db_password,
            db_name,
        }
    }
}

fn read_local_secrets() -> HashMap<String, String> {
    let mut secrets = HashMap::new();
    // The .secrets file should not be commited to any version control system
    match env::current_dir() {
        Ok(cwd) => {
            let secrets_file = cwd.join(".secrets");

            if !secrets_file.exists() {
                return secrets;
            }

            match fs::read_to_string(secrets_file) {
                Ok(contents) => {
                    for line in contents.lines() {
                        if line.is_empty() {
                            continue;
                        }
                        let Some((key, value)) = line.split_once("=") else {
                            eprintln!("Invalid secret line");
                            continue;
                        };
                        secrets.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
                Err(error) => {
                    eprintln!("Error reading secrets file: {}", error);
                }
            }
        }
        Err(error) => {
            eprintln!(
                "Error reading secrets file: {}. Secrets will be empty",
                error
            );
        }
    }
    secrets
}
