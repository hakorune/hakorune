//! Phase 171-fix: Condition expression environment.
//!
//! This box keeps the name -> JoinIR ValueId mapping needed by condition lowering.
//! It does not perform AST lowering or host/join remapping.

use crate::mir::ValueId;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct ConditionEnv {
    name_to_join: BTreeMap<String, ValueId>,
    pub captured: BTreeMap<String, ValueId>,
}

impl ConditionEnv {
    pub fn new() -> Self {
        Self {
            name_to_join: BTreeMap::new(),
            captured: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, join_id: ValueId) {
        self.name_to_join.insert(name, join_id);
    }

    pub fn get(&self, name: &str) -> Option<ValueId> {
        self.name_to_join
            .get(name)
            .copied()
            .or_else(|| self.captured.get(name).copied())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.name_to_join.contains_key(name) || self.captured.contains_key(name)
    }

    pub fn is_captured(&self, name: &str) -> bool {
        self.captured.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.name_to_join.len() + self.captured.len()
    }

    pub fn is_empty(&self) -> bool {
        self.name_to_join.is_empty() && self.captured.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &ValueId)> {
        self.name_to_join.iter()
    }

    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<_> = self
            .name_to_join
            .keys()
            .chain(self.captured.keys())
            .cloned()
            .collect();
        names.sort();
        names.dedup();
        names
    }

    pub fn max_value_id(&self) -> Option<u32> {
        let name_max = self.name_to_join.values().map(|v| v.0).max();
        let captured_max = self.captured.values().map(|v| v.0).max();

        match (name_max, captured_max) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConditionBinding {
    pub name: String,
    pub host_value: ValueId,
    pub join_value: ValueId,
}

impl ConditionBinding {
    pub fn new(name: String, host_value: ValueId, join_value: ValueId) -> Self {
        Self {
            name,
            host_value,
            join_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_env_basic() {
        let mut env = ConditionEnv::new();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);

        env.insert("i".to_string(), ValueId(0));
        assert!(!env.is_empty());
        assert_eq!(env.len(), 1);
        assert!(env.contains("i"));
        assert_eq!(env.get("i"), Some(ValueId(0)));
    }

    #[test]
    fn test_condition_env_multiple_vars() {
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(0));
        env.insert("start".to_string(), ValueId(1));
        env.insert("end".to_string(), ValueId(2));

        assert_eq!(env.len(), 3);
        assert_eq!(env.get("i"), Some(ValueId(0)));
        assert_eq!(env.get("start"), Some(ValueId(1)));
        assert_eq!(env.get("end"), Some(ValueId(2)));
        assert_eq!(env.get("nonexistent"), None);
    }

    #[test]
    fn test_condition_binding() {
        let binding = ConditionBinding::new("start".to_string(), ValueId(33), ValueId(1));

        assert_eq!(binding.name, "start");
        assert_eq!(binding.host_value, ValueId(33));
        assert_eq!(binding.join_value, ValueId(1));
    }
}
