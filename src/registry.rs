use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde_json::Value;

use crate::rules::{
    Rule, RuleCharset, RuleColPriv, RuleDelimiter, RuleEndEarly, RuleEvent, RuleForeignKey,
    RuleFullText, RuleFunction, RuleInfo, RuleMysqlFunc, RuleOptimTrace, RuleProcedure,
    RuleSavepoint, RuleSpatial, RuleSysSchema, RuleTrigger, RuleUnknown, RuleXA, Trigger,
};

pub static REGISTRY: Lazy<RuleRegistry> = Lazy::new(|| {
    let mut reg = RuleRegistry::new();
    // register rule of `head` type
    reg.register(RuleFunction {});
    reg.register(RuleTrigger {});
    reg.register(RuleEvent {});
    reg.register(RuleProcedure {});
    reg.register(RuleFullText {});
    reg.register(RuleSavepoint {});
    reg.register(RuleXA {});

    // register rule of `mid` type
    reg.register(RuleForeignKey {});
    reg.register(RuleMysqlFunc {});
    reg.register(RuleSpatial {});
    reg.register(RuleCharset {});
    reg.register(RuleSysSchema {});
    reg.register(RuleOptimTrace {});
    reg.register(RuleColPriv {});

    // register rule of `special` type
    reg.register(RuleUnknown {});
    reg.register(RuleDelimiter {});
    reg.register(RuleEndEarly {});
    reg
});

pub struct RuleRegistry {
    equal_keys_judge: HashMap<String, Vec<(fn(&Value) -> bool, RuleInfo)>>,
    equal_string_elem: HashMap<String, Vec<RuleInfo>>,
}

impl RuleRegistry {
    fn new() -> Self {
        Self {
            equal_keys_judge: HashMap::new(),
            equal_string_elem: HashMap::new(),
        }
    }

    fn register(&mut self, rule: impl Rule) {
        let info = rule.info();
        if let Trigger::StringElemEqual(v) = rule.trigger() {
            for s in v {
                match self.equal_string_elem.get_mut(&s) {
                    Some(rules) => rules.push(info.clone()),
                    None => {
                        let _ = self.equal_string_elem.insert(s, vec![info.clone()]);
                    }
                }
            }
            return;
        }
        let (s, f) = match rule.trigger() {
            Trigger::KeyEqualJudge((s, f)) => (s, f),
            Trigger::KeyEqual(s) => {
                let f = |_: &Value| true;
                (s, f as fn(&Value) -> bool)
            }
            Trigger::StringElemEqual(_) => {
                unreachable!()
            }
        };

        let elem = self.equal_keys_judge.get_mut(&s);
        match elem {
            Some(rules) => rules.push((f, info)),
            None => {
                let _ = self.equal_keys_judge.insert(s, vec![(f, info)]);
            }
        }
    }

    pub fn check_key_equal(&self, key: &String, v: &Value) -> Vec<RuleInfo> {
        match self.equal_keys_judge.get(key) {
            Some(ops) => ops
                .iter()
                .filter(|(f, _info)| f(v))
                .map(|(_f, info)| info.clone())
                .collect(),
            None => vec![],
        }
    }

    pub fn check_string_elem(&self, v: &Value) -> Vec<RuleInfo> {
        match v.as_str() {
            Some(s) => match self.equal_string_elem.get(s) {
                Some(v) => v.clone(),
                None => {
                    vec![]
                }
            },
            None => vec![],
        }
    }
}
