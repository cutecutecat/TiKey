use std::path::PathBuf;

use once_cell::sync::Lazy;
use tikey::{
    check_file,
    rules::{Rule, RuleDelimiter, RuleEndEarly, RuleUnknown},
};

const WARNING_PREFIX: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("tests/warning"));

#[test]
fn s1_unknown() {
    let mut path = WARNING_PREFIX.clone();
    path.push("unknown.sql");

    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();
        assert_eq!(*error_uid, RuleUnknown::uid());
    }
}

#[test]
fn s2_delimiter() {
    let mut path = WARNING_PREFIX.clone();
    path.push("delimiter_good.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();
        assert_eq!(*error_uid, RuleDelimiter::uid());
    }

    let mut path = WARNING_PREFIX.clone();
    path.push("delimiter_bad.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();
        assert_eq!(*error_uid, RuleDelimiter::uid());
    }
}

#[test]
fn s3_create_select() {
    let mut path = WARNING_PREFIX.clone();
    path.push("create_select.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleEndEarly::uid());
    }
}
