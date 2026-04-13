/*!
 * Narrow string corridor relation layer.
 *
 * This module consumes the generic MIR PHI base-relation seam and records
 * string-corridor continuity as metadata. It does not own PHI semantics, and
 * it does not emit placement/effect candidates itself.
 */

use super::{
    phi_query::{collect_phi_carry_relations, PhiBaseRelation},
    resolve_value_origin, build_value_def_map, ValueDefMap,
    string_corridor_recognizer::{
        match_add_in_block, match_len_call, match_substring_call,
        match_substring_concat3_helper_call, string_source_identity,
    },
    MirFunction, MirInstruction, MirModule, ValueId,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorRelationKind {
    PhiCarryBase,
    StableLengthScalar,
}

impl std::fmt::Display for StringCorridorRelationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PhiCarryBase => f.write_str("phi_carry_base"),
            Self::StableLengthScalar => f.write_str("stable_length_scalar"),
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
    pub witness_value: Option<ValueId>,
    pub reason: &'static str,
}

impl StringCorridorRelation {
    pub fn summary(&self) -> String {
        match self.witness_value {
            Some(witness) => format!(
                "{} base=%{} witness=%{} window={} {}",
                self.kind, self.base_value.0, witness.0, self.window_contract, self.reason
            ),
            None => format!(
                "{} base=%{} window={} {}",
                self.kind, self.base_value.0, self.window_contract, self.reason
            ),
        }
    }
}

fn find_phi_inputs(
    function: &MirFunction,
    phi_value: ValueId,
) -> Option<Vec<(super::BasicBlockId, ValueId)>> {
    for block in function.blocks.values() {
        for inst in &block.instructions {
            if let MirInstruction::Phi { dst, inputs, .. } = inst {
                if *dst == phi_value {
                    return Some(inputs.clone());
                }
            }
        }
    }
    None
}

fn value_is_const_i64(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    expected: i64,
) -> bool {
    let root = resolve_value_origin(function, &def_map, value);
    let Some((bbid, idx)) = def_map.get(&root).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&bbid) else {
        return false;
    };
    matches!(
        block.instructions.get(idx),
        Some(MirInstruction::Const {
            value: super::ConstValue::Integer(actual),
            ..
        }) if *actual == expected
    )
}

fn same_or_same_const_i64(
    function: &MirFunction,
    def_map: &ValueDefMap,
    lhs: ValueId,
    rhs: ValueId,
) -> bool {
    if lhs == rhs {
        return true;
    }

    let lhs_root = resolve_value_origin(function, &def_map, lhs);
    let rhs_root = resolve_value_origin(function, &def_map, rhs);
    if lhs_root == rhs_root {
        return true;
    }

    let Some((lhs_bbid, lhs_idx)) = def_map.get(&lhs_root).copied() else {
        return false;
    };
    let Some((rhs_bbid, rhs_idx)) = def_map.get(&rhs_root).copied() else {
        return false;
    };
    let Some(lhs_block) = function.blocks.get(&lhs_bbid) else {
        return false;
    };
    let Some(rhs_block) = function.blocks.get(&rhs_bbid) else {
        return false;
    };
    matches!(
        (lhs_block.instructions.get(lhs_idx), rhs_block.instructions.get(rhs_idx)),
        (
            Some(MirInstruction::Const {
                value: super::ConstValue::Integer(lhs_val),
                ..
            }),
            Some(MirInstruction::Const {
                value: super::ConstValue::Integer(rhs_val),
                ..
            })
        ) if lhs_val == rhs_val
    )
}

fn entry_length_value_for_phi(
    function: &MirFunction,
    def_map: &ValueDefMap,
    phi_value: ValueId,
) -> Option<ValueId> {
    let inputs = find_phi_inputs(function, phi_value)?;
    let (entry_bbid, entry_value) = inputs.iter().min_by_key(|(bbid, _)| bbid.0).copied()?;
    let entry_identity = string_source_identity(function, &def_map, entry_value)?;
    let block = function.blocks.get(&entry_bbid)?;

    for inst in &block.instructions {
        let Some((dst, receiver, _effects)) = match_len_call(inst) else {
            continue;
        };
        let Some(receiver_identity) = string_source_identity(function, &def_map, receiver) else {
            continue;
        };
        if receiver_identity == entry_identity {
            return Some(resolve_value_origin(function, &def_map, dst));
        }
    }

    None
}

