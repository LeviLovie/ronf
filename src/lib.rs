//! RONF is a configuration library based on [config-rs](https://github.com/rust-cli/config-rs).
//!
//! Configuration is stored in a `Config` structure. It can be created using a builder.
//! ```rust
//! use ronf::Config;
//! let config = Config::builder().build().unwrap();
//! ```
//!
//! On the builder there is a function `add_file(file: File)` which adds a file to read
//! configuration from.
//! File can be created with `ronf::File::new("config.json", ronf::FileFormat::Json,
//! "{\"key\":\"value\"")` or loaded from disk if `read_file` feature is enabled.
//! ```rust
//! use ronf::{Config, File, FileFormat};
//! let file: File = File::new_str(
//!     "config.json",
//!     ronf::FileFormat::Json,
//!     "{\"key\":\"value\"}",
//! );
//! let config = Config::builder()
//!     .add_file(file)
//!     .build()
//!     .unwrap();
//! println!("\"key\": {}", config.get("key").unwrap());
//! ```
//!
//! Check `examples/saves.rs` to see how to save changes to a config.

mod config;
pub mod error;
mod file;
mod format;
mod value;

pub use crate::config::{Config, ConfigBuilder};
pub use crate::file::{File, FileFormat};
pub use crate::value::Value;
