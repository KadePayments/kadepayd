use crate::data::config::Config;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;

pub struct Storage {
    pool: Pool<PostgresConnectionManager<MakeTlsConnector>>,
}
impl Storage {
    const MAX_CONNECTIONS: u32 = 13;
    const MIN_IDLE_CONNECTIONS: u32 = 3;

    pub async fn new() -> Result<Storage, String> {
        let config = Config::new();
        let connection_string = format!(
            "host={} user={} password={}",
            config.db_url, config.db_user, config.db_password
        );
        match TlsConnector::builder().build() {
            Ok(tls_connector) => {
                let tls = MakeTlsConnector::new(tls_connector);
                match PostgresConnectionManager::new_from_stringlike(connection_string, tls) {
                    Ok(pool_connection_manager) => {
                        match Pool::builder()
                            .max_size(Self::MAX_CONNECTIONS)
                            .min_idle(Self::MIN_IDLE_CONNECTIONS)
                            .build(pool_connection_manager)
                            .await
                        {
                            Ok(pool) => Ok(Storage { pool }),
                            Err(error) => Err(format!("{}", error)),
                        }
                    }
                    Err(error) => Err(format!("{}", error)),
                }
            }
            Err(error) => Err(format!("{}", error)),
        }
    }

    pub async fn init(&self, create_table_commands: &[&str]) {
        for create_table_sql in create_table_commands {
            match self.pool.get().await {
                Ok(connection) => match connection.prepare(create_table_sql).await {
                    Ok(statement) => match connection.execute(&statement, &[]).await {
                        Ok(_) => {
                            println!("Successfully created new table in the database");
                        }
                        Err(error) => {
                            println!("Failed to create table in database: {}", error);
                        }
                    },
                    Err(error) => {
                        println!("Failed to prepare sql statement: {}", error);
                    }
                },
                Err(error) => {
                    println!(
                        "Failed to get connection from database connection pool: {}",
                        error
                    );
                }
            }
        }
    }

    pub async fn query(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, String> {
        match self.pool.get().await {
            Ok(connection) => match connection.prepare(sql).await {
                Ok(statement) => {
                    let result = connection.query(&statement, &params).await;
                    match result {
                        Ok(rows) => Ok(rows),
                        Err(error) => Err(format!("Failed to execute query: {}", error)),
                    }
                }
                Err(error) => Err(format!("Failed to prepare sql statement: {}", error)),
            },
            Err(error) => Err(format!(
                "Failed to get connection from database connection pool: {}",
                error
            )),
        }
    }

    pub async fn query_one(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, String> {
        match self.pool.get().await {
            Ok(connection) => match connection.prepare(sql).await {
                Ok(statement) => {
                    let result = connection.query_one(&statement, &params).await;
                    match result {
                        Ok(row) => Ok(row),
                        Err(error) => Err(format!("Failed to execute query: {}", error)),
                    }
                }
                Err(error) => Err(format!("Failed to prepare sql statement: {}", error)),
            },
            Err(error) => Err(format!(
                "Failed to get connection from database connection pool: {}",
                error
            )),
        }
    }

    pub async fn execute(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, String> {
        match self.pool.get().await {
            Ok(connection) => match connection.prepare(sql).await {
                Ok(statement) => {
                    let result = connection.execute(&statement, params).await;
                    match result {
                        Ok(rows) => Ok(rows),
                        Err(e) => Err(format!("Failed to execute query: {}", e)),
                    }
                }
                Err(error) => Err(format!("Failed to prepare sql statement: {}", error)),
            },
            Err(error) => Err(format!(
                "Failed to get connection from database connection pool: {}",
                error
            )),
        }
    }
}
