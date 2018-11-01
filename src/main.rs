extern crate humansize;
extern crate ignore;
extern crate num_cpus;

use std::collections::HashSet;
use std::os::unix::fs::MetadataExt;
use std::sync::mpsc::channel;
use std::thread;

use humansize::{file_size_opts, FileSize};
use ignore::WalkBuilder;

fn main() {
    let mut builder = WalkBuilder::new("./");
    builder.hidden(false);
    builder.parents(false);
    builder.ignore(false);
    builder.git_global(false);
    builder.git_ignore(false);
    builder.git_exclude(false);
    builder.follow_links(false);

    builder.threads(num_cpus::get());

    let walker = builder.build_parallel();

    let (tx, rx) = channel();

    let receiver_thread = thread::spawn(move || {
        let mut total = 0;
        let mut ids = HashSet::new();
        for (unique_id, size) in rx {
            if let Some(unique_id) = unique_id {
                // Only count this entry if the ID has not been seen
                if ids.insert(unique_id) {
                    total += size;
                }
            } else {
                total += size;
            }
        }
        println!(
            "{} ({} bytes)",
            total.file_size(file_size_opts::DECIMAL).unwrap(),
            total
        );
    });

    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            if let Ok(entry) = result {
                let metadata = entry.metadata().unwrap();

                // If the entry has more than one hard link, generate
                // a unique ID consisting of device and inode in order
                // not to count this entry twice.
                let unique_id = if metadata.is_file() && metadata.nlink() > 1 {
                    Some((metadata.dev(), metadata.ino()))
                } else {
                    None
                };

                let size = metadata.len();

                tx.send((unique_id, size)).unwrap();
            }

            return ignore::WalkState::Continue;
        })
    });

    drop(tx);
    receiver_thread.join().unwrap();
}
