use serde_json::Value;

use crate::dialect::AddupStatement;

use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleEvent {}

impl Rule for RuleEvent {
    fn uid() -> String {
        return "h3".to_string();
    }

    fn trigger(&self) -> Trigger {
        let judger = |v: &Value| {
            let v = match v.as_object() {
                Some(o) => o,
                None => return false,
            };
            let v = match v.get("object_name") {
                Some(v) => v,
                None => return false,
            };
            let v = match v.as_array() {
                Some(a) => a,
                None => return false,
            };
            if v.len() != 1 {
                return false;
            }
            let v = match v.first() {
                Some(e) => e,
                None => return false,
            };
            let v = match v.as_object() {
                Some(o) => o,
                None => return false,
            };
            let v = match v.get("value") {
                Some(v) => v,
                None => return false,
            };
            match v.as_str() {
                Some(s) if s == AddupStatement::CreateEvent.to_string() => true,
                _ => return false,
            }
        };
        Trigger::KeyEqualJudge(("Comment".to_string(), judger))
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::NoPlan;

        let description: String = "TiDB not supported events".to_string();

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
