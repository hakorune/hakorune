//! Caller-zero all-route closeout products for the M8 all19 row.
//!
//! `VerifiedLoopAllRouteObservationSetV1` seals exactly one typed outcome
//! per canonical route — `RecipeBacked` only where the landed cohort
//! inventory attests the route, otherwise `PreEffectDeclined`. It is
//! migration inventory: it proves every route has a truthful typed
//! outcome and selects nothing. `WholeUnitLoopCoverageProofV1` binds a
//! sealed set to one unit's family-window lease identity; the S2
//! selector consumes it to open `NoCandidate`. No AST, source
//! re-observation, Recipe, Builder, MIR, or physical identity enters.

use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_recipe_contract::LoopRecipeProducerIdV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1, VerifiedLoopFamilyWindowLeaseV1,
};

use super::policy_evidence::LoopRoutePolicySourceDeclineReasonV1;
use super::schema::CANONICAL_LOOP_ROUTE_ORDER_V1;

/// Landed portable cohort backing a `RecipeBacked` route row.
///
/// `PortableProducer` names a `LoopRecipeV1` producer id; `ScanWithInitV2`
/// names the S6C forward ScanWithInit cohort, whose product rides the V2
/// wire. Its wire provenance is `LoopRecipeProducerIdV1::ScanWithInitV2`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteRecipeBackingV1 {
    PortableProducer(LoopRecipeProducerIdV1),
    ScanWithInitV2,
}

/// Typed outcome for one canonical route in a unit's observation set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteObservationOutcomeV1 {
    RecipeBacked(LoopRouteRecipeBackingV1),
    PreEffectDeclined(LoopRoutePolicySourceDeclineReasonV1),
}

/// One unsealed input row. The issuer validates canonical order,
/// attestation, and backing cardinality before sealing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopAllRouteObservationRowV1 {
    pub(crate) route: LoopRouteId,
    pub(crate) outcome: LoopRouteObservationOutcomeV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopAllRouteObservationSetRejectV1 {
    RowCountMismatch { expected: usize, actual: usize },
    RouteOrderMismatch {
        raw_cursor: usize,
        expected: LoopRouteId,
        found: LoopRouteId,
    },
    UnattestedRecipeBacking { route: LoopRouteId },
    MultipleRecipeBacked {
        first: LoopRouteId,
        second: LoopRouteId,
    },
}

/// Routes whose bounded source profile is covered by a landed portable
/// cohort. Every other route must arrive as `PreEffectDeclined`; a
/// `RecipeBacked` claim outside this table is a typed reject.
pub(crate) const ATTESTED_RECIPE_BACKED_V1: &[(LoopRouteId, LoopRouteRecipeBackingV1)] = &[
    (
        LoopRouteId::LoopSimpleWhile,
        LoopRouteRecipeBackingV1::PortableProducer(
            LoopRecipeProducerIdV1::VariableAccumRecurrenceV1,
        ),
    ),
    (
        LoopRouteId::ScanWithInit,
        LoopRouteRecipeBackingV1::ScanWithInitV2,
    ),
    (
        LoopRouteId::AccumConstLoop,
        LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::DirectAccumV1),
    ),
    (
        LoopRouteId::NestedLoopMinimal,
        LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::NestedPredicateV1),
    ),
    (
        LoopRouteId::LoopTrueBreakContinue,
        LoopRouteRecipeBackingV1::PortableProducer(
            LoopRecipeProducerIdV1::LoopTrueBreakContinueV1,
        ),
    ),
    (
        LoopRouteId::LoopCondBreakContinue,
        LoopRouteRecipeBackingV1::PortableProducer(
            LoopRecipeProducerIdV1::LoopCondBreakContinueV1,
        ),
    ),
];

#[derive(Debug, PartialEq, Eq)]
struct LoopAllRouteObservationSetSealV1;

#[derive(Debug, PartialEq, Eq)]
struct WholeUnitLoopCoverageSealV1;

