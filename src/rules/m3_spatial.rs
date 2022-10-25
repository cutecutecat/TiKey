use serde_json::Value;

use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleSpatial {}

impl Rule for RuleSpatial {
    fn uid() -> String {
        return "m3".to_string();
    }

    fn trigger(&self) -> Trigger {
        let judger = |v: &Value| match v.as_str() {
            Some("geometry") => true,
            _ => false,
        };

        Trigger::KeyEqualJudge(("value".to_string(), judger))
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported SPATIAL (also known as GIS/GEOMETRY) functions, data types and indexes".to_string();

        let url: Option<String> = Some("https://github.com/pingcap/tidb/issues/6347".to_string());

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
