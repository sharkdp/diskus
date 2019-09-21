use std::os::unix::fs::MetadataExt;

use crate::walk::unique_id::UniqueID;

pub fn generate_unique_id(metadata: &std::fs::Metadata) -> Option<UniqueID> {
    // If the entry has more than one hard link, generate
    // a unique ID consisting of device and inode in order
    // not to count this entry twice.
    if metadata.is_file() && metadata.nlink() > 1 {
        Some(UniqueID {
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    } else {
        None
    }
}
