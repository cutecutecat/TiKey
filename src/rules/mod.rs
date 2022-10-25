mod h1_function;
mod h2_trigger;
mod h3_event;
mod h4_procedure;
mod h5_fulltext;
mod h6_savepoint;
mod h7_xa;
mod m1_foreign_key;
mod m2_mysql_function;
mod m3_spatial;
mod m4_charset;
mod m5_sys_schema;
mod m6_optim_trace;
mod m7_column_privilege;
mod s1_unknown;
mod s2_delimiter;
mod s3_end_early;

pub use h1_function::RuleFunction;
pub use h2_trigger::RuleTrigger;
pub use h3_event::RuleEvent;
pub use h4_procedure::RuleProcedure;
pub use h5_fulltext::RuleFullText;
pub use h6_savepoint::RuleSavepoint;
pub use h7_xa::RuleXA;
pub use m1_foreign_key::RuleForeignKey;
pub use m2_mysql_function::RuleMysqlFunc;
pub use m3_spatial::RuleSpatial;
pub use m4_charset::RuleCharset;
pub use m5_sys_schema::RuleSysSchema;
pub use m6_optim_trace::RuleOptimTrace;
pub use m7_column_privilege::RuleColPriv;
pub use s1_unknown::RuleUnknown;
pub use s2_delimiter::RuleDelimiter;
pub use s3_end_early::RuleEndEarly;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfoLevel {
    WARNING,
    ERROR,
}

impl ToString for InfoLevel {
    fn to_string(&self) -> String {
        match self {
            InfoLevel::WARNING => "warning".to_string(),
            InfoLevel::ERROR => "error".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DBVersion {
    Earliest,
    Latest,
    Version(String),
}

impl ToString for DBVersion {
    fn to_string(&self) -> String {
        match self {
            DBVersion::Earliest => "earliest".to_string(),
            DBVersion::Latest => "latest".to_string(),
            DBVersion::Version(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuleFuture {
    WillSupport,
    NoPlan,
}

impl ToString for RuleFuture {
    fn to_string(&self) -> String {
        match self {
            RuleFuture::WillSupport => "have a plan to support".to_string(),
            RuleFuture::NoPlan => "no plan to support".to_string(),
        }
    }
}

pub enum Trigger {
    KeyEqual(String),
    KeyEqualJudge((String, fn(&Value) -> bool)),
    StringElemEqual(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct RuleInfo {
    uid: String,
    info_level: InfoLevel,
    db_version_range: (DBVersion, DBVersion),
    future: RuleFuture,
    description: String,
    url: Option<String>,
}

impl RuleInfo {
    pub fn new(
        uid: String,
        info_level: InfoLevel,
        db_version_range: (DBVersion, DBVersion),
        future: RuleFuture,
        description: String,
        url: Option<String>,
    ) -> Self {
        Self {
            uid,
            info_level,
            db_version_range,
            future,
            description,
            url,
        }
    }

    pub fn uid(&self) -> &String {
        return &self.uid;
    }

    pub fn info_level(&self) -> &InfoLevel {
        return &self.info_level;
    }
    pub fn db_version_range(&self) -> &(DBVersion, DBVersion) {
        return &self.db_version_range;
    }
    pub fn future(&self) -> &RuleFuture {
        return &self.future;
    }
    pub fn description(&self) -> &String {
        return &self.description;
    }
    pub fn url(&self) -> &Option<String> {
        return &self.url;
    }
}

pub trait Rule {
    fn uid() -> String;
    fn trigger(&self) -> Trigger;
    fn info(&self) -> RuleInfo;
}
