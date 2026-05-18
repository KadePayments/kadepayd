use crate::data::errors::StorageError;
use crate::server::config::Config;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use postgresql_embedded::{PostgreSQL, Settings};
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;

#[derive(Debug)]
pub struct Storage {
    pub pool: Pool<PostgresConnectionManager<MakeTlsConnector>>,
    pub db_process: Option<PostgreSQL>,
}

impl Storage {
    const MAX_CONNECTIONS: u32 = 13;
    const MIN_IDLE_CONNECTIONS: u32 = 3;

    pub async fn new(embedded: bool) -> Result<Storage, StorageError> {
        let (connection_string, db_process): (String, Option<PostgreSQL>) = if embedded {
            let (conn_s, db_p) = Self::create_embedded_db().await?;
            (conn_s, Some(db_p))
        } else {
            let config = Config::new();
            (
                format!(
                    "host={} user={} password={} dbname={}",
                    config.kadepay_db_url,
                    config.kadepay_db_user,
                    config.kadepay_db_password,
                    config.kadepay_db_name
                ),
                None,
            )
        };
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
        Ok(Storage { pool, db_process })
    }

    // This is an in-memory database for testing
    async fn create_embedded_db() -> Result<(String, PostgreSQL), StorageError> {
        let settings = Settings::new();
        let mut postgresql = PostgreSQL::new(settings);

        postgresql
            .setup()
            .await
            .map_err(|error| StorageError::new(format!("Failed to setup database: {}", error)))?;
        postgresql
            .start()
            .await
            .map_err(|error| StorageError::new(format!("Failed to start database: {}", error)))?;

        let database_name = "kade_test_db";
        postgresql
            .create_database(database_name)
            .await
            .map_err(|error| StorageError::new(format!("Failed to create database: {}", error)))?;
        let connection_string = postgresql.settings().url(database_name);
        Ok((connection_string, postgresql))
    }

    pub async fn init(&self, create_table_commands: &[&str]) -> Result<(), StorageError> {
        let mut connection = self.pool.get().await.map_err(|error| {
            StorageError::new(format!("Error connecting to database: {}", error))
        })?;
        let transaction = connection.transaction().await.map_err(|error| {
            StorageError::new(format!("Failed to start transaction: {}", error))
        })?;

        for create_table_sql in create_table_commands {
            transaction
                .batch_execute(&create_table_sql)
                .await
                .map_err(|error| StorageError::new(format!("Failed to create table: {}", error)))?;
        }
        transaction.commit().await.map_err(|error| {
            StorageError::new(format!("Failed to commit transaction: {}", error))
        })?;
        Ok(())
    }

    pub async fn query(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, StorageError> {
        let connection = self.pool.get().await.map_err(|error| {
            StorageError::new(format!("Error connecting to database: {}", error))
        })?;
        let statement = connection.prepare(sql).await.map_err(|error| {
            StorageError::new(format!(
                "Error preparing statement from database: {}",
                error
            ))
        })?;
        let rows = connection
            .query(&statement, params)
            .await
            .map_err(|error| {
                StorageError::new(format!(
                    "Error executing statement from database: {}",
                    error
                ))
            })?;
        Ok(rows)
    }

    pub async fn query_one(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, StorageError> {
        let connection = self.pool.get().await.map_err(|error| {
            StorageError::new(format!("Error connecting to database: {}", error))
        })?;
        let statement = connection.prepare(sql).await.map_err(|error| {
            StorageError::new(format!(
                "Error preparing statement from database: {}",
                error
            ))
        })?;
        let row = connection
            .query_one(&statement, params)
            .await
            .map_err(|error| {
                StorageError::new(format!(
                    "Error executing statement from database: {}",
                    error
                ))
            })?;
        Ok(row)
    }

    pub async fn execute(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, StorageError> {
        let connection = self.pool.get().await.map_err(|error| {
            StorageError::new(format!("Error connecting to database: {}", error))
        })?;
        let statement = connection.prepare(sql).await.map_err(|error| {
            StorageError::new(format!(
                "Error preparing statement from database: {}",
                error
            ))
        })?;
        let number_of_rows = connection
            .execute(&statement, params)
            .await
            .map_err(|error| {
                StorageError::new(format!(
                    "Error executing statement from database: {}",
                    error
                ))
            })?;
        Ok(number_of_rows)
    }
}
