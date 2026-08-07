//! Caller-zero source-map to portable Recipe co-seal.
//!
//! This module is test-only until the production Recipe selector is opened.
//! It consumes the resolver-issued source map exactly once, reuses the common
//! Recipe/JoinSig/source-bound Core owners, and publishes only logical source
//! contracts. No Builder, MIR, physical ID, retry, or fallback is allowed.

#![cfg(test)]

use crate::mir::loop_recipe_contract::{
    issue_source_bound_core_for_test, LoopBindingEffectAnchorV1, LoopBindingEffectRelationV1,
    LoopBindingEffectRoleV1, LoopBindingKeyV1, LoopCarrierKeyV1, LoopItemKeyV1,
    LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, LoopNodeKeyV1, LoopRecipeArtifactV1,
    LoopRecipeBindingRelationV1, LoopRecipeProducerIdV1, LoopRecipeProvenanceV1,
    LoopRecipeRejectReasonV1, LoopValueClassV1, LoopValueKeyV1, VerifiedLoopContinuationContractV1,
    VerifiedLoopCoreProductV1, VerifiedLoopPhysicalBoundaryV1, VerifiedLoopSemanticContextV1,
};
use crate::mir::loop_structural_facts::bind_resolved_loop_root_v1;
use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, ResolvedCallableRefV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};

use super::callable_single_loop_source_map::{
    CallableSourceMapRoleV1, CallableSourceMapRowV1, VerifiedCallableSingleLoopSourceMapV1,
};
use super::callable_single_loop_source_shapes::{
    SourceCallBoundaryShapeV1, SourceLiteralShapeV1, SyntaxBinaryOperatorV1,
};
use crate::mir::resolved_semantics::CallableSemanticSourceLedgerView;

#[path = "callable_single_loop_recipe_shape.rs"]
mod callable_single_loop_recipe_shape;
use callable_single_loop_recipe_shape::callable_recipe;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopInputRelationV1 {
    statement: SourceStmtSiteV1,
    initializer: SourceExprSiteV1,
    source_binding: BindingRefV1,
    recipe_value: LoopValueKeyV1,
    class: LoopValueClassV1,
}

