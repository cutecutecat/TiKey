use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleForeignKey {}

impl Rule for RuleForeignKey {
    fn uid() -> String {
        return "m1".to_string();
    }

    fn trigger(&self) -> Trigger {
        Trigger::KeyEqual("ForeignKey".to_string())
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported FOREIGN KEY constraints".to_string();

        let url: Option<String> = Some("https://github.com/pingcap/tidb/issues/18209".to_string());

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
