mod foreign_key;
mod function;
mod trigger;

pub use foreign_key::RuleForeignKey;
pub use function::RuleFunction;
pub use trigger::RuleTrigger;

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum InfoLevel {
    WARNING,
    ERROR,
}

#[derive(Debug, Clone)]
pub enum DBVersion {
    Earliest,
    Latest,
    Version(String),
}

#[derive(Debug, Clone)]
pub enum RuleFuture {
    WillSupport,
    NoPlan,
}

pub enum Trigger {
    KeyEqual(String),
    KeyEqualJudge((String, fn(&Value) -> bool)),
}

#[derive(Debug, Clone)]
pub struct RuleInfo {
    info_level: InfoLevel,
    db_version_range: (DBVersion, DBVersion),
    future: RuleFuture,
    description: String,
    url: Option<String>,
}

impl RuleInfo {
    pub fn new(
        info_level: InfoLevel,
        db_version_range: (DBVersion, DBVersion),
        future: RuleFuture,
        description: String,
        url: Option<String>,
    ) -> Self {
        Self {
            info_level,
            db_version_range,
            future,
            description,
            url,
        }
    }
}

pub trait Rule {
    fn trigger(&self) -> Trigger;
    fn info(&self) -> RuleInfo;
}
