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
    pub fn new() -> Config {
        let local_secrets = read_local_secrets();

        let host = env::var("KADEPAY_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("KADEPAY_INVOICES_PORT")
            .ok()
            .or_else(|| local_secrets.get("kadepay_invoices_port").cloned())
            .expect("Missing KADEPAY_INVOICES_PORT environment variable or kadepay_invoices_port in secrets");
        let server_url = format!("{}:{}", host, port);
        let kadepay_invoices_server_addr = match server_url.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => panic!("Invalid server url: {}", server_url),
        };

        let mut db_host: String = "".to_string();
        let mut db_url: String = "".to_string();
        let db_host_option = env::var("KADEPAY_DB_HOST")
            .ok()
            .or_else(|| local_secrets.get("kadepay_db_host").cloned());
        let db_url_option = env::var("KADEPAY_DB_URL")
            .ok()
            .or_else(|| local_secrets.get("kadepay_db_url").cloned());

        if db_host_option == None && db_url_option == None {
            panic!("Database host or url must be set");
        } else {
            db_host = db_host_option.unwrap_or_else(|| db_host);
            db_url = db_url_option.unwrap_or_else(|| db_url);
        }

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
