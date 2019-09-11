//! # Basic usage
//!
//! ```
//! extern crate diskus;
//!
//! use std::path::PathBuf;
//! use diskus::Walk;
//!
//! let num_threads = 4;
//! let root_directories = &[PathBuf::from(".")];
//! let walk = Walk::new(root_directories, num_threads);
//! let size_in_bytes = walk.run();
//! ```

extern crate crossbeam_channel;
extern crate crossbeam_utils;

pub mod walk;

pub use walk::Walk;
