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
        let db_url = env::var("DB_URL").unwrap_or_else(|_| local_secrets["db_url"].to_string());
        let db_user = env::var("DB_USER").unwrap_or_else(|_| local_secrets["db_user"].to_string());
        let db_password =
            env::var("DB_PASSWORD").unwrap_or_else(|_| local_secrets["db_password"].to_string());
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
                        let parts: Vec<&str> = line.split('=').collect();
                        let key = parts[0];
                        let value = parts[1];
                        secrets.insert(key.to_string(), value.to_string());
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
