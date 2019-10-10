#[derive(Eq, PartialEq, Hash)]
pub struct UniqueID {
    device: u64,
    inode: u64,
}

#[cfg(not(windows))]
pub fn generate_unique_id(metadata: &std::fs::Metadata) -> Option<UniqueID> {
    use std::os::unix::fs::MetadataExt;
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

#[cfg(windows)]
pub fn generate_unique_id(_metadata: &std::fs::Metadata) -> Option<UniqueID> {
    // Windows-internal tools such as Powershell, Explorer or `dir` are not respecting hardlinks
    // or junction points when determining the size of a directory. `diskus` does the same and
    // counts such entries multiple times (on Unix systems, multiple hardlinks to a single file are
    // counted just once).
    //
    // See: https://github.com/sharkdp/diskus/issues/32
    None
}
