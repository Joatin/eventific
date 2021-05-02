


#[derive(Debug, thiserror::Error)]
pub enum PostgresStoreError {
    #[error("Something went wrong with the client connection, internal error was {0}")]
    ClientError(#[source] tokio_postgres::Error),
    #[error("Failed to create event store table, internal error was {0}")]
    CreateTableError(#[source] tokio_postgres::Error),
    #[error("Failed to serialize data structure, internal error was {0}")]
    SerializationError(#[source] serde_json::error::Error),
    #[error("internal error was {0}")]
    PoolError(#[source] bb8::RunError<tokio_postgres::Error>)
}