/// Sealed all-route observation set: exactly one typed row per canonical
/// route, in canonical order, with at most one `RecipeBacked` row.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopAllRouteObservationSetV1 {
    rows: Box<[LoopAllRouteObservationRowV1]>,
    _seal: LoopAllRouteObservationSetSealV1,
}

impl VerifiedLoopAllRouteObservationSetV1 {
    pub(crate) fn rows(&self) -> &[LoopAllRouteObservationRowV1] {
        &self.rows
    }

    /// True when every route row is a typed pre-effect decline — the
    /// only shape under which `NoCandidate` may be returned.
    pub(crate) fn all_pre_effect_declined(&self) -> bool {
        self.rows.iter().all(|row| {
            matches!(
                row.outcome,
                LoopRouteObservationOutcomeV1::PreEffectDeclined(_)
            )
        })
    }
}

pub(crate) fn issue_all_route_observation_set_v1(
    rows: Box<[LoopAllRouteObservationRowV1]>,
) -> Result<VerifiedLoopAllRouteObservationSetV1, LoopAllRouteObservationSetRejectV1> {
    if rows.len() != CANONICAL_LOOP_ROUTE_ORDER_V1.len() {
        return Err(LoopAllRouteObservationSetRejectV1::RowCountMismatch {
            expected: CANONICAL_LOOP_ROUTE_ORDER_V1.len(),
            actual: rows.len(),
        });
    }
    let mut backed: Option<LoopRouteId> = None;
    for (raw_cursor, row) in rows.iter().enumerate() {
        let expected = CANONICAL_LOOP_ROUTE_ORDER_V1[raw_cursor];
        if row.route != expected {
            return Err(LoopAllRouteObservationSetRejectV1::RouteOrderMismatch {
                raw_cursor,
                expected,
                found: row.route,
            });
        }
        if let LoopRouteObservationOutcomeV1::RecipeBacked(backing) = row.outcome {
            if !ATTESTED_RECIPE_BACKED_V1
                .iter()
                .any(|(route, attested)| *route == row.route && *attested == backing)
            {
                return Err(LoopAllRouteObservationSetRejectV1::UnattestedRecipeBacking {
                    route: row.route,
                });
            }
            if let Some(first) = backed {
                return Err(LoopAllRouteObservationSetRejectV1::MultipleRecipeBacked {
                    first,
                    second: row.route,
                });
            }
            backed = Some(row.route);
        }
    }
    Ok(VerifiedLoopAllRouteObservationSetV1 {
        rows,
        _seal: LoopAllRouteObservationSetSealV1,
    })
}

/// Per-unit coverage proof: binds a sealed all-route observation set to
/// one family-window lease identity. The selector requires the retained
/// identity to match the window's lease before `NoCandidate` may be
/// returned.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct WholeUnitLoopCoverageProofV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame_key: LoopExecutionFrameKeyV1,
    set: VerifiedLoopAllRouteObservationSetV1,
    _seal: WholeUnitLoopCoverageSealV1,
}

impl WholeUnitLoopCoverageProofV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) const fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame_key
    }

    pub(crate) const fn observation_set(&self) -> &VerifiedLoopAllRouteObservationSetV1 {
        &self.set
    }

    pub(crate) fn matches_lease(&self, lease: &VerifiedLoopFamilyWindowLeaseV1) -> bool {
        self.owner == lease.owner()
            && self.origin == lease.function_origin()
            && self.source_kind == lease.source_kind()
            && &self.site == lease.site()
            && self.frame_key.matches(&lease.frame())
    }
}

pub(crate) fn issue_whole_unit_loop_coverage_proof_v1(
    set: VerifiedLoopAllRouteObservationSetV1,
    lease: &VerifiedLoopFamilyWindowLeaseV1,
) -> WholeUnitLoopCoverageProofV1 {
    WholeUnitLoopCoverageProofV1 {
        owner: lease.owner(),
        origin: lease.function_origin(),
        source_kind: lease.source_kind(),
        site: lease.site().clone(),
        frame_key: lease.frame(),
        set,
        _seal: WholeUnitLoopCoverageSealV1,
    }
}