fn plan_window_preserves_length_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    start: ValueId,
    end: ValueId,
    length_value: ValueId,
) -> bool {
    let start_root = resolve_value_origin(function, &def_map, start);
    let end_root = resolve_value_origin(function, &def_map, end);
    let length_root = resolve_value_origin(function, &def_map, length_value);

    if end_root == length_root {
        return value_is_const_i64(function, def_map, start_root, 0);
    }

    let Some((end_bbid, _)) = def_map.get(&end_root).copied() else {
        return false;
    };
    let Some(add_shape) = match_add_in_block(function, end_bbid, &def_map, end_root) else {
        return false;
    };
    let lhs_root = resolve_value_origin(function, &def_map, add_shape.lhs);
    let rhs_root = resolve_value_origin(function, &def_map, add_shape.rhs);
    (same_or_same_const_i64(function, def_map, lhs_root, start_root) && rhs_root == length_root)
        || (lhs_root == length_root
            && same_or_same_const_i64(function, def_map, rhs_root, start_root))
}

fn stable_length_relation_for_phi(
    function: &MirFunction,
    def_map: &ValueDefMap,
    phi_value: ValueId,
    base_value: ValueId,
) -> Option<StringCorridorRelation> {
    let length_value = entry_length_value_for_phi(function, def_map, phi_value)?;
    let base_root = resolve_value_origin(function, &def_map, base_value);
    let (bbid, idx) = def_map.get(&base_root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let inst = block.instructions.get(idx)?;
    let (start, end) = if let Some(shape) = match_substring_concat3_helper_call(inst) {
        (shape.start, shape.end)
    } else if let Some((_dst, _receiver, start, end, _effects)) = match_substring_call(inst) {
        (start, end)
    } else {
        return None;
    };
    if !plan_window_preserves_length_value(function, def_map, start, end, length_value) {
        return None;
    }

    Some(StringCorridorRelation {
        kind: StringCorridorRelationKind::StableLengthScalar,
        base_value,
        witness_value: Some(length_value),
        window_contract: StringCorridorWindowContract::StopAtMerge,
        reason:
            "merged phi route keeps the entry scalar source length stable even while the proof-bearing plan window stops at the merge",
    })
}

pub fn refresh_module_string_corridor_relations(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_corridor_relations(function);
    }
}

pub fn refresh_function_string_corridor_relations(function: &mut MirFunction) {
    function.metadata.string_corridor_relations.clear();
    let anchors = function
        .metadata
        .string_corridor_facts
        .keys()
        .copied()
        .collect::<BTreeSet<_>>();
    if anchors.is_empty() {
        return;
    }
    let def_map = build_value_def_map(function);

    for relation in collect_phi_carry_relations(function, &anchors) {
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
                witness_value: None,
                reason: if relation.window_safe {
                    "single-input phi continuity keeps the current string corridor lane and preserves the proof-bearing plan window"
                } else {
                    "merged phi continuity keeps the current string corridor lane but stops the proof-bearing plan window at the merge"
                },
            });

        if !relation.window_safe {
            if let Some(stable_length) =
                stable_length_relation_for_phi(function, &def_map, relation.phi_value, base_value)
            {
                function
                    .metadata
                    .string_corridor_relations
                    .entry(relation.phi_value)
                    .or_default()
                    .push(stable_length);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        refresh_function_string_corridor_facts, BasicBlock, BasicBlockId, Callee, ConstValue,
        EffectMask, FunctionSignature, MirCompiler, MirInstruction, MirType,
    };
    use crate::runner::modes::common_util::source_hint::prepare_source_minimal;
    use crate::NyashParser;

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        let _ = std::panic::catch_unwind(|| {
            init_global_ring0(default_ring0());
        });
    }

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

    #[test]
    fn refresh_function_skips_phi_scan_when_no_string_corridor_anchors_exist() {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        function
            .get_block_mut(BasicBlockId(0))
            .expect("entry")
            .instructions
            .push(MirInstruction::Phi {
                dst: ValueId(1),
                inputs: vec![(BasicBlockId(0), ValueId(2))],
                type_hint: Some(MirType::Integer),
            });

        refresh_function_string_corridor_relations(&mut function);

        assert!(function.metadata.string_corridor_relations.is_empty());
    }

    #[test]
    fn refresh_function_records_stable_length_scalar_on_substring_concat_loop() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_substring_concat.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared = prepare_source_minimal(&source, path).expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");
        let main = result.module.functions.get("main").expect("main");
        let header_relations = main
            .metadata
            .string_corridor_relations
            .get(&ValueId(21))
            .expect("phi %21 relations");

        assert!(header_relations.iter().any(|relation| {
            relation.kind == StringCorridorRelationKind::StableLengthScalar
                && relation.base_value == ValueId(36)
                && relation.witness_value == Some(ValueId(5))
                && relation.window_contract == StringCorridorWindowContract::StopAtMerge
        }));
    }
}
