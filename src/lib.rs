//! # Basic usage
//!
//! ```
//! use std::path::PathBuf;
//! use diskus::{Walk, FilesizeType};
//!
//! let num_threads = 4;
//! let root_directories = &[PathBuf::from(".")];
//! let walk = Walk::new(root_directories, num_threads, FilesizeType::DiskUsage);
//! let (size_in_bytes, errors) = walk.run();
//! ```

mod filesize;
mod unique_id;
pub mod walk;

pub use crate::filesize::FilesizeType;
pub use crate::walk::{Error, Walk};
