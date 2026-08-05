//! Test-only neutral family-selection consumer for D4-S3-S2.
//!
//! This file is intentionally separate from the legacy 19-route evaluator in
//! `policy.rs`.  It owns only the typed outcome boundary; no route ID, cursor,
//! schedule, AST, facts extraction, Recipe, Builder, or MIR authority enters.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericFamilyEvidenceV1 {
    V0Only,
    V1Only,
    Both,
    Neither,
    NoStandaloneRow,
    PlannerModeUnsealed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectorCoverageV1 {
    WindowComplete,
    WholeUnitNoLoopEnvelope(WholeUnitNoLoopEnvelopeProofV1),
}

/// A proof that a whole resolved unit has no Loop-family envelope. There is no
/// constructor in this test-only product; a future source bridge must issue it
/// only after sealing every semantic family as typed `Declined`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WholeUnitNoLoopEnvelopeProofV1;

/// Neutral input issued by the S1 adapter. `WindowComplete` is deliberately
/// weaker than the whole-unit proof required for `NoCandidate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalFamilySelectorInputV1 {
    coverage: SelectorCoverageV1,
    generic: GenericFamilyEvidenceV1,
}

impl CanonicalFamilySelectorInputV1 {
    pub(crate) const fn window(generic: GenericFamilyEvidenceV1) -> Self {
        Self {
            coverage: SelectorCoverageV1::WindowComplete,
            generic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FamilySelectionRejectV1 {
    IncompleteCoverage,
    LegacyPolicyInput,
    SourceIdentityMismatch,
    ModeMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FamilySelectionUnresolvedV1 {
    GenericV0Only,
    GenericV1Only,
    GenericOverlap,
    GenericNeither,
    NoStandaloneRow,
    PlannerModeUnsealed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectedFamilyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalFamilySelectionOutcomeV1 {
    Selected(SelectedFamilyV1),
    NoCandidate(WholeUnitNoLoopEnvelopeProofV1),
    Rejected(FamilySelectionRejectV1),
    Unresolved(FamilySelectionUnresolvedV1),
}

/// Pure test-only consumer for the neutral family boundary.
///
/// Current S1 input is window-scoped, so it cannot produce `Selected` or
/// `NoCandidate`. The two positive arms remain type-level vocabulary until a
/// later source bridge seals a family candidate or a whole-unit negative proof.
pub(crate) fn select_canonical_family_for_test(
    input: CanonicalFamilySelectorInputV1,
) -> CanonicalFamilySelectionOutcomeV1 {
    match input.coverage {
        SelectorCoverageV1::WindowComplete => match input.generic {
            GenericFamilyEvidenceV1::V0Only => CanonicalFamilySelectionOutcomeV1::Unresolved(
                FamilySelectionUnresolvedV1::GenericV0Only,
            ),
            GenericFamilyEvidenceV1::V1Only => CanonicalFamilySelectionOutcomeV1::Unresolved(
                FamilySelectionUnresolvedV1::GenericV1Only,
            ),
            GenericFamilyEvidenceV1::Both => CanonicalFamilySelectionOutcomeV1::Unresolved(
                FamilySelectionUnresolvedV1::GenericOverlap,
            ),
            GenericFamilyEvidenceV1::Neither => CanonicalFamilySelectionOutcomeV1::Unresolved(
                FamilySelectionUnresolvedV1::GenericNeither,
            ),
            GenericFamilyEvidenceV1::NoStandaloneRow => {
                CanonicalFamilySelectionOutcomeV1::Unresolved(
                    FamilySelectionUnresolvedV1::NoStandaloneRow,
                )
            }
            GenericFamilyEvidenceV1::PlannerModeUnsealed => {
                CanonicalFamilySelectionOutcomeV1::Unresolved(
                    FamilySelectionUnresolvedV1::PlannerModeUnsealed,
                )
            }
        },
        SelectorCoverageV1::WholeUnitNoLoopEnvelope(proof) => {
            CanonicalFamilySelectionOutcomeV1::NoCandidate(proof)
        }
    }
}
