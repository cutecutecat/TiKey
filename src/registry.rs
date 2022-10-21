use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde_json::Value;

use crate::rules::{Rule, RuleForeignKey, RuleFunction, RuleInfo, RuleTrigger, Trigger};

pub static REGISTRY: Lazy<RuleRegistry> = Lazy::new(|| {
    let mut reg = RuleRegistry::new();
    reg.register(RuleForeignKey {});
    reg.register(RuleFunction {});
    reg.register(RuleTrigger {});
    reg
});

pub struct RuleRegistry {
    equal_keys_judge: HashMap<String, Vec<(fn(&Value) -> bool, RuleInfo)>>,
}

impl RuleRegistry {
    fn new() -> Self {
        Self {
            equal_keys_judge: HashMap::new(),
        }
    }

    fn register(&mut self, rule: impl Rule) {
        match rule.trigger() {
            Trigger::KeyEqual(s) => {
                let f = |_: &Value| true;
                let info = rule.info();
                if !self.equal_keys_judge.contains_key(&s) {
                    self.equal_keys_judge.insert(s, vec![(f, info)]);
                } else {
                    let rules = self.equal_keys_judge.get_mut(&s).unwrap();
                    rules.push((f, info));
                }
            }
            Trigger::KeyEqualJudge((s, f)) => {
                let info = rule.info();
                if !self.equal_keys_judge.contains_key(&s) {
                    self.equal_keys_judge.insert(s, vec![(f, info)]);
                } else {
                    let rules = self.equal_keys_judge.get_mut(&s).unwrap();
                    rules.push((f, info));
                }
            }
        }
    }

    pub fn check_key_equal(&self, key: &String, v: &Value) -> Vec<RuleInfo> {
        let mut ret: Vec<RuleInfo> = vec![];
        if self.equal_keys_judge.contains_key(key) {
            for (f, info) in self.equal_keys_judge.get(key).unwrap() {
                if f(v) {
                    ret.push(info.clone());
                }
            }
        }
        return ret;
    }
}
