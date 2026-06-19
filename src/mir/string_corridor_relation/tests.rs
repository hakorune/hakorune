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
fn refresh_function_preserves_typed_stable_length_relation() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "typed_stable_length".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    let base = ValueId(1);
    let witness = ValueId(2);
    function
        .get_block_mut(BasicBlockId(0))
        .expect("entry")
        .instructions
        .extend([
            MirInstruction::Const {
                dst: base,
                value: ConstValue::String("abc".to_string()),
            },
            MirInstruction::Const {
                dst: witness,
                value: ConstValue::Integer(3),
            },
        ]);
    function
        .metadata
        .string_corridor_relations
        .entry(base)
        .or_default()
        .push(StringCorridorRelation {
            kind: StringCorridorRelationKind::StableLengthScalar,
            base_value: base,
            witness_value: Some(witness),
            window_contract: StringCorridorWindowContract::PreservePlanWindow,
            reason: "typed stable length relation from producer",
        });

    refresh_function_string_corridor_relations(&mut function);

    let relations = function
        .metadata
        .string_corridor_relations
        .get(&base)
        .expect("typed relation should survive refresh");
    assert!(relations.iter().any(|relation| {
        relation.kind == StringCorridorRelationKind::StableLengthScalar
            && relation.base_value == base
            && relation.witness_value == Some(witness)
            && relation.window_contract == StringCorridorWindowContract::PreservePlanWindow
    }));
}

#[test]
fn refresh_function_records_stable_length_scalar_on_substring_only_benchmark() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_micro_substring_only.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared = prepare_source_minimal(&source, path).expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");
    let main = result.module.functions.get("main").expect("main");
    let def_map = build_value_def_map(main);
    let mut length_diagnostics = Vec::new();
    for (bbid, block) in &main.blocks {
        for inst in &block.instructions {
            if let Some((dst, receiver, _effects)) = match_len_call(inst) {
                let receiver_root = resolve_value_origin(main, &def_map, receiver);
                let root_inst = def_map
                    .get(&receiver_root)
                    .and_then(|(root_bbid, root_idx)| {
                        main.blocks
                            .get(root_bbid)
                            .and_then(|root_block| root_block.instructions.get(*root_idx))
                    })
                    .cloned();
                length_diagnostics.push(format!(
                    "bb{} dst=%{} recv=%{} root=%{} root_inst={root_inst:?}",
                    bbid.0, dst.0, receiver.0, receiver_root.0
                ));
            }
        }
    }
    let relation_summary = main
        .metadata
        .string_corridor_relations
        .iter()
        .map(|(base, relations)| {
            format!(
                "base=%{}:[{}]",
                base.0,
                relations
                    .iter()
                    .map(StringCorridorRelation::summary)
                    .collect::<Vec<_>>()
                    .join(" | ")
            )
        })
        .collect::<Vec<_>>();

    assert!(
        main.metadata
            .string_corridor_relations
            .values()
            .flatten()
            .any(|relation| relation.kind == StringCorridorRelationKind::StableLengthScalar),
        "substring_only benchmark should expose at least one stable length relation; length_calls={length_diagnostics:?}; hints={:?}; relations={relation_summary:?}",
        main.metadata.optimization_hints
    );
}

#[test]
fn refresh_function_records_stable_length_scalar_on_len_substring_views_benchmark() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_micro_len_substring_views.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared = prepare_source_minimal(&source, path).expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");
    let main = result.module.functions.get("main").expect("main");
    let def_map = build_value_def_map(main);
    let mut length_diagnostics = Vec::new();
    for (bbid, block) in &main.blocks {
        for inst in &block.instructions {
            if let Some((dst, receiver, _effects)) = match_len_call(inst) {
                let receiver_root = resolve_value_origin(main, &def_map, receiver);
                let root_inst = def_map
                    .get(&receiver_root)
                    .and_then(|(root_bbid, root_idx)| {
                        main.blocks
                            .get(root_bbid)
                            .and_then(|root_block| root_block.instructions.get(*root_idx))
                    })
                    .cloned();
                length_diagnostics.push(format!(
                    "bb{} dst=%{} recv=%{} root=%{} root_inst={root_inst:?}",
                    bbid.0, dst.0, receiver.0, receiver_root.0
                ));
            }
        }
    }
    let relation_summary = main
        .metadata
        .string_corridor_relations
        .iter()
        .map(|(base, relations)| {
            format!(
                "base=%{}:[{}]",
                base.0,
                relations
                    .iter()
                    .map(StringCorridorRelation::summary)
                    .collect::<Vec<_>>()
                    .join(" | ")
            )
        })
        .collect::<Vec<_>>();

    assert!(
        main.metadata
            .string_corridor_relations
            .values()
            .flatten()
            .any(|relation| relation.kind == StringCorridorRelationKind::StableLengthScalar),
        "len_substring_views benchmark should expose at least one stable length relation; length_calls={length_diagnostics:?}; hints={:?}; relations={relation_summary:?}",
        main.metadata.optimization_hints
    );
}
