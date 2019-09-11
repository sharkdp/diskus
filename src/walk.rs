use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crossbeam_channel as channel;
use crossbeam_utils::thread as thread;

#[derive(Eq, PartialEq, Hash)]
struct UniqueID(u64, u64);

enum WalkMessage<'a> {
    Entries(Cow<'a, [PathBuf]>),
    Done,
}

enum CountMessage {
    SizeEntry(Option<UniqueID>, u64),
    FinishedEntries(usize, usize),
    NoMetadataForPath(PathBuf),
    CouldNotReadDir(PathBuf),
}

fn walk<'a>(
    walk_rx: &'a channel::Receiver<WalkMessage>,
    walk_tx: &'a channel::Sender<WalkMessage>,
    count_tx: &'a channel::Sender<CountMessage>,
) {
    for msg in walk_rx {
        match msg {
            WalkMessage::Entries(entries) => {
                let mut count = 0;
                for entry in entries.iter() {
                    count += handle_entry(entry, walk_tx, count_tx);
                }
                count_tx
                    .send(CountMessage::FinishedEntries(entries.len(), count))
                    .unwrap();
            }
            WalkMessage::Done => {
                // Send another "Done" message to ensure all the walkers exit
                walk_tx
                    .send(WalkMessage::Done)
                    .unwrap();
                break;
            }
        }
    }
}

fn handle_entry<'a>(
    entry: &Path,
    walk_tx: &'a channel::Sender<WalkMessage>,
    count_tx: &'a channel::Sender<CountMessage>,
) -> usize {
    let mut count = 0;

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

        count_tx
            .send(CountMessage::SizeEntry(unique_id, size))
            .unwrap();

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
                    count_tx
                        .send(CountMessage::CouldNotReadDir(entry.to_owned()))
                        .unwrap();
                }
            }

            count = children.len();

            walk_tx
                .send(WalkMessage::Entries(Cow::from(children)))
                .unwrap();
        };
    } else {
        count_tx
            .send(CountMessage::NoMetadataForPath(entry.to_owned()))
            .unwrap();
    };

    count
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
        let (count_tx, count_rx) = channel::unbounded();
        let (walk_tx, walk_rx) = channel::unbounded();

        thread::scope(|s| {
            // Ensure we create at least one thread so we don't get stuck
            let thread_count = self.num_threads.max(1);
            for _ in 0..thread_count {
                s.spawn(|_| walk(&walk_rx, &walk_tx, &count_tx));
            }

            let root_entries = Cow::from(self.root_directories);
            walk_tx
                .send(WalkMessage::Entries(root_entries))
                .unwrap();

            let mut total = 0;
            let mut entry_count = self.root_directories.len();
            let mut ids = HashSet::new();
            for msg in &count_rx {
                match msg {
                    CountMessage::SizeEntry(unique_id, size) => {
                        if let Some(unique_id) = unique_id {
                            // Only count this entry if the ID has not been seen
                            if ids.insert(unique_id) {
                                total += size;
                            }
                        } else {
                            total += size;
                        }
                    }
                    CountMessage::FinishedEntries(done, new) => {
                        entry_count = entry_count + new - done;
                        if entry_count == 0 {
                            // We have finished processing everything
                            break;
                        }
                    }
                    CountMessage::NoMetadataForPath(path) => {
                        eprintln!(
                            "diskus: could not retrieve metadata for path '{}'",
                            path.to_string_lossy()
                        );
                    }
                    CountMessage::CouldNotReadDir(path) => {
                        eprintln!(
                            "diskus: could not read contents of directory '{}'",
                            path.to_string_lossy()
                        );
                    }
                }
            }

            walk_tx
                .send(WalkMessage::Done)
                .unwrap();

            total
        }).unwrap()
    }
}
