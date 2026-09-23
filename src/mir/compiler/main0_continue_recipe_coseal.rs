//! Caller-zero Main0 Continue source-map to portable Recipe co-seal.
//!
//! This module is a production-scope logical issuer, but it is not selected by
//! a production route yet. It consumes the resolver-issued Main0 Continue
//! source map exactly once, assembles the Recipe through `LoopRecipeDraftV1`
//! (canonical key allocation plus co-recorded source anchors), and co-seals
//! the verified Recipe, JoinSig, source-bound Core, initialized inputs,
//! operation evidence, and continuation contract in one admission.
//!
//! No Builder, MIR, physical ID, retry, or fallback is allowed. The verifier
//! remains the sole semantic authority; this module never mints a semantic
//! claim itself.

use std::collections::BTreeMap;

use crate::mir::loop_recipe_contract::{
    issue_initialized_local_input_source_set_v1, issue_source_bound_core_from_artifact_v1,
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopExitKindV1,
    LoopInitializedLocalInputSourceSetRejectV1, LoopJoinSigElaboratorV1,
    LoopJoinSigRejectReasonV1, LoopOperationEffectRejectV1, LoopRecipeArtifactV1,
    LoopRecipeDraftRejectV1, LoopRecipeDraftV1, LoopRecipeProducerIdV1, LoopRecipeProvenanceV1,
    LoopRecipeRejectReasonV1, LoopValueClassV1, VerifiedLoopContinuationContractV1,
    VerifiedLoopInitializedLocalInputSourceSetV1, VerifiedLoopOperationEffectProductV1,
    VerifiedLoopSemanticContextV1,
};
use crate::mir::loop_structural_facts::bind_resolved_loop_root_v1;
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, FunctionOwnerIdV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};

use super::callable_single_loop_source_shapes::SourceLiteralShapeV1;
use super::main0_continue_source_map::{
    Main0ContinueMapRoleV1, Main0ContinueMapRowV1, VerifiedMain0ContinueSourceMapV1,
};

/// Source receipt for the If/Continue control shape admitted by this issuer.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0ContinueControlSourceV1 {
    owner: FunctionOwnerIdV1,
    if_site: SourceStmtSiteV1,
    continue_site: SourceStmtSiteV1,
}

impl VerifiedMain0ContinueControlSourceV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn if_site(&self) -> &SourceStmtSiteV1 {
        &self.if_site
    }

    pub(crate) fn continue_site(&self) -> &SourceStmtSiteV1 {
        &self.continue_site
    }
}

/// Tail `return <carrier>` receipt kept outside the Recipe. The Recipe owns
/// the loop; the post-loop tail is handed to the canonical Main root product
/// separately.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0ContinueTailV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    binding: BindingRefV1,
}

impl VerifiedMain0ContinueTailV1 {
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

/// The sealed Main0 Continue semantic-program product.
///
/// `operations` owns the source-bound Core; the input set, semantic context,
/// continuation contract, tail, and control receipts were issued inside the
/// same admission and cannot be recombined with another owner's products.
#[derive(Debug)]
pub(crate) struct VerifiedMain0ContinueRecipeProductV1 {
    operations: VerifiedLoopOperationEffectProductV1,
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    context: VerifiedLoopSemanticContextV1,
    continuation: VerifiedLoopContinuationContractV1,
    tail: VerifiedMain0ContinueTailV1,
    control: VerifiedMain0ContinueControlSourceV1,
}

impl VerifiedMain0ContinueRecipeProductV1 {
    pub(crate) fn operations(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operations
    }

    pub(crate) fn input(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.input
    }

    pub(crate) fn context(&self) -> &VerifiedLoopSemanticContextV1 {
        &self.context
    }

    pub(crate) fn continuation(&self) -> &VerifiedLoopContinuationContractV1 {
        &self.continuation
    }

    pub(crate) fn tail(&self) -> &VerifiedMain0ContinueTailV1 {
        &self.tail
    }

