//! FACT0-P0-T0: in-process publication timing witnesses.
//!
//! These tests observe current producer behavior only. They deliberately do
//! not route a failure around the producer or repair a transient type fact.

use super::{CallTarget, UnifiedCallEmitterBox};
use crate::ast::FieldDecl;
use crate::mir::builder::emission::phi_lifecycle;
use crate::mir::builder::ssa::local;
use crate::mir::builder::MirBuilder;
use crate::mir::function::{FunctionSignature, MirFunction, MirModule};
use crate::mir::{BasicBlock, BasicBlockId, EffectMask, MirInstruction, MirType, ValueId};
use hakorune_mir_core::MirValueKind;
use std::collections::BTreeSet;

const RECEIVER_OWNER: &str = "Fact0TemporalReceiverV1";
const FIELD_OWNER: &str = "Fact0TemporalFieldOwnerV1";
const CALL_TARGET: &str = "Fact0TemporalCall.answer/0";

fn builder_with_entry(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn builder_with_method_entry(name: &str, owner: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder
        .create_method_skeleton(name.to_string(), owner, &[], &[])
        .unwrap();
    builder
}

fn transient_type(builder: &MirBuilder, value: ValueId) -> Option<MirType> {
    builder
        .function_state
        .type_ctx
        .value_types
        .get(&value)
        .cloned()
}

fn metadata_type(builder: &MirBuilder, value: ValueId) -> Option<MirType> {
    builder
        .function_state
        .current_function
        .as_ref()
        .and_then(|function| function.metadata.value_types.get(&value))
        .cloned()
}

fn all_instructions(builder: &MirBuilder) -> impl Iterator<Item = &MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .into_iter()
        .flat_map(|function| function.blocks.values())
        .flat_map(|block| block.instructions.iter())
}

fn receiver_parameter(builder: &mut MirBuilder, owner: &str) -> ValueId {
    builder.setup_method_params(owner, &[]).unwrap();
    *builder
        .function_state
        .variable_ctx
        .variable_map
        .get("me")
        .unwrap()
}

