//! Caller-zero, resolver-issued source claims for Nested Predicate.
//!
//! This module is deliberately below the physical adapter. It consumes the
//! sealed source handoff and resolved-function records once, then exposes only
//! exact sites, bindings, values, and scope pairs. It has no AST, Builder,
//! physical IDs, SSA, PHI, route, or retry authority.

use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, RegionId, RegionKindV1,
    ResolvedScopeRegionPairV1, ScopeId, ScopeKindV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

use super::nested_predicate_source_handoff::VerifiedNestedPhysicalSourceHandoffV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPrefixBindingRoleV1 {
    RootInductionI,
    AncestorAccumulatorSum,
    ChildRecurrenceJ,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPrefixInitializedBindingV1 {
    role: NestedPrefixBindingRoleV1,
    declaration_site: SourceBindingSiteV1,
    kind: BindingKindV1,
    name: Box<str>,
    binding: BindingRefV1,
    initializer_statement_site: SourceStmtSiteV1,
    initializer_value_site: SourceExprSiteV1,
    initial: i64,
}

impl VerifiedNestedPrefixInitializedBindingV1 {
    pub(crate) const fn role(&self) -> NestedPrefixBindingRoleV1 {
        self.role
    }

    pub(crate) fn declaration_site(&self) -> &SourceBindingSiteV1 {
        &self.declaration_site
    }

    pub(crate) const fn kind(&self) -> BindingKindV1 {
        self.kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn initializer_statement_site(&self) -> &SourceStmtSiteV1 {
        &self.initializer_statement_site
    }

    pub(crate) fn initializer_value_site(&self) -> &SourceExprSiteV1 {
        &self.initializer_value_site
    }

    pub(crate) const fn initial(&self) -> i64 {
        self.initial
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPrefixUninitializedBindingV1 {
    role: NestedPrefixBindingRoleV1,
    declaration_site: SourceBindingSiteV1,
    kind: BindingKindV1,
    name: Box<str>,
    binding: BindingRefV1,
    lexical_scope: ScopeId,
}

impl VerifiedNestedPrefixUninitializedBindingV1 {
    pub(crate) const fn role(&self) -> NestedPrefixBindingRoleV1 {
        self.role
    }

    pub(crate) fn declaration_site(&self) -> &SourceBindingSiteV1 {
        &self.declaration_site
    }

    pub(crate) const fn kind(&self) -> BindingKindV1 {
        self.kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn lexical_scope(&self) -> ScopeId {
        self.lexical_scope
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPrefixInputV1 {
    owner: FunctionOwnerIdV1,
    frame_key: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
    initialized: [VerifiedNestedPrefixInitializedBindingV1; 2],
    uninitialized: VerifiedNestedPrefixUninitializedBindingV1,
    root_loop_pair: ResolvedScopeRegionPairV1,
    child_loop_pair: ResolvedScopeRegionPairV1,
    _seal: VerifiedNestedPrefixInputSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedNestedPrefixInputSealV1;

impl VerifiedNestedPrefixInputV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn frame_key(&self) -> &crate::mir::resolved_semantics::LoopExecutionFrameKeyV1 {
        &self.frame_key
    }

    pub(crate) fn initialized(&self) -> &[VerifiedNestedPrefixInitializedBindingV1; 2] {
        &self.initialized
    }

    pub(crate) fn uninitialized(&self) -> &VerifiedNestedPrefixUninitializedBindingV1 {
        &self.uninitialized
    }

    pub(crate) const fn root_loop_pair(&self) -> ResolvedScopeRegionPairV1 {
        self.root_loop_pair
    }

    pub(crate) const fn child_loop_pair(&self) -> ResolvedScopeRegionPairV1 {
        self.child_loop_pair
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NestedBindingEffectRoleV1 {
    RootPredicateReadI,
    ChildInitializeWriteJ,
    ChildPredicateReadJ,
    ChildAncestorReadSum,
    ChildAncestorWriteSum,
    ChildReadJ,
    ChildWriteJ,
    RootStepReadI,
    RootStepWriteI,
}

impl NestedBindingEffectRoleV1 {
    pub(crate) const ALL: [Self; 9] = [
        Self::RootPredicateReadI,
        Self::ChildInitializeWriteJ,
        Self::ChildPredicateReadJ,
        Self::ChildAncestorReadSum,
        Self::ChildAncestorWriteSum,
        Self::ChildReadJ,
        Self::ChildWriteJ,
        Self::RootStepReadI,
        Self::RootStepWriteI,
    ];
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NestedReadClaimV1 {
    role: NestedBindingEffectRoleV1,
    site: SourceExprSiteV1,
    binding: BindingRefV1,
}

impl NestedReadClaimV1 {
    pub(crate) const fn role(&self) -> NestedBindingEffectRoleV1 {
        self.role
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NestedAssignmentClaimV1 {
    role: NestedBindingEffectRoleV1,
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    value_site: SourceExprSiteV1,
    lhs_site: SourceExprSiteV1,
    binding: BindingRefV1,
    delta: i64,
}

impl NestedAssignmentClaimV1 {
    pub(crate) const fn role(&self) -> NestedBindingEffectRoleV1 {
        self.role
    }

    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn target_site(&self) -> &SourceExprSiteV1 {
        &self.target_site
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) fn lhs_site(&self) -> &SourceExprSiteV1 {
        &self.lhs_site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn delta(&self) -> i64 {
        self.delta
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NestedFirstAssignmentClaimV1 {
    role: NestedBindingEffectRoleV1,
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    value_site: SourceExprSiteV1,
    binding: BindingRefV1,
    value: i64,
}

impl NestedFirstAssignmentClaimV1 {
    pub(crate) const fn role(&self) -> NestedBindingEffectRoleV1 {
        self.role
    }

    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn target_site(&self) -> &SourceExprSiteV1 {
        &self.target_site
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NestedBindingEffectEntryV1 {
    Read(NestedReadClaimV1),
    Assignment(NestedAssignmentClaimV1),
    FirstAssignment(NestedFirstAssignmentClaimV1),
}

impl NestedBindingEffectEntryV1 {
    pub(crate) const fn role(&self) -> NestedBindingEffectRoleV1 {
        match self {
            Self::Read(claim) => claim.role(),
            Self::Assignment(claim) => claim.role(),
            Self::FirstAssignment(claim) => claim.role(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedScopeRetirementBoundaryV1 {
    RootLoopRegionExit,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedScopeRetirementClaimV1 {
    scope: ScopeId,
    region: RegionId,
    binding: BindingRefV1,
    root_loop_site: SourceStmtSiteV1,
    boundary: NestedScopeRetirementBoundaryV1,
}

impl VerifiedNestedScopeRetirementClaimV1 {
    pub(crate) const fn scope(&self) -> ScopeId {
        self.scope
    }

    pub(crate) const fn region(&self) -> RegionId {
        self.region
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn root_loop_site(&self) -> &SourceStmtSiteV1 {
        &self.root_loop_site
    }

    pub(crate) const fn boundary(&self) -> NestedScopeRetirementBoundaryV1 {
        self.boundary
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedBindingEffectPlanV1 {
    owner: FunctionOwnerIdV1,
    frame_key: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
    entries: [NestedBindingEffectEntryV1; 9],
    retirement: VerifiedNestedScopeRetirementClaimV1,
    _seal: VerifiedNestedBindingEffectPlanSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedNestedBindingEffectPlanSealV1;

impl VerifiedNestedBindingEffectPlanV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn frame_key(&self) -> &crate::mir::resolved_semantics::LoopExecutionFrameKeyV1 {
        &self.frame_key
    }

    pub(crate) fn entries(&self) -> &[NestedBindingEffectEntryV1; 9] {
        &self.entries
    }

    pub(crate) fn entry(&self, role: NestedBindingEffectRoleV1) -> &NestedBindingEffectEntryV1 {
        self.entries
            .iter()
            .find(|entry| entry.role() == role)
            .expect("all Nested effect roles are sealed")
    }

    pub(crate) fn retirement(&self) -> &VerifiedNestedScopeRetirementClaimV1 {
        &self.retirement
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedBindingExecutionClaimsV1 {
    prefix: VerifiedNestedPrefixInputV1,
    effect_plan: VerifiedNestedBindingEffectPlanV1,
    _seal: VerifiedNestedBindingExecutionClaimsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedNestedBindingExecutionClaimsSealV1;

impl VerifiedNestedBindingExecutionClaimsV1 {
    pub(crate) fn prefix(&self) -> &VerifiedNestedPrefixInputV1 {
        &self.prefix
    }

    pub(crate) fn effect_plan(&self) -> &VerifiedNestedBindingEffectPlanV1 {
        &self.effect_plan
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedNestedPrefixInputV1,
        VerifiedNestedBindingEffectPlanV1,
    ) {
        (self.prefix, self.effect_plan)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedBindingExecutionClaimsRejectV1 {
    OwnerMismatch,
    FrameMismatch,
    MissingBinding,
    BindingOriginMismatch,
    BindingKindMismatch,
    ScopeMismatch,
    RegionMismatch,
    InitializerMismatch,
    EffectShapeMismatch,
}

pub(crate) fn issue_nested_binding_execution_claims_v1(
    function: &VerifiedResolvedFunctionV1,
    handoff: &VerifiedNestedPhysicalSourceHandoffV1,
) -> Result<VerifiedNestedBindingExecutionClaimsV1, NestedBindingExecutionClaimsRejectV1> {
    if function.owner() != handoff.owner()
        || handoff
            .bindings()
            .iter()
            .any(|evidence| evidence.binding.owner() != function.owner())
    {
        return Err(NestedBindingExecutionClaimsRejectV1::OwnerMismatch);
    }
    let resolved_root = function
        .resolved_loop_source(handoff.root_site())
        .map_err(|_| NestedBindingExecutionClaimsRejectV1::FrameMismatch)?;
    if resolved_root.frame_key() != *handoff.root_frame_key() {
        return Err(NestedBindingExecutionClaimsRejectV1::FrameMismatch);
    }
    let root_pair = loop_pair(function, handoff.root_site())?;
    let child_pair = loop_pair(function, handoff.child_site())?;
    let initialized = [
        issue_initialized(
            function,
            handoff,
            0,
            NestedPrefixBindingRoleV1::RootInductionI,
        )?,
        issue_initialized(
            function,
            handoff,
            1,
            NestedPrefixBindingRoleV1::AncestorAccumulatorSum,
        )?,
    ];
    let child_binding = handoff.bindings()[2].binding;
    let child = issue_uninitialized(function, handoff, child_binding, root_pair.scope())?;
    let prefix = VerifiedNestedPrefixInputV1 {
        owner: function.owner(),
        frame_key: handoff.root_frame_key().clone(),
        initialized,
        uninitialized: child,
        root_loop_pair: root_pair,
        child_loop_pair: child_pair,
        _seal: VerifiedNestedPrefixInputSealV1,
    };
    let effect_plan = issue_effect_plan(function, handoff, root_pair)?;
    Ok(VerifiedNestedBindingExecutionClaimsV1 {
        prefix,
        effect_plan,
        _seal: VerifiedNestedBindingExecutionClaimsSealV1,
    })
}

fn issue_initialized(
    function: &VerifiedResolvedFunctionV1,
    handoff: &VerifiedNestedPhysicalSourceHandoffV1,
    index: usize,
    role: NestedPrefixBindingRoleV1,
) -> Result<VerifiedNestedPrefixInitializedBindingV1, NestedBindingExecutionClaimsRejectV1> {
    let initializer = handoff
        .root_initializers()
        .get(index)
        .ok_or(NestedBindingExecutionClaimsRejectV1::InitializerMismatch)?;
    let declaration_site = SourceBindingSiteV1::Local {
        statement: initializer.statement_site.clone(),
        ordinal: index as u32,
    };
    let binding = declaration_binding(function, &declaration_site)?;
    if binding != initializer.binding {
        return Err(NestedBindingExecutionClaimsRejectV1::InitializerMismatch);
    }
    let record = exact_local_record(function, binding, &declaration_site, index as u32)?;
    Ok(VerifiedNestedPrefixInitializedBindingV1 {
        role,
        declaration_site,
        kind: record.kind(),
        name: record.diagnostic_name().into(),
        binding,
        initializer_statement_site: initializer.statement_site.clone(),
        initializer_value_site: initializer.value_site.clone(),
        initial: initializer.value,
    })
}

fn issue_uninitialized(
    function: &VerifiedResolvedFunctionV1,
    handoff: &VerifiedNestedPhysicalSourceHandoffV1,
    binding: BindingRefV1,
    expected_scope: ScopeId,
) -> Result<VerifiedNestedPrefixUninitializedBindingV1, NestedBindingExecutionClaimsRejectV1> {
    let site = handoff.child_declaration_site();
    let declared = declaration_binding(function, site)?;
    if declared != binding {
        return Err(NestedBindingExecutionClaimsRejectV1::BindingOriginMismatch);
    }
    let record = exact_local_record(function, binding, site, 0)?;
    let lexical_scope = record.owner_scope();
    if lexical_scope != expected_scope {
        return Err(NestedBindingExecutionClaimsRejectV1::ScopeMismatch);
    }
    Ok(VerifiedNestedPrefixUninitializedBindingV1 {
        role: NestedPrefixBindingRoleV1::ChildRecurrenceJ,
        declaration_site: site.clone(),
        kind: record.kind(),
        name: record.diagnostic_name().into(),
        binding,
        lexical_scope,
    })
}

fn issue_effect_plan(
    function: &VerifiedResolvedFunctionV1,
    handoff: &VerifiedNestedPhysicalSourceHandoffV1,
    root_pair: ResolvedScopeRegionPairV1,
) -> Result<VerifiedNestedBindingEffectPlanV1, NestedBindingExecutionClaimsRejectV1> {
    let conditions = handoff.conditions();
    let updates = handoff.updates();
    let root_i = handoff.bindings()[0].binding;
    let sum = handoff.bindings()[1].binding;
    let child = handoff.bindings()[2].binding;
    if conditions[0].binding != root_i
        || conditions[1].binding != child
        || updates[0].binding != child
        || updates[1].binding != root_i
        || updates[2].binding != sum
        || updates[3].binding != child
    {
        return Err(NestedBindingExecutionClaimsRejectV1::EffectShapeMismatch);
    }
    let entries = [
        NestedBindingEffectEntryV1::Read(NestedReadClaimV1 {
            role: NestedBindingEffectRoleV1::RootPredicateReadI,
            site: conditions[0].lhs_site.clone(),
            binding: root_i,
        }),
        NestedBindingEffectEntryV1::FirstAssignment(NestedFirstAssignmentClaimV1 {
            role: NestedBindingEffectRoleV1::ChildInitializeWriteJ,
            statement_site: updates[0].statement_site.clone(),
            target_site: updates[0].target_site.clone(),
            value_site: updates[0].value_site.clone(),
            binding: child,
            value: updates[0].delta,
        }),
        NestedBindingEffectEntryV1::Read(NestedReadClaimV1 {
            role: NestedBindingEffectRoleV1::ChildPredicateReadJ,
            site: conditions[1].lhs_site.clone(),
            binding: child,
        }),
        NestedBindingEffectEntryV1::Read(NestedReadClaimV1 {
            role: NestedBindingEffectRoleV1::ChildAncestorReadSum,
            site: updates[2].lhs_site.clone(),
            binding: sum,
        }),
        NestedBindingEffectEntryV1::Assignment(NestedAssignmentClaimV1 {
            role: NestedBindingEffectRoleV1::ChildAncestorWriteSum,
            statement_site: updates[2].statement_site.clone(),
            target_site: updates[2].target_site.clone(),
            value_site: updates[2].value_site.clone(),
            lhs_site: updates[2].lhs_site.clone(),
            binding: sum,
            delta: updates[2].delta,
        }),
        NestedBindingEffectEntryV1::Read(NestedReadClaimV1 {
            role: NestedBindingEffectRoleV1::ChildReadJ,
            site: updates[3].lhs_site.clone(),
            binding: child,
        }),
        NestedBindingEffectEntryV1::Assignment(NestedAssignmentClaimV1 {
            role: NestedBindingEffectRoleV1::ChildWriteJ,
            statement_site: updates[3].statement_site.clone(),
            target_site: updates[3].target_site.clone(),
            value_site: updates[3].value_site.clone(),
            lhs_site: updates[3].lhs_site.clone(),
            binding: child,
            delta: updates[3].delta,
        }),
        NestedBindingEffectEntryV1::Read(NestedReadClaimV1 {
            role: NestedBindingEffectRoleV1::RootStepReadI,
            site: updates[1].lhs_site.clone(),
            binding: root_i,
        }),
        NestedBindingEffectEntryV1::Assignment(NestedAssignmentClaimV1 {
            role: NestedBindingEffectRoleV1::RootStepWriteI,
            statement_site: updates[1].statement_site.clone(),
            target_site: updates[1].target_site.clone(),
            value_site: updates[1].value_site.clone(),
            lhs_site: updates[1].lhs_site.clone(),
            binding: root_i,
            delta: updates[1].delta,
        }),
    ];
    let child_record = function
        .binding(child)
        .ok_or(NestedBindingExecutionClaimsRejectV1::MissingBinding)?;
    if child_record.owner_scope() != handoff.bindings()[2].lexical_scope {
        return Err(NestedBindingExecutionClaimsRejectV1::ScopeMismatch);
    }
    Ok(VerifiedNestedBindingEffectPlanV1 {
        owner: function.owner(),
        frame_key: handoff.root_frame_key().clone(),
        entries,
        retirement: VerifiedNestedScopeRetirementClaimV1 {
            scope: child_record.owner_scope(),
            region: root_pair.region(),
            binding: child,
            root_loop_site: handoff.root_site().clone(),
            boundary: NestedScopeRetirementBoundaryV1::RootLoopRegionExit,
        },
        _seal: VerifiedNestedBindingEffectPlanSealV1,
    })
}

fn declaration_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceBindingSiteV1,
) -> Result<BindingRefV1, NestedBindingExecutionClaimsRejectV1> {
    function
        .declaration_binding(site)
        .ok_or(NestedBindingExecutionClaimsRejectV1::MissingBinding)
}

fn exact_local_record<'a>(
    function: &'a VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
    site: &SourceBindingSiteV1,
    ordinal: u32,
) -> Result<
    &'a crate::mir::resolved_semantics::ResolvedBindingRecordV1,
    NestedBindingExecutionClaimsRejectV1,
> {
    let record = function
        .binding(binding)
        .ok_or(NestedBindingExecutionClaimsRejectV1::MissingBinding)?;
    if record.origin() != &BindingOriginV1::Source(site.clone()) {
        return Err(NestedBindingExecutionClaimsRejectV1::BindingOriginMismatch);
    }
    if record.kind() != (BindingKindV1::Local { ordinal }) {
        return Err(NestedBindingExecutionClaimsRejectV1::BindingKindMismatch);
    }
    Ok(record)
}

fn loop_pair(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceStmtSiteV1,
) -> Result<ResolvedScopeRegionPairV1, NestedBindingExecutionClaimsRejectV1> {
    let pair = function
        .loop_region_bundle(site)
        .map_err(|_| NestedBindingExecutionClaimsRejectV1::RegionMismatch)?
        .loop_pair();
    if function.scope(pair.scope()).map(|scope| scope.kind()) != Some(ScopeKindV1::LoopBody) {
        return Err(NestedBindingExecutionClaimsRejectV1::ScopeMismatch);
    }
    if function.region(pair.region()).map(|region| region.kind()) != Some(RegionKindV1::Loop) {
        return Err(NestedBindingExecutionClaimsRejectV1::RegionMismatch);
    }
    Ok(pair)
}
