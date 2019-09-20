pub fn generate_unique_id(_metadata: &std::fs::Metadata) -> Option<super::UniqueID> {
    // Windows-internal tools such as Powershell, Explorer or `dir` are not respecting hardlinks
    // or junction points when determining the size of a directory. `diskus` does the same and
    // counts such entries multiple times (on Unix systems, multiple hardlinks to a single file are
    // counted just once).
    //
    // See: https://github.com/sharkdp/diskus/issues/32
    None
}
