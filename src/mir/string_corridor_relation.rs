/*!
 * Narrow string corridor relation layer.
 *
 * This module consumes the generic MIR PHI base-relation seam and records
 * string-corridor continuity as metadata. It does not own PHI semantics, and
 * it does not emit placement/effect candidates itself.
 */

use super::{
    phi_query::{collect_phi_carry_relations, PhiBaseRelation},
    string_corridor_recognizer::{build_def_map, resolve_copy_chain_source},
    MirFunction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorRelationKind {
    PhiCarryBase,
}

impl std::fmt::Display for StringCorridorRelationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PhiCarryBase => f.write_str("phi_carry_base"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorWindowContract {
    PreservePlanWindow,
    StopAtMerge,
}

impl StringCorridorWindowContract {
    pub fn preserves_plan_window(self) -> bool {
        matches!(self, Self::PreservePlanWindow)
    }
}

impl std::fmt::Display for StringCorridorWindowContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreservePlanWindow => f.write_str("preserve_plan_window"),
            Self::StopAtMerge => f.write_str("stop_at_merge"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCorridorRelation {
    pub kind: StringCorridorRelationKind,
    pub base_value: ValueId,
    pub window_contract: StringCorridorWindowContract,
    pub reason: &'static str,
}

impl StringCorridorRelation {
    pub fn summary(&self) -> String {
        format!(
            "{} base=%{} window={} {}",
            self.kind, self.base_value.0, self.window_contract, self.reason
        )
    }
}

pub fn refresh_module_string_corridor_relations(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_corridor_relations(function);
    }
}

pub fn refresh_function_string_corridor_relations(function: &mut MirFunction) {
    function.metadata.string_corridor_relations.clear();
    let def_map = build_def_map(function);

    for relation in collect_phi_carry_relations(
        function,
        &def_map,
        |value| resolve_copy_chain_source(function, &def_map, value),
        |value| function.metadata.string_corridor_facts.contains_key(&value),
    ) {
        let PhiBaseRelation::SameBase(base_value) = relation.relation else {
            continue;
        };
        if base_value == relation.phi_value {
            continue;
        }
        function
            .metadata
            .string_corridor_relations
            .entry(relation.phi_value)
            .or_default()
            .push(StringCorridorRelation {
                kind: StringCorridorRelationKind::PhiCarryBase,
                base_value,
                window_contract: if relation.window_safe {
                    StringCorridorWindowContract::PreservePlanWindow
                } else {
                    StringCorridorWindowContract::StopAtMerge
                },
                reason: if relation.window_safe {
                    "single-input phi continuity keeps the current string corridor lane and preserves the proof-bearing plan window"
                } else {
                    "merged phi continuity keeps the current string corridor lane but stops the proof-bearing plan window at the merge"
                },
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        refresh_function_string_corridor_facts, BasicBlock, BasicBlockId, Callee, ConstValue,
        EffectMask, FunctionSignature, MirInstruction, MirType,
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
            inputs: vec![
                (BasicBlockId(0), ValueId(0)),
                (BasicBlockId(3), ValueId(22)),
            ],
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
            callee: Some(Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId(26),
                ValueId(66),
                ValueId(27),
                ValueId(71),
                ValueId(72),
            ],
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
    fn refresh_function_records_string_corridor_phi_relations() {
        let mut function = build_narrow_phi_function();

        refresh_function_string_corridor_relations(&mut function);

        let latch_relations = function
            .metadata
            .string_corridor_relations
            .get(&ValueId(22))
            .expect("phi %22 relations");
        assert!(latch_relations.iter().any(|relation| {
            relation.kind == StringCorridorRelationKind::PhiCarryBase
                && relation.base_value == ValueId(36)
                && relation.window_contract == StringCorridorWindowContract::PreservePlanWindow
        }));

        let header_relations = function
            .metadata
            .string_corridor_relations
            .get(&ValueId(21))
            .expect("phi %21 relations");
        assert!(header_relations.iter().any(|relation| {
            relation.kind == StringCorridorRelationKind::PhiCarryBase
                && relation.base_value == ValueId(36)
                && relation.window_contract == StringCorridorWindowContract::StopAtMerge
        }));
    }
}
