//! AST-free facts for the bounded variable-accumulator recurrence family.
//!
//! The compiler-side observer owns syntax navigation.  This module receives
//! only resolver-branded source sites and typed observations, then co-seals one
//! move-only facts aggregate.  It does not import AST, Builder, Recipe, or
//! route policy.  Recipe keys and physical identities are issued later by the
//! producer.

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    ResolvedScopeRegionPairV1, SemanticOwnerSourceKindV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceV1,
};

/// The two source bindings admitted by this family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumRecurrenceBindingRoleV1 {
    Induction,
    Accumulator,
}

/// The two declaration/initializer relations co-sealed with the Loop facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumRecurrenceInputRoleV1 {
    InductionInitial,
    AccumulatorInitial,
}

/// Source roles for the canonical eleven-operation Recipe projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumRecurrenceSourceRoleV1 {
    ConditionBound,
    ConditionInductionRead,
    ConditionCompare,
    AccumulatorRead,
    AccumulatorInductionRead,
    AccumulatorAdd,
    AccumulatorWrite,
    StepInductionRead,
    StepDelta,
    StepAdd,
    StepInductionWrite,
}

impl VariableAccumRecurrenceSourceRoleV1 {
    pub(crate) const ALL: [Self; 11] = [
        Self::ConditionBound,
        Self::ConditionInductionRead,
        Self::ConditionCompare,
        Self::AccumulatorRead,
        Self::AccumulatorInductionRead,
        Self::AccumulatorAdd,
        Self::AccumulatorWrite,
        Self::StepInductionRead,
        Self::StepDelta,
        Self::StepAdd,
        Self::StepInductionWrite,
    ];

    pub(crate) const fn ordinal(self) -> u8 {
        match self {
            Self::ConditionBound => 0,
            Self::ConditionInductionRead => 1,
            Self::ConditionCompare => 2,
            Self::AccumulatorRead => 3,
            Self::AccumulatorInductionRead => 4,
            Self::AccumulatorAdd => 5,
            Self::AccumulatorWrite => 6,
            Self::StepInductionRead => 7,
            Self::StepDelta => 8,
            Self::StepAdd => 9,
            Self::StepInductionWrite => 10,
        }
    }
}

/// The bounded cohort has one value class.  Keeping it typed avoids making
/// the producer infer type facts from source sites or names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumRecurrenceValueClassV1 {
    I64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumRecurrenceBindingObservationV1 {
    role: VariableAccumRecurrenceBindingRoleV1,
    binding: BindingRefV1,
    declaration: SourceBindingSiteV1,
    value_class: VariableAccumRecurrenceValueClassV1,
}

impl VariableAccumRecurrenceBindingObservationV1 {
    pub(crate) const fn new(
        role: VariableAccumRecurrenceBindingRoleV1,
        binding: BindingRefV1,
        declaration: SourceBindingSiteV1,
        value_class: VariableAccumRecurrenceValueClassV1,
    ) -> Self {
        Self {
            role,
            binding,
            declaration,
            value_class,
        }
    }

    pub(crate) const fn role(&self) -> VariableAccumRecurrenceBindingRoleV1 {
        self.role
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }

