//! Focused witnesses for the source-neutral generic Call receipt.

use super::physical_terminal::{
    CompletedUnifiedCallEmissionV1, UnifiedCallAlternateRouteV1, UnifiedCallEmissionOutcomeV1,
};
use super::{CallTarget, UnifiedCallEmitterBox, UnifiedValueCallReceiptErrorV1};
use crate::mir::builder::MirBuilder;
use crate::mir::{Callee, MirInstruction, ValueId};
use hakorune_mir_defs::{CanonicalGlobalTargetV1, CanonicalSameModuleGlobalTargetV1};

fn builder_with_entry(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn call_destinations(builder: &MirBuilder) -> Vec<Option<ValueId>> {
    builder
        .current_function_instructions()
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call(call) => Some(call.dst),
            MirInstruction::LegacyCallV0 { dst, .. } => Some(*dst),
            _ => None,
        })
        .collect()
}

fn runtime_static_method_target() -> CallTarget {
    CallTarget::Global(
        CanonicalGlobalTargetV1::new_static_box_method("MapBox".into(), "get".into(), 0)
            .expect("test target must have non-empty static components"),
    )
}

fn emitted_callee(builder: &MirBuilder) -> Callee {
    builder
        .current_function_instructions()
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Call(call) => Some(call.callee.clone()),
            MirInstruction::LegacyCallV0 {
                callee: Some(callee),
                ..
            } => Some(callee.clone()),
            _ => None,
        })
        .expect("the policy witness must emit one typed Call")
}

#[test]
fn canonical_snapshot_preserves_runtime_static_global_target() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("methodize_canonical/0");

        UnifiedCallEmitterBox::emit_unified_call(
            &mut builder,
            None,
            runtime_static_method_target(),
            vec![],
        )
        .expect("canonical target should emit");

        assert!(matches!(
            emitted_callee(&builder),
            Callee::Global(CanonicalGlobalTargetV1::SameModule(
                CanonicalSameModuleGlobalTargetV1::StaticBoxMethod { .. }
            ))
        ));
    });
}

#[test]
fn typed_core_ignores_disabled_profile_without_legacy_reentry() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "off", || {
        let mut builder = builder_with_entry("physical_core_profile_independent/0");

        UnifiedCallEmitterBox::emit_unified_call(
            &mut builder,
            None,
            runtime_static_method_target(),
            vec![],
        )
        .expect("typed core must not consult the outer profile gate");

        let (func, callee) = builder
            .current_function_instructions()
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Call(call) => Some((ValueId::INVALID, call.callee.clone())),
                MirInstruction::LegacyCallV0 {
                    func,
                    callee: Some(callee),
                    ..
                } => Some((*func, callee.clone())),
                _ => None,
            })
            .expect("typed core must emit one Call");
        assert_eq!(func, ValueId::INVALID);
        assert!(matches!(callee, Callee::Global(_)));
    });
}

#[test]
fn generic_value_call_receipt_matches_the_emitted_final_destination() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("physical_receipt_success/0");
        builder.recursion_depth = 7;
        let destination = builder.alloc_value_for_test();

        let receipt = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Global(crate::mir::test_global_target(
                "physical_receipt_probe/0".to_string(),
            )),
            vec![],
            None,
        )
        .expect("generic physical Call receipt");

        assert_eq!(receipt.final_destination(), destination);
        assert_eq!(call_destinations(&builder), vec![Some(destination)]);
        assert_eq!(builder.recursion_depth, 7);
    });
}

#[test]
fn failed_generic_call_emission_issues_no_receipt() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("physical_receipt_failure/0");
        builder.recursion_depth = 7;
        let destination = builder.alloc_value_for_test();
        builder.function_state.current_block = None;

        let error = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Global(crate::mir::test_global_target(
                "physical_receipt_probe/0".to_string(),
            )),
            vec![],
            None,
        )
        .expect_err("physical Call failure must issue no receipt");

        assert_eq!(
            error,
            UnifiedValueCallReceiptErrorV1::Emission {
                detail: "No current basic block".into(),
            }
        );
        assert!(call_destinations(&builder).is_empty());
        assert_eq!(builder.recursion_depth, 7);
    });
}

#[test]
fn unified_depth_overflow_restores_entry_depth_without_publication() {
    let _ = std::panic::catch_unwind(|| {
        crate::runtime::ring0::init_global_ring0(crate::runtime::ring0::default_ring0())
    });
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("unified_depth_overflow/0");
        builder.recursion_depth = 100;

        let error = UnifiedCallEmitterBox::emit_unified_call(
            &mut builder,
            None,
            CallTarget::Global(crate::mir::test_global_target(
                "depth_overflow_probe/0".to_string(),
            )),
            vec![],
        )
        .expect_err("unified depth overflow must reject");

        assert!(error.contains("101"));
        assert_eq!(builder.recursion_depth, 100);
        assert!(call_destinations(&builder).is_empty());
    });
}