fn explicit_unknown_parameter_builder() -> (MirBuilder, ValueId) {
    let entry = BasicBlockId::new(0);
    let function = MirFunction::new(
        FunctionSignature {
            name: "fact0_temporal_explicit_parameter/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let parameter = function.params[0];
    let mut builder = MirBuilder::new();
    builder.function_state.current_function = Some(function);
    builder.function_state.current_block = Some(entry);
    (builder, parameter)
}

fn finalize_metadata_type(builder: &mut MirBuilder, value: ValueId) -> Option<MirType> {
    builder
        .finalize_function_draft(false)
        .unwrap()
        .metadata
        .value_types
        .get(&value)
        .cloned()
}

fn install_integer_call_target(builder: &mut MirBuilder) {
    let signature = FunctionSignature {
        name: CALL_TARGET.to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut module = MirModule::new("fact0-temporal-call-module".to_string());
    module.add_function(MirFunction::new(signature, BasicBlockId::new(0)));
    builder.current_module = Some(module);
}

fn call_count(builder: &MirBuilder) -> usize {
    all_instructions(builder)
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count()
}

#[test]
fn parameter_publishes_exact_receiver_before_metadata_snapshot() {
    let mut builder = builder_with_method_entry("fact0_temporal_parameter/0", RECEIVER_OWNER);
    let receiver = receiver_parameter(&mut builder, RECEIVER_OWNER);

    assert_eq!(
        transient_type(&builder, receiver),
        Some(MirType::Box(RECEIVER_OWNER.to_string()))
    );
    assert_eq!(
        builder.get_value_kind(receiver),
        Some(MirValueKind::Parameter(0))
    );
    assert_eq!(metadata_type(&builder, receiver), None);
    assert_eq!(
        finalize_metadata_type(&mut builder, receiver),
        Some(MirType::Box(RECEIVER_OWNER.to_string()))
    );
}

#[test]
fn explicit_unknown_parameter_remains_a_legacy_non_fact() {
    let (mut builder, parameter) = explicit_unknown_parameter_builder();
    builder.setup_function_params(&["arg".to_string()]).unwrap();

    assert_eq!(transient_type(&builder, parameter), Some(MirType::Unknown));
    assert_eq!(metadata_type(&builder, parameter), None);
    assert_eq!(
        finalize_metadata_type(&mut builder, parameter),
        Some(MirType::Unknown)
    );
}

#[test]
fn copy_publishes_only_after_the_copy_instruction_commits() {
    let mut builder = builder_with_method_entry("fact0_temporal_copy/0", RECEIVER_OWNER);
    let receiver = receiver_parameter(&mut builder, RECEIVER_OWNER);
    let copy = local::field_base(&mut builder, receiver);

    assert_ne!(copy, receiver);
    assert!(all_instructions(&builder).any(
        |instruction| matches!(instruction, MirInstruction::Copy { dst, src } if *dst == copy && *src == receiver)
    ));
    assert_eq!(
        transient_type(&builder, copy),
        Some(MirType::Box(RECEIVER_OWNER.to_string()))
    );
    assert_eq!(metadata_type(&builder, copy), None);
    assert_eq!(
        finalize_metadata_type(&mut builder, copy),
        Some(MirType::Box(RECEIVER_OWNER.to_string()))
    );
}

#[derive(Clone, Copy)]
struct PhiIds {
    receiver: ValueId,
    phi: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
    merge_block: BasicBlockId,
    consumer_block: BasicBlockId,
}

fn phi_builder() -> (MirBuilder, PhiIds) {
    let entry = BasicBlockId::new(0);
    let then_block = BasicBlockId::new(1);
    let else_block = BasicBlockId::new(2);
    let merge_block = BasicBlockId::new(3);
    let consumer_block = BasicBlockId::new(4);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "fact0_temporal_phi/1".to_string(),
            params: vec![MirType::Box(RECEIVER_OWNER.to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let receiver = function.next_value_id();
    let phi = function.next_value_id();
    function.params.push(receiver);
    for block in [then_block, else_block, merge_block, consumer_block] {
        function.add_block(BasicBlock::new(block));
    }
    function
        .get_block_mut(entry)
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: receiver,
            then_bb: then_block,
            else_bb: else_block,
            then_edge_args: None,
            else_edge_args: None,
        });
    for (block, target) in [
        (then_block, merge_block),
        (else_block, merge_block),
        (merge_block, consumer_block),
    ] {
        function
            .get_block_mut(block)
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target,
                edge_args: None,
            });
    }
    function.update_cfg();

    let mut builder = MirBuilder::new();
    builder.function_state.current_function = Some(function);
    builder.function_state.current_block = Some(merge_block);
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(receiver, MirType::Box(RECEIVER_OWNER.to_string()));
    (
        builder,
        PhiIds {
            receiver,
            phi,
            then_block,
            else_block,
            merge_block,
            consumer_block,
        },
    )
}

#[test]
fn phi_completion_is_visible_to_the_immediate_copy_before_metadata_snapshot() {
    let (mut builder, ids) = phi_builder();
    phi_lifecycle::define_phi_final_with_type_hint(
        &mut builder,
        ids.merge_block,
        ids.phi,
        vec![
            (ids.then_block, ids.receiver),
            (ids.else_block, ids.receiver),
        ],
        None,
        "fact0-p0-t0",
    )
    .unwrap();
    builder.function_state.current_block = Some(ids.consumer_block);
    let copy = local::field_base(&mut builder, ids.phi);

    assert!(all_instructions(&builder).any(
        |instruction| matches!(instruction, MirInstruction::Phi { dst, .. } if *dst == ids.phi)
    ));
    assert!(all_instructions(&builder).any(
        |instruction| matches!(instruction, MirInstruction::Copy { dst, src } if *dst == copy && *src == ids.phi)
    ));
    for value in [ids.phi, copy] {
        assert_eq!(
            transient_type(&builder, value),
            Some(MirType::Box(RECEIVER_OWNER.to_string()))
        );
        assert_eq!(metadata_type(&builder, value), None);
    }
    let draft = builder.finalize_function_draft(false).unwrap();
    for value in [ids.phi, copy] {
        assert_eq!(
            draft.metadata.value_types.get(&value),
            Some(&MirType::Box(RECEIVER_OWNER.to_string()))
        );
    }
}

#[test]
fn unified_call_publishes_after_successful_call_commit() {
    let mut builder = builder_with_entry("fact0_temporal_call_success/0");
    install_integer_call_target(&mut builder);
    let dst = builder.alloc_value_for_test();

    UnifiedCallEmitterBox::emit_unified_call_impl(
        &mut builder,
        Some(dst),
        CallTarget::Global(CALL_TARGET.to_string()),
        Vec::new(),
    )
    .unwrap();

    assert_eq!(call_count(&builder), 1);
    assert_eq!(transient_type(&builder, dst), Some(MirType::Integer));
    assert_eq!(metadata_type(&builder, dst), None);
    assert_eq!(
        finalize_metadata_type(&mut builder, dst),
        Some(MirType::Integer)
    );
}

#[test]
fn unified_call_failure_leaves_legacy_annotation_residual() {
    let mut builder = builder_with_entry("fact0_temporal_call_failure/0");
    install_integer_call_target(&mut builder);
    let dst = builder.alloc_value_for_test();
    builder.function_state.current_block = None;

    let error = UnifiedCallEmitterBox::emit_unified_call_impl(
        &mut builder,
        Some(dst),
        CallTarget::Global(CALL_TARGET.to_string()),
        Vec::new(),
    )
    .unwrap_err();

    assert_eq!(error, "No current basic block");
    assert_eq!(call_count(&builder), 0);
    assert_eq!(transient_type(&builder, dst), Some(MirType::Integer));
    assert_eq!(metadata_type(&builder, dst), None);
}

fn install_typed_field(builder: &mut MirBuilder) -> ValueId {
    builder.comp_ctx.register_user_box_with_field_decls(
        FIELD_OWNER.to_string(),
        vec![FieldDecl {
            name: "items".to_string(),
            declared_type_name: Some("ArrayBox".to_string()),
            is_weak: false,
            default_value: None,
        }],
    );
    receiver_parameter(builder, FIELD_OWNER)
}

#[test]
fn typed_field_get_publishes_before_fieldget_emission_then_finalizes() {
    let mut builder = builder_with_method_entry("fact0_temporal_field_success/0", FIELD_OWNER);
    let base = install_typed_field(&mut builder);
    let dst = builder
        .build_field_access_from_value(base, "items".to_string())
        .unwrap();

    assert!(all_instructions(&builder).any(
        |instruction| matches!(instruction, MirInstruction::FieldGet { dst: field_dst, declared_type: Some(MirType::Box(name)), .. } if *field_dst == dst && name == "ArrayBox")
    ));
    assert_eq!(
        transient_type(&builder, dst),
        Some(MirType::Box("ArrayBox".to_string()))
    );
    assert_eq!(metadata_type(&builder, dst), None);
    assert_eq!(
        finalize_metadata_type(&mut builder, dst),
        Some(MirType::Box("ArrayBox".to_string()))
    );
}

#[test]
fn typed_field_get_failure_leaves_pre_emission_type_residual() {
    let mut builder = builder_with_method_entry("fact0_temporal_field_failure/0", FIELD_OWNER);
    let base = install_typed_field(&mut builder);
    let before = builder
        .function_state
        .type_ctx
        .value_types
        .keys()
        .copied()
        .collect::<BTreeSet<_>>();
    builder.function_state.current_block = None;

    let error = builder
        .build_field_access_from_value(base, "items".to_string())
        .unwrap_err();
    let residuals = builder
        .function_state
        .type_ctx
        .value_types
        .iter()
        .filter(|(value, _)| !before.contains(value))
        .map(|(value, ty)| (*value, ty.clone()))
        .collect::<Vec<_>>();

    assert_eq!(error, "No current basic block");
    assert_eq!(residuals.len(), 1);
    assert_eq!(residuals[0].1, MirType::Box("ArrayBox".to_string()));
    assert!(!all_instructions(&builder)
        .any(|instruction| matches!(instruction, MirInstruction::FieldGet { .. })));
}
