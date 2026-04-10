/*!
 * Narrow string corridor PHI continuity helpers.
 *
 * This module consumes the generic MIR PHI base-relation seam for the current
 * string corridor lane. It maps generic PHI continuity onto string-corridor
 * carries only; it does not own PHI semantics and it does not widen plan
 * windows across PHI.
 */

use super::{
    phi_query::{collect_phi_carry_relations, PhiBaseRelation},
    string_corridor_recognizer::resolve_copy_chain_source,
    BasicBlockId, MirFunction, ValueId,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StringCorridorPhiCarry {
    pub phi_value: ValueId,
    pub base_value: ValueId,
    pub carries_plan_window: bool,
}

pub(crate) fn collect_string_corridor_phi_carries(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Vec<StringCorridorPhiCarry> {
    collect_phi_carry_relations(
        function,
        def_map,
        |value| resolve_copy_chain_source(function, def_map, value),
        |value| function.metadata.string_corridor_facts.contains_key(&value),
    )
    .into_iter()
    .filter_map(|relation| match relation.relation {
        PhiBaseRelation::SameBase(base_value) if base_value != relation.phi_value => {
            Some(StringCorridorPhiCarry {
                phi_value: relation.phi_value,
                base_value,
                carries_plan_window: relation.window_safe,
            })
        }
        _ => None,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        refresh_function_string_corridor_facts, BasicBlock, Callee, ConstValue, EffectMask,
        FunctionSignature, MirInstruction, MirType,
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
            carry.phi_value == ValueId(22)
                && carry.base_value == ValueId(36)
                && carry.carries_plan_window
        }));
        assert!(carries.iter().any(|carry| {
            carry.phi_value == ValueId(21)
                && carry.base_value == ValueId(36)
                && !carry.carries_plan_window
        }));
    }
}
