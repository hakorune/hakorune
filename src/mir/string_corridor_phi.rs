/*!
 * Narrow string corridor PHI continuity helpers.
 *
 * This module is the PHI-side SSOT for the current string corridor lane.
 * It decides only whether a PHI continues an existing corridor value through
 * the current narrow loop-carried shapes. It does not emit optimization
 * candidates and it does not widen plan windows across PHI.
 */

use super::{
    string_corridor_recognizer::resolve_copy_chain_source, BasicBlockId, MirFunction,
    MirInstruction, ValueId,
};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StringCorridorPhiCarry {
    pub phi_value: ValueId,
    pub base_value: ValueId,
}

pub(crate) fn collect_string_corridor_phi_carries(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Vec<StringCorridorPhiCarry> {
    let mut out = Vec::new();
    for block in function.blocks.values() {
        for inst in &block.instructions {
            let MirInstruction::Phi { dst, .. } = inst else {
                continue;
            };
            let Some(base_value) = infer_string_corridor_phi_base(function, def_map, *dst) else {
                continue;
            };
            if base_value == *dst {
                continue;
            }
            out.push(StringCorridorPhiCarry {
                phi_value: *dst,
                base_value,
            });
        }
    }
    out
}

pub(crate) fn infer_string_corridor_phi_base(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<ValueId> {
    let mut visited = BTreeSet::new();
    infer_string_corridor_phi_base_inner(function, def_map, value, &mut visited)
}

fn infer_string_corridor_phi_base_inner(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
    visited: &mut BTreeSet<ValueId>,
) -> Option<ValueId> {
    let root = resolve_copy_chain_source(function, def_map, value);
    if !visited.insert(root) {
        return None;
    }
    if function.metadata.string_corridor_facts.contains_key(&root) {
        return Some(root);
    }

    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let MirInstruction::Phi { inputs, .. } = block.instructions.get(idx)? else {
        return None;
    };

    match inputs.as_slice() {
        [(_, carried)] => infer_string_corridor_phi_base_inner(function, def_map, *carried, visited),
        [(_, lhs), (_, rhs)] => {
            let lhs_base =
                infer_string_corridor_phi_base_inner(function, def_map, *lhs, visited);
            let rhs_base =
                infer_string_corridor_phi_base_inner(function, def_map, *rhs, visited);
            match (lhs_base, rhs_base) {
                (Some(base), None) => Some(base),
                (None, Some(base)) => Some(base),
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        refresh_function_string_corridor_facts, BasicBlock, Callee, ConstValue, EffectMask,
        FunctionSignature, MirType,
    };

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn build_narrow_phi_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        function.add_block(BasicBlock::new(BasicBlockId(1)));
        function.add_block(BasicBlock::new(BasicBlockId(2)));
        function.add_block(BasicBlock::new(BasicBlockId(3)));

        let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        let header = function.blocks.get_mut(&BasicBlockId(1)).expect("header");
        header.instructions.push(MirInstruction::Phi {
            dst: ValueId(21),
            inputs: vec![(BasicBlockId(0), ValueId(0)), (BasicBlockId(3), ValueId(22))],
            type_hint: Some(MirType::Box("RuntimeDataBox".to_string())),
        });
        header.instruction_spans.push(Span::unknown());
        header.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(2),
            edge_args: None,
        });

        let body = function.blocks.get_mut(&BasicBlockId(2)).expect("body");
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(46),
            value: ConstValue::Integer(0),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(47),
            value: ConstValue::Integer(1),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(48),
            value: ConstValue::Integer(2),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(method_call(
            ValueId(26),
            ValueId(21),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(46), ValueId(47)],
        ));
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(method_call(
            ValueId(27),
            ValueId(21),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(47), ValueId(48)],
        ));
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(66),
            value: ConstValue::String("xx".to_string()),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(71),
            value: ConstValue::Integer(1),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(72),
            value: ConstValue::Integer(3),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(36)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("nyash.string.substring_concat3_hhhii".to_string())),
            args: vec![ValueId(26), ValueId(66), ValueId(27), ValueId(71), ValueId(72)],
            effects: EffectMask::PURE,
        });
        body.instruction_spans.push(Span::unknown());
        body.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });

        let latch = function.blocks.get_mut(&BasicBlockId(3)).expect("latch");
        latch.instructions.push(MirInstruction::Phi {
            dst: ValueId(22),
            inputs: vec![(BasicBlockId(2), ValueId(36))],
            type_hint: Some(MirType::Box("RuntimeDataBox".to_string())),
        });
        latch.instruction_spans.push(Span::unknown());
        latch.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        refresh_function_string_corridor_facts(&mut function);
        function
    }

    #[test]
    fn collect_string_corridor_phi_carries_detects_narrow_loop_route() {
        let function = build_narrow_phi_function();
        let def_map = crate::mir::string_corridor_recognizer::build_def_map(&function);

        let carries = collect_string_corridor_phi_carries(&function, &def_map);
        assert!(carries.iter().any(|carry| {
            carry.phi_value == ValueId(22) && carry.base_value == ValueId(36)
        }));
        assert!(carries.iter().any(|carry| {
            carry.phi_value == ValueId(21) && carry.base_value == ValueId(36)
        }));
    }
}
