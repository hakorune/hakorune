//! Common five-row admission assembler for Loop-family observations.
//!
//! This module is the only owner of the cross-family admission window.  It
//! consumes one resolver-issued identity lease and one move-only row from
//! each family.  It validates identity, mode, coverage, and row disposition;
//! it does not count candidates, reject semantic overlap, select a winner, or
//! issue Recipe/Builder/MIR products.

use crate::mir::loop_structural_facts::{
    DirectAccumObservationCoverageV1, DirectAccumObservationModeV1,
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1,
    LoopCondObservationCoverageV1, LoopCondObservationModeV1,
    LoopTrueObservationCoverageV1, LoopTrueObservationModeV1,
    NestedPredicateObservationCoverageV1, NestedPredicateObservationModeV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    SemanticOwnerSourceKindV1, SourceStmtSiteV1, VerifiedLoopFamilyWindowLeaseV1,
};

use super::{
    DirectAccumFamilyObservationV1, GenericG0FamilyObservationV1,
    LoopCondFamilyObservationV1, LoopTrueFamilyObservationV1,
    NestedPredicateFamilyObservationV1,
};

const FAMILY_TAGS: [LoopFamilyTagV1; 5] = [
    LoopFamilyTagV1::DirectAccum,
    LoopFamilyTagV1::NestedPredicate,
    LoopFamilyTagV1::LoopTrueBreakContinue,
    LoopFamilyTagV1::LoopCondBreakContinue,
    LoopFamilyTagV1::GenericG0,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LoopFamilyTagV1 {
    DirectAccum,
    NestedPredicate,
    LoopTrueBreakContinue,
    LoopCondBreakContinue,
    GenericG0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopFamilyAdmissionModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopFamilyAdmissionCoverageV1 {
    Complete,
    Incomplete,
}

/// A typed, move-only family row.  The payload is kept opaque to the
/// assembler after its borrowed evidence view is inspected; candidate count
/// and candidate-overlap semantics belong to the later selector.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopFamilyObservationRowV1 {
    DirectAccum(DirectAccumFamilyObservationV1),
    NestedPredicate(NestedPredicateFamilyObservationV1),
    LoopTrue(LoopTrueFamilyObservationV1),
    LoopCond(LoopCondFamilyObservationV1),
    GenericG0(GenericG0FamilyObservationV1),
}

impl LoopFamilyObservationRowV1 {
    fn tag(&self) -> LoopFamilyTagV1 {
        match self {
            Self::DirectAccum(_) => LoopFamilyTagV1::DirectAccum,
            Self::NestedPredicate(_) => LoopFamilyTagV1::NestedPredicate,
            Self::LoopTrue(_) => LoopFamilyTagV1::LoopTrueBreakContinue,
            Self::LoopCond(_) => LoopFamilyTagV1::LoopCondBreakContinue,
            Self::GenericG0(_) => LoopFamilyTagV1::GenericG0,
        }
    }

    fn evidence_view(&self) -> LoopFamilyRowEvidenceView<'_> {
        match self {
            Self::DirectAccum(row) => direct_evidence(row),
            Self::NestedPredicate(row) => nested_evidence(row),
            Self::LoopTrue(row) => loop_true_evidence(row),
            Self::LoopCond(row) => loop_cond_evidence(row),
            Self::GenericG0(row) => generic_evidence(row),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopFamilyRowDispositionV1 {
    Candidate,
    Declined,
    Unresolved,
    Rejected,
}

#[derive(Debug)]
struct LoopFamilyIdentityView<'a> {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: &'a SourceStmtSiteV1,
    frame: &'a LoopExecutionFrameKeyV1,
}

impl<'a> LoopFamilyIdentityView<'a> {
    fn new(
        owner: FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        site: &'a SourceStmtSiteV1,
        frame: &'a LoopExecutionFrameKeyV1,
    ) -> Self {
        Self {
            owner,
            origin,
            source_kind,
            site,
            frame,
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.origin == other.origin
            && self.source_kind == other.source_kind
            && self.site == other.site
            && self.frame.matches(other.frame)
    }

    fn matches_lease(
        &self,
        lease: &VerifiedLoopFamilyWindowLeaseV1,
        lease_frame: &LoopExecutionFrameKeyV1,
    ) -> bool {
        self.owner == lease.owner()
            && self.origin == lease.function_origin()
            && self.source_kind == lease.source_kind()
            && self.site == lease.site()
            && self.frame.matches(lease_frame)
    }
}

#[derive(Debug)]
struct LoopFamilyRowEvidenceView<'a> {
    expected: LoopFamilyIdentityView<'a>,
    observed: LoopFamilyIdentityView<'a>,
    expected_mode: Option<LoopFamilyAdmissionModeV1>,
    observed_mode: Option<LoopFamilyAdmissionModeV1>,
    expected_coverage: LoopFamilyAdmissionCoverageV1,
    observed_coverage: LoopFamilyAdmissionCoverageV1,
    disposition: LoopFamilyRowDispositionV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopFamilyAdmissionIssueV1 {
    TooManyRows { actual: usize },
    MissingFamilyObservation { missing: Box<[LoopFamilyTagV1]> },
    DuplicateFamily(LoopFamilyTagV1),
    IdentityMismatch(LoopFamilyTagV1),
    ForeignIdentity(LoopFamilyTagV1),
    ModeUnsealed(LoopFamilyTagV1),
    ModeMismatch(LoopFamilyTagV1),
    CoverageIncomplete(LoopFamilyTagV1),
    CoverageMismatch(LoopFamilyTagV1),
    RowUnresolved(LoopFamilyTagV1),
    RowRejected(LoopFamilyTagV1),
}

impl LoopFamilyAdmissionIssueV1 {
    fn is_rejected(&self) -> bool {
        matches!(
            self,
            Self::TooManyRows { .. }
                | Self::DuplicateFamily(_)
                | Self::IdentityMismatch(_)
                | Self::ForeignIdentity(_)
                | Self::ModeMismatch(_)
                | Self::CoverageMismatch(_)
                | Self::RowRejected(_)
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LoopFamilyAdmissionFailureEvidenceV1 {
    lease: VerifiedLoopFamilyWindowLeaseV1,
    rows: Box<[LoopFamilyObservationRowV1]>,
    issues: Box<[LoopFamilyAdmissionIssueV1]>,
}

impl LoopFamilyAdmissionFailureEvidenceV1 {
    pub(crate) fn lease(&self) -> &VerifiedLoopFamilyWindowLeaseV1 {
        &self.lease
    }

    pub(crate) fn rows(&self) -> &[LoopFamilyObservationRowV1] {
        &self.rows
    }

    pub(crate) fn issues(&self) -> &[LoopFamilyAdmissionIssueV1] {
        &self.issues
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopFamilyWindowLeaseV1,
        Box<[LoopFamilyObservationRowV1]>,
        Box<[LoopFamilyAdmissionIssueV1]>,
    ) {
        (self.lease, self.rows, self.issues)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopFamilyAdmissionAssemblyOutcomeV1 {
    Ready(VerifiedLoopFamilyAdmissionWindowV1),
    Unresolved(LoopFamilyAdmissionFailureEvidenceV1),
    Rejected(LoopFamilyAdmissionFailureEvidenceV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopFamilyAdmissionWindowV1 {
    lease: VerifiedLoopFamilyWindowLeaseV1,
    rows: VerifiedLoopFamilyAdmissionRowsV1,
    mode: LoopFamilyAdmissionModeV1,
    coverage: LoopFamilyAdmissionCoverageV1,
}

impl VerifiedLoopFamilyAdmissionWindowV1 {
    pub(crate) fn lease(&self) -> &VerifiedLoopFamilyWindowLeaseV1 {
        &self.lease
    }

    pub(crate) fn rows(&self) -> &VerifiedLoopFamilyAdmissionRowsV1 {
        &self.rows
    }

    pub(crate) const fn mode(&self) -> LoopFamilyAdmissionModeV1 {
        self.mode
    }

    pub(crate) const fn coverage(&self) -> LoopFamilyAdmissionCoverageV1 {
        self.coverage
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopFamilyWindowLeaseV1,
        VerifiedLoopFamilyAdmissionRowsV1,
        LoopFamilyAdmissionModeV1,
        LoopFamilyAdmissionCoverageV1,
    ) {
        (self.lease, self.rows, self.mode, self.coverage)
    }

    #[cfg(test)]
    // This constructor is only for selector-cardinality tests. Production
    // callers must obtain the brand from `assemble_loop_family_admission_window_v1`.
    pub(crate) fn from_parts_for_test(
        lease: VerifiedLoopFamilyWindowLeaseV1,
        rows: VerifiedLoopFamilyAdmissionRowsV1,
        mode: LoopFamilyAdmissionModeV1,
        coverage: LoopFamilyAdmissionCoverageV1,
    ) -> Self {
        Self {
            lease,
            rows,
            mode,
            coverage,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopFamilyAdmissionRowsV1 {
    direct_accum: DirectAccumFamilyObservationV1,
    nested_predicate: NestedPredicateFamilyObservationV1,
    loop_true: LoopTrueFamilyObservationV1,
    loop_cond: LoopCondFamilyObservationV1,
    generic_g0: GenericG0FamilyObservationV1,
}

impl VerifiedLoopFamilyAdmissionRowsV1 {
    pub(crate) fn direct_accum(&self) -> &DirectAccumFamilyObservationV1 {
        &self.direct_accum
    }

    pub(crate) fn nested_predicate(&self) -> &NestedPredicateFamilyObservationV1 {
        &self.nested_predicate
    }

    pub(crate) fn loop_true(&self) -> &LoopTrueFamilyObservationV1 {
        &self.loop_true
    }

    pub(crate) fn loop_cond(&self) -> &LoopCondFamilyObservationV1 {
        &self.loop_cond
    }

    pub(crate) fn generic_g0(&self) -> &GenericG0FamilyObservationV1 {
        &self.generic_g0
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        DirectAccumFamilyObservationV1,
        NestedPredicateFamilyObservationV1,
        LoopTrueFamilyObservationV1,
        LoopCondFamilyObservationV1,
        GenericG0FamilyObservationV1,
    ) {
        (
            self.direct_accum,
            self.nested_predicate,
            self.loop_true,
            self.loop_cond,
            self.generic_g0,
        )
    }

    #[cfg(test)]
    // This constructor is only for selector-cardinality tests. It deliberately
    // does not replace the assembler's identity/mode/coverage validation.
    pub(crate) fn from_parts_for_test(
        direct_accum: DirectAccumFamilyObservationV1,
        nested_predicate: NestedPredicateFamilyObservationV1,
        loop_true: LoopTrueFamilyObservationV1,
        loop_cond: LoopCondFamilyObservationV1,
        generic_g0: GenericG0FamilyObservationV1,
    ) -> Self {
        Self {
            direct_accum,
            nested_predicate,
            loop_true,
            loop_cond,
            generic_g0,
        }
    }
}

pub(crate) fn assemble_loop_family_admission_window_v1(
    lease: VerifiedLoopFamilyWindowLeaseV1,
    rows: Box<[LoopFamilyObservationRowV1]>,
) -> LoopFamilyAdmissionAssemblyOutcomeV1 {
    let mut issues = Vec::new();
    let mut counts = [0usize; FAMILY_TAGS.len()];
    let mut borrowed_slots: [Option<&LoopFamilyObservationRowV1>; FAMILY_TAGS.len()] =
        [None; FAMILY_TAGS.len()];

    if rows.len() > FAMILY_TAGS.len() {
        issues.push(LoopFamilyAdmissionIssueV1::TooManyRows { actual: rows.len() });
    }

    for row in rows.iter() {
        let index = tag_index(row.tag());
        counts[index] += 1;
        if counts[index] == 1 {
            borrowed_slots[index] = Some(row);
        } else {
            issues.push(LoopFamilyAdmissionIssueV1::DuplicateFamily(row.tag()));
        }
    }

    let missing: Box<[LoopFamilyTagV1]> = FAMILY_TAGS
        .iter()
        .zip(counts.iter())
        .filter_map(|(tag, count)| (*count == 0).then_some(*tag))
        .collect();
    if !missing.is_empty() {
        issues.push(LoopFamilyAdmissionIssueV1::MissingFamilyObservation { missing });
    }

    let lease_frame = lease.frame();
    let mut common_mode = None;
    let mut common_coverage = None;
    for (index, tag) in FAMILY_TAGS.iter().copied().enumerate() {
        let Some(row) = borrowed_slots[index] else {
            continue;
        };
        let view = row.evidence_view();
        if !view.expected.matches(&view.observed) {
            issues.push(LoopFamilyAdmissionIssueV1::IdentityMismatch(tag));
        }
        if !view.expected.matches_lease(&lease, &lease_frame)
            || !view.observed.matches_lease(&lease, &lease_frame)
        {
            issues.push(LoopFamilyAdmissionIssueV1::ForeignIdentity(tag));
        }

        match (view.expected_mode, view.observed_mode) {
            (Some(expected), Some(observed)) if expected == observed => {
                if let Some(common) = common_mode {
                    if common != expected {
                        issues.push(LoopFamilyAdmissionIssueV1::ModeMismatch(tag));
                    }
                } else {
                    common_mode = Some(expected);
                }
            }
            (None, _) | (_, None) => {
                issues.push(LoopFamilyAdmissionIssueV1::ModeUnsealed(tag));
            }
            (Some(_), Some(_)) => {
                issues.push(LoopFamilyAdmissionIssueV1::ModeMismatch(tag));
            }
        }

        match (view.expected_coverage, view.observed_coverage) {
            (LoopFamilyAdmissionCoverageV1::Complete, LoopFamilyAdmissionCoverageV1::Complete) => {
                if let Some(common) = common_coverage {
                    if common != LoopFamilyAdmissionCoverageV1::Complete {
                        issues.push(LoopFamilyAdmissionIssueV1::CoverageMismatch(tag));
                    }
                } else {
                    common_coverage = Some(LoopFamilyAdmissionCoverageV1::Complete);
                }
            }
            _ => issues.push(LoopFamilyAdmissionIssueV1::CoverageIncomplete(tag)),
        }

        match view.disposition {
            LoopFamilyRowDispositionV1::Rejected => {
                issues.push(LoopFamilyAdmissionIssueV1::RowRejected(tag));
            }
            LoopFamilyRowDispositionV1::Unresolved => {
                issues.push(LoopFamilyAdmissionIssueV1::RowUnresolved(tag));
            }
            LoopFamilyRowDispositionV1::Candidate | LoopFamilyRowDispositionV1::Declined => {}
        }
    }

    let issues = issues.into_boxed_slice();
    if issues.iter().any(LoopFamilyAdmissionIssueV1::is_rejected) {
        return LoopFamilyAdmissionAssemblyOutcomeV1::Rejected(
            LoopFamilyAdmissionFailureEvidenceV1 {
                lease,
                rows,
                issues,
            },
        );
    }
    if !issues.is_empty() {
        return LoopFamilyAdmissionAssemblyOutcomeV1::Unresolved(
            LoopFamilyAdmissionFailureEvidenceV1 {
                lease,
                rows,
                issues,
            },
        );
    }

    let mut owned_slots: [Option<LoopFamilyObservationRowV1>; FAMILY_TAGS.len()] =
        [None, None, None, None, None];
    for row in rows {
        let index = tag_index(row.tag());
        owned_slots[index] = Some(row);
    }
    let [
        Some(LoopFamilyObservationRowV1::DirectAccum(direct_accum)),
        Some(LoopFamilyObservationRowV1::NestedPredicate(nested_predicate)),
        Some(LoopFamilyObservationRowV1::LoopTrue(loop_true)),
        Some(LoopFamilyObservationRowV1::LoopCond(loop_cond)),
        Some(LoopFamilyObservationRowV1::GenericG0(generic_g0)),
    ] = owned_slots
    else {
        unreachable!("validated five-row admission must canonicalize exactly five rows")
    };

    LoopFamilyAdmissionAssemblyOutcomeV1::Ready(VerifiedLoopFamilyAdmissionWindowV1 {
        lease,
        rows: VerifiedLoopFamilyAdmissionRowsV1 {
            direct_accum,
            nested_predicate,
            loop_true,
            loop_cond,
            generic_g0,
        },
        mode: common_mode.expect("validated rows must co-seal one mode"),
        coverage: common_coverage.expect("validated rows must co-seal complete coverage"),
    })
}

fn tag_index(tag: LoopFamilyTagV1) -> usize {
    match tag {
        LoopFamilyTagV1::DirectAccum => 0,
        LoopFamilyTagV1::NestedPredicate => 1,
        LoopFamilyTagV1::LoopTrueBreakContinue => 2,
        LoopFamilyTagV1::LoopCondBreakContinue => 3,
        LoopFamilyTagV1::GenericG0 => 4,
    }
}

fn identity_view<'a>(
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: &'a SourceStmtSiteV1,
    frame: &'a LoopExecutionFrameKeyV1,
) -> LoopFamilyIdentityView<'a> {
    LoopFamilyIdentityView::new(owner, origin, source_kind, site, frame)
}

fn direct_evidence(row: &DirectAccumFamilyObservationV1) -> LoopFamilyRowEvidenceView<'_> {
    let evidence = row.evidence();
    let expected = evidence.expected().identity();
    let observed = evidence.observed_identity();
    LoopFamilyRowEvidenceView {
        expected: identity_view(
            expected.owner(),
            expected.function_origin(),
            expected.source_kind(),
            expected.site(),
            expected.frame(),
        ),
        observed: identity_view(
            observed.owner(),
            observed.function_origin(),
            observed.source_kind(),
            observed.site(),
            observed.frame(),
        ),
        expected_mode: evidence.expected().mode().map(direct_mode),
        observed_mode: evidence.observed_mode().map(direct_mode),
        expected_coverage: direct_coverage(evidence.expected().coverage()),
        observed_coverage: direct_coverage(evidence.observed_coverage()),
        disposition: direct_disposition(row),
    }
}

fn nested_evidence(row: &NestedPredicateFamilyObservationV1) -> LoopFamilyRowEvidenceView<'_> {
    let evidence = row.evidence();
    let expected = evidence.expected().identity();
    let observed = evidence.observed_identity();
    LoopFamilyRowEvidenceView {
        expected: identity_view(
            expected.owner(),
            expected.function_origin(),
            expected.source_kind(),
            expected.site(),
            expected.frame(),
        ),
        observed: identity_view(
            observed.owner(),
            observed.function_origin(),
            observed.source_kind(),
            observed.site(),
            observed.frame(),
        ),
        expected_mode: evidence.expected().mode().map(nested_mode),
        observed_mode: evidence.observed_mode().map(nested_mode),
        expected_coverage: nested_coverage(evidence.expected().coverage()),
        observed_coverage: nested_coverage(evidence.observed_coverage()),
        disposition: nested_disposition(row),
    }
}

fn loop_true_evidence(row: &LoopTrueFamilyObservationV1) -> LoopFamilyRowEvidenceView<'_> {
    let evidence = row.evidence();
    let expected = evidence.expected().identity();
    let observed = evidence.observed_identity();
    LoopFamilyRowEvidenceView {
        expected: identity_view(
            expected.owner(),
            expected.function_origin(),
            expected.source_kind(),
            expected.site(),
            expected.frame(),
        ),
        observed: identity_view(
            observed.owner(),
            observed.function_origin(),
            observed.source_kind(),
            observed.site(),
            observed.frame(),
        ),
        expected_mode: evidence.expected().mode().map(loop_true_mode),
        observed_mode: evidence.observed_mode().map(loop_true_mode),
        expected_coverage: loop_true_coverage(evidence.expected().coverage()),
        observed_coverage: loop_true_coverage(evidence.observed_coverage()),
        disposition: loop_true_disposition(row),
    }
}

fn loop_cond_evidence(row: &LoopCondFamilyObservationV1) -> LoopFamilyRowEvidenceView<'_> {
    let evidence = row.evidence();
    let expected = evidence.expected().identity();
    let observed = evidence.observed_identity();
    LoopFamilyRowEvidenceView {
        expected: identity_view(
            expected.owner(),
            expected.function_origin(),
            expected.source_kind(),
            expected.site(),
            expected.frame(),
        ),
        observed: identity_view(
            observed.owner(),
            observed.function_origin(),
            observed.source_kind(),
            observed.site(),
            observed.frame(),
        ),
        expected_mode: evidence.expected().mode().map(loop_cond_mode),
        observed_mode: evidence.observed_mode().map(loop_cond_mode),
        expected_coverage: loop_cond_coverage(evidence.expected().coverage()),
        observed_coverage: loop_cond_coverage(evidence.observed_coverage()),
        disposition: loop_cond_disposition(row),
    }
}

fn generic_evidence(row: &GenericG0FamilyObservationV1) -> LoopFamilyRowEvidenceView<'_> {
    let evidence = row.evidence();
    let expected = evidence.expected().identity();
    let observed = evidence.observed_identity();
    LoopFamilyRowEvidenceView {
        expected: identity_view(
            expected.owner(),
            expected.function_origin(),
            expected.source_kind(),
            expected.site(),
            expected.frame(),
        ),
        observed: identity_view(
            observed.owner(),
            observed.function_origin(),
            observed.source_kind(),
            observed.site(),
            observed.frame(),
        ),
        expected_mode: evidence.expected().mode().map(generic_mode),
        observed_mode: evidence.observed_mode().map(generic_mode),
        expected_coverage: generic_coverage(evidence.expected().coverage()),
        observed_coverage: generic_coverage(evidence.observed_coverage()),
        disposition: generic_disposition(row),
    }
}

fn direct_mode(mode: DirectAccumObservationModeV1) -> LoopFamilyAdmissionModeV1 {
    match mode {
        DirectAccumObservationModeV1::Release => LoopFamilyAdmissionModeV1::Release,
        DirectAccumObservationModeV1::Strict => LoopFamilyAdmissionModeV1::Strict,
        DirectAccumObservationModeV1::StrictPlannerRequired => {
            LoopFamilyAdmissionModeV1::StrictPlannerRequired
        }
    }
}

fn nested_mode(mode: NestedPredicateObservationModeV1) -> LoopFamilyAdmissionModeV1 {
    match mode {
        NestedPredicateObservationModeV1::Release => LoopFamilyAdmissionModeV1::Release,
        NestedPredicateObservationModeV1::Strict => LoopFamilyAdmissionModeV1::Strict,
        NestedPredicateObservationModeV1::StrictPlannerRequired => {
            LoopFamilyAdmissionModeV1::StrictPlannerRequired
        }
    }
}

fn loop_true_mode(mode: LoopTrueObservationModeV1) -> LoopFamilyAdmissionModeV1 {
    match mode {
        LoopTrueObservationModeV1::Release => LoopFamilyAdmissionModeV1::Release,
        LoopTrueObservationModeV1::Strict => LoopFamilyAdmissionModeV1::Strict,
        LoopTrueObservationModeV1::StrictPlannerRequired => {
            LoopFamilyAdmissionModeV1::StrictPlannerRequired
        }
    }
}

fn loop_cond_mode(mode: LoopCondObservationModeV1) -> LoopFamilyAdmissionModeV1 {
    match mode {
        LoopCondObservationModeV1::Release => LoopFamilyAdmissionModeV1::Release,
        LoopCondObservationModeV1::Strict => LoopFamilyAdmissionModeV1::Strict,
        LoopCondObservationModeV1::StrictPlannerRequired => {
            LoopFamilyAdmissionModeV1::StrictPlannerRequired
        }
    }
}

fn generic_mode(mode: GenericG0ObservationModeV1) -> LoopFamilyAdmissionModeV1 {
    match mode {
        GenericG0ObservationModeV1::Release => LoopFamilyAdmissionModeV1::Release,
        GenericG0ObservationModeV1::Strict => LoopFamilyAdmissionModeV1::Strict,
        GenericG0ObservationModeV1::StrictPlannerRequired => {
            LoopFamilyAdmissionModeV1::StrictPlannerRequired
        }
    }
}

fn direct_coverage(coverage: DirectAccumObservationCoverageV1) -> LoopFamilyAdmissionCoverageV1 {
    match coverage {
        DirectAccumObservationCoverageV1::Complete => LoopFamilyAdmissionCoverageV1::Complete,
        DirectAccumObservationCoverageV1::Incomplete => LoopFamilyAdmissionCoverageV1::Incomplete,
    }
}

fn nested_coverage(
    coverage: NestedPredicateObservationCoverageV1,
) -> LoopFamilyAdmissionCoverageV1 {
    match coverage {
        NestedPredicateObservationCoverageV1::Complete => LoopFamilyAdmissionCoverageV1::Complete,
        NestedPredicateObservationCoverageV1::Incomplete => {
            LoopFamilyAdmissionCoverageV1::Incomplete
        }
    }
}

fn loop_true_coverage(coverage: LoopTrueObservationCoverageV1) -> LoopFamilyAdmissionCoverageV1 {
    match coverage {
        LoopTrueObservationCoverageV1::Complete => LoopFamilyAdmissionCoverageV1::Complete,
        LoopTrueObservationCoverageV1::Incomplete => LoopFamilyAdmissionCoverageV1::Incomplete,
    }
}

fn loop_cond_coverage(coverage: LoopCondObservationCoverageV1) -> LoopFamilyAdmissionCoverageV1 {
    match coverage {
        LoopCondObservationCoverageV1::Complete => LoopFamilyAdmissionCoverageV1::Complete,
        LoopCondObservationCoverageV1::Incomplete => LoopFamilyAdmissionCoverageV1::Incomplete,
    }
}

fn generic_coverage(coverage: GenericG0ObservationCoverageV1) -> LoopFamilyAdmissionCoverageV1 {
    match coverage {
        GenericG0ObservationCoverageV1::Complete => LoopFamilyAdmissionCoverageV1::Complete,
        GenericG0ObservationCoverageV1::Incomplete => LoopFamilyAdmissionCoverageV1::Incomplete,
    }
}

fn direct_disposition(row: &DirectAccumFamilyObservationV1) -> LoopFamilyRowDispositionV1 {
    match row {
        DirectAccumFamilyObservationV1::Candidate(_) => LoopFamilyRowDispositionV1::Candidate,
        DirectAccumFamilyObservationV1::Declined { .. } => LoopFamilyRowDispositionV1::Declined,
        DirectAccumFamilyObservationV1::Unresolved { .. } => {
            LoopFamilyRowDispositionV1::Unresolved
        }
        DirectAccumFamilyObservationV1::Rejected { .. } => LoopFamilyRowDispositionV1::Rejected,
    }
}

fn nested_disposition(row: &NestedPredicateFamilyObservationV1) -> LoopFamilyRowDispositionV1 {
    match row {
        NestedPredicateFamilyObservationV1::Candidate(_) => LoopFamilyRowDispositionV1::Candidate,
        NestedPredicateFamilyObservationV1::Declined { .. } => LoopFamilyRowDispositionV1::Declined,
        NestedPredicateFamilyObservationV1::Unresolved { .. } => {
            LoopFamilyRowDispositionV1::Unresolved
        }
        NestedPredicateFamilyObservationV1::Rejected { .. } => LoopFamilyRowDispositionV1::Rejected,
    }
}

fn loop_true_disposition(row: &LoopTrueFamilyObservationV1) -> LoopFamilyRowDispositionV1 {
    match row {
        LoopTrueFamilyObservationV1::Candidate(_) => LoopFamilyRowDispositionV1::Candidate,
        LoopTrueFamilyObservationV1::Declined { .. } => LoopFamilyRowDispositionV1::Declined,
        LoopTrueFamilyObservationV1::Unresolved { .. } => LoopFamilyRowDispositionV1::Unresolved,
        LoopTrueFamilyObservationV1::Rejected { .. } => LoopFamilyRowDispositionV1::Rejected,
    }
}

fn loop_cond_disposition(row: &LoopCondFamilyObservationV1) -> LoopFamilyRowDispositionV1 {
    match row {
        LoopCondFamilyObservationV1::Candidate(_) => LoopFamilyRowDispositionV1::Candidate,
        LoopCondFamilyObservationV1::Declined { .. } => LoopFamilyRowDispositionV1::Declined,
        LoopCondFamilyObservationV1::Unresolved { .. } => LoopFamilyRowDispositionV1::Unresolved,
        LoopCondFamilyObservationV1::Rejected { .. } => LoopFamilyRowDispositionV1::Rejected,
    }
}

fn generic_disposition(row: &GenericG0FamilyObservationV1) -> LoopFamilyRowDispositionV1 {
    match row {
        GenericG0FamilyObservationV1::Candidate(_) => LoopFamilyRowDispositionV1::Candidate,
        GenericG0FamilyObservationV1::Declined { .. } => LoopFamilyRowDispositionV1::Declined,
        GenericG0FamilyObservationV1::Unresolved { .. } => LoopFamilyRowDispositionV1::Unresolved,
        GenericG0FamilyObservationV1::Rejected { .. } => LoopFamilyRowDispositionV1::Rejected,
    }
}
