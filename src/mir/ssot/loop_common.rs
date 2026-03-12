use crate::mir::query::{MirQuery, MirQueryBox};
use crate::mir::{BasicBlockId, BinaryOp, ConstValue, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Apply `var += step` before continue so that header sees updated value.
/// Returns the new ValueId of the variable if updated, otherwise None.
pub fn apply_increment_before_continue(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    vars: &mut BTreeMap<String, ValueId>,
    var_name: &str,
    step: i64,
) -> Option<ValueId> {
    let cur_val = match vars.get(var_name) {
        Some(v) => *v,
        None => return None,
    };
    // If the variable already has a fresh definition in this basic block, do not apply
    // the increment hint again (would double-increment on explicit `var = var + 1; continue`).
    if let Some(bb) = f.blocks.get(&cur_bb) {
        let q = MirQueryBox::new(f);
        let already_written_in_block = bb
            .instructions
            .iter()
            .any(|inst| q.writes_of(inst).iter().any(|w| *w == cur_val));
        if already_written_in_block {
            return None;
        }
    }
    // Emit const step
    let step_v = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.add_instruction(MirInstruction::Const {
            dst: step_v,
            value: ConstValue::Integer(step),
        });
    }
    // Emit add
    let new_v = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.add_instruction(MirInstruction::BinOp {
            dst: new_v,
            op: BinaryOp::Add,
            lhs: cur_val,
            rhs: step_v,
        });
    }
    vars.insert(var_name.to_string(), new_v);
    Some(new_v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{basic_block::BasicBlock, EffectMask, FunctionSignature, MirType};

    fn dummy_func() -> MirFunction {
        let sig = FunctionSignature {
            name: "test.fn/0".into(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        };
        MirFunction::new(sig, BasicBlockId::new(0))
    }

    #[test]
    fn does_not_double_increment_when_var_written_in_block() {
        let mut f = dummy_func();
        let bb0 = BasicBlockId::new(0);
        let bb1 = BasicBlockId::new(1);
        f.blocks.insert(bb1, BasicBlock::new(bb1));

        let cur_val = f.next_value_id();
        f.blocks
            .get_mut(&bb1)
            .unwrap()
            .instructions
            .push(MirInstruction::Const {
                dst: cur_val,
                value: ConstValue::Integer(1),
            });

        let mut vars = BTreeMap::new();
        vars.insert("i".to_string(), cur_val);

        let before = f.blocks.get(&bb1).unwrap().instructions.len();
        let out = apply_increment_before_continue(&mut f, bb1, &mut vars, "i", 1);
        let after = f.blocks.get(&bb1).unwrap().instructions.len();

        assert!(out.is_none());
        assert_eq!(before, after);

        // sanity: still applies when value was defined in a different block
        let cur_val2 = f.next_value_id();
        f.blocks.insert(bb0, BasicBlock::new(bb0));
        f.blocks
            .get_mut(&bb0)
            .unwrap()
            .instructions
            .push(MirInstruction::Const {
                dst: cur_val2,
                value: ConstValue::Integer(1),
            });
        vars.insert("i".to_string(), cur_val2);
        f.blocks.get_mut(&bb1).unwrap().instructions.clear();
        let before2 = f.blocks.get(&bb1).unwrap().instructions.len();
        let out2 = apply_increment_before_continue(&mut f, bb1, &mut vars, "i", 1);
        let after2 = f.blocks.get(&bb1).unwrap().instructions.len();
        assert!(out2.is_some());
        assert_eq!(before2 + 2, after2);
    }
}
