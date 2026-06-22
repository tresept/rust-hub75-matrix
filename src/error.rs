#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid matrix configuration: {0}")]
    InvalidConfig(String),
    #[error("matrix initialization failed: {0}")]
    Initialization(String),
    #[error("RGB buffer length mismatch: expected {expected} bytes, got {actual}")]
    BufferLengthMismatch { expected: usize, actual: usize },
    #[error("brightness must be between 0 and 100, got {0}")]
    InvalidBrightness(u8),
    #[error("matrix dimensions overflow")]
    DimensionOverflow,
    #[error("value does not fit into the native integer type: {field}")]
    IntegerConversion { field: &'static str },
    #[error("string contains an interior NUL byte: {field}")]
    InvalidCString { field: &'static str },
    #[error("native matrix backend returned error code {0}")]
    Native(i32),
}

pub type Result<T> = std::result::Result<T, Error>;