    pub(crate) const fn value_class(&self) -> VariableAccumRecurrenceValueClassV1 {
        self.value_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumRecurrenceInputObservationV1 {
    role: VariableAccumRecurrenceInputRoleV1,
    declaration: SourceBindingSiteV1,
    initializer: SourceExprSiteV1,
    binding: BindingRefV1,
    value_class: VariableAccumRecurrenceValueClassV1,
}

impl VariableAccumRecurrenceInputObservationV1 {
    pub(crate) const fn new(
        role: VariableAccumRecurrenceInputRoleV1,
        declaration: SourceBindingSiteV1,
        initializer: SourceExprSiteV1,
        binding: BindingRefV1,
        value_class: VariableAccumRecurrenceValueClassV1,
    ) -> Self {
        Self {
            role,
            declaration,
            initializer,
            binding,
            value_class,
        }
    }

    pub(crate) const fn role(&self) -> VariableAccumRecurrenceInputRoleV1 {
        self.role
    }

    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }

    pub(crate) fn initializer(&self) -> &SourceExprSiteV1 {
        &self.initializer
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn value_class(&self) -> VariableAccumRecurrenceValueClassV1 {
        self.value_class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceConditionOperatorV1 {
    Less,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumRecurrenceConditionObservationV1 {
    site: SourceExprSiteV1,
    lhs: SourceExprSiteV1,
    rhs: SourceExprSiteV1,
    induction: BindingRefV1,
    bound: i64,
    operator: VariableAccumRecurrenceConditionOperatorV1,
}

impl VariableAccumRecurrenceConditionObservationV1 {
    pub(crate) const fn new(
        site: SourceExprSiteV1,
        lhs: SourceExprSiteV1,
        rhs: SourceExprSiteV1,
        induction: BindingRefV1,
        bound: i64,
        operator: VariableAccumRecurrenceConditionOperatorV1,
    ) -> Self {
        Self {
            site,
            lhs,
            rhs,
            induction,
            bound,
            operator,
        }
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn lhs(&self) -> &SourceExprSiteV1 {
        &self.lhs
    }

    pub(crate) fn rhs(&self) -> &SourceExprSiteV1 {
        &self.rhs
    }

    pub(crate) const fn induction(&self) -> BindingRefV1 {
        self.induction
    }

    pub(crate) const fn bound(&self) -> i64 {
        self.bound
    }

    pub(crate) const fn operator(&self) -> VariableAccumRecurrenceConditionOperatorV1 {
        self.operator
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumRecurrenceAccumulatorUpdateV1 {
    statement: SourceStmtSiteV1,
    target: SourceExprSiteV1,
    value: SourceExprSiteV1,
    lhs: SourceExprSiteV1,
    rhs: SourceExprSiteV1,
    accumulator: BindingRefV1,
    induction: BindingRefV1,
}

impl VariableAccumRecurrenceAccumulatorUpdateV1 {
    pub(crate) const fn new(
        statement: SourceStmtSiteV1,
        target: SourceExprSiteV1,
        value: SourceExprSiteV1,
        lhs: SourceExprSiteV1,
        rhs: SourceExprSiteV1,
        accumulator: BindingRefV1,
        induction: BindingRefV1,
    ) -> Self {
        Self {
            statement,
            target,
            value,
            lhs,
            rhs,
            accumulator,
            induction,
        }
    }

    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) fn target(&self) -> &SourceExprSiteV1 {
        &self.target
    }

    pub(crate) fn value(&self) -> &SourceExprSiteV1 {
        &self.value
    }

    pub(crate) fn lhs(&self) -> &SourceExprSiteV1 {
        &self.lhs
    }

    pub(crate) fn rhs(&self) -> &SourceExprSiteV1 {
        &self.rhs
    }

    pub(crate) const fn accumulator(&self) -> BindingRefV1 {
        self.accumulator
    }

    pub(crate) const fn induction(&self) -> BindingRefV1 {
        self.induction
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumRecurrenceInductionStepV1 {
    statement: SourceStmtSiteV1,
    target: SourceExprSiteV1,
    value: SourceExprSiteV1,
    lhs: SourceExprSiteV1,
    rhs: SourceExprSiteV1,
    induction: BindingRefV1,
    delta: i64,
}

impl VariableAccumRecurrenceInductionStepV1 {
    pub(crate) const fn new(
        statement: SourceStmtSiteV1,
        target: SourceExprSiteV1,
        value: SourceExprSiteV1,
        lhs: SourceExprSiteV1,
        rhs: SourceExprSiteV1,
        induction: BindingRefV1,
        delta: i64,
    ) -> Self {
        Self {
            statement,
            target,
            value,
            lhs,
            rhs,
            induction,
            delta,
        }
    }

    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) fn target(&self) -> &SourceExprSiteV1 {
        &self.target
    }

    pub(crate) fn value(&self) -> &SourceExprSiteV1 {
        &self.value
    }

    pub(crate) fn lhs(&self) -> &SourceExprSiteV1 {
        &self.lhs
    }

    pub(crate) fn rhs(&self) -> &SourceExprSiteV1 {
        &self.rhs
    }

    pub(crate) const fn induction(&self) -> BindingRefV1 {
        self.induction
    }

    pub(crate) const fn delta(&self) -> i64 {
        self.delta
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumRecurrenceCoverageV1 {
    root_statement_count: u32,
    body_statement_sites: Box<[SourceStmtSiteV1]>,
    operation_roles: Box<[VariableAccumRecurrenceSourceRoleV1]>,
}

impl VariableAccumRecurrenceCoverageV1 {
    pub(crate) fn new(
        root_statement_count: u32,
        body_statement_sites: Box<[SourceStmtSiteV1]>,
        operation_roles: Box<[VariableAccumRecurrenceSourceRoleV1]>,
    ) -> Self {
        Self {
            root_statement_count,
            body_statement_sites,
            operation_roles,
        }
    }

    pub(crate) const fn root_statement_count(&self) -> u32 {
        self.root_statement_count
    }

    pub(crate) fn body_statement_sites(&self) -> &[SourceStmtSiteV1] {
        &self.body_statement_sites
    }

    pub(crate) fn operation_roles(&self) -> &[VariableAccumRecurrenceSourceRoleV1] {
        &self.operation_roles
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumRecurrenceSourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

impl VariableAccumRecurrenceSourceIdentityV1 {
    pub(crate) fn from_source(
        owner: FunctionOwnerIdV1,
        source: &VerifiedResolvedLoopSourceV1,
    ) -> Self {
        Self {
            owner,
            function_origin: source.function_origin(),
            source_kind: source.source_kind(),
            site: source.site().clone(),
            frame: source.frame_key(),
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }
}

/// One complete source meaning.  It is intentionally non-`Clone` because it
/// retains the resolver-issued non-Clone source capability until the producer
/// consumes the Candidate.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedVariableAccumRecurrenceFactsV1 {
    source: VerifiedResolvedLoopSourceV1,
    owner: FunctionOwnerIdV1,
    scope_region: ResolvedScopeRegionPairV1,
    bindings: [VariableAccumRecurrenceBindingObservationV1; 2],
    inputs: [VariableAccumRecurrenceInputObservationV1; 2],
    condition: VariableAccumRecurrenceConditionObservationV1,
    accumulator_update: VariableAccumRecurrenceAccumulatorUpdateV1,
    induction_step: VariableAccumRecurrenceInductionStepV1,
    coverage: VariableAccumRecurrenceCoverageV1,
    _seal: VariableAccumRecurrenceFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VariableAccumRecurrenceFactsSealV1;

impl VerifiedVariableAccumRecurrenceFactsV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedResolvedLoopSourceV1,
        FunctionOwnerIdV1,
        ResolvedScopeRegionPairV1,
        [VariableAccumRecurrenceBindingObservationV1; 2],
        [VariableAccumRecurrenceInputObservationV1; 2],
        VariableAccumRecurrenceConditionObservationV1,
        VariableAccumRecurrenceAccumulatorUpdateV1,
        VariableAccumRecurrenceInductionStepV1,
        VariableAccumRecurrenceCoverageV1,
    ) {
        (
            self.source,
            self.owner,
            self.scope_region,
            self.bindings,
            self.inputs,
            self.condition,
            self.accumulator_update,
            self.induction_step,
            self.coverage,
        )
    }

    pub(crate) fn source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.source
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.source.function_origin()
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source.source_kind()
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        self.source.site()
    }

    pub(crate) fn frame(&self) -> LoopExecutionFrameKeyV1 {
        self.source.frame_key()
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    pub(crate) fn bindings(&self) -> &[VariableAccumRecurrenceBindingObservationV1; 2] {
        &self.bindings
    }

    pub(crate) fn inputs(&self) -> &[VariableAccumRecurrenceInputObservationV1; 2] {
        &self.inputs
    }

    pub(crate) fn condition(&self) -> &VariableAccumRecurrenceConditionObservationV1 {
        &self.condition
    }

    pub(crate) fn accumulator_update(&self) -> &VariableAccumRecurrenceAccumulatorUpdateV1 {
        &self.accumulator_update
    }

    pub(crate) fn induction_step(&self) -> &VariableAccumRecurrenceInductionStepV1 {
        &self.induction_step
    }

    pub(crate) fn coverage(&self) -> &VariableAccumRecurrenceCoverageV1 {
        &self.coverage
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceSourceDeclineV1 {
    NotVariableAccumRecurrenceShape,
    UnsupportedOperator,
    UnsupportedBodyShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceSourceUnresolvedV1 {
    SourceNavigation,
    MissingEvidence,
    OpaqueExpression,
    IncompleteCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceSourceRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    ForeignFrame,
    DuplicateRole,
    BindingConflict,
    InputConflict,
    CoverageConflict,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceSourceAttemptOutcomeV1 {
    Candidate(VerifiedVariableAccumRecurrenceFactsV1),
    Declined(VariableAccumRecurrenceSourceDeclineV1),
    Unresolved(VariableAccumRecurrenceSourceUnresolvedV1),
    Rejected(VariableAccumRecurrenceSourceRejectV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceObservationCoverageV1 {
    Complete,
    Incomplete,
}

/// Envelope for non-Candidate rows.  Candidate remains the sole owner of the
/// resolver source capability; other rows retain only identity/coverage so a
/// missing or foreign observation cannot be mistaken for a valid Facts value.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedVariableAccumRecurrenceSourceAttemptV1 {
    outcome: VariableAccumRecurrenceSourceAttemptOutcomeV1,
    identity: VariableAccumRecurrenceSourceIdentityV1,
    coverage: VariableAccumRecurrenceObservationCoverageV1,
    _seal: VariableAccumRecurrenceAttemptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VariableAccumRecurrenceAttemptSealV1;

impl VerifiedVariableAccumRecurrenceSourceAttemptV1 {
    pub(crate) fn new(
        outcome: VariableAccumRecurrenceSourceAttemptOutcomeV1,
        identity: VariableAccumRecurrenceSourceIdentityV1,
        coverage: VariableAccumRecurrenceObservationCoverageV1,
    ) -> Self {
        Self {
            outcome,
            identity,
            coverage,
            _seal: VariableAccumRecurrenceAttemptSealV1,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VariableAccumRecurrenceSourceAttemptOutcomeV1,
        VariableAccumRecurrenceSourceIdentityV1,
        VariableAccumRecurrenceObservationCoverageV1,
    ) {
        (self.outcome, self.identity, self.coverage)
    }

    pub(crate) fn outcome(&self) -> &VariableAccumRecurrenceSourceAttemptOutcomeV1 {
        &self.outcome
    }

    pub(crate) fn identity(&self) -> &VariableAccumRecurrenceSourceIdentityV1 {
        &self.identity
    }

    pub(crate) const fn coverage(&self) -> VariableAccumRecurrenceObservationCoverageV1 {
        self.coverage
    }
}

/// Typed failures from the single atomic Facts issuer.  Projection maps these
/// to `Rejected` because an issuer failure means identity/coverage consistency
/// was contradicted, not that another family should be selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceFactsIssueV1 {
    ForeignOwner,
    ForeignFrame,
    BindingConflict,
    InputConflict,
    CoverageConflict,
    RoleConflict,
}

/// Co-seals the complete source meaning.  The cardinality and role checks are
/// intentionally local to this owner; no producer is allowed to repair them.
pub(crate) fn issue_variable_accum_recurrence_facts_v1(
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    scope_region: ResolvedScopeRegionPairV1,
    bindings: [VariableAccumRecurrenceBindingObservationV1; 2],
    inputs: [VariableAccumRecurrenceInputObservationV1; 2],
    condition: VariableAccumRecurrenceConditionObservationV1,
    accumulator_update: VariableAccumRecurrenceAccumulatorUpdateV1,
    induction_step: VariableAccumRecurrenceInductionStepV1,
    coverage: VariableAccumRecurrenceCoverageV1,
) -> Result<VerifiedVariableAccumRecurrenceFactsV1, VariableAccumRecurrenceFactsIssueV1> {
    if bindings.iter().any(|row| row.binding().owner() != owner)
        || inputs.iter().any(|row| row.binding().owner() != owner)
        || condition.induction().owner() != owner
        || accumulator_update.accumulator().owner() != owner
        || accumulator_update.induction().owner() != owner
        || induction_step.induction().owner() != owner
    {
        return Err(VariableAccumRecurrenceFactsIssueV1::ForeignOwner);
    }
    if !scope_region.scope().owner().eq(&owner) || !scope_region.region().owner().eq(&owner) {
        return Err(VariableAccumRecurrenceFactsIssueV1::ForeignFrame);
    }
    let induction = binding_for_role(&bindings, VariableAccumRecurrenceBindingRoleV1::Induction)
        .ok_or(VariableAccumRecurrenceFactsIssueV1::RoleConflict)?;
    let accumulator =
        binding_for_role(&bindings, VariableAccumRecurrenceBindingRoleV1::Accumulator)
            .ok_or(VariableAccumRecurrenceFactsIssueV1::RoleConflict)?;
    if induction == accumulator {
        return Err(VariableAccumRecurrenceFactsIssueV1::BindingConflict);
    }
    if condition.induction() != induction
        || accumulator_update.induction() != induction
        || induction_step.induction() != induction
        || accumulator_update.accumulator() != accumulator
    {
        return Err(VariableAccumRecurrenceFactsIssueV1::BindingConflict);
    }
    if !has_input(
        &inputs,
        VariableAccumRecurrenceInputRoleV1::InductionInitial,
        induction,
    ) || !has_input(
        &inputs,
        VariableAccumRecurrenceInputRoleV1::AccumulatorInitial,
        accumulator,
    ) {
        return Err(VariableAccumRecurrenceFactsIssueV1::InputConflict);
    }
    if bindings
        .iter()
        .any(|row| row.value_class() != VariableAccumRecurrenceValueClassV1::I64)
        || inputs
            .iter()
            .any(|row| row.value_class() != VariableAccumRecurrenceValueClassV1::I64)
    {
        return Err(VariableAccumRecurrenceFactsIssueV1::BindingConflict);
    }
    if !inputs.iter().all(|input| {
        bindings.iter().any(|binding| {
            binding.role()
                == match input.role() {
                    VariableAccumRecurrenceInputRoleV1::InductionInitial => {
                        VariableAccumRecurrenceBindingRoleV1::Induction
                    }
                    VariableAccumRecurrenceInputRoleV1::AccumulatorInitial => {
                        VariableAccumRecurrenceBindingRoleV1::Accumulator
                    }
                }
                && binding.binding() == input.binding()
                && binding.declaration() == input.declaration()
        })
    }) {
        return Err(VariableAccumRecurrenceFactsIssueV1::InputConflict);
    }
    let expected_roles = VariableAccumRecurrenceSourceRoleV1::ALL;
    if coverage.operation_roles() != expected_roles.as_slice() {
        return Err(VariableAccumRecurrenceFactsIssueV1::CoverageConflict);
    }
    if coverage.root_statement_count() != 5
        || coverage.body_statement_sites().len() != 2
        || coverage.body_statement_sites()[0] != *accumulator_update.statement()
        || coverage.body_statement_sites()[1] != *induction_step.statement()
    {
        return Err(VariableAccumRecurrenceFactsIssueV1::CoverageConflict);
    }
    if condition.operator() != VariableAccumRecurrenceConditionOperatorV1::Less {
        return Err(VariableAccumRecurrenceFactsIssueV1::RoleConflict);
    }

    Ok(VerifiedVariableAccumRecurrenceFactsV1 {
        source,
        owner,
        scope_region,
        bindings,
        inputs,
        condition,
        accumulator_update,
        induction_step,
        coverage,
        _seal: VariableAccumRecurrenceFactsSealV1,
    })
}

fn binding_for_role(
    bindings: &[VariableAccumRecurrenceBindingObservationV1; 2],
    role: VariableAccumRecurrenceBindingRoleV1,
) -> Option<BindingRefV1> {
    bindings
        .iter()
        .find(|row| row.role() == role)
        .map(|row| row.binding())
}

fn has_input(
    inputs: &[VariableAccumRecurrenceInputObservationV1; 2],
    role: VariableAccumRecurrenceInputRoleV1,
    binding: BindingRefV1,
) -> bool {
    inputs
        .iter()
        .any(|row| row.role() == role && row.binding() == binding)
}
