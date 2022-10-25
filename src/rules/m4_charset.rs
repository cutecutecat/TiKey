// sometimes will fallback to `end_early`
use super::{DBVersion, InfoLevel, Rule, RuleFuture, RuleInfo, Trigger};

pub struct RuleCharset {}

impl Rule for RuleCharset {
    fn uid() -> String {
        return "m4".to_string();
    }

    fn trigger(&self) -> Trigger {
        let elems = vec![
            "armscii8", "big5", "cp1250", "cp1251", "cp1256", "cp1257", "cp850", "cp852", "cp866",
            "cp932", "dec8", "eucjpms", "euckr", "gb18030", "gb2312", "geostd8", "greek", "greek",
            "hp8", "keybcs2", "koi8r", "koi8u", "latin1", "latin2", "latin5", "latin7", "macce",
            "macroman", "sjis", "swe7", "tis620", "ucs2", "ujis", "utf16", "utf16le", "utf32",
            "utf8mb3",
        ];
        Trigger::StringElemEqual(elems.into_iter().map(|s| s.to_string()).collect())
    }

    fn info(&self) -> RuleInfo {
        let info_level: InfoLevel = InfoLevel::ERROR;

        let db_version_range: (DBVersion, DBVersion) = (DBVersion::Earliest, DBVersion::Latest);

        let future: RuleFuture = RuleFuture::WillSupport;

        let description: String = "TiDB not supported random charset".to_string();

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
