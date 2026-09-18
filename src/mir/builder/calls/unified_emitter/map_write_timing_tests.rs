//! MAP-WRITE-OBSERVE0 temporal witnesses for MapBox write observation.
//!
//! They require fact replay only after the selected Call receipt while
//! preserving the established successful receiver coverage by route.

use super::{CallTarget, UnifiedCallEmitterBox};
use crate::mir::builder::MirBuilder;
use crate::mir::{EffectMask, MirInstruction, MirType, ValueId};
use std::collections::BTreeSet;

const UNIFIED_CALL_ENV: &str = "NYASH_MIR_UNIFIED_CALL";

fn builder_with_entry(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn map_facts(builder: &MirBuilder) -> BTreeSet<ValueId> {
    builder
        .function_state
        .type_ctx
        .map_value_types
        .keys()
        .copied()
        .chain(
            builder
                .function_state
                .type_ctx
                .map_literal_value_types
                .keys()
                .map(|(receiver, _)| *receiver),
        )
        .collect()
}

fn call_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .into_iter()
        .flat_map(|function| function.blocks.values())
        .flat_map(|block| block.instructions.iter())
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::Call(_) | MirInstruction::LegacyCallV0 { .. }
            )
        })
        .count()
}

fn install_map_set_inputs(builder: &mut MirBuilder) -> (ValueId, ValueId, ValueId) {
    let receiver = builder.alloc_value_for_test();
    let key = builder.alloc_value_for_test();
    let value = builder.alloc_value_for_test();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(receiver, MirType::Box("MapBox".to_string()));
    builder
        .function_state
        .type_ctx
        .string_literals
        .insert(key, "answer".to_string());
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    (receiver, key, value)
}

fn seed_map_fact(builder: &mut MirBuilder, receiver: ValueId) {
    builder
        .function_state
        .type_ctx
        .map_value_types
        .insert(receiver, MirType::Integer);
    builder
        .function_state
        .type_ctx
        .map_literal_value_types
        .insert((receiver, "answer".to_string()), MirType::Integer);
}

fn assert_seed_map_fact(builder: &MirBuilder, receiver: ValueId) {
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_value_types
            .get(&receiver),
        Some(&MirType::Integer)
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_literal_value_types
            .get(&(receiver, "answer".to_string())),
        Some(&MirType::Integer)
    );
}

#[test]
fn direct_unified_set_failure_publishes_no_map_fact_residual() {
    let mut builder = builder_with_entry("map_write_unified_failure/0");
    let (receiver, key, value) = install_map_set_inputs(&mut builder);
    builder.function_state.current_block = None;

    let error = UnifiedCallEmitterBox::emit_unified_call_impl(
        &mut builder,
        None,
        CallTarget::Method {
            box_type: Some("MapBox".to_string()),
            method: "set".to_string(),
            receiver,
        },
        vec![key, value],
    )
    .unwrap_err();

    assert_eq!(error, "No current basic block");
    assert_eq!(call_count(&builder), 0);
    assert!(map_facts(&builder).is_empty());
}

#[test]
fn direct_unified_delete_and_clear_failure_retain_seeded_facts() {
    for method in ["delete", "clear"] {
        let mut builder = builder_with_entry(&format!("map_write_{method}_failure/0"));
        let (receiver, key, _) = install_map_set_inputs(&mut builder);
        seed_map_fact(&mut builder, receiver);
        builder.function_state.current_block = None;

        let args = if method == "delete" {
            vec![key]
        } else {
            Vec::new()
        };
        let error = UnifiedCallEmitterBox::emit_unified_call_impl(
            &mut builder,
            None,
            CallTarget::Method {
                box_type: Some("MapBox".to_string()),
                method: method.to_string(),
                receiver,
            },
            args,
        )
        .unwrap_err();

        assert_eq!(error, "No current basic block");
        assert_eq!(call_count(&builder), 0);
        assert_seed_map_fact(&builder, receiver);
    }
}