#[test]
fn generic_call_without_destination_has_no_value_receipt() {
    let mut builder = builder_with_entry("physical_receipt_no_destination/0");

    let outcome = UnifiedCallEmitterBox::emit_unified_call_outcome_impl_with_lookup_and_map_replay(
        &mut builder,
        None,
        CallTarget::Global(crate::mir::test_global_target(
            "physical_receipt_probe/0".to_string(),
        )),
        vec![],
        None,
        None,
        super::post_success::UnifiedCallSignaturePublicationV1::Existing,
    )
    .expect("generic no-destination Call");

    assert!(matches!(
        outcome,
        UnifiedCallEmissionOutcomeV1::Generic(CompletedUnifiedCallEmissionV1::NoDestination)
    ));
    assert_eq!(call_destinations(&builder), vec![None]);
}

#[test]
fn early_string_rewrite_never_becomes_a_generic_value_receipt() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("physical_receipt_early_rewrite/0");
        let receiver = builder.alloc_value_for_test();
        let destination = builder.alloc_value_for_test();

        let error = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Method {
                box_type: Some("IntegerBox".to_string()),
                method: "toString".to_string(),
                receiver,
            },
            vec![],
            None,
        )
        .expect_err("early rewrite is not the generic Call terminal");

        assert_eq!(
            error,
            UnifiedValueCallReceiptErrorV1::AlternateRoute {
                route: UnifiedCallAlternateRouteV1::EarlyStringLikeRewrite,
            }
        );
        assert_eq!(call_destinations(&builder), vec![Some(destination)]);
    });
}

#[test]
fn rewrite_retire_user_method_uses_canonical_method_terminal() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("rewrite_retire_user_method/0");
        let receiver = builder.alloc_value_for_test();
        let destination = builder.alloc_value_for_test();

        let receipt = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Method {
                box_type: Some("UserBox".to_string()),
                method: "run".to_string(),
                receiver,
            },
            vec![],
            None,
        )
        .expect("user methods should reach the generic typed terminal");

        assert_eq!(receipt.final_destination(), destination);
        assert!(matches!(
            emitted_callee(&builder),
            Callee::Method {
                box_name,
                method,
                receiver: Some(_),
                ..
            } if box_name == "UserBox" && method == "run"
        ));
    });
}

#[test]
fn rewrite_retire_equals_uses_canonical_method_terminal() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("rewrite_retire_equals/0");
        let receiver = builder.alloc_value_for_test();
        let argument = builder.alloc_value_for_test();
        let destination = builder.alloc_value_for_test();

        let receipt = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Method {
                box_type: Some("UserBox".to_string()),
                method: "equals".to_string(),
                receiver,
            },
            vec![argument],
            None,
        )
        .expect("equals should use the generic typed method terminal");

        assert_eq!(receipt.final_destination(), destination);
        assert!(matches!(
            emitted_callee(&builder),
            Callee::Method {
                box_name,
                method,
                receiver: Some(_),
                ..
            } if box_name == "UserBox" && method == "equals"
        ));
    });
}

#[test]
fn boxcall_route_never_becomes_a_generic_value_receipt() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("physical_receipt_boxcall/0");
        let receiver = builder.alloc_value_for_test();
        let destination = builder.alloc_value_for_test();

        let error = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Method {
                box_type: Some("UnknownBox".to_string()),
                method: "length".to_string(),
                receiver,
            },
            vec![],
            None,
        )
        .expect_err("BoxCall is not the generic Call terminal");

        assert_eq!(
            error,
            UnifiedValueCallReceiptErrorV1::AlternateRoute {
                route: UnifiedCallAlternateRouteV1::BoxCall,
            }
        );
    });
}

#[test]
fn receipt_requirement_rejects_unified_disabled_without_legacy_fallback() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "off", || {
        let mut receipt_builder = builder_with_entry("physical_receipt_disabled/0");
        let destination = receipt_builder.alloc_value_for_test();

        let error = receipt_builder
            .emit_unified_value_call_with_lookup_receipt_v1(
                destination,
                CallTarget::Global(crate::mir::test_global_target(
                    "physical_receipt_probe/0".to_string(),
                )),
                vec![],
                None,
            )
            .expect_err("receipt-required route must not retry through legacy emission");

        assert_eq!(error, UnifiedValueCallReceiptErrorV1::UnifiedDisabled);
        assert!(call_destinations(&receipt_builder).is_empty());

        let mut ordinary_builder = builder_with_entry("physical_receipt_legacy_parity/0");
        let ordinary_destination = ordinary_builder.alloc_value_for_test();
        ordinary_builder
            .emit_unified_call(
                Some(ordinary_destination),
                CallTarget::Global(crate::mir::test_global_target(
                    "physical_receipt_probe/0".to_string(),
                )),
                vec![],
            )
            .expect("ordinary compatibility facade retains legacy behavior");
        assert_eq!(
            call_destinations(&ordinary_builder),
            vec![Some(ordinary_destination)]
        );
        let legacy_func = ordinary_builder
            .current_function_instructions()
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::LegacyCallV0 { func, .. } => Some(*func),
                _ => None,
            })
            .expect("legacy facade must emit one Call");
        assert_ne!(legacy_func, ValueId::INVALID);
    });
}
