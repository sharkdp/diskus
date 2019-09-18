pub fn generate_unique_id(_metadata: &std::fs::Metadata) -> Option<super::UniqueID> {
    // Since even the Windows-internal tools such as (but not limited to)
    // - Powershell,
    // - Explorer,
    // - dir,
    // are not respecting hardlinks or junction points when determining the
    // size of a directory [1], it has been decided that diskus will count
    // any such entries multiple times. too.
    //
    // Footnotes:
    // [1] https://github.com/sharkdp/diskus/issues/32#issuecomment-532817905
    None
}
