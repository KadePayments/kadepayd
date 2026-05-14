use crate::data::config::Config;
use crate::data::errors::StorageError;
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

    pub async fn new() -> Result<Storage, StorageError> {
        let config = Config::new();
        let connection_string = format!(
            "host={} user={} password={}",
            config.db_url, config.db_user, config.db_password
        );
        let tls_connector = TlsConnector::builder().build().map_err(|error| {
            StorageError::new(format!("Failed to build TLS connector: {}", error))
        })?;
        let tls = MakeTlsConnector::new(tls_connector);
        let pool_connection_manager =
            PostgresConnectionManager::new_from_stringlike(connection_string, tls).map_err(
                |error| StorageError::new(format!("Failed to create pool connection: {}", error)),
            )?;
        let pool = Pool::builder()
            .max_size(Self::MAX_CONNECTIONS)
            .min_idle(Self::MIN_IDLE_CONNECTIONS)
            .build(pool_connection_manager)
            .await
            .map_err(|error| {
                StorageError::new(format!("Failed to build connection pool: {}", error))
            })?;
        Ok(Storage { pool })
    }

    pub async fn init(&self, create_table_commands: &[&str]) -> Result<(), String> {
        for create_table_sql in create_table_commands {
            let connection = self
                .pool
                .get()
                .await
                .map_err(|error| format!("Error connecting to database: {}", error))?;
            let statement = connection
                .prepare(create_table_sql)
                .await
                .map_err(|error| format!("Error preparing statement from database: {}", error))?;
            connection
                .execute(&statement, &[])
                .await
                .map_err(|error| format!("Error executing statement from database: {}", error))?;
        }
        Ok(())
    }

    pub async fn query(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, String> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|error| format!("Error connecting to database: {}", error))?;
        let statement = connection
            .prepare(sql)
            .await
            .map_err(|error| format!("Error preparing statement from database: {}", error))?;
        let rows = connection
            .query(&statement, &params)
            .await
            .map_err(|error| format!("Error executing statement from database: {}", error))?;
        Ok(rows)
    }

    pub async fn query_one(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, String> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|error| format!("Error connecting to database: {}", error))?;
        let statement = connection
            .prepare(sql)
            .await
            .map_err(|error| format!("Error preparing statement from database: {}", error))?;
        let row = connection
            .query_one(&statement, &params)
            .await
            .map_err(|error| format!("Error executing statement from database: {}", error))?;
        Ok(row)
    }

    pub async fn execute(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, String> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|error| format!("Error connecting to database: {}", error))?;
        let statement = connection
            .prepare(sql)
            .await
            .map_err(|error| format!("Error preparing statement from database: {}", error))?;
        let number_of_rows = connection
            .execute(&statement, params)
            .await
            .map_err(|error| format!("Error executing statement from database: {}", error))?;
        Ok(number_of_rows)
    }
}
