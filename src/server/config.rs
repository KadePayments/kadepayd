use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::{env, fs};

pub struct Config {
    pub(crate) kadepay_invoices_server_addr: SocketAddr,
    pub(crate) kadepay_db_host: String,
    pub(crate) kadepay_db_url: String,
    pub(crate) kadepay_db_user: String,
    pub(crate) kadepay_db_password: String,
    pub(crate) kadepay_db_name: String,
}

impl Config {
    const KADEPAY_HOST_KEY: &'static str = "KADEPAY_HOST";
    const KADEPAY_INVOICES_PORT_KEY: &'static str = "KADEPAY_INVOICES_PORT";
    const KADEPAY_DB_HOST_KEY: &'static str = "KADEPAY_DB_HOST";
    const KADEPAY_DB_URL_KEY: &'static str = "KADEPAY_DB_URL";
    const KADEPAY_DB_USER_KEY: &'static str = "KADEPAY_DB_USER";
    const KADEPAY_DB_PASSWORD_KEY: &'static str = "KADEPAY_DB_PASSWORD";
    const KADEPAY_DB_NAME_KEY: &'static str = "KADEPAY_DB_NAME";

    pub fn new() -> Config {
        let local_secrets = read_local_secrets();

        let host = env::var(Self::KADEPAY_HOST_KEY).unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var(Self::KADEPAY_INVOICES_PORT_KEY)
            .ok()
            .or_else(|| local_secrets.get(Self::KADEPAY_INVOICES_PORT_KEY).cloned())
            .expect("Missing KADEPAY_INVOICES_PORT in environment variables or secrets");
        let server_url = format!("{}:{}", host, port);
        let kadepay_invoices_server_addr = match server_url.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => panic!("Invalid server url: {}", server_url),
        };

        let db_host_option = env::var(Self::KADEPAY_DB_HOST_KEY)
            .ok()
            .or_else(|| local_secrets.get(Self::KADEPAY_DB_HOST_KEY).cloned());
        let db_url_option = env::var(Self::KADEPAY_DB_URL_KEY)
            .ok()
            .or_else(|| local_secrets.get(Self::KADEPAY_DB_URL_KEY).cloned());

        if db_host_option.is_none() && db_url_option.is_none() {
            panic!("Database host or url must be set");
        }

        let db_host = db_host_option.unwrap_or_default();
        let db_url = db_url_option.unwrap_or_default();

        let db_user = env::var(Self::KADEPAY_DB_USER_KEY)
            .ok()
            .or_else(|| local_secrets.get(Self::KADEPAY_DB_USER_KEY).cloned())
            .expect("Missing KADEPAY_DB_USER in environment variables or secrets");
        let db_password = env::var(Self::KADEPAY_DB_PASSWORD_KEY)
            .ok()
            .or_else(|| local_secrets.get(Self::KADEPAY_DB_PASSWORD_KEY).cloned())
            .expect("Missing KADEPAY_DB_PASSWORD in environment variables or secrets");
        let db_name = env::var(Self::KADEPAY_DB_NAME_KEY)
            .ok()
            .or_else(|| local_secrets.get(Self::KADEPAY_DB_NAME_KEY).cloned())
            .expect("Missing KADEPAY_DB_NAME in environment variables or secrets");

        Config {
            kadepay_invoices_server_addr,
            kadepay_db_host: db_host,
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
    let secrets_file = Path::new(".secrets");

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
    secrets
}
