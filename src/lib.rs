pub mod client;
mod dialect;
mod display;
mod registry;
pub mod rules;

use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use rules::InfoLevel;
use serde_json::Value;
use sqlparser::parser::Parser;

use crate::dialect::FixedStatement;
use crate::dialect::MysqlBeyondDialect;
use crate::registry::REGISTRY;
use crate::rules::RuleInfo;

pub use display::{format_records, format_summary};

#[derive(Debug, Clone)]
pub struct OnceInfo {
    sql: String,
    records: Vec<RuleInfo>,
    file: Option<PathBuf>,
}

impl OnceInfo {
    pub fn sql(&self) -> &String {
        &self.sql
    }

    pub fn records(&self) -> &Vec<RuleInfo> {
        &self.records
    }

    pub fn file(&self) -> &Option<PathBuf> {
        &self.file
    }
}

#[derive(Debug)]
pub struct Summary {
    file_count: u128,
    sql_count: u128,
    errors: u128,
    warnings: u128,
    time_cost: Duration,
}

impl Summary {
    pub fn file_count(&self) -> &u128 {
        &self.file_count
    }

    pub fn sql_count(&self) -> &u128 {
        &self.sql_count
    }

    pub fn errors(&self) -> &u128 {
        &self.errors
    }

    pub fn warnings(&self) -> &u128 {
        &self.warnings
    }

    pub fn time_cost(&self) -> &Duration {
        &self.time_cost
    }
}

impl Add for Summary {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            file_count: self.file_count + other.file_count,
            sql_count: self.sql_count + other.sql_count,
            errors: self.errors + other.errors,
            warnings: self.warnings + other.warnings,
            time_cost: self.time_cost + other.time_cost,
        }
    }
}

fn read_value(value: &Value, check_result: &mut Vec<RuleInfo>) -> Result<()> {
    if !value.is_array() && !value.is_object() {
        let key_equal_result = REGISTRY.check_string_elem(value);
        check_result.extend(key_equal_result.into_iter());
        return Ok(());
    }
    if value.is_array() {
        for inner in value.as_array().ok_or_else(|| anyhow!("impossable"))? {
            read_value(inner, check_result)?;
        }
        return Ok(());
    }
    for val in value.as_object().ok_or_else(|| anyhow!("impossable"))? {
        let (key, v) = val;
        let key_equal_result = REGISTRY.check_key_equal(key, v);
        check_result.extend(key_equal_result.into_iter());

        read_value(&v, check_result)?;
    }
    return Ok(());
}

pub fn check_statements(contents: String) -> Result<(Summary, Vec<OnceInfo>)> {
    let contents = contents.to_lowercase();
    let start_clock = Instant::now();
    let dialect = MysqlBeyondDialect {
        is_recalled: AtomicBool::new(false),
    };
    let ast = Parser::parse_sql(&dialect, &contents)
        .with_context(|| format!("Parsing SQL statements: {}", contents))?;

    let mut all_info: Vec<OnceInfo> = vec![];
    for statement in &ast {
        let value = serde_json::to_value(statement.clone())
            .with_context(|| format!("Serde SQL statement: {}", statement.to_string()))?;
        println!("{:?}", value);
        let mut check_result: Vec<RuleInfo> = vec![];
        read_value(&value, &mut check_result)?;
        if !check_result.is_empty() {
            all_info.push(OnceInfo {
                sql: FixedStatement(statement.clone()).to_string(),
                records: check_result,
                file: None,
            })
        }
    }
    let (error_count, warning_count) = count_info(&all_info);
    let summary = Summary {
        file_count: 0,
        sql_count: ast.len() as u128,
        errors: error_count,
        warnings: warning_count,
        time_cost: Instant::now() - start_clock,
    };
    Ok((summary, all_info))
}

pub fn check_file<P: AsRef<Path>>(path: P) -> Result<(Summary, Vec<OnceInfo>)> {
    let path = path.as_ref();
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .with_context(|| format!("At file: {:?}", path))?;
    let (mut summary, info_statement) =
        check_statements(contents).with_context(|| format!("At file: {:?}", path))?;
    summary.file_count = 1;
    let info_statement = info_statement
        .into_iter()
        .map(|info| {
            let mut revised_info = info;
            revised_info.file = Some(path.to_path_buf());
            revised_info
        })
        .collect();
    Ok((summary, info_statement))
}

pub fn check_files<P: AsRef<Path>>(paths: Vec<P>) -> Result<(Summary, Vec<OnceInfo>)> {
    if paths.is_empty() {
        return Err(anyhow!("cannot handle empty paths"));
    }
    let check_rets: Vec<Result<(Summary, Vec<OnceInfo>)>> =
        paths.into_iter().map(|p| check_file(p)).collect();

    let mut rets: Vec<(Summary, Vec<OnceInfo>)> = vec![];
    for ret in check_rets {
        match ret {
            Ok(t) => rets.push(t),
            Err(_) => return ret,
        }
    }
    rets.into_iter()
        .reduce(|(left_sum, left_info), (right_sum, right_info)| {
            (left_sum + right_sum, [left_info, right_info].concat())
        })
        .ok_or_else(|| anyhow!("impossable"))
}

fn count_info(all_info: &Vec<OnceInfo>) -> (u128, u128) {
    let mut error_count: u128 = 0;
    let mut warning_count: u128 = 0;
    for info in all_info {
        for rec in &info.records {
            if *rec.info_level() == InfoLevel::ERROR {
                error_count += 1;
            } else {
                warning_count += 1;
            }
        }
    }
    (error_count, warning_count)
}
