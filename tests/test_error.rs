use std::path::PathBuf;

use once_cell::sync::Lazy;
use tikey::{
    check_file,
    rules::{
        Rule, RuleCharset, RuleColPriv, RuleEndEarly, RuleEvent, RuleForeignKey, RuleFullText,
        RuleFunction, RuleMysqlFunc, RuleOptimTrace, RuleProcedure, RuleSavepoint, RuleSpatial,
        RuleSysSchema, RuleTrigger, RuleUnknown,
    },
};

const ERROR_PREFIX: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("tests/error"));

#[test]
fn h1_function() {
    let mut path = ERROR_PREFIX.clone();
    path.push("function.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleFunction::uid());
    }
}

#[test]
fn h2_trigger() {
    let mut path = ERROR_PREFIX.clone();
    path.push("trigger.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleTrigger::uid());
    }
}

#[test]
fn h3_event() {
    let mut path = ERROR_PREFIX.clone();
    path.push("event.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleEvent::uid());
    }
}

#[test]
fn h4_procedure() {
    let mut path = ERROR_PREFIX.clone();
    path.push("procedure.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleProcedure::uid());
    }
}

#[test]
fn h5_fulltext() {
    let mut path = ERROR_PREFIX.clone();
    path.push("fulltext.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 2);
    assert_eq!(
        *right[0].records().first().unwrap().uid(),
        RuleFullText::uid()
    );
    assert_eq!(
        *right[1].records().first().unwrap().uid(),
        RuleEndEarly::uid()
    );
}

#[test]
fn h6_save_point() {
    let mut path = ERROR_PREFIX.clone();
    path.push("save_point.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleSavepoint::uid());
    }
}

#[test]
fn m1_foreign_key() {
    let mut path = ERROR_PREFIX.clone();
    path.push("foreign_key.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleForeignKey::uid());
    }
}

#[test]
fn m2_mysql_function() {
    let mut path = ERROR_PREFIX.clone();
    path.push("mysql_function.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 4);
    for info in right {
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleMysqlFunc::uid());
    }
}

#[test]
fn m3_spatial() {
    let mut path = ERROR_PREFIX.clone();
    path.push("spatial.sql");
    let (_summary, right) = check_file(path).unwrap();
    println!("{:?}", right);
    assert_eq!(right.len(), 1);
    for info in right {
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleSpatial::uid());
    }
}

#[test]
fn m4_charset() {
    let mut path = ERROR_PREFIX.clone();
    path.push("charset.sql");
    let (_summary, right) = check_file(path).unwrap();
    println!("{:?}", right);
    assert_eq!(right.len(), 4);
    assert_eq!(
        *right[0].records().first().unwrap().uid(),
        RuleCharset::uid()
    );
    assert_eq!(
        *right[1].records().first().unwrap().uid(),
        RuleCharset::uid()
    );
    assert_eq!(
        *right[2].records().first().unwrap().uid(),
        RuleUnknown::uid()
    );
    assert_eq!(
        *right[3].records().first().unwrap().uid(),
        RuleEndEarly::uid()
    );
}

#[test]
fn m5_sys_schema() {
    let mut path = ERROR_PREFIX.clone();
    path.push("sys_schema.sql");
    let (_summary, right) = check_file(path).unwrap();
    println!("{:?}", right);
    assert_eq!(right.len(), 1);
    for info in right {
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleSysSchema::uid());
    }
}

#[test]
fn m6_optim_trace() {
    let mut path = ERROR_PREFIX.clone();
    path.push("optimizer_trace.sql");
    let (_summary, right) = check_file(path).unwrap();
    println!("{:?}", right);
    assert_eq!(right.len(), 3);
    for info in right {
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleOptimTrace::uid());
    }
}

#[test]
fn m7_column_privilege() {
    let mut path = ERROR_PREFIX.clone();
    path.push("column_privilege.sql");
    let (_summary, right) = check_file(path).unwrap();

    assert_eq!(right.len(), 1);
    for info in right {
        assert_eq!(info.records().len(), 1);
        let error_uid = info.records().first().unwrap().uid();

        assert_eq!(*error_uid, RuleColPriv::uid());
    }
}
