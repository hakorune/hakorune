//! Test-only ratchet for the all-route Loop pre-effect product boundary.
//!
//! This records the current execution shape without authorizing a route change.
//! The future all-route producer must replace these post-effect `None` paths;
//! this P0 module only makes the debt explicit and cardinality-checked.

use super::{route_id::LoopRouteId, ENTRIES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QualificationBoundary {
    FactsAndContract,
    FactsContractAndReleaseGate,
    FactsAndNestedGate,
    FactsContractAndNestedGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComposerMutationFamily {
    LoopV0FrameThenAstLower,
    NestedBlockIdsThenAstLower,
    LoopTrueSkeletonThenAstLower,
    LoopCondFrameThenAstLower,
    GenericSkeletonThenAstLower,
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
    first_mutation: ComposerMutationFamily,
    post_effect_none: PostEffectNone,
}

// `route` order is intentionally the production registry order. The remaining
// fields describe source-level boundaries only; they are not route policy.
const EFFECT_ORDER_MATRIX: &[EffectOrderRow] = &[
    EffectOrderRow {
        route: LoopRouteId::LoopBreakRecipe,
        selection: "loop_break",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::IfPhiJoin,
        selection: "if_phi_join",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopContinueOnly,
        selection: "loop_continue_only",
        qualification: QualificationBoundary::FactsContractAndNestedGate,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopTrueEarlyExit,
        selection: "loop_true_early_exit",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopSimpleWhile,
        selection: "loop_simple_while && !nested",
        qualification: QualificationBoundary::FactsContractAndNestedGate,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCharMap,
        selection: "loop_char_map",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopArrayJoin,
        selection: "loop_array_join",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::ScanWithInit,
        selection: "scan_with_init",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::SplitScan,
        selection: "split_scan",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::BoolPredicateScan,
        selection: "bool_predicate_scan",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::AccumConstLoop,
        selection: "accum_const_loop",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopV0FrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::NestedLoopMinimal,
        selection: "nested_loop_minimal",
        qualification: QualificationBoundary::FactsAndNestedGate,
        first_mutation: ComposerMutationFamily::NestedBlockIdsThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopTrueBreakContinue,
        selection: "loop_true_break_continue && !loop_break",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_mutation: ComposerMutationFamily::LoopTrueSkeletonThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondBreakContinue,
        selection: "loop_cond_break_continue; !loop_break; !scan; !return_only",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_mutation: ComposerMutationFamily::LoopCondFrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondContinueOnly,
        selection: "loop_cond_continue_only && !loop_continue_only",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_mutation: ComposerMutationFamily::LoopCondFrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondContinueWithReturn,
        selection: "loop_cond_continue_with_return",
        qualification: QualificationBoundary::FactsContractAndReleaseGate,
        first_mutation: ComposerMutationFamily::LoopCondFrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::LoopCondReturnInBody,
        selection: "loop_cond_return_in_body; !scan; return_only",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::LoopCondFrameThenAstLower,
        post_effect_none: PostEffectNone::LowererResultFlowsToScheduler,
    },
    EffectOrderRow {
        route: LoopRouteId::GenericLoopV0,
        selection: "generic_loop_v0",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::GenericSkeletonThenAstLower,
        post_effect_none: PostEffectNone::GenericReleaseFailureBecomesNone,
    },
    EffectOrderRow {
        route: LoopRouteId::GenericLoopV1,
        selection: "generic_loop_v1; !break; !simple; !cond_break; !scan",
        qualification: QualificationBoundary::FactsAndContract,
        first_mutation: ComposerMutationFamily::GenericSkeletonThenAstLower,
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
                row.first_mutation,
                ComposerMutationFamily::LoopV0FrameThenAstLower
                    | ComposerMutationFamily::NestedBlockIdsThenAstLower
                    | ComposerMutationFamily::LoopTrueSkeletonThenAstLower
                    | ComposerMutationFamily::LoopCondFrameThenAstLower
                    | ComposerMutationFamily::GenericSkeletonThenAstLower
            )
    }));
}

#[test]
fn mutation_families_cover_every_current_composer_without_collapsing_nested_or_loop_true() {
    let count = |family| {
        EFFECT_ORDER_MATRIX
            .iter()
            .filter(|row| row.first_mutation == family)
            .count()
    };

    assert_eq!(count(ComposerMutationFamily::LoopV0FrameThenAstLower), 11);
    assert_eq!(count(ComposerMutationFamily::NestedBlockIdsThenAstLower), 1);
    assert_eq!(
        count(ComposerMutationFamily::LoopTrueSkeletonThenAstLower),
        1
    );
    assert_eq!(count(ComposerMutationFamily::LoopCondFrameThenAstLower), 4);
    assert_eq!(
        count(ComposerMutationFamily::GenericSkeletonThenAstLower),
        2
    );
}
