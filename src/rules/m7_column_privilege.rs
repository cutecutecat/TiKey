use serde_json::Value;

use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleColPriv {}

impl Rule for RuleColPriv {
    // "Grant": Object{"privileges": Object {"Actions": Array [Object {"Select":..}]}}
    fn uid() -> String {
        return "m7".to_string();
    }

    fn trigger(&self) -> Trigger {
        let judger = |v: &Value| {
            let v = match v.as_object() {
                Some(t) => t,
                _ => return false,
            };

            let v = match v.get("privileges") {
                Some(o) => o,
                None => return false,
            };
            let v = match v.as_object() {
                Some(o) => o,
                None => return false,
            };
            let v = match v.get("Actions") {
                Some(s) => s,
                None => return false,
            };
            let v = match v.as_array() {
                Some(t) => t,
                _ => return false,
            };
            if v.len() != 1 {
                return false;
            }
            let v = match v.first() {
                Some(t) => t,
                _ => return false,
            };
            let v = match v.as_object() {
                Some(t) => t,
                _ => return false,
            };
            let v = match v.get("Select") {
                Some(s) => s,
                None => return false,
            };
            let v = match v.as_object() {
                Some(t) => t,
                _ => return false,
            };
            match v.get("columns") {
                Some(_) => true,
                None => false,
            }
        };

        Trigger::KeyEqualJudge(("Grant".to_string(), judger))
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported Column-level privileges".to_string();

        let url: Option<String> = Some("https://github.com/pingcap/tidb/issues/9766".to_string());

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
