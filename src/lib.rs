//! RONF is a configuration library based on [config-rs](https://github.com/rust-cli/config-rs).

mod config;
mod error;
mod file;
mod format;
mod value;

pub use crate::config::{Config, ConfigBuilder};
pub use crate::file::{File, FileFormat};
pub use crate::value::Value;
