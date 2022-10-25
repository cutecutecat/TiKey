use serde_json::Value;

use crate::dialect::AddupStatement;

use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleDelimiter {}

impl Rule for RuleDelimiter {
    fn uid() -> String {
        return "s2".to_string();
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
                Some(s) if s == AddupStatement::Delimiter.to_string() => true,
                _ => return false,
            }
        };
        Trigger::KeyEqualJudge(("Comment".to_string(), judger))
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::WARNING;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::NoPlan;

        let description: String = "Tikey is unable to detect statement inside Delimiter block\
            (illegal in most cases as something like function)! \
            If you want to check them precisely, please change to undelimitered version"
            .to_string();

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
