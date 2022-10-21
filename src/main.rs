pub mod dialect;
pub mod registry;
mod rules;

use std::fmt::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use serde_json::Value;
use sqlparser::parser::Parser;

use crate::dialect::FixedStatement;
use crate::dialect::MysqlBeyondDialect;
use crate::registry::REGISTRY;
use crate::rules::RuleInfo;

#[derive(Debug)]
struct OnceInfo {
    sql: String,
    info: Vec<RuleInfo>,
}

fn main() {
    // let file = File::open("test/bad/stored_procedure.sql").unwrap();
    let file = File::open("test/bad/fulltext_search.sql").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let dialect = MysqlBeyondDialect {}; // or AnsiDialect, or your own dialect ...
    let ast = Parser::parse_sql(&dialect, &contents).unwrap();

    let mut all_info: Vec<OnceInfo> = vec![];
    for statement in ast {
        let value = serde_json::to_value(statement.clone()).unwrap();
        println!("{}", value);
        let mut check_result: Vec<RuleInfo> = vec![];
        read_value(&value, &mut check_result);
        if !check_result.is_empty() {
            all_info.push(OnceInfo {
                sql: FixedStatement(statement).to_string(),
                info: check_result,
            })
        }
    }
    println!("{:?}", all_info);
}

fn read_value(value: &Value, check_result: &mut Vec<RuleInfo>) {
    if !value.is_array() && !value.is_object() {
        return;
    }
    if value.is_array() {
        for inner in value.as_array().unwrap() {
            read_value(inner, check_result);
        }
        return;
    }
    for val in value.as_object().unwrap() {
        let (key, v) = val;
        let key_equal_result = REGISTRY.check_key_equal(key, v);
        check_result.extend(key_equal_result.into_iter());

        read_value(&v, check_result);
    }
    return;
}