    pub(crate) fn control(&self) -> &VerifiedMain0ContinueControlSourceV1 {
        &self.control
    }

    /// Consume the caller-zero product at the next explicit boundary.
    ///
    /// Move-only so a later physical prepare step cannot retain a second
    /// co-seal, tail, or control owner.
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
        VerifiedLoopSemanticContextV1,
        VerifiedLoopContinuationContractV1,
        VerifiedMain0ContinueTailV1,
        VerifiedMain0ContinueControlSourceV1,
    ) {
        (
            self.operations,
            self.input,
            self.context,
            self.continuation,
            self.tail,
            self.control,
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Main0ContinueCoSealRejectV1 {
    ForeignOwner,
    MissingRole(Main0ContinueMapRoleV1),
    DuplicateRole(Main0ContinueMapRoleV1),
    UnexpectedRole(Main0ContinueMapRoleV1),
    WrongSite(Main0ContinueMapRoleV1),
    WrongTarget(Main0ContinueMapRoleV1),
    UnsupportedLiteral(Main0ContinueMapRoleV1),
    MissingBindingRecord,
    MissingDeclaration,
    DuplicateDeclaration,
    NonLocalDeclaration,
    Draft(LoopRecipeDraftRejectV1),
    SourceRoot(crate::mir::loop_structural_facts::LoopRootSourceBindingRejectV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    OperationEffect(LoopOperationEffectRejectV1),
    InputSource(LoopInitializedLocalInputSourceSetRejectV1),
}

const ROLES: [Main0ContinueMapRoleV1; 18] = [
    Main0ContinueMapRoleV1::CarrierDeclaration,
    Main0ContinueMapRoleV1::BoundDeclaration,
    Main0ContinueMapRoleV1::ConditionCarrierRead,
    Main0ContinueMapRoleV1::ConditionBoundRead,
    Main0ContinueMapRoleV1::ConditionOperator,
    Main0ContinueMapRoleV1::GuardCarrierRead,
    Main0ContinueMapRoleV1::GuardBound,
    Main0ContinueMapRoleV1::GuardOperator,
    Main0ContinueMapRoleV1::ThenStepRead,
    Main0ContinueMapRoleV1::ThenStepDelta,
    Main0ContinueMapRoleV1::ThenStepOperator,
    Main0ContinueMapRoleV1::ThenStepWrite,
    Main0ContinueMapRoleV1::ContinueTransfer,
    Main0ContinueMapRoleV1::NormalStepRead,
    Main0ContinueMapRoleV1::NormalStepDelta,
    Main0ContinueMapRoleV1::NormalStepOperator,
    Main0ContinueMapRoleV1::NormalStepWrite,
    Main0ContinueMapRoleV1::TailReturnRead,
];

/// Issue the complete Main0 Continue semantic-program product from the
/// resolver-joined source map.
///
/// One admission covers: Recipe draft emission with co-recorded anchors, the
/// sole Recipe verification boundary, the JoinSig elaboration, the
/// source-bound Core, the initialized-local input set, the operation-evidence
/// product, and the continuation contract. The tail return stays outside the
/// Recipe as a source receipt for the canonical Main root handoff.
pub(crate) fn issue_main0_continue_recipe_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    map: VerifiedMain0ContinueSourceMapV1,
) -> Result<VerifiedMain0ContinueRecipeProductV1, Main0ContinueCoSealRejectV1> {
    if map.owner() != ledger.owner() {
        return Err(Main0ContinueCoSealRejectV1::ForeignOwner);
    }
    let (owner, origin, source_kind, loop_source, frame, scope_region, if_site, continue_site, rows) =
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

    let mut by_role = BTreeMap::new();
    for row in rows.into_vec() {
        let role = row.role();
        if by_role.insert(role, row).is_some() {
            return Err(Main0ContinueCoSealRejectV1::DuplicateRole(role));
        }
    }
    for role in ROLES {
        if !by_role.contains_key(&role) {
            return Err(Main0ContinueCoSealRejectV1::MissingRole(role));
        }
    }
    if let Some(role) = by_role.keys().copied().find(|role| !ROLES.contains(role)) {
        return Err(Main0ContinueCoSealRejectV1::UnexpectedRole(role));
    }

    let carrier_decl = take_role(&mut by_role, Main0ContinueMapRoleV1::CarrierDeclaration)?;
    let bound_decl = take_role(&mut by_role, Main0ContinueMapRoleV1::BoundDeclaration)?;
    let condition_carrier_read =
        take_role(&mut by_role, Main0ContinueMapRoleV1::ConditionCarrierRead)?;
    let condition_bound_read =
        take_role(&mut by_role, Main0ContinueMapRoleV1::ConditionBoundRead)?;
    let condition_operator = take_role(&mut by_role, Main0ContinueMapRoleV1::ConditionOperator)?;
    let guard_carrier_read = take_role(&mut by_role, Main0ContinueMapRoleV1::GuardCarrierRead)?;
    let guard_bound = take_role(&mut by_role, Main0ContinueMapRoleV1::GuardBound)?;
    let guard_operator = take_role(&mut by_role, Main0ContinueMapRoleV1::GuardOperator)?;
    let then_read = take_role(&mut by_role, Main0ContinueMapRoleV1::ThenStepRead)?;
    let then_delta = take_role(&mut by_role, Main0ContinueMapRoleV1::ThenStepDelta)?;
    let then_operator = take_role(&mut by_role, Main0ContinueMapRoleV1::ThenStepOperator)?;
    let then_write = take_role(&mut by_role, Main0ContinueMapRoleV1::ThenStepWrite)?;
    let continue_row = take_role(&mut by_role, Main0ContinueMapRoleV1::ContinueTransfer)?;
    let normal_read = take_role(&mut by_role, Main0ContinueMapRoleV1::NormalStepRead)?;
    let normal_delta = take_role(&mut by_role, Main0ContinueMapRoleV1::NormalStepDelta)?;
    let normal_operator = take_role(&mut by_role, Main0ContinueMapRoleV1::NormalStepOperator)?;
    let normal_write = take_role(&mut by_role, Main0ContinueMapRoleV1::NormalStepWrite)?;
    let tail_row = take_role(&mut by_role, Main0ContinueMapRoleV1::TailReturnRead)?;

    // Decode the two declared locals and their resolver bindings. The map row
    // sites are the initializer expressions; the declaration sites come from
    // the resolver ledger.
    let (carrier_binding, _carrier_literal) = decl_target(&carrier_decl)?;
    let (bound_binding, _bound_literal) = decl_target(&bound_decl)?;
    let (carrier_decl_site, _) = declaration_for_binding(ledger, carrier_binding)?;
    let (bound_decl_site, _) = declaration_for_binding(ledger, bound_binding)?;

    // Assemble the Recipe through the draft builder. Every push records its
    // source anchor at emission time; no second correspondence table exists.
    let mut draft = LoopRecipeDraftV1::new(owner);
    let carrier_key = draft.declare_local_binding(
        binding_label(ledger, carrier_binding)?,
        LoopValueClassV1::I64,
        carrier_binding,
        carrier_decl_site,
    );
    let bound_key = draft.declare_local_binding(
        binding_label(ledger, bound_binding)?,
        LoopValueClassV1::I64,
        bound_binding,
        bound_decl_site,
    );
    let loop_key = draft
        .open_root_loop(context.loop_site().clone())
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;

    // Both locals are read inside the loop, so both are carriers seeded from
    // their initialized input values.
    draft
        .carrier_input(loop_key, carrier_key, expr_site(&carrier_decl)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    draft
        .carrier_input(loop_key, bound_key, expr_site(&bound_decl)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;

    // The only in-loop transfer is `continue` targeting this loop.
    let continue_exit = draft
        .declare_exit(loop_key, LoopExitKindV1::Continue { target_loop: loop_key })
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;

    // Predicate block: `carrier < bound`.
    let condition_block = draft
        .open_condition_block(loop_key)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, carrier_now) = draft
        .push_read_binding(condition_block, carrier_key, &expr_site(&condition_carrier_read)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, bound_now) = draft
        .push_read_binding(condition_block, bound_key, &expr_site(&condition_bound_read)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, condition_value) = draft
        .push_compare_i64(
            condition_block,
            LoopCompareI64OpV1::Less,
            carrier_now,
            bound_now,
            &expr_site(&condition_operator)?,
        )
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    draft
        .seal_condition(loop_key, condition_block, condition_value)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;

    // Body block: `if carrier == <guard> { carrier += <delta>; continue }
    // carrier += <delta>`.
    let body = draft
        .open_body_block(loop_key)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, guard_read) = draft
        .push_read_binding(body, carrier_key, &expr_site(&guard_carrier_read)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, guard_const) = draft
        .push_const_i64(body, integer_literal(&guard_bound)?, &expr_site(&guard_bound)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, guard_value) = draft
        .push_compare_i64(
            body,
            LoopCompareI64OpV1::Equal,
            guard_read,
            guard_const,
            &expr_site(&guard_operator)?,
        )
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_if_item, then_block) = draft
        .push_if(body, guard_value)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, then_read_value) = draft
        .push_read_binding(then_block, carrier_key, &expr_site(&then_read)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, then_delta_value) = draft
        .push_const_i64(then_block, integer_literal(&then_delta)?, &expr_site(&then_delta)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, then_sum) = draft
        .push_binary_i64(
            then_block,
            LoopBinaryI64OpV1::Add,
            then_read_value,
            then_delta_value,
            &expr_site(&then_operator)?,
        )
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    draft
        .push_write_binding(then_block, carrier_key, then_sum, &expr_site(&then_write)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    draft
        .push_exit(then_block, continue_exit)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, normal_read_value) = draft
        .push_read_binding(body, carrier_key, &expr_site(&normal_read)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, normal_delta_value) = draft
        .push_const_i64(body, integer_literal(&normal_delta)?, &expr_site(&normal_delta)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    let (_, normal_sum) = draft
        .push_binary_i64(
            body,
            LoopBinaryI64OpV1::Add,
            normal_read_value,
            normal_delta_value,
            &expr_site(&normal_operator)?,
        )
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;
    draft
        .push_write_binding(body, carrier_key, normal_sum, &expr_site(&normal_write)?)
        .map_err(Main0ContinueCoSealRejectV1::Draft)?;

    let (recipe, bindings, effects, inputs, evidence) = draft
        .finish()
        .map_err(Main0ContinueCoSealRejectV1::Draft)?
        .into_parts();

    // Seal: the verifier is the sole semantic authority. The claim, JoinSig,
    // Core, input set, operation evidence, and continuation are all issued
    // from the verified recipe inside this one admission.
    let source_root = bind_resolved_loop_root_v1(loop_source)
        .map_err(Main0ContinueCoSealRejectV1::SourceRoot)?;
    let verified_recipe = crate::mir::loop_recipe_contract::LoopRecipeVerifierV1::verify(
        recipe.clone(),
    )
    .map_err(Main0ContinueCoSealRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_recipe);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::Main0ContinueV1),
        source_binding,
        recipe,
    );
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(Main0ContinueCoSealRejectV1::JoinSig)?;
    let after = join_sig
        .require_after_binding(loop_key, carrier_key, LoopValueClassV1::I64)
        .map_err(Main0ContinueCoSealRejectV1::JoinSig)?;
    let core = issue_source_bound_core_from_artifact_v1(artifact, join_sig, owner, bindings, effects)
        .map_err(Main0ContinueCoSealRejectV1::Recipe)?;
    let input = issue_initialized_local_input_source_set_v1(&core, inputs)
        .map_err(Main0ContinueCoSealRejectV1::InputSource)?;
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, evidence)
        .map_err(Main0ContinueCoSealRejectV1::OperationEffect)?;
    let continuation = VerifiedLoopContinuationContractV1::from_after(owner, after);

    let (tail_statement, tail_binding) = tail_row
        .target()
        .tail()
        .ok_or(Main0ContinueCoSealRejectV1::WrongTarget(
            Main0ContinueMapRoleV1::TailReturnRead,
        ))?;
    if tail_binding != carrier_binding {
        return Err(Main0ContinueCoSealRejectV1::WrongTarget(
            Main0ContinueMapRoleV1::TailReturnRead,
        ));
    }
    let tail = VerifiedMain0ContinueTailV1 {
        owner,
        statement: tail_statement.clone(),
        value_site: expr_site(&tail_row)?,
        binding: tail_binding,
    };
    let continue_target = continue_row
        .target()
        .continue_target()
        .ok_or(Main0ContinueCoSealRejectV1::WrongTarget(
            Main0ContinueMapRoleV1::ContinueTransfer,
        ))?;
    if continue_target != scope_region.region() {
        return Err(Main0ContinueCoSealRejectV1::WrongTarget(
            Main0ContinueMapRoleV1::ContinueTransfer,
        ));
    }
    let control = VerifiedMain0ContinueControlSourceV1 {
        owner,
        if_site,
        continue_site,
    };

    Ok(VerifiedMain0ContinueRecipeProductV1 {
        operations,
        input,
        context,
        continuation,
        tail,
        control,
    })
}

