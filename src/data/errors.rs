use std::error::Error;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug)]
pub struct StorageError {
    pub message: String,
}

impl StorageError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl Error for StorageError {}
