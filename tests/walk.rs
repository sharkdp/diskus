use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use tempdir::TempDir;

use diskus::{FilesizeType, Walk};

#[test]
fn size_of_single_file() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new("diskus-tests")?;

    let file_path = tmp_dir.path().join("file-100-byte");
    File::create(&file_path)?.write(&vec![0u8; 100])?;

    let num_threads = 1;
    let root_directories = &[PathBuf::from(file_path)];
    let walk = Walk::new(root_directories, num_threads, FilesizeType::ApparentSize);

    let (walk_result, errors) = walk.run();

    assert!(errors.is_empty());
    assert_eq!(walk_result.size_in_bytes, 100);
    assert_eq!(walk_result.file_count, 1);
    assert_eq!(walk_result.directory_count, 0);

    Ok(())
}