fn take_role(
    rows: &mut BTreeMap<Main0ContinueMapRoleV1, Main0ContinueMapRowV1>,
    role: Main0ContinueMapRoleV1,
) -> Result<Main0ContinueMapRowV1, Main0ContinueCoSealRejectV1> {
    rows.remove(&role)
        .ok_or(Main0ContinueCoSealRejectV1::MissingRole(role))
}

fn expr_site(
    row: &Main0ContinueMapRowV1,
) -> Result<SourceExprSiteV1, Main0ContinueCoSealRejectV1> {
    row.site()
        .expression()
        .cloned()
        .ok_or(Main0ContinueCoSealRejectV1::WrongSite(row.role()))
}

fn decl_target(
    row: &Main0ContinueMapRowV1,
) -> Result<(BindingRefV1, SourceLiteralShapeV1), Main0ContinueCoSealRejectV1> {
    row.target()
        .local_declaration()
        .map(|(binding, literal)| (binding, literal.clone()))
        .ok_or(Main0ContinueCoSealRejectV1::WrongTarget(row.role()))
}

fn integer_literal(row: &Main0ContinueMapRowV1) -> Result<i64, Main0ContinueCoSealRejectV1> {
    match row.target().literal() {
        Some(SourceLiteralShapeV1::Integer(value)) => Ok(*value),
        _ => Err(Main0ContinueCoSealRejectV1::UnsupportedLiteral(row.role())),
    }
}

fn binding_label(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: BindingRefV1,
) -> Result<String, Main0ContinueCoSealRejectV1> {
    ledger
        .binding(binding)
        .map(|record| record.diagnostic_name().to_owned())
        .ok_or(Main0ContinueCoSealRejectV1::MissingBindingRecord)
}

fn declaration_for_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: BindingRefV1,
) -> Result<(SourceBindingSiteV1, SourceStmtSiteV1), Main0ContinueCoSealRejectV1> {
    let mut matches = ledger
        .declaration_sites()
        .filter(|site| ledger.declaration_binding(site) == Some(binding));
    let Some(site) = matches.next() else {
        return Err(Main0ContinueCoSealRejectV1::MissingDeclaration);
    };
    if matches.next().is_some() {
        return Err(Main0ContinueCoSealRejectV1::DuplicateDeclaration);
    }
    match site {
        SourceBindingSiteV1::Local { statement, .. } => Ok((site.clone(), statement.clone())),
        _ => Err(Main0ContinueCoSealRejectV1::NonLocalDeclaration),
    }
}
