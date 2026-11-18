//! Phase 171-fix: Condition expression environment.
//!
//! This box keeps the name -> JoinIR ValueId mapping needed by condition lowering.
//! It does not perform AST lowering or host/join remapping.

use crate::mir::ValueId;



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
    fn test_condition_binding() {
        let binding = ConditionBinding::new("start".to_string(), ValueId(33), ValueId(1));

        assert_eq!(binding.name, "start");
        assert_eq!(binding.host_value, ValueId(33));
        assert_eq!(binding.join_value, ValueId(1));
    }
}
