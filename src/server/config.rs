use std::collections::HashMap;
use std::net::SocketAddr;
use std::{env, fs};

pub struct Config {
    pub kadepay_invoice_server_addr: SocketAddr,
    pub kadepay_db_url: String,
    pub kadepay_db_user: String,
    pub kadepay_db_password: String,
    pub kadepay_db_name: String,
}

impl Config {
    pub fn new() -> Config {
        let local_secrets = read_local_secrets();

        let host = env::var("KADEPAY_URL").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("KADEPAY_PORT").unwrap_or_else(|_| "50051".to_string());
        let server_url = format!("{}:{}", host, port);
        let kade_invoice_server_addr = match server_url.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => panic!("Invalid server url: {}", server_url),
        };

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

        Config {
            kadepay_invoice_server_addr: kade_invoice_server_addr,
            kadepay_db_url: db_url,
            kadepay_db_user: db_user,
            kadepay_db_password: db_password,
            kadepay_db_name: db_name,
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
