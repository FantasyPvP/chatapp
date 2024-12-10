#[derive(Debug)]
pub enum BackendError {
    DbError(String),
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BackendError::DbError(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for BackendError {}

impl From<surrealdb::Error> for BackendError {
    fn from(err: surrealdb::Error) -> Self {
        BackendError::DbError(err.to_string())
    }
}