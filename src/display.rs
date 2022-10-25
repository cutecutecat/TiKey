use std::path::PathBuf;

use comfy_table::{ContentArrangement, Table};

use crate::{rules::RuleInfo, OnceInfo, Summary};

pub fn format_summary(sum: Summary) -> String {
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(70)
        .set_header(vec![
            "File Count",
            "SQL Count",
            "Errors",
            "Warnings",
            "Time Cost",
        ])
        .add_row(vec![
            sum.file_count().to_string(),
            sum.sql_count().to_string(),
            sum.errors().to_string(),
            sum.warnings().to_string(),
            format!("{:?}", sum.time_cost()),
        ]);
    table.to_string()
}

pub fn format_records(info: OnceInfo) -> Vec<String> {
    info.records
        .into_iter()
        .map(|rec| format_record(info.sql.clone(), rec, info.file.clone()))
        .collect()
}

fn format_record(sql: String, rec: RuleInfo, file: Option<PathBuf>) -> String {
    let mut table = Table::new();

    let (db_version_range_from, db_version_range_to) = &rec.db_version_range();
    let db_version_description = format!(
        "{} - {}",
        db_version_range_from.to_string(),
        db_version_range_to.to_string()
    );

    let url = rec.url().clone().unwrap_or("".to_string());
    let file = file
        .map(|p| p.into_os_string().into_string().unwrap_or("".to_string()))
        .unwrap_or("".to_string());
    table
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(70)
        .add_row(vec!["Error code", rec.uid()])
        .add_row(vec!["Level", &rec.info_level().to_string()])
        .add_row(vec!["TiDB version", &db_version_description])
        .add_row(vec!["Future plan", &rec.future().to_string()])
        .add_row(vec!["Description", &rec.description().to_string()])
        .add_row(vec!["URL", &url])
        .add_row(vec!["SQL", &sql])
        .add_row(vec!["File", &file]);
    table.to_string()
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{
        format_records, format_summary,
        rules::{DBVersion, InfoLevel, RuleFuture, RuleInfo},
        OnceInfo, Summary,
    };

    #[test]
    fn test_format_summary() {
        let summary = Summary {
            file_count: 1,
            sql_count: 2,
            errors: 3,
            warnings: 4,
            time_cost: Duration::from_millis(114514),
        };
        let table = format_summary(summary);
        let expect = "\
        +------------+-----------+--------+----------+-----------+\n\
        | File Count | SQL Count | Errors | Warnings | Time Cost |\n\
        +========================================================+\n\
        | 1          | 2         | 3      | 4        | 114.514s  |\n\
        +------------+-----------+--------+----------+-----------+";
        assert_eq!(table, expect);
    }

    #[test]
    fn test_format_rule_info() {
        let sql = "select mysql from TiDB".to_string();
        let uid = "P5".to_string();
        let info_level = InfoLevel::ERROR;
        let db_version_range = (DBVersion::Earliest, DBVersion::Version("1.0.0".to_string()));
        let future = RuleFuture::NoPlan;
        let description = "oh, yeah!".to_string();
        let url = None;

        let rec = RuleInfo::new(uid, info_level, db_version_range, future, description, url);

        let info = OnceInfo {
            sql,
            records: vec![rec],
            file: Some([r"Shimo", "kitazawa", "Koji", "Tadokoro"].iter().collect()),
        };
        let mut tables = format_records(info);
        assert_eq!(tables.len(), 1);
        let table = tables.pop().unwrap();

        let expect = "\
        +--------------+------------------------------+\n\
        | Error code   | P5                           |\n\
        |--------------+------------------------------|\n\
        | Level        | error                        |\n\
        |--------------+------------------------------|\n\
        | TiDB version | earliest - 1.0.0             |\n\
        |--------------+------------------------------|\n\
        | Future plan  | no plan to support           |\n\
        |--------------+------------------------------|\n\
        | Description  | oh, yeah!                    |\n\
        |--------------+------------------------------|\n\
        | URL          |                              |\n\
        |--------------+------------------------------|\n\
        | SQL          | select mysql from TiDB       |\n\
        |--------------+------------------------------|\n\
        | File         | Shimo\\kitazawa\\Koji\\Tadokoro |\n\
        +--------------+------------------------------+";
        assert_eq!(table, expect);
    }
}
