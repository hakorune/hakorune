//! AST-free Facts for the bounded accumulator-with-break source family.
//!
//! The compiler projection owns syntax navigation.  This module only seals
//! resolver-branded source sites and typed source roles.  It never creates
//! Recipe keys, JoinSig edges, route IDs, or physical identities.

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    ResolvedScopeRegionPairV1, SemanticOwnerSourceKindV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumBreakBindingRoleV1 {
    Induction,
    Accumulator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumBreakInputRoleV1 {
    InductionInitial,
    AccumulatorInitial,
}

/// Canonical source roles for the 20 Recipe items in this bounded cohort.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumBreakOperationRoleV1 {
    LoopConditionBound,
    LoopConditionInductionRead,
    LoopConditionCompare,
    BranchConditionInductionRead,
    BranchConditionBound,
    BranchConditionCompare,
    BranchIf,
    TerminalAccumulatorRead,
    TerminalDelta,
    TerminalAdd,
    TerminalWrite,
    TerminalBreak,
    NormalAccumulatorRead,
    NormalDelta,
    NormalAdd,
    NormalWrite,
    StepInductionRead,
    StepDelta,
    StepAdd,
    StepInductionWrite,
}

impl VariableAccumBreakOperationRoleV1 {
    pub(crate) const ALL: [Self; 20] = [
        Self::LoopConditionBound,
        Self::LoopConditionInductionRead,
        Self::LoopConditionCompare,
        Self::BranchConditionInductionRead,
        Self::BranchConditionBound,
        Self::BranchConditionCompare,
        Self::BranchIf,
        Self::TerminalAccumulatorRead,
        Self::TerminalDelta,
        Self::TerminalAdd,
        Self::TerminalWrite,
        Self::TerminalBreak,
        Self::NormalAccumulatorRead,
        Self::NormalDelta,
        Self::NormalAdd,
        Self::NormalWrite,
        Self::StepInductionRead,
        Self::StepDelta,
        Self::StepAdd,
        Self::StepInductionWrite,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VariableAccumBreakValueClassV1 {
    I64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumBreakBindingObservationV1 {
    role: VariableAccumBreakBindingRoleV1,
    binding: BindingRefV1,
    declaration: SourceBindingSiteV1,
    value_class: VariableAccumBreakValueClassV1,
}

impl VariableAccumBreakBindingObservationV1 {
    pub(crate) const fn new(
        role: VariableAccumBreakBindingRoleV1,
        binding: BindingRefV1,
        declaration: SourceBindingSiteV1,
        value_class: VariableAccumBreakValueClassV1,
    ) -> Self {
        Self {
            role,
            binding,
            declaration,
            value_class,
        }
    }
    pub(crate) const fn role(&self) -> VariableAccumBreakBindingRoleV1 {
        self.role
    }
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }
    pub(crate) const fn value_class(&self) -> VariableAccumBreakValueClassV1 {
        self.value_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumBreakInputObservationV1 {
    role: VariableAccumBreakInputRoleV1,
    declaration: SourceBindingSiteV1,
    initializer: SourceExprSiteV1,
    binding: BindingRefV1,
    value_class: VariableAccumBreakValueClassV1,
}

impl VariableAccumBreakInputObservationV1 {
    pub(crate) const fn new(
        role: VariableAccumBreakInputRoleV1,
        declaration: SourceBindingSiteV1,
        initializer: SourceExprSiteV1,
        binding: BindingRefV1,
        value_class: VariableAccumBreakValueClassV1,
    ) -> Self {
        Self {
            role,
            declaration,
            initializer,
            binding,
            value_class,
        }
    }
    pub(crate) const fn role(&self) -> VariableAccumBreakInputRoleV1 {
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
    pub(crate) const fn value_class(&self) -> VariableAccumBreakValueClassV1 {
        self.value_class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakCompareV1 {
    Less,
    Equal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumBreakConditionObservationV1 {
    site: SourceExprSiteV1,
    lhs: SourceExprSiteV1,
    rhs: SourceExprSiteV1,
    binding: BindingRefV1,
    bound: i64,
    operator: VariableAccumBreakCompareV1,
}

impl VariableAccumBreakConditionObservationV1 {
    pub(crate) const fn new(
        site: SourceExprSiteV1,
        lhs: SourceExprSiteV1,
        rhs: SourceExprSiteV1,
        binding: BindingRefV1,
        bound: i64,
        operator: VariableAccumBreakCompareV1,
    ) -> Self {
        Self {
            site,
            lhs,
            rhs,
            binding,
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
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) const fn bound(&self) -> i64 {
        self.bound
    }
    pub(crate) const fn operator(&self) -> VariableAccumBreakCompareV1 {
        self.operator
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumBreakAssignmentObservationV1 {
    statement: SourceStmtSiteV1,
    target: SourceExprSiteV1,
    value: SourceExprSiteV1,
    lhs: SourceExprSiteV1,
    rhs: SourceExprSiteV1,
    target_binding: BindingRefV1,
    lhs_binding: BindingRefV1,
    delta: i64,
}

impl VariableAccumBreakAssignmentObservationV1 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        statement: SourceStmtSiteV1,
        target: SourceExprSiteV1,
        value: SourceExprSiteV1,
        lhs: SourceExprSiteV1,
        rhs: SourceExprSiteV1,
        target_binding: BindingRefV1,
        lhs_binding: BindingRefV1,
        delta: i64,
    ) -> Self {
        Self {
            statement,
            target,
            value,
            lhs,
            rhs,
            target_binding,
            lhs_binding,
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
    pub(crate) const fn target_binding(&self) -> BindingRefV1 {
        self.target_binding
    }
    pub(crate) const fn lhs_binding(&self) -> BindingRefV1 {
        self.lhs_binding
    }
    pub(crate) const fn delta(&self) -> i64 {
        self.delta
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumBreakCoverageV1 {
    root_statement_count: u32,
    statement_sites: Box<[SourceStmtSiteV1]>,
    operation_roles: Box<[VariableAccumBreakOperationRoleV1]>,
}

impl VariableAccumBreakCoverageV1 {
    pub(crate) fn new(
        root_statement_count: u32,
        statement_sites: Box<[SourceStmtSiteV1]>,
        operation_roles: Box<[VariableAccumBreakOperationRoleV1]>,
    ) -> Self {
        Self {
            root_statement_count,
            statement_sites,
            operation_roles,
        }
    }
    pub(crate) const fn root_statement_count(&self) -> u32 {
        self.root_statement_count
    }
    pub(crate) fn statement_sites(&self) -> &[SourceStmtSiteV1] {
        &self.statement_sites
    }
    pub(crate) fn operation_roles(&self) -> &[VariableAccumBreakOperationRoleV1] {
        &self.operation_roles
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariableAccumBreakSourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

impl VariableAccumBreakSourceIdentityV1 {
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedVariableAccumBreakFactsV1 {
    source: VerifiedResolvedLoopSourceV1,
    owner: FunctionOwnerIdV1,
    scope_region: ResolvedScopeRegionPairV1,
    bindings: [VariableAccumBreakBindingObservationV1; 2],
    inputs: [VariableAccumBreakInputObservationV1; 2],
    loop_condition: VariableAccumBreakConditionObservationV1,
    branch_condition: VariableAccumBreakConditionObservationV1,
    terminal_update: VariableAccumBreakAssignmentObservationV1,
    normal_update: VariableAccumBreakAssignmentObservationV1,
    induction_step: VariableAccumBreakAssignmentObservationV1,
    branch_site: SourceStmtSiteV1,
    break_site: SourceStmtSiteV1,
    coverage: VariableAccumBreakCoverageV1,
    _seal: VariableAccumBreakFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VariableAccumBreakFactsSealV1;

impl VerifiedVariableAccumBreakFactsV1 {
    #[allow(clippy::type_complexity)]
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedResolvedLoopSourceV1,
        FunctionOwnerIdV1,
        ResolvedScopeRegionPairV1,
        [VariableAccumBreakBindingObservationV1; 2],
        [VariableAccumBreakInputObservationV1; 2],
        VariableAccumBreakConditionObservationV1,
        VariableAccumBreakConditionObservationV1,
        VariableAccumBreakAssignmentObservationV1,
        VariableAccumBreakAssignmentObservationV1,
        VariableAccumBreakAssignmentObservationV1,
        SourceStmtSiteV1,
        SourceStmtSiteV1,
        VariableAccumBreakCoverageV1,
    ) {
        (
            self.source,
            self.owner,
            self.scope_region,
            self.bindings,
            self.inputs,
            self.loop_condition,
            self.branch_condition,
            self.terminal_update,
            self.normal_update,
            self.induction_step,
            self.branch_site,
            self.break_site,
            self.coverage,
        )
    }
    pub(crate) fn source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.source
    }
    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        self.source.site()
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn frame(&self) -> LoopExecutionFrameKeyV1 {
        self.source.frame_key()
    }
    pub(crate) fn bindings(&self) -> &[VariableAccumBreakBindingObservationV1; 2] {
        &self.bindings
    }
    pub(crate) fn inputs(&self) -> &[VariableAccumBreakInputObservationV1; 2] {
        &self.inputs
    }
    pub(crate) fn loop_condition(&self) -> &VariableAccumBreakConditionObservationV1 {
        &self.loop_condition
    }
    pub(crate) fn branch_condition(&self) -> &VariableAccumBreakConditionObservationV1 {
        &self.branch_condition
    }
    pub(crate) fn terminal_update(&self) -> &VariableAccumBreakAssignmentObservationV1 {
        &self.terminal_update
    }
    pub(crate) fn normal_update(&self) -> &VariableAccumBreakAssignmentObservationV1 {
        &self.normal_update
    }
    pub(crate) fn induction_step(&self) -> &VariableAccumBreakAssignmentObservationV1 {
        &self.induction_step
    }
    pub(crate) fn branch_site(&self) -> &SourceStmtSiteV1 {
        &self.branch_site
    }
    pub(crate) fn break_site(&self) -> &SourceStmtSiteV1 {
        &self.break_site
    }
    pub(crate) fn coverage(&self) -> &VariableAccumBreakCoverageV1 {
        &self.coverage
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakSourceDeclineV1 {
    NotVariableAccumBreakShape,
    UnsupportedOperator,
    UnsupportedBodyShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakSourceUnresolvedV1 {
    SourceNavigation,
    MissingEvidence,
    IncompleteCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakSourceRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    SourceSiteConflict,
    ForeignFrame,
    DuplicateRole,
    BindingConflict,
    InputConflict,
    CoverageConflict,
    ExitTargetMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakSourceAttemptOutcomeV1 {
    Candidate(VerifiedVariableAccumBreakFactsV1),
    Declined(VariableAccumBreakSourceDeclineV1),
    Unresolved(VariableAccumBreakSourceUnresolvedV1),
    Rejected(VariableAccumBreakSourceRejectV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakObservationCoverageV1 {
    Complete,
    Incomplete,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedVariableAccumBreakSourceAttemptV1 {
    outcome: VariableAccumBreakSourceAttemptOutcomeV1,
    identity: VariableAccumBreakSourceIdentityV1,
    coverage: VariableAccumBreakObservationCoverageV1,
    _seal: VariableAccumBreakAttemptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VariableAccumBreakAttemptSealV1;

impl VerifiedVariableAccumBreakSourceAttemptV1 {
    pub(crate) fn new(
        outcome: VariableAccumBreakSourceAttemptOutcomeV1,
        identity: VariableAccumBreakSourceIdentityV1,
        coverage: VariableAccumBreakObservationCoverageV1,
    ) -> Self {
        Self {
            outcome,
            identity,
            coverage,
            _seal: VariableAccumBreakAttemptSealV1,
        }
    }
    pub(crate) fn into_parts(
        self,
    ) -> (
        VariableAccumBreakSourceAttemptOutcomeV1,
        VariableAccumBreakSourceIdentityV1,
        VariableAccumBreakObservationCoverageV1,
    ) {
        (self.outcome, self.identity, self.coverage)
    }
    pub(crate) fn outcome(&self) -> &VariableAccumBreakSourceAttemptOutcomeV1 {
        &self.outcome
    }
    pub(crate) fn identity(&self) -> &VariableAccumBreakSourceIdentityV1 {
        &self.identity
    }
    pub(crate) const fn coverage(&self) -> VariableAccumBreakObservationCoverageV1 {
        self.coverage
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakFactsIssueV1 {
    ForeignOwner,
    ForeignFrame,
    SourceSiteConflict,
    RoleConflict,
    BindingConflict,
    InputConflict,
    CoverageConflict,
}

pub(crate) fn issue_variable_accum_break_facts_v1(
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    scope_region: ResolvedScopeRegionPairV1,
    bindings: [VariableAccumBreakBindingObservationV1; 2],
    inputs: [VariableAccumBreakInputObservationV1; 2],
    loop_condition: VariableAccumBreakConditionObservationV1,
    branch_condition: VariableAccumBreakConditionObservationV1,
    terminal_update: VariableAccumBreakAssignmentObservationV1,
    normal_update: VariableAccumBreakAssignmentObservationV1,
    induction_step: VariableAccumBreakAssignmentObservationV1,
    branch_site: SourceStmtSiteV1,
    break_site: SourceStmtSiteV1,
    coverage: VariableAccumBreakCoverageV1,
) -> Result<VerifiedVariableAccumBreakFactsV1, VariableAccumBreakFactsIssueV1> {
    if scope_region.scope().owner() != owner {
        return Err(VariableAccumBreakFactsIssueV1::ForeignOwner);
    }
    if bindings[0].binding() == bindings[1].binding()
        || bindings[0].binding().owner() != owner
        || bindings[1].binding().owner() != owner
    {
        return Err(VariableAccumBreakFactsIssueV1::BindingConflict);
    }
    if bindings[0].role() != VariableAccumBreakBindingRoleV1::Induction
        || bindings[1].role() != VariableAccumBreakBindingRoleV1::Accumulator
    {
        return Err(VariableAccumBreakFactsIssueV1::RoleConflict);
    }
    let induction = bindings[0].binding();
    let accumulator = bindings[1].binding();
    if inputs[0].role() != VariableAccumBreakInputRoleV1::InductionInitial
        || inputs[1].role() != VariableAccumBreakInputRoleV1::AccumulatorInitial
        || inputs[0].binding() != induction
        || inputs[1].binding() != accumulator
    {
        return Err(VariableAccumBreakFactsIssueV1::InputConflict);
    }
    if loop_condition.binding() != induction
        || branch_condition.binding() != induction
        || terminal_update.target_binding() != accumulator
        || terminal_update.lhs_binding() != accumulator
        || normal_update.target_binding() != accumulator
        || normal_update.lhs_binding() != accumulator
        || induction_step.target_binding() != induction
        || induction_step.lhs_binding() != induction
    {
        return Err(VariableAccumBreakFactsIssueV1::BindingConflict);
    }
    if loop_condition.operator() != VariableAccumBreakCompareV1::Less
        || branch_condition.operator() != VariableAccumBreakCompareV1::Equal
        || loop_condition.bound() < 0
        || terminal_update.delta() != 10
        || normal_update.delta() != 1
        || induction_step.delta() != 1
    {
        return Err(VariableAccumBreakFactsIssueV1::RoleConflict);
    }
    if coverage.operation_roles() != VariableAccumBreakOperationRoleV1::ALL {
        return Err(VariableAccumBreakFactsIssueV1::CoverageConflict);
    }
    // Statement sites are resolver-branded source coordinates, not owner-bearing
    // handles.  Owner coherence is sealed by the source capability, scope pair,
    // and BindingRef checks above; do not reconstruct it from coordinates here.
    Ok(VerifiedVariableAccumBreakFactsV1 {
        source,
        owner,
        scope_region,
        bindings,
        inputs,
        loop_condition,
        branch_condition,
        terminal_update,
        normal_update,
        induction_step,
        branch_site,
        break_site,
        coverage,
        _seal: VariableAccumBreakFactsSealV1,
    })
}
