use serde_json::Value;

use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleOptimTrace {}

impl Rule for RuleOptimTrace {
    // "value": String("OPTIMIZER_TRACE")
    fn uid() -> String {
        return "m6".to_string();
    }

    fn trigger(&self) -> Trigger {
        let judger = |v: &Value| match v.as_str() {
            Some("optimizer_trace") => true,
            _ => false,
        };

        Trigger::KeyEqualJudge(("value".to_string(), judger))
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported optimizer trace".to_string();

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
