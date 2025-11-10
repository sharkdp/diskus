use assert_cmd::Command;
use tempfile::tempdir;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_diskus_output_lines() {
    let tmp = tempdir().unwrap();
    let dir1 = tmp.path().join("dir1");
    let dir2 = tmp.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();

    let mut f1 = File::create(dir1.join("a.txt")).unwrap();
    f1.write_all(&vec![0u8; 100]).unwrap();
    let mut f2 = File::create(dir2.join("b.txt")).unwrap();
    f2.write_all(&vec![0u8; 200]).unwrap();

    // Testing for exact outputs, i.e. filesizes is problematic across platforms. 
    // Hence we just verify, that the amount of printed lines varies depending on aggregation.

    // ---- Default mode (no aggregation) ----
    let mut cmd = Command::cargo_bin("diskus").unwrap();
    cmd.arg(dir1.to_str().unwrap())
       .arg(dir2.to_str().unwrap());

    let output = cmd.assert().success().get_output().stdout.clone();
    let out_str = String::from_utf8_lossy(&output);

    // Each path prints one header line + one size line
    // e.g., "4196\tdir1"
    let lines: Vec<&str> = out_str.lines().collect();
    assert_eq!(lines.len(), 2);

    // ---- Aggregate mode (-a) ----
    let mut cmd = Command::cargo_bin("diskus").unwrap();
    cmd.arg("-a")
       .arg(dir1.to_str().unwrap())
       .arg(dir2.to_str().unwrap());

    let output = cmd.assert().success().get_output().stdout.clone();
    let out_str = String::from_utf8_lossy(&output);

    let lines: Vec<&str> = out_str.lines().collect();
    assert_eq!(lines.len(), 1); // Only one line for aggregated size
}

