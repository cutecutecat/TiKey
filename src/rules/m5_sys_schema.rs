use serde_json::Value;

use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleSysSchema {}

impl Rule for RuleSysSchema {
    // "name": Array [Object {"quote_style": Null, "value": String("sys")},
    // Object {"quote_style": Null, "value": String("host_summary_by_statement_type")}]
    fn uid() -> String {
        return "m5".to_string();
    }

    fn trigger(&self) -> Trigger {
        let judger = |v: &Value| {
            let v = match v.as_array() {
                Some(t) => t,
                _ => return false,
            };
            if v.len() != 2 {
                return false;
            }
            let v = match v.first() {
                Some(o) => o,
                None => return false,
            };
            let v = match v.as_object() {
                Some(o) => o,
                None => return false,
            };
            let v = match v.get("value") {
                Some(s) => s,
                None => return false,
            };
            match v.as_str() {
                Some("sys") => true,
                _ => false,
            }
        };

        Trigger::KeyEqualJudge(("name".to_string(), judger))
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported SYS schema".to_string();

        let url: Option<String> = None;

        RuleInfo::new(
            Self::uid(),
            info_level,
            db_version_range,
            future,
            description,
            url,
        )
    }
}
