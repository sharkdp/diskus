#[derive(Debug, Clone, Copy)]
pub enum FilesizeType {
    DiskUsage,
    ApparentSize,
}

impl FilesizeType {
    #[cfg(not(windows))]
    pub fn size(self, metadata: &std::fs::Metadata) -> u64 {
        use std::os::unix::fs::MetadataExt;

        match self {
            FilesizeType::ApparentSize => metadata.len(),
            // block size is always 512 byte, see stat(2) manpage
            FilesizeType::DiskUsage => metadata.blocks() * 512,
        }
    }

    #[cfg(windows)]
    pub fn size(self, metadata: &std::fs::Metadata) -> u64 {
        metadata.len()
    }
}
