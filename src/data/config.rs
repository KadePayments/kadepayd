use std::collections::HashMap;
use std::{env, fs};

pub struct Config {
    pub db_url: String,
    pub db_user: String,
    pub db_password: String,
}

impl Config {
    pub fn new() -> Self {
        let local_secrets = read_local_secrets();
        let db_url = env::var("DB_URL")
            .ok()
            .or_else(|| local_secrets.get("db_url").cloned())
            .expect("Missing DB_URL environment variable or db_url in secrets");
        let db_user = env::var("DB_USER")
            .ok()
            .or_else(|| local_secrets.get("db_user").cloned())
            .expect("Missing DB_USER environment variable or db_user in secrets");
        let db_password = env::var("DB_PASSWORD")
            .ok()
            .or_else(|| local_secrets.get("db_password").cloned())
            .expect("Missing DB_PASSWORD environment variable or db_password in secrets");
        Self {
            db_url,
            db_user,
            db_password,
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
