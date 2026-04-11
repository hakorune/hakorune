/*!
 * Generic value-origin queries.
 *
 * This module owns copy-chain normalization for MIR values. Domain recognizers
 * may consume these queries, but they must not re-own the traversal rules.
 */

use super::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeSet, HashMap};

pub type ValueDefMap = HashMap<ValueId, (BasicBlockId, usize)>;
pub type CopyParentMap = HashMap<ValueId, ValueId>;

pub fn build_value_def_map(function: &MirFunction) -> ValueDefMap {
    let mut defs = ValueDefMap::new();
    for (bbid, block) in &function.blocks {
        for (idx, inst) in block.instructions.iter().enumerate() {
            if let Some(dst) = inst.dst_value() {
                defs.insert(dst, (*bbid, idx));
            }
        }
    }
    defs
}

pub fn resolve_value_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    mut value: ValueId,
) -> ValueId {
    let mut visited = BTreeSet::new();
    while visited.insert(value) {
        let Some((bbid, idx)) = def_map.get(&value).copied() else {
            break;
        };
        let Some(block) = function.blocks.get(&bbid) else {
            break;
        };
        let Some(inst) = block.instructions.get(idx) else {
            break;
        };
        match inst {
            MirInstruction::Copy { src, .. } => value = *src,
            _ => break,
        }
    }
    value
}

pub fn resolve_value_origin_from_copy_parents(
    mut value: ValueId,
    copy_parents: &CopyParentMap,
) -> ValueId {
    let mut seen = BTreeSet::new();
    while let Some(parent) = copy_parents.get(&value) {
        if !seen.insert(value) {
            break;
        }
        value = *parent;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirModule, MirType};

    #[test]
    fn resolve_value_origin_follows_copy_chain_in_function() {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Point".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        function.add_block(block);

        let def_map = build_value_def_map(&function);
        assert_eq!(
            resolve_value_origin(&function, &def_map, ValueId::new(3)),
            ValueId::new(1)
        );
    }

    #[test]
    fn resolve_value_origin_from_copy_parents_breaks_cycles() {
        let mut copy_parents = CopyParentMap::new();
        copy_parents.insert(ValueId::new(2), ValueId::new(1));
        copy_parents.insert(ValueId::new(1), ValueId::new(2));

        let resolved = resolve_value_origin_from_copy_parents(ValueId::new(2), &copy_parents);
        assert!(matches!(resolved, ValueId(1) | ValueId(2)));
    }

    #[test]
    fn build_value_def_map_records_defined_values() {
        let mut module = MirModule::new("value_origin_test".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(MirInstruction::Const {
                dst: ValueId::new(1),
                value: crate::mir::ConstValue::Integer(1),
            });
        module.add_function(function);

        let function = module.get_function("main").expect("main");
        let def_map = build_value_def_map(function);
        assert!(def_map.contains_key(&ValueId::new(1)));
    }
}
