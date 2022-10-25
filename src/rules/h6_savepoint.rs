use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleSavepoint {}

impl Rule for RuleSavepoint {
    fn uid() -> String {
        return "h6".to_string();
    }

    fn trigger(&self) -> Trigger {
        Trigger::KeyEqual("Savepoint".to_string())
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported savepoint".to_string();

        let url: Option<String> = Some("https://github.com/pingcap/tidb/issues/6840".to_string());

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
