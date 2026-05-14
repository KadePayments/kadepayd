pub struct StorageError {
    pub message: String,
}

impl StorageError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
