//! [![v](https://img.shields.io/badge/v-0.2.0-blueviolet)]()
//!
//! # Overview
//! The project that is motivated by the fact that github actions system lacks
//! some brief consize representation.
//! It aims to provide wrapper CLI or just library for interacting with Github Actions API.
//!
//! TODO: server binary to run and account for all failures
//! across workflows, runs and jobs in one repo.
//!
//!
//! # Status
//! WIP

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
