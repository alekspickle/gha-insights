use thiserror::Error;

pub type GenericResult<T = (), E = Box<dyn std::error::Error + Send + Sync>> =
    std::result::Result<T, E>;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),

    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },

    #[error("JWT error")]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[error("Service error")]
    Octocrab(#[from] octocrab::Error),

    #[error("unknown data store error")]
    Unknown,
}
