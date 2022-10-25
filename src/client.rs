use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

use crate::{
    check_file, check_files, check_statements, format_records, format_summary, OnceInfo, Summary,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct TiKeyArgs {
    #[command(subcommand)]
    target: Target,
    /// Entity of input to be checked, decided by target
    ///
    /// SQL statement when `target == Statement`
    ///
    /// file path when `target == File`
    ///
    /// directory path when `target == Dir`
    #[arg(short, long)]
    inp: String,

    /// path of output report file
    #[arg(short, long)]
    out: Option<PathBuf>,
}

impl TiKeyArgs {
    pub fn new(target: Target, inp: String, out: Option<PathBuf>) -> Self {
        Self { target, inp, out }
    }
}

#[derive(Debug, Subcommand)]
pub enum Target {
    Statement,
    File,
    Dir,
}

fn show_report(sum: Summary, all_info: Vec<OnceInfo>) {
    println!("{}", format_summary(sum));
    for info in all_info {
        for table_rec in format_records(info) {
            println!("{}", table_rec);
        }
    }
}

fn save_report(path: Option<&Path>, sum: Summary, all_info: Vec<OnceInfo>) -> Result<()> {
    let path = path.unwrap_or_else(|| Path::new("report.txt"));
    if path.is_dir() {
        panic!("path is a file!");
    }
    let mut file = File::create(path)?;
    let summary = format!("{}\n", format_summary(sum));
    file.write(summary.as_bytes())?;
    for info in all_info {
        for table_rec in format_records(info) {
            file.write(table_rec.as_bytes())?;
            file.write(b"\n")?;
        }
    }
    Ok(())
}

pub fn client(args: TiKeyArgs) -> Result<()> {
    let (sum, all_info) = match args.target {
        Target::Statement => check_statements(args.inp)?,
        Target::File => check_file(args.inp)?,
        Target::Dir => {
            let mut all_files: Vec<PathBuf> = vec![];
            for entry in WalkDir::new(args.inp) {
                let entry = entry?;
                all_files.push(entry.into_path());
            }
            let all_files = all_files;
            let all_files = all_files
                .into_iter()
                .filter(|p| match p.extension() {
                    Some(s) => match s.to_str() {
                        Some("sql") => true,
                        _ => false,
                    },
                    None => false,
                })
                .collect();
            check_files(all_files)?
        }
    };
    match args.out {
        Some(path) => save_report(Some(path.as_path()), sum, all_info)?,
        None => show_report(sum, all_info),
    };
    Ok(())
}