impl VerifiedLoopInputRelationV1 {
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }
    pub(crate) fn initializer(&self) -> &SourceExprSiteV1 {
        &self.initializer
    }
    pub(crate) const fn source_binding(&self) -> BindingRefV1 {
        self.source_binding
    }
    pub(crate) const fn recipe_value(&self) -> LoopValueKeyV1 {
        self.recipe_value
    }
    pub(crate) const fn class(&self) -> LoopValueClassV1 {
        self.class
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopOperationSourceRelationV1 {
    role: CallableSourceMapRoleV1,
    item: LoopItemKeyV1,
    site: SourceExprSiteV1,
    operation: LoopRecipeOperationViewV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRecipeOperationViewV1 {
    ReadBinding {
        binding: LoopBindingKeyV1,
        result: LoopValueKeyV1,
    },
    ConstI64 {
        result: LoopValueKeyV1,
        value: i64,
    },
    CompareI64 {
        op: SyntaxBinaryOperatorV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    BinaryI64 {
        op: SyntaxBinaryOperatorV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    WriteBinding {
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
    },
}

impl VerifiedLoopOperationSourceRelationV1 {
    #[cfg(test)]
    pub(crate) fn new_for_test(
        role: CallableSourceMapRoleV1,
        item: LoopItemKeyV1,
        site: SourceExprSiteV1,
        operation: LoopRecipeOperationViewV1,
    ) -> Self {
        Self {
            role,
            item,
            site,
            operation,
        }
    }

    pub(crate) const fn role(&self) -> CallableSourceMapRoleV1 {
        self.role
    }
    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    pub(crate) const fn operation(&self) -> LoopRecipeOperationViewV1 {
        self.operation
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallablePreludeV1 {
    owner: FunctionOwnerIdV1,
    site: SourceExprSiteV1,
    binding: BindingRefV1,
    call: SourceCallBoundaryShapeV1,
    direct_callable: Option<ResolvedCallableRefV1>,
}

impl VerifiedCallablePreludeV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn call(&self) -> &SourceCallBoundaryShapeV1 {
        &self.call
    }
    pub(crate) const fn direct_callable(&self) -> Option<ResolvedCallableRefV1> {
        self.direct_callable
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallableTailV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    binding: BindingRefV1,
}

impl VerifiedCallableTailV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }
    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedLoopRecipeCoSealV1 {
    core: VerifiedLoopCoreProductV1,
    input: VerifiedLoopInputRelationV1,
    operations: Box<[VerifiedLoopOperationSourceRelationV1]>,
    context: VerifiedLoopSemanticContextV1,
    continuation: VerifiedLoopContinuationContractV1,
}

impl VerifiedLoopRecipeCoSealV1 {
    pub(crate) fn core(&self) -> &VerifiedLoopCoreProductV1 {
        &self.core
    }
    pub(crate) fn input(&self) -> &VerifiedLoopInputRelationV1 {
        &self.input
    }
    pub(crate) fn operations(&self) -> &[VerifiedLoopOperationSourceRelationV1] {
        &self.operations
    }
    pub(crate) fn context(&self) -> &VerifiedLoopSemanticContextV1 {
        &self.context
    }
    pub(crate) fn continuation(&self) -> &VerifiedLoopContinuationContractV1 {
        &self.continuation
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopCoreProductV1,
        VerifiedLoopInputRelationV1,
        Box<[VerifiedLoopOperationSourceRelationV1]>,
        VerifiedLoopSemanticContextV1,
        VerifiedLoopContinuationContractV1,
    ) {
        (
            self.core,
            self.input,
            self.operations,
            self.context,
            self.continuation,
        )
    }

    pub(crate) fn into_physical_boundary(self) -> VerifiedLoopPhysicalBoundaryV1 {
        let Self {
            core,
            continuation,
            input: _,
            operations: _,
            context: _,
        } = self;
        VerifiedLoopPhysicalBoundaryV1::from_parts(core, continuation.into_after())
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableSingleLoopRecipeProductV1 {
    co_seal: VerifiedLoopRecipeCoSealV1,
    prelude: VerifiedCallablePreludeV1,
    tail: VerifiedCallableTailV1,
}

impl VerifiedCallableSingleLoopRecipeProductV1 {
    pub(crate) fn co_seal(&self) -> &VerifiedLoopRecipeCoSealV1 {
        &self.co_seal
    }
    pub(crate) fn prelude(&self) -> &VerifiedCallablePreludeV1 {
        &self.prelude
    }
    pub(crate) fn tail(&self) -> &VerifiedCallableTailV1 {
        &self.tail
    }

    /// Consume the caller-zero product at the next explicit boundary.
    ///
    /// Keeping this move-only is important: a later physical prepare step
    /// must not be able to retain a second co-seal, prelude, or tail owner.
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopRecipeCoSealV1,
        VerifiedCallablePreludeV1,
        VerifiedCallableTailV1,
    ) {
        (self.co_seal, self.prelude, self.tail)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CallableRecipeCoSealRejectV1 {
    ForeignOwner,
    MissingRole(CallableSourceMapRoleV1),
    DuplicateRole(CallableSourceMapRoleV1),
    UnexpectedRole(CallableSourceMapRoleV1),
    WrongSite(CallableSourceMapRoleV1),
    WrongTarget(CallableSourceMapRoleV1),
    UnsupportedLiteral(CallableSourceMapRoleV1),
    UnsupportedOperator(CallableSourceMapRoleV1),
    MissingDeclaration,
    DuplicateDeclaration,
    NonLocalDeclaration,
    PrefixTailBindingMismatch,
    TailContinuationFusion,
    SourceRoot(crate::mir::loop_structural_facts::LoopRootSourceBindingRejectV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
}

const ROLES: [CallableSourceMapRoleV1; 9] = [
    CallableSourceMapRoleV1::InitialCarrier,
    CallableSourceMapRoleV1::ConditionRead,
    CallableSourceMapRoleV1::ConditionBound,
    CallableSourceMapRoleV1::ConditionOperator,
    CallableSourceMapRoleV1::StepRead,
    CallableSourceMapRoleV1::StepDelta,
    CallableSourceMapRoleV1::StepOperator,
    CallableSourceMapRoleV1::StepWrite,
    CallableSourceMapRoleV1::TailReturnRead,
];

pub(crate) fn issue_callable_single_loop_recipe_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    map: VerifiedCallableSingleLoopSourceMapV1,
) -> Result<VerifiedCallableSingleLoopRecipeProductV1, CallableRecipeCoSealRejectV1> {
    if map.owner() != ledger.owner() {
        return Err(CallableRecipeCoSealRejectV1::ForeignOwner);
    }
    let (owner, origin, source_kind, loop_source, frame, scope_region, rows, prefix_row) =
        map.into_parts();
    let loop_site = loop_source.site().clone();
    let context = VerifiedLoopSemanticContextV1::from_parts(
        owner,
        origin,
        source_kind,
        loop_site,
        frame,
        scope_region,
    );
    let prelude = decode_prelude(owner, prefix_row)?;
    let mut by_role = std::collections::BTreeMap::new();
    for row in rows.into_vec() {
        let role = row.role();
        if by_role.insert(role, row).is_some() {
            return Err(CallableRecipeCoSealRejectV1::DuplicateRole(role));
        }
    }
    for role in ROLES {
        if !by_role.contains_key(&role) {
            return Err(CallableRecipeCoSealRejectV1::MissingRole(role));
        }
    }
    let extra = by_role.keys().copied().find(|role| !ROLES.contains(role));
    if let Some(role) = extra {
        return Err(CallableRecipeCoSealRejectV1::UnexpectedRole(role));
    }

    let initial = take_role(&mut by_role, CallableSourceMapRoleV1::InitialCarrier)?;
    let condition_read = take_role(&mut by_role, CallableSourceMapRoleV1::ConditionRead)?;
    let condition_bound = take_role(&mut by_role, CallableSourceMapRoleV1::ConditionBound)?;
    let condition_operator = take_role(&mut by_role, CallableSourceMapRoleV1::ConditionOperator)?;
    let step_read = take_role(&mut by_role, CallableSourceMapRoleV1::StepRead)?;
    let step_delta = take_role(&mut by_role, CallableSourceMapRoleV1::StepDelta)?;
    let step_operator = take_role(&mut by_role, CallableSourceMapRoleV1::StepOperator)?;
    let step_write = take_role(&mut by_role, CallableSourceMapRoleV1::StepWrite)?;
    let tail_row = take_role(&mut by_role, CallableSourceMapRoleV1::TailReturnRead)?;
    let tail = decode_tail(owner, tail_row)?;
    if prelude.binding() != tail.binding() {
        return Err(CallableRecipeCoSealRejectV1::PrefixTailBindingMismatch);
    }
    let (carrier_binding, initial_literal) =
        initial
            .target()
            .initial_carrier()
            .ok_or(CallableRecipeCoSealRejectV1::WrongTarget(
                CallableSourceMapRoleV1::InitialCarrier,
            ))?;
    if !matches!(initial_literal, SourceLiteralShapeV1::Integer(0)) {
        return Err(CallableRecipeCoSealRejectV1::UnsupportedLiteral(
            CallableSourceMapRoleV1::InitialCarrier,
        ));
    }
    let condition_read_binding = binding_target(&condition_read)?;
    let step_read_binding = binding_target(&step_read)?;
    let step_write_binding = binding_target(&step_write)?;
    if condition_read_binding != carrier_binding
        || step_read_binding != carrier_binding
        || step_write_binding != carrier_binding
    {
        return Err(CallableRecipeCoSealRejectV1::WrongTarget(
            CallableSourceMapRoleV1::StepWrite,
        ));
    }
    let condition_bound_literal = literal_target(&condition_bound)?;
    let step_delta_literal = literal_target(&step_delta)?;
    if !matches!(condition_bound_literal, SourceLiteralShapeV1::Integer(1)) {
        return Err(CallableRecipeCoSealRejectV1::UnsupportedLiteral(
            CallableSourceMapRoleV1::ConditionBound,
        ));
    }
    if !matches!(step_delta_literal, SourceLiteralShapeV1::Integer(1)) {
        return Err(CallableRecipeCoSealRejectV1::UnsupportedLiteral(
            CallableSourceMapRoleV1::StepDelta,
        ));
    }
    if operator_target(&condition_operator)? != SyntaxBinaryOperatorV1::Less {
        return Err(CallableRecipeCoSealRejectV1::UnsupportedOperator(
            CallableSourceMapRoleV1::ConditionOperator,
        ));
    }
    if operator_target(&step_operator)? != SyntaxBinaryOperatorV1::Add {
        return Err(CallableRecipeCoSealRejectV1::UnsupportedOperator(
            CallableSourceMapRoleV1::StepOperator,
        ));
    }
    if tail.binding() == carrier_binding {
        return Err(CallableRecipeCoSealRejectV1::TailContinuationFusion);
    }

    let (declaration, statement) = declaration_for_binding(ledger, carrier_binding)?;
    let source_root = bind_resolved_loop_root_v1(loop_source)
        .map_err(CallableRecipeCoSealRejectV1::SourceRoot)?;
    let recipe = callable_recipe();
    let verified_recipe =
        crate::mir::loop_recipe_contract::LoopRecipeVerifierV1::verify(recipe.clone())
            .map_err(CallableRecipeCoSealRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_recipe);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::CallableSingleLoopV1),
        source_binding,
        recipe,
    );
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(CallableRecipeCoSealRejectV1::JoinSig)?;
    let after = join_sig
        .require_after_binding(
            LoopNodeKeyV1::new(0),
            LoopBindingKeyV1::new(0),
            LoopValueClassV1::I64,
        )
        .map_err(CallableRecipeCoSealRejectV1::JoinSig)?;
    let (input, operations, bindings, effects) = relations(
        &initial,
        &condition_read,
        &condition_bound,
        &condition_operator,
        &step_read,
        &step_delta,
        &step_operator,
        &step_write,
        carrier_binding,
        context.loop_site().clone(),
        statement,
        declaration,
    )?;
    let core = issue_source_bound_core_for_test(artifact, join_sig, owner, bindings, effects)
        .map_err(CallableRecipeCoSealRejectV1::Recipe)?;
    let continuation = VerifiedLoopContinuationContractV1::from_after(owner, after);
    Ok(VerifiedCallableSingleLoopRecipeProductV1 {
        co_seal: VerifiedLoopRecipeCoSealV1 {
            core,
            input,
            operations: operations.into_boxed_slice(),
            context,
            continuation,
        },
        prelude,
        tail,
    })
}

fn take_role(
    rows: &mut std::collections::BTreeMap<CallableSourceMapRoleV1, CallableSourceMapRowV1>,
    role: CallableSourceMapRoleV1,
) -> Result<CallableSourceMapRowV1, CallableRecipeCoSealRejectV1> {
    rows.remove(&role)
        .ok_or(CallableRecipeCoSealRejectV1::MissingRole(role))
}

fn decode_prelude(
    owner: FunctionOwnerIdV1,
    row: CallableSourceMapRowV1,
) -> Result<VerifiedCallablePreludeV1, CallableRecipeCoSealRejectV1> {
    let site = row
        .site()
        .expression()
        .cloned()
        .ok_or(CallableRecipeCoSealRejectV1::WrongSite(
            CallableSourceMapRoleV1::PrefixBoundary,
        ))?;
    let (binding, call, direct_callable) =
        row.target()
            .prefix()
            .ok_or(CallableRecipeCoSealRejectV1::WrongTarget(
                CallableSourceMapRoleV1::PrefixBoundary,
            ))?;
    Ok(VerifiedCallablePreludeV1 {
        owner,
        site,
        binding,
        call: call.clone(),
        direct_callable,
    })
}

fn decode_tail(
    owner: FunctionOwnerIdV1,
    row: CallableSourceMapRowV1,
) -> Result<VerifiedCallableTailV1, CallableRecipeCoSealRejectV1> {
    let value_site =
        row.site()
            .expression()
            .cloned()
            .ok_or(CallableRecipeCoSealRejectV1::WrongSite(
                CallableSourceMapRoleV1::TailReturnRead,
            ))?;
    let (statement, binding) =
        row.target()
            .tail()
            .ok_or(CallableRecipeCoSealRejectV1::WrongTarget(
                CallableSourceMapRoleV1::TailReturnRead,
            ))?;
    Ok(VerifiedCallableTailV1 {
        owner,
        statement: statement.clone(),
        value_site,
        binding,
    })
}

fn binding_target(
    row: &CallableSourceMapRowV1,
) -> Result<BindingRefV1, CallableRecipeCoSealRejectV1> {
    row.target()
        .binding()
        .ok_or(CallableRecipeCoSealRejectV1::WrongTarget(row.role()))
}

fn literal_target(
    row: &CallableSourceMapRowV1,
) -> Result<&SourceLiteralShapeV1, CallableRecipeCoSealRejectV1> {
    row.target()
        .literal()
        .ok_or(CallableRecipeCoSealRejectV1::WrongTarget(row.role()))
}

fn operator_target(
    row: &CallableSourceMapRowV1,
) -> Result<SyntaxBinaryOperatorV1, CallableRecipeCoSealRejectV1> {
    row.target()
        .operator()
        .ok_or(CallableRecipeCoSealRejectV1::WrongTarget(row.role()))
}

fn declaration_for_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: BindingRefV1,
) -> Result<(BindingOriginV1, SourceStmtSiteV1), CallableRecipeCoSealRejectV1> {
    let mut matches = ledger
        .declaration_sites()
        .filter(|site| ledger.declaration_binding(site) == Some(binding));
    let Some(site) = matches.next() else {
        return Err(CallableRecipeCoSealRejectV1::MissingDeclaration);
    };
    if matches.next().is_some() {
        return Err(CallableRecipeCoSealRejectV1::DuplicateDeclaration);
    }
    match site {
        SourceBindingSiteV1::Local { statement, .. } => {
            Ok((BindingOriginV1::Source(site.clone()), statement.clone()))
        }
        _ => Err(CallableRecipeCoSealRejectV1::NonLocalDeclaration),
    }
}

fn relations(
    initial: &CallableSourceMapRowV1,
    condition_read: &CallableSourceMapRowV1,
    condition_bound: &CallableSourceMapRowV1,
    condition_operator: &CallableSourceMapRowV1,
    step_read: &CallableSourceMapRowV1,
    step_delta: &CallableSourceMapRowV1,
    step_operator: &CallableSourceMapRowV1,
    step_write: &CallableSourceMapRowV1,
    binding: BindingRefV1,
    loop_site: SourceStmtSiteV1,
    declaration_statement: SourceStmtSiteV1,
    declaration: BindingOriginV1,
) -> Result<
    (
        VerifiedLoopInputRelationV1,
        Vec<VerifiedLoopOperationSourceRelationV1>,
        Vec<LoopRecipeBindingRelationV1>,
        Vec<LoopBindingEffectRelationV1>,
    ),
    CallableRecipeCoSealRejectV1,
> {
    let initial_site = initial
        .site()
        .expression()
        .cloned()
        .ok_or(CallableRecipeCoSealRejectV1::WrongSite(initial.role()))?;
    let condition_read_site = expr_site(condition_read)?;
    let condition_bound_site = expr_site(condition_bound)?;
    let condition_operator_site = expr_site(condition_operator)?;
    let step_read_site = expr_site(step_read)?;
    let step_delta_site = expr_site(step_delta)?;
    let step_operator_site = expr_site(step_operator)?;
    let step_write_site = expr_site(step_write)?;
    let input = VerifiedLoopInputRelationV1 {
        statement: declaration_statement.clone(),
        initializer: initial_site,
        source_binding: binding,
        recipe_value: LoopValueKeyV1::new(0),
        class: LoopValueClassV1::I64,
    };
    let operations = vec![
        operation(
            CallableSourceMapRoleV1::ConditionRead,
            LoopItemKeyV1::new(1),
            condition_read_site.clone(),
            LoopRecipeOperationViewV1::ReadBinding {
                binding: LoopBindingKeyV1::new(0),
                result: LoopValueKeyV1::new(1),
            },
        ),
        operation(
            CallableSourceMapRoleV1::ConditionBound,
            LoopItemKeyV1::new(0),
            condition_bound_site.clone(),
            LoopRecipeOperationViewV1::ConstI64 {
                result: LoopValueKeyV1::new(2),
                value: 1,
            },
        ),
        operation(
            CallableSourceMapRoleV1::ConditionOperator,
            LoopItemKeyV1::new(2),
            condition_operator_site.clone(),
            LoopRecipeOperationViewV1::CompareI64 {
                op: SyntaxBinaryOperatorV1::Less,
                left: LoopValueKeyV1::new(1),
                right: LoopValueKeyV1::new(2),
                result: LoopValueKeyV1::new(3),
            },
        ),
        operation(
            CallableSourceMapRoleV1::StepRead,
            LoopItemKeyV1::new(3),
            step_read_site.clone(),
            LoopRecipeOperationViewV1::ReadBinding {
                binding: LoopBindingKeyV1::new(0),
                result: LoopValueKeyV1::new(4),
            },
        ),
        operation(
            CallableSourceMapRoleV1::StepDelta,
            LoopItemKeyV1::new(4),
            step_delta_site.clone(),
            LoopRecipeOperationViewV1::ConstI64 {
                result: LoopValueKeyV1::new(5),
                value: 1,
            },
        ),
        operation(
            CallableSourceMapRoleV1::StepOperator,
            LoopItemKeyV1::new(5),
            step_operator_site.clone(),
            LoopRecipeOperationViewV1::BinaryI64 {
                op: SyntaxBinaryOperatorV1::Add,
                left: LoopValueKeyV1::new(4),
                right: LoopValueKeyV1::new(5),
                result: LoopValueKeyV1::new(6),
            },
        ),
        operation(
            CallableSourceMapRoleV1::StepWrite,
            LoopItemKeyV1::new(6),
            step_write_site.clone(),
            LoopRecipeOperationViewV1::WriteBinding {
                binding: LoopBindingKeyV1::new(0),
                value: LoopValueKeyV1::new(6),
            },
        ),
    ];
    let binding_rows = vec![LoopRecipeBindingRelationV1::new(
        LoopBindingKeyV1::new(0),
        binding,
        LoopValueClassV1::I64,
        declaration,
    )];
    let effects = vec![
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            LoopBindingKeyV1::new(0),
            binding,
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner: binding.owner(),
                source_loop: loop_site,
                carrier: LoopCarrierKeyV1::new(0),
            },
        ),
        effect(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            binding,
            condition_read_site,
        ),
        effect(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 1 },
            binding,
            step_read_site,
        ),
        effect(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            binding,
            step_write_site,
        ),
    ];
    Ok((input, operations, binding_rows, effects))
}

fn expr_site(
    row: &CallableSourceMapRowV1,
) -> Result<SourceExprSiteV1, CallableRecipeCoSealRejectV1> {
    row.site()
        .expression()
        .cloned()
        .ok_or(CallableRecipeCoSealRejectV1::WrongSite(row.role()))
}

fn operation(
    role: CallableSourceMapRoleV1,
    item: LoopItemKeyV1,
    site: SourceExprSiteV1,
    operation: LoopRecipeOperationViewV1,
) -> VerifiedLoopOperationSourceRelationV1 {
    VerifiedLoopOperationSourceRelationV1 {
        role,
        item,
        site,
        operation,
    }
}

fn effect(
    role: LoopBindingEffectRoleV1,
    binding: BindingRefV1,
    site: SourceExprSiteV1,
) -> LoopBindingEffectRelationV1 {
    LoopBindingEffectRelationV1::new(
        role,
        LoopBindingKeyV1::new(0),
        binding,
        LoopValueClassV1::I64,
        LoopBindingEffectAnchorV1::Expr(crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            binding.owner(),
            site,
        )),
    )
}
