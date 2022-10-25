use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleMysqlFunc {}

impl Rule for RuleMysqlFunc {
    fn uid() -> String {
        return "m2".to_string();
    }

    fn trigger(&self) -> Trigger {
        Trigger::KeyEqual("Function".to_string())
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported mysql functions".to_string();

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
