use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::thread;

use crossbeam_channel as channel;

use rayon;
use rayon::prelude::*;

#[derive(Eq, PartialEq, Hash)]
pub struct UniqueID(u64, u64);

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(not(target_os = "windows"))]
pub use self::unix::*;

enum Message {
    SizeEntry(Option<UniqueID>, u64),
    NoMetadataForPath(PathBuf),
    CouldNotReadDir(PathBuf),
}

fn walk(tx: channel::Sender<Message>, entries: &[PathBuf]) {
    entries.into_par_iter().for_each_with(tx, |tx_ref, entry| {
        if let Ok(metadata) = entry.symlink_metadata() {
            let unique_id = generate_unique_id(&metadata);

            let size = metadata.len();

            tx_ref.send(Message::SizeEntry(unique_id, size)).unwrap();

            if metadata.is_dir() {
                let mut children = vec![];
                match fs::read_dir(entry) {
                    Ok(child_entries) => {
                        for child_entry in child_entries {
                            if let Ok(child_entry) = child_entry {
                                children.push(child_entry.path());
                            }
                        }
                    }
                    Err(_) => {
                        tx_ref
                            .send(Message::CouldNotReadDir(entry.clone()))
                            .unwrap();
                    }
                }

                walk(tx_ref.clone(), &children[..]);
            };
        } else {
            tx_ref
                .send(Message::NoMetadataForPath(entry.clone()))
                .unwrap();
        };
    });
}

pub struct Walk<'a> {
    root_directories: &'a [PathBuf],
    num_threads: usize,
}

impl<'a> Walk<'a> {
    pub fn new(root_directories: &'a [PathBuf], num_threads: usize) -> Walk {
        Walk {
            root_directories,
            num_threads,
        }
    }

    pub fn run(&self) -> u64 {
        let (tx, rx) = channel::unbounded();

        let receiver_thread = thread::spawn(move || {
            let mut total = 0;
            let mut ids = HashSet::new();
            for msg in rx {
                match msg {
                    Message::SizeEntry(unique_id, size) => {
                        if let Some(unique_id) = unique_id {
                            // Only count this entry if the ID has not been seen
                            if ids.insert(unique_id) {
                                total += size;
                            }
                        } else {
                            total += size;
                        }
                    }
                    Message::NoMetadataForPath(path) => {
                        eprintln!(
                            "diskus: could not retrieve metadata for path '{}'",
                            path.to_string_lossy()
                        );
                    }
                    Message::CouldNotReadDir(path) => {
                        eprintln!(
                            "diskus: could not read contents of directory '{}'",
                            path.to_string_lossy()
                        );
                    }
                }
            }

            total
        });

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.num_threads)
            .build()
            .unwrap();
        pool.install(|| walk(tx, self.root_directories));

        receiver_thread.join().unwrap()
    }
}
