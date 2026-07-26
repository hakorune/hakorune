//! Focused witnesses for the source-neutral generic Call receipt.

use super::physical_terminal::{
    CompletedUnifiedCallEmissionV1, UnifiedCallAlternateRouteV1, UnifiedCallEmissionOutcomeV1,
};
use super::{CallTarget, UnifiedCallEmitterBox, UnifiedValueCallReceiptErrorV1};
use crate::mir::builder::MirBuilder;
use crate::mir::{MirInstruction, ValueId};

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
            MirInstruction::Call { dst, .. } => Some(*dst),
            _ => None,
        })
        .collect()
}

#[test]
fn generic_value_call_receipt_matches_the_emitted_final_destination() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("physical_receipt_success/0");
        let destination = builder.alloc_value_for_test();

        let receipt = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Global("physical_receipt_probe/0".to_string()),
            vec![],
            None,
        )
        .expect("generic physical Call receipt");

        assert_eq!(receipt.final_destination(), destination);
        assert_eq!(call_destinations(&builder), vec![Some(destination)]);
    });
}

#[test]
fn failed_generic_call_emission_issues_no_receipt() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder_with_entry("physical_receipt_failure/0");
        let destination = builder.alloc_value_for_test();
        builder.function_state.current_block = None;

        let error = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut builder,
            destination,
            CallTarget::Global("physical_receipt_probe/0".to_string()),
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
    });
}

#[test]
fn generic_call_without_destination_has_no_value_receipt() {
    let mut builder = builder_with_entry("physical_receipt_no_destination/0");

    let outcome = UnifiedCallEmitterBox::emit_unified_call_outcome_impl_with_lookup_and_map_replay(
        &mut builder,
        None,
        CallTarget::Global("physical_receipt_probe/0".to_string()),
        vec![],
        None,
        None,
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

        let error = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            &mut receipt_builder,
            destination,
            CallTarget::Global("physical_receipt_probe/0".to_string()),
            vec![],
            None,
        )
        .expect_err("receipt-required route must not retry through legacy emission");

        assert_eq!(error, UnifiedValueCallReceiptErrorV1::UnifiedDisabled);
        assert!(call_destinations(&receipt_builder).is_empty());

        let mut ordinary_builder = builder_with_entry("physical_receipt_legacy_parity/0");
        let ordinary_destination = ordinary_builder.alloc_value_for_test();
        UnifiedCallEmitterBox::emit_unified_call(
            &mut ordinary_builder,
            Some(ordinary_destination),
            CallTarget::Global("physical_receipt_probe/0".to_string()),
            vec![],
        )
        .expect("ordinary compatibility facade retains legacy behavior");
        assert_eq!(
            call_destinations(&ordinary_builder),
            vec![Some(ordinary_destination)]
        );
    });
}
