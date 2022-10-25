use std::path::PathBuf;

use once_cell::sync::Lazy;
use tikey::{
    check_files,
    client::{client, Target, TiKeyArgs},
};
use walkdir::WalkDir;

const INTEGRATION_PREFIX: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("tests"));

#[test]
fn use_as_lib() {
    let path = INTEGRATION_PREFIX.clone();
    let all_files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .map(|a| a.unwrap().into_path())
        .filter(|p| match p.extension() {
            Some(s) => match s.to_str() {
                Some("sql") => true,
                _ => false,
            },
            None => false,
        })
        .collect();

    let (summary, _right) = check_files(all_files).unwrap();

    assert_eq!(*summary.file_count(), 28);
    assert_eq!(*summary.sql_count(), 99);
    assert_eq!(*summary.errors(), 90);
    assert_eq!(*summary.warnings(), 16);
}

#[test]
fn use_as_command() {
    let path = INTEGRATION_PREFIX.clone();
    let out: PathBuf = [r"./report.txt"].iter().collect();

    let args = TiKeyArgs::new(
        Target::Dir,
        path.into_os_string().into_string().unwrap(),
        Some(out),
    );
    client(args).unwrap()
}
