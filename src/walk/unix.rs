use std::os::unix::fs::MetadataExt;

use super::UniqueID;

pub fn generate_unique_id(metadata: &std::fs::Metadata) -> Option<UniqueID> {
    // If the entry has more than one hard link, generate
    // a unique ID consisting of device and inode in order
    // not to count this entry twice.
    if metadata.is_file() && metadata.nlink() > 1 {
        Some(UniqueID(metadata.dev(), metadata.ino()))
    } else {
        None
    }
}