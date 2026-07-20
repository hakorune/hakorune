//! Temporal witnesses for direct ArrayElementWrite receiver observation.

use super::{CallTarget, UnifiedCallEmitterBox};
use crate::mir::builder::MirBuilder;
use crate::mir::{EffectMask, MirInstruction, MirType, ValueId};

fn builder_with_entry(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
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

fn array_write_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .into_iter()
        .flat_map(|function| function.blocks.values())
        .flat_map(|block| block.instructions.iter())
        .filter(|instruction| matches!(instruction, MirInstruction::ArrayElementWrite { .. }))
        .count()
}

fn install_array_receiver_and_integer_value(builder: &mut MirBuilder) -> (ValueId, ValueId) {
    let receiver = builder.alloc_value_for_test();
    let value = builder.alloc_value_for_test();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(receiver, MirType::Box("ArrayBox".to_string()));
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    (receiver, value)
}

fn assert_array_receiver_is_unobserved(builder: &MirBuilder, receiver: ValueId) {
    assert_eq!(array_write_count(builder), 0);
    assert_eq!(
        transient_type(builder, receiver),
        Some(MirType::Box("ArrayBox".to_string()))
    );
}

fn assert_array_receiver_is_observed(builder: &MirBuilder, receiver: ValueId) {
    assert_eq!(array_write_count(builder), 1);
    assert_eq!(
        transient_type(builder, receiver),
        Some(MirType::Array(Box::new(MirType::Integer)))
    );
}

#[test]
fn unified_array_write_failure_publishes_no_receiver_observation() {
    let mut builder = builder_with_entry("array_write_unified_failure/0");
    let (receiver, value) = install_array_receiver_and_integer_value(&mut builder);
    builder.function_state.current_block = None;

    let error = UnifiedCallEmitterBox::emit_unified_call_impl(
        &mut builder,
        None,
        CallTarget::Method {
            box_type: Some("ArrayBox".to_string()),
            method: "push".to_string(),
            receiver,
        },
        vec![value],
    )
    .unwrap_err();

    assert_eq!(error, "No current basic block");
    assert_array_receiver_is_unobserved(&builder, receiver);
}

#[test]
fn unified_array_write_success_observes_semantic_receiver_after_emission() {
    let mut builder = builder_with_entry("array_write_unified_success/0");
    let (receiver, value) = install_array_receiver_and_integer_value(&mut builder);

    UnifiedCallEmitterBox::emit_unified_call_impl(
        &mut builder,
        None,
        CallTarget::Method {
            box_type: Some("ArrayBox".to_string()),
            method: "push".to_string(),
            receiver,
        },
        vec![value],
    )
    .unwrap();

    assert_array_receiver_is_observed(&builder, receiver);
}

#[test]
fn boxcall_array_write_failure_publishes_no_receiver_observation() {
    let mut builder = builder_with_entry("array_write_boxcall_failure/0");
    let (receiver, value) = install_array_receiver_and_integer_value(&mut builder);
    builder.function_state.current_block = None;

    let error = builder
        .emit_box_or_plugin_call(
            None,
            receiver,
            "push".to_string(),
            None,
            vec![value],
            EffectMask::PURE,
        )
        .unwrap_err();

    assert_eq!(error, "No current basic block");
    assert_array_receiver_is_unobserved(&builder, receiver);
}

#[test]
fn boxcall_array_write_success_observes_semantic_receiver_after_emission() {
    let mut builder = builder_with_entry("array_write_boxcall_success/0");
    let (receiver, value) = install_array_receiver_and_integer_value(&mut builder);

    builder
        .emit_box_or_plugin_call(
            None,
            receiver,
            "push".to_string(),
            None,
            vec![value],
            EffectMask::PURE,
        )
        .unwrap();

    assert_array_receiver_is_observed(&builder, receiver);
}
