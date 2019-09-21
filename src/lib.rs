//! # Basic usage
//!
//! ```
//! use std::path::PathBuf;
//! use diskus::Walk;
//!
//! let num_threads = 4;
//! let root_directories = &[PathBuf::from(".")];
//! let walk = Walk::new(root_directories, num_threads);
//! let size_in_bytes = walk.run();
//! ```

pub mod walk;

pub use crate::walk::Walk;
