use serde_json::Value;

use crate::dialect::AddupStatement;

use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleFunction {}

impl Rule for RuleFunction {
    fn trigger(&self) -> Trigger {
        // {"comment":"myfun_getAvg ( num1 int , num2 int ) comment 'calculation mean' returns int return ( num1 + num2 ) / 2 ; ","object_name":[{"quote_style":null,"value":"CreateFunction"}],"object_type":"Table"}}
        let judger = |v: &Value| {
            if !v.is_object() {
                return false;
            }
            let v = v.as_object().unwrap();
            if !v.contains_key("object_name") {
                return false;
            }
            let v = v.get("object_name").unwrap();
            if !v.is_array() {
                return false;
            }
            let v = v.as_array().unwrap().first();
            if v.is_none() || !v.unwrap().is_object() {
                return false;
            }
            let v = v.unwrap().as_object().unwrap();
            if !v.contains_key("value") {
                return false;
            }
            let v = v.get("value").unwrap();
            if v.is_string() && v.as_str().unwrap() == AddupStatement::CreateFunction.to_string() {
                return true;
            }
            false
        };
        Trigger::KeyEqualJudge(("Comment".to_string(), judger))
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::NoPlan;

        let description: String = "TiDB not supported functions".to_string();

        let url: Option<String> = None;

        RuleInfo::new(info_level, db_version_range, future, description, url)
    }
}
