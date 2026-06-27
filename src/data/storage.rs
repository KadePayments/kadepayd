use crate::data::errors::StorageError;
use crate::server::config::Config;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use postgresql_embedded::{PostgreSQL, Settings};
use tokio_postgres::types::ToSql;
use tokio_postgres::{Row, Transaction};

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
                    config.kadepay_db_host,
                    config.kadepay_db_user,
                    config.kadepay_db_password,
                    config.kadepay_db_name
                ),
                None,
            )
        };
        let tls_connector = TlsConnector::builder().build()?;
        let tls = MakeTlsConnector::new(tls_connector);
        let pool_connection_manager =
            PostgresConnectionManager::new_from_stringlike(connection_string, tls)?;
        let pool = Pool::builder()
            .max_size(Self::MAX_CONNECTIONS)
            .min_idle(Self::MIN_IDLE_CONNECTIONS)
            .build(pool_connection_manager)
            .await?;
        Ok(Storage { pool, db_process })
    }

    // This is an in-memory database for testing
    async fn create_embedded_db() -> Result<(String, PostgreSQL), StorageError> {
        let settings = Settings::new();
        let mut postgresql = PostgreSQL::new(settings);

        postgresql.setup().await?;
        postgresql.start().await?;

        let database_name = "kade_test_db";
        postgresql.create_database(database_name).await?;
        let connection_string = postgresql.settings().url(database_name);
        Ok((connection_string, postgresql))
    }

    pub async fn init(&self, create_table_commands: &[&str]) -> Result<(), StorageError> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;

        for create_table_sql in create_table_commands {
            transaction.batch_execute(&create_table_sql).await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    pub async fn query(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, StorageError> {
        let connection = self.pool.get().await?;
        let statement = connection.prepare(sql).await?;
        let rows = connection.query(&statement, params).await?;
        Ok(rows)
    }

    pub async fn query_one(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, StorageError> {
        let connection = self.pool.get().await?;
        let statement = connection.prepare(sql).await?;
        let row = connection.query_one(&statement, params).await?;
        Ok(row)
    }

    pub async fn query_opt(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, StorageError> {
        let connection = self.pool.get().await?;
        let statement = connection.prepare(sql).await?;
        let row = connection.query_opt(&statement, params).await?;
        Ok(row)
    }

    pub async fn execute(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, StorageError> {
        let connection = self.pool.get().await?;
        let statement = connection.prepare(sql).await?;
        let number_of_rows = connection.execute(&statement, params).await?;
        Ok(number_of_rows)
    }

    pub async fn tx_query(
        tx: &Transaction<'_>,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, StorageError> {
        let statement = tx.prepare(sql).await?;
        let rows = tx.query(&statement, params).await?;
        Ok(rows)
    }

    pub async fn tx_query_one(
        tx: &Transaction<'_>,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, StorageError> {
        let statement = tx.prepare(sql).await?;
        let row = tx.query_one(&statement, params).await?;
        Ok(row)
    }

    pub async fn tx_query_opt(
        tx: &Transaction<'_>,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, StorageError> {
        let statement = tx.prepare(sql).await?;
        let row = tx.query_opt(&statement, params).await?;
        Ok(row)
    }

    pub async fn tx_execute(
        tx: &Transaction<'_>,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, StorageError> {
        let statement = tx.prepare(sql).await?;
        let number_of_rows = tx.execute(&statement, params).await?;
        Ok(number_of_rows)
    }

    pub async fn tx_commit(tx: Transaction<'_>) -> Result<(), StorageError> {
        tx.commit().await?;
        Ok(())
    }
}
