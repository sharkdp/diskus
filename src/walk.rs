use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::thread;

use crossbeam_channel as channel;

use rayon::prelude::*;

#[derive(Eq,PartialEq,Hash)]
struct UniqueID(u64, u64);

struct SizeEntry(Option<UniqueID>, u64);

fn walk(tx: channel::Sender<SizeEntry>, entries: &[PathBuf]) {
    entries.into_par_iter().for_each_with(tx, |tx_ref, entry| {
        if let Ok(metadata) = entry.symlink_metadata() {
            // If the entry has more than one hard link, generate
            // a unique ID consisting of device and inode in order
            // not to count this entry twice.
            let unique_id = if metadata.is_file() && metadata.nlink() > 1 {
                Some(UniqueID(metadata.dev(), metadata.ino()))
            } else {
                None
            };

            let size = metadata.len();

            tx_ref.send(SizeEntry(unique_id, size)).unwrap();

            if metadata.is_dir() {
                let mut children = vec![];
                for child_entry in fs::read_dir(entry).unwrap() {
                    if let Ok(child_entry) = child_entry {
                        children.push(child_entry.path());
                    }
                }

                walk(tx_ref.clone(), &children[..]);
            };
        } else {
            eprintln!("Could not get file metadata: '{}'", entry.to_string_lossy());
        };
    });
}

pub struct Walk<'a> {
    root_directories: &'a [PathBuf],
    threads: usize,
}

impl<'a> Walk<'a> {
    pub fn new(root_directories: &'a [PathBuf], threads: usize) -> Walk {
        Walk {
            root_directories,
            threads,
        }
    }

    pub fn run(&self) -> u64 {
        let (tx, rx) = channel::unbounded();

        let receiver_thread = thread::spawn(move || {
            let mut total = 0;
            let mut ids = HashSet::new();
            for SizeEntry(unique_id, size) in rx {
                if let Some(unique_id) = unique_id {
                    // Only count this entry if the ID has not been seen
                    if ids.insert(unique_id) {
                        total += size;
                    }
                } else {
                    total += size;
                }
            }

            total
        });

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()
            .unwrap();
        pool.install(|| walk(tx, self.root_directories));

        receiver_thread.join().unwrap()
    }
}
