#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("local_data_dir not found")]
    LocalDataDirNotFound,

    #[error("Cannot create app data directory")]
    DataDirCreationFailed,

    #[error("A database error occurred")]
    DatabaseQueryError(#[from] diesel::result::Error),

    #[error("Database connection failed")]
    DatabaseConnectionError(#[from] diesel::result::ConnectionError),
}

// we must manually implement serde::Serialize
impl serde::Serialize for MyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