#[test]
fn direct_unified_delete_and_clear_success_remove_seeded_facts_after_one_call() {
    for method in ["delete", "clear"] {
        let mut builder = builder_with_entry(&format!("map_write_{method}_success/0"));
        let (receiver, key, _) = install_map_set_inputs(&mut builder);
        seed_map_fact(&mut builder, receiver);

        let args = if method == "delete" {
            vec![key]
        } else {
            Vec::new()
        };
        UnifiedCallEmitterBox::emit_unified_call_impl(
            &mut builder,
            None,
            CallTarget::Method {
                box_type: Some("MapBox".to_string()),
                method: method.to_string(),
                receiver,
            },
            args,
        )
        .unwrap();

        assert_eq!(call_count(&builder), 1);
        assert!(map_facts(&builder).is_empty(), "{method} success policy");
    }
}

#[test]
fn direct_unified_set_success_preserves_source_and_final_receiver_coverage() {
    let mut builder = builder_with_entry("map_write_unified_success/0");
    let (receiver, key, value) = install_map_set_inputs(&mut builder);

    UnifiedCallEmitterBox::emit_unified_call_impl(
        &mut builder,
        None,
        CallTarget::Method {
            box_type: Some("MapBox".to_string()),
            method: "set".to_string(),
            receiver,
        },
        vec![key, value],
    )
    .unwrap();

    assert_eq!(call_count(&builder), 1);
    assert!(map_facts(&builder).contains(&receiver));
    assert!(
        map_facts(&builder).len() >= 2,
        "LocalSSA final receiver coverage"
    );
}

#[test]
fn terminal_boxcall_set_failure_publishes_no_map_fact_residual() {
    crate::test_support::with_env_var(UNIFIED_CALL_ENV, "off", || {
        let mut builder = builder_with_entry("map_write_boxcall_failure/0");
        let (receiver, key, value) = install_map_set_inputs(&mut builder);
        builder.function_state.current_block = None;

        let error = builder
            .emit_box_or_plugin_call(
                None,
                receiver,
                "set".to_string(),
                None,
                vec![key, value],
                EffectMask::PURE,
            )
            .unwrap_err();

        assert_eq!(error, "No current basic block");
        assert_eq!(call_count(&builder), 0);
        assert!(map_facts(&builder).is_empty());
    });
}

#[test]
fn terminal_boxcall_set_success_observes_only_the_semantic_source_receiver() {
    crate::test_support::with_env_var(UNIFIED_CALL_ENV, "off", || {
        let mut builder = builder_with_entry("map_write_boxcall_success/0");
        let (receiver, key, value) = install_map_set_inputs(&mut builder);

        builder
            .emit_box_or_plugin_call(
                None,
                receiver,
                "set".to_string(),
                None,
                vec![key, value],
                EffectMask::PURE,
            )
            .unwrap();

        assert_eq!(call_count(&builder), 1);
        assert_eq!(map_facts(&builder), BTreeSet::from([receiver]));
    });
}

#[test]
fn boxcall_delegation_success_retains_source_and_local_receiver_coverage() {
    // The unified-on mode is what adds the delegated LocalSSA receiver to the
    // fact set — pin it explicitly so a concurrent env mutation elsewhere can
    // never interleave a different mode (PROCESS_STATE_LOCK serializes all
    // env-mutating tests).
    crate::test_support::with_env_var(UNIFIED_CALL_ENV, "1", || {
        let mut builder = builder_with_entry("map_write_boxcall_delegate_success/0");
        let (receiver, key, value) = install_map_set_inputs(&mut builder);

        builder
            .emit_box_or_plugin_call(
                None,
                receiver,
                "set".to_string(),
                None,
                vec![key, value],
                EffectMask::PURE,
            )
            .unwrap();

        assert_eq!(call_count(&builder), 1);
        assert!(map_facts(&builder).contains(&receiver));
        assert!(
            map_facts(&builder).len() >= 2,
            "delegated LocalSSA receiver coverage"
        );
    });
}
