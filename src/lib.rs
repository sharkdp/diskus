extern crate crossbeam_channel;
extern crate humansize;
extern crate num_cpus;
extern crate rayon;

pub mod walk;

/// Basic usage of diskus library
///
/// # Examples
///
/// ```
/// extern crate diskus;
/// 
/// use diskus::Walk;
/// use std::path::PathBuf;
/// 
/// let path = PathBuf::from("/");
/// let paths = vec![path];
/// let walk = Walk::new(&paths, 4);
/// let size = walk.run();
/// ```
pub use walk::Walk;