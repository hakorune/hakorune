//! Test-only ratchet for the all-route Loop pre-effect product boundary.
//!
//! This records the current execution shape without authorizing a route change.
//! The future all-route producer must replace these post-effect `None` paths;
//! this P0 module only makes the debt explicit and cardinality-checked.

use super::{types::LoopRouteId, ENTRIES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QualificationBoundary {
    FactsAndContract,
    FactsContractAndReleaseGate,
    FactsAndNestedGate,
    FactsContractAndNestedGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FirstBuilderEffect {
    LoopV0Frame,
    DirectComposerBlocks,
    LoopCondFrame,
    GenericSkeleton,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PostEffectNone {
    LowererResultFlowsToScheduler,
    GenericReleaseFailureBecomesNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EffectOrderRow {
    route: LoopRouteId,
    selection: &'static str,
    qualification: QualificationBoundary,
    first_effect: FirstBuilderEffect,
    post_effect_none: PostEffectNone,
}

// `route` order is intentionally the production registry order. The remaining
// fields describe source-level boundaries only; they are not route policy.
const EFFECT_ORDER_MATRIX: &[EffectOrderRow] = &[
    EffectOrderRow {
        route: LoopRouteId::LoopBreakRecipe,
        selection: "loop_break",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::IfPhiJoin,
        selection: "if_phi_join",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopContinueOnly,
        selection: "loop_continue_only",
        qualification: QualificationBoundary::FactsContractAndNestedGate,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopTrueEarlyExit,
        selection: "loop_true_early_exit",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopSimpleWhile,
        selection: "loop_simple_while && !nested",
        qualification: QualificationBoundary::FactsContractAndNestedGate,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCharMap,
        selection: "loop_char_map",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopArrayJoin,
        selection: "loop_array_join",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::ScanWithInit,
        selection: "scan_with_init",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::SplitScan,
        selection: "split_scan",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::BoolPredicateScan,
        selection: "bool_predicate_scan",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::AccumConstLoop,
        selection: "accum_const_loop",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopV0Frame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::NestedLoopMinimal,
        selection: "nested_loop_minimal",
        qualification: QualificationBoundary::FactsAndNestedGate,
        first_effect: FirstBuilderEffect::DirectComposerBlocks,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopTrueBreakContinue,
        selection: "loop_true_break_continue && !loop_break",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_effect: FirstBuilderEffect::DirectComposerBlocks,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondBreakContinue,
        selection: "loop_cond_break_continue; !loop_break; !scan; !return_only",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_effect: FirstBuilderEffect::LoopCondFrame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondContinueOnly,
        selection: "loop_cond_continue_only && !loop_continue_only",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_effect: FirstBuilderEffect::LoopCondFrame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondContinueWithReturn,
        selection: "loop_cond_continue_with_return",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_effect: FirstBuilderEffect::LoopCondFrame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondReturnInBody,
        selection: "loop_cond_return_in_body; !scan; return_only",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::LoopCondFrame,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::GenericLoopV0,
        selection: "generic_loop_v0",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::GenericSkeleton,
        post_effect_none: PostEffectNone::GenericReleaseFailureBecomesNone,
    },
    EffectOrderRow {
        route: LoopRouteId::GenericLoopV1,
        selection: "generic_loop_v1; !break; !simple; !cond_break; !scan",
        qualification: QualificationBoundary::FactsAndContract,
        first_effect: FirstBuilderEffect::GenericSkeleton,
        post_effect_none: PostEffectNone::GenericReleaseFailureBecomesNone,
    },
];

#[test]
fn effect_order_matrix_is_exactly_the_production_registry() {
    let production_order = ENTRIES.iter().map(|entry| entry.id).collect::<Vec<_>>();
    let matrix_order = EFFECT_ORDER_MATRIX
        .iter()
        .map(|row| row.route)
        .collect::<Vec<_>>();

    assert_eq!(matrix_order, production_order);
    assert_eq!(EFFECT_ORDER_MATRIX.len(), 19);
}

#[test]
fn generic_release_failures_are_the_only_explicit_post_effect_none_conversion() {
    let generic_routes = EFFECT_ORDER_MATRIX
        .iter()
        .filter_map(|row| {
            (row.post_effect_none == PostEffectNone::GenericReleaseFailureBecomesNone)
                .then_some(row.route)
        })
        .collect::<Vec<_>>();

    assert_eq!(
        generic_routes,
        [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
}

#[test]
fn every_route_reaches_a_named_physical_boundary_after_qualification() {
    assert!(EFFECT_ORDER_MATRIX.iter().all(|row| {
        !row.selection.is_empty()
            && matches!(
                row.qualification,
                QualificationBoundary::FactsAndContract
                    | QualificationBoundary::FactsContractAndReleaseGate
                    | QualificationBoundary::FactsAndNestedGate
                    | QualificationBoundary::FactsContractAndNestedGate
            )
            && matches!(
                row.first_effect,
                FirstBuilderEffect::LoopV0Frame
                    | FirstBuilderEffect::DirectComposerBlocks
                    | FirstBuilderEffect::LoopCondFrame
                    | FirstBuilderEffect::GenericSkeleton
            )
    }));
}
