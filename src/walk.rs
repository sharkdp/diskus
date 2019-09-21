use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::thread;

use crossbeam_channel as channel;

use rayon::{self, prelude::*};

use crate::filesize::FilesizeType;
use crate::unique_id::{generate_unique_id, UniqueID};

pub enum Error {
    NoMetadataForPath(PathBuf),
    CouldNotReadDir(PathBuf),
}

enum Message {
    SizeEntry(Option<UniqueID>, u64),
    Error { error: Error },
}

fn walk(tx: channel::Sender<Message>, entries: &[PathBuf], filesize_type: FilesizeType) {
    entries.into_par_iter().for_each_with(tx, |tx_ref, entry| {
        if let Ok(metadata) = entry.symlink_metadata() {
            let unique_id = generate_unique_id(&metadata);

            let size = filesize_type.size(&metadata);

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
                            .send(Message::Error {
                                error: Error::CouldNotReadDir(entry.clone()),
                            })
                            .unwrap();
                    }
                }

                walk(tx_ref.clone(), &children[..], filesize_type);
            };
        } else {
            tx_ref
                .send(Message::Error {
                    error: Error::NoMetadataForPath(entry.clone()),
                })
                .unwrap();
        };
    });
}

pub struct Walk<'a> {
    root_directories: &'a [PathBuf],
    num_threads: usize,
    filesize_type: FilesizeType,
}

impl<'a> Walk<'a> {
    pub fn new(
        root_directories: &'a [PathBuf],
        num_threads: usize,
        filesize_type: FilesizeType,
    ) -> Walk {
        Walk {
            root_directories,
            num_threads,
            filesize_type,
        }
    }

    pub fn run(&self) -> (u64, Vec<Error>) {
        let (tx, rx) = channel::unbounded();

        let receiver_thread = thread::spawn(move || {
            let mut total = 0;
            let mut ids = HashSet::new();
            let mut error_messages: Vec<Error> = Vec::new();
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
                    Message::Error { error } => {
                        error_messages.push(error);
                    }
                }
            }
            (total, error_messages)
        });

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.num_threads)
            .build()
            .unwrap();
        pool.install(|| walk(tx, self.root_directories, self.filesize_type));

        receiver_thread.join().unwrap()
    }
}
