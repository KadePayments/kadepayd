use bb8::RunError;
use std::fmt::{Display, Formatter, Write};
use tokio_postgres::Error;
use tokio_postgres::error::SqlState;
use tonic::Status;

#[derive(Debug)]
pub enum StorageError {
    DatabaseError(Error),
    PoolTimeoutError,
    TLSError(native_tls::Error),
    EmbeddedDatabaseError(postgresql_embedded::Error),
}

impl std::error::Error for StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl From<RunError<Error>> for StorageError {
    fn from(value: RunError<Error>) -> Self {
        match value {
            RunError::User(error) => StorageError::DatabaseError(error.into()),
            RunError::TimedOut => StorageError::PoolTimeoutError,
        }
    }
}

impl From<Error> for StorageError {
    fn from(value: Error) -> Self {
        StorageError::DatabaseError(value)
    }
}

impl From<native_tls::Error> for StorageError {
    fn from(value: native_tls::Error) -> Self {
        StorageError::TLSError(value.into())
    }
}

impl From<postgresql_embedded::Error> for StorageError {
    fn from(value: postgresql_embedded::Error) -> Self {
        StorageError::EmbeddedDatabaseError(value)
    }
}

pub fn handle_storage_error(error: StorageError, message: &str) -> Status {
    match error {
        StorageError::DatabaseError(error) => {
            eprintln!("{}", error);
            if error.code() == Some(&SqlState::UNIQUE_VIOLATION) {
                return Status::already_exists(message);
            }
            Status::internal("Internal server error")
        }
        StorageError::PoolTimeoutError => {
            eprintln!("Pool connection timed out");
            Status::internal("Internal server error")
        }
        StorageError::TLSError(error) => {
            eprintln!("{}", error);
            Status::internal("Internal server error")
        }
        StorageError::EmbeddedDatabaseError(error) => {
            eprintln!("{}", error);
            Status::internal("Internal server error")
        }
    }
}
