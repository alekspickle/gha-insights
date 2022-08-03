pub mod cli;
pub mod defs;
pub mod error;
#[cfg(feature = "server")]
pub mod server;

pub use cli::*;
pub use defs::*;
pub use error::*;
#[cfg(feature = "server")]
pub use server::*;

pub use octocrab::Octocrab;

pub type GenericResult<T = (), E = Box<dyn std::error::Error + Send + Sync>> =
    std::result::Result<T, E>;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
