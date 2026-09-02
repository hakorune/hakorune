//! Active MIR ValueId inventory for lifecycle contracts.
//!
//! This is intentionally not a remapper.  It records the ValueIds present in
//! an instruction, including its destination, so lifecycle checks can compare
//! typed rows with the completed function.  Call target operands come from
//! `Callee`'s canonical projection; no JoinIR map or target inference belongs
//! here.

use crate::mir::{BasicBlock, MirInstruction, ValueId};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct MirValueIdInventory;

impl MirValueIdInventory {
    pub(crate) const fn new() -> Self {
        Self
    }

    pub(crate) fn collect_values_in_block(&self, block: &BasicBlock) -> Vec<ValueId> {
        let mut values = Vec::new();
        for instruction in &block.instructions {
            values.extend(self.collect_values_in_instruction(instruction));
        }
        if let Some(terminator) = block.terminator.as_ref() {
            values.extend(self.collect_values_in_instruction(terminator));
        }
        values
    }

    pub(crate) fn collect_values_in_instruction(
        &self,
        instruction: &MirInstruction,
    ) -> Vec<ValueId> {
        if let MirInstruction::LegacyCallV0 {
            dst,
            func,
            callee,
            args,
            ..
        } = instruction
        {
            let mut values = Vec::new();
            if let Some(callee) = callee {
                callee.for_each_value_operand(|value| values.push(value));
            } else if *func != ValueId::INVALID {
                values.push(*func);
            }
            if let Some(dst) = dst {
                values.push(*dst);
            }
            values.extend(args.iter().copied());
            return values;
        }

        let mut values = Vec::new();
        if let Some(dst) = instruction.dst_value() {
            values.push(dst);
        }
        values.extend(instruction.used_values());
        values
    }
}

#[cfg(test)]
mod tests {
    use super::MirValueIdInventory;
    use crate::mir::{Callee, EffectMask, MirInstruction, ValueId};

    #[test]
    fn call_inventory_uses_callee_operand_projection_before_destination_and_args() {
        let instruction = MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(30)),
            func: ValueId::INVALID,
            callee: Some(Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![("a".to_string(), ValueId::new(7))],
                me_capture: Some(ValueId::new(9)),
            }),
            args: vec![ValueId::new(11)],
            effects: EffectMask::PURE,
        };

        assert_eq!(
            MirValueIdInventory::new().collect_values_in_instruction(&instruction),
            vec![
                ValueId::new(7),
                ValueId::new(9),
                ValueId::new(30),
                ValueId::new(11)
            ]
        );
    }

    #[test]
    fn legacy_call_inventory_ignores_invalid_sentinel_but_keeps_real_func() {
        let instruction = MirInstruction::LegacyCallV0 {
            dst: None,
            func: ValueId::new(4),
            callee: None,
            args: vec![ValueId::new(5)],
            effects: EffectMask::PURE,
        };

        assert_eq!(
            MirValueIdInventory::new().collect_values_in_instruction(&instruction),
            vec![ValueId::new(4), ValueId::new(5)]
        );
    }
}
