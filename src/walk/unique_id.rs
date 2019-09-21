#[derive(Eq, PartialEq, Hash)]
pub struct UniqueID {
    pub device: u64,
    pub inode: u64,
}
