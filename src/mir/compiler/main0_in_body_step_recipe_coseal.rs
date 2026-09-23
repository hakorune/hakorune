//! Caller-zero Main0 in-body-step source-map to portable Recipe co-seal.
//!
//! This module consumes the resolver-issued Main0 in-body-step source map
//! exactly once, assembles the Recipe through `LoopRecipeDraftV1` (canonical
//! key allocation plus co-recorded source anchors), and co-seals the
//! verified Recipe, JoinSig, source-bound Core, initialized inputs,
//! operation evidence, and continuation contract in one admission.
//!
//! No Builder, MIR, physical ID, retry, or fallback is allowed. The verifier
//! remains the sole semantic authority; this module never mints a semantic
//! claim itself.

use std::collections::BTreeMap;

use crate::mir::loop_recipe_contract::{
    issue_initialized_local_input_source_set_v1, issue_source_bound_core_from_artifact_v1,
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopInitializedLocalInputSourceSetRejectV1,
    LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, LoopOperationEffectRejectV1,
    LoopRecipeArtifactV1, LoopRecipeDraftRejectV1, LoopRecipeDraftV1, LoopRecipeProducerIdV1,
    LoopRecipeProvenanceV1, LoopRecipeRejectReasonV1, LoopValueClassV1,
    VerifiedLoopContinuationContractV1, VerifiedLoopInitializedLocalInputSourceSetV1,
    VerifiedLoopOperationEffectProductV1, VerifiedLoopSemanticContextV1,
};
use crate::mir::loop_structural_facts::bind_resolved_loop_root_v1;
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, FunctionOwnerIdV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};

use super::callable_single_loop_source_shapes::SourceLiteralShapeV1;
use super::main0_in_body_step_source_map::{
    Main0InBodyStepMapRoleV1, Main0InBodyStepMapRowV1, VerifiedMain0InBodyStepSourceMapV1,
};

/// Source receipt for the write-only effect rebind admitted by this issuer.
/// There is no explicit transfer site; the predicate's false edge is the
/// only loop exit.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0InBodyStepControlSourceV1 {
    owner: FunctionOwnerIdV1,
    effect_site: SourceStmtSiteV1,
}

impl VerifiedMain0InBodyStepControlSourceV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn effect_site(&self) -> &SourceStmtSiteV1 {
        &self.effect_site
    }
}

/// Tail `return <carrier>` receipt kept outside the Recipe. The Recipe owns
/// the loop; the post-loop tail is handed to the canonical Main root product
/// separately.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0InBodyStepTailV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    binding: BindingRefV1,
}

impl VerifiedMain0InBodyStepTailV1 {
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

/// The sealed Main0 in-body-step semantic-program product.
///
/// `operations` owns the source-bound Core; the input set, semantic context,
/// continuation contract, tail, and control receipts were issued inside the
/// same admission and cannot be recombined with another owner's products.
#[derive(Debug)]
pub(crate) struct VerifiedMain0InBodyStepRecipeProductV1 {
    operations: VerifiedLoopOperationEffectProductV1,
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    context: VerifiedLoopSemanticContextV1,
    continuation: VerifiedLoopContinuationContractV1,
    tail: VerifiedMain0InBodyStepTailV1,
    control: VerifiedMain0InBodyStepControlSourceV1,
}

impl VerifiedMain0InBodyStepRecipeProductV1 {
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

    pub(crate) fn tail(&self) -> &VerifiedMain0InBodyStepTailV1 {
        &self.tail
    }

    pub(crate) fn control(&self) -> &VerifiedMain0InBodyStepControlSourceV1 {
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
        VerifiedMain0InBodyStepTailV1,
        VerifiedMain0InBodyStepControlSourceV1,
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
pub(crate) enum Main0InBodyStepCoSealRejectV1 {
    ForeignOwner,
    MissingRole(Main0InBodyStepMapRoleV1),
    DuplicateRole(Main0InBodyStepMapRoleV1),
    UnexpectedRole(Main0InBodyStepMapRoleV1),
    WrongSite(Main0InBodyStepMapRoleV1),
    WrongTarget(Main0InBodyStepMapRoleV1),
    UnsupportedLiteral(Main0InBodyStepMapRoleV1),
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

const ROLES: [Main0InBodyStepMapRoleV1; 12] = [
    Main0InBodyStepMapRoleV1::CarrierDeclaration,
    Main0InBodyStepMapRoleV1::EffectDeclaration,
    Main0InBodyStepMapRoleV1::ConditionCarrierRead,
    Main0InBodyStepMapRoleV1::ConditionBound,
    Main0InBodyStepMapRoleV1::ConditionOperator,
    Main0InBodyStepMapRoleV1::StepRead,
    Main0InBodyStepMapRoleV1::StepDelta,
    Main0InBodyStepMapRoleV1::StepOperator,
    Main0InBodyStepMapRoleV1::StepWrite,
    Main0InBodyStepMapRoleV1::EffectValue,
    Main0InBodyStepMapRoleV1::EffectWrite,
    Main0InBodyStepMapRoleV1::TailReturnRead,
];

/// Issue the complete Main0 in-body-step semantic-program product from the
/// resolver-joined source map.
///
/// One admission covers: Recipe draft emission with co-recorded anchors, the
/// sole Recipe verification boundary, the JoinSig elaboration, the
/// source-bound Core, the initialized-local input set, the operation-
/// evidence product, and the continuation contract. The tail return stays
/// outside the Recipe as a source receipt for the canonical Main root
/// handoff.
pub(crate) fn issue_main0_in_body_step_recipe_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    map: VerifiedMain0InBodyStepSourceMapV1,
) -> Result<VerifiedMain0InBodyStepRecipeProductV1, Main0InBodyStepCoSealRejectV1> {
    if map.owner() != ledger.owner() {
        return Err(Main0InBodyStepCoSealRejectV1::ForeignOwner);
    }
    let (owner, origin, source_kind, loop_source, frame, scope_region, effect_site, rows) =
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
            return Err(Main0InBodyStepCoSealRejectV1::DuplicateRole(role));
        }
    }
    for role in ROLES {
        if !by_role.contains_key(&role) {
            return Err(Main0InBodyStepCoSealRejectV1::MissingRole(role));
        }
    }
    if let Some(role) = by_role.keys().copied().find(|role| !ROLES.contains(role)) {
        return Err(Main0InBodyStepCoSealRejectV1::UnexpectedRole(role));
    }

    let carrier_decl = take_role(&mut by_role, Main0InBodyStepMapRoleV1::CarrierDeclaration)?;
    let effect_decl = take_role(&mut by_role, Main0InBodyStepMapRoleV1::EffectDeclaration)?;
    let condition_carrier_read =
        take_role(&mut by_role, Main0InBodyStepMapRoleV1::ConditionCarrierRead)?;
    let condition_bound = take_role(&mut by_role, Main0InBodyStepMapRoleV1::ConditionBound)?;
    let condition_operator =
        take_role(&mut by_role, Main0InBodyStepMapRoleV1::ConditionOperator)?;
    let step_read = take_role(&mut by_role, Main0InBodyStepMapRoleV1::StepRead)?;
    let step_delta = take_role(&mut by_role, Main0InBodyStepMapRoleV1::StepDelta)?;
    let step_operator = take_role(&mut by_role, Main0InBodyStepMapRoleV1::StepOperator)?;
    let step_write = take_role(&mut by_role, Main0InBodyStepMapRoleV1::StepWrite)?;
    let effect_value = take_role(&mut by_role, Main0InBodyStepMapRoleV1::EffectValue)?;
    let effect_write = take_role(&mut by_role, Main0InBodyStepMapRoleV1::EffectWrite)?;
    let tail_row = take_role(&mut by_role, Main0InBodyStepMapRoleV1::TailReturnRead)?;

    // Decode the two declared locals and their resolver bindings. The map
    // row sites are the initializer expressions; the declaration sites come
    // from the resolver ledger.
    let (carrier_binding, _carrier_literal) = decl_target(&carrier_decl)?;
    let (effect_binding, _effect_literal) = decl_target(&effect_decl)?;
    let (carrier_decl_site, _) = declaration_for_binding(ledger, carrier_binding)?;
    let (effect_decl_site, _) = declaration_for_binding(ledger, effect_binding)?;

    // Assemble the Recipe through the draft builder. Every push records its
    // source anchor at emission time; no second correspondence table exists.
    let mut draft = LoopRecipeDraftV1::new(owner);
    let carrier_key = draft.declare_local_binding(
        binding_label(ledger, carrier_binding)?,
        LoopValueClassV1::I64,
        carrier_binding,
        carrier_decl_site,
    );
    let effect_key = draft.declare_local_binding(
        binding_label(ledger, effect_binding)?,
        LoopValueClassV1::I64,
        effect_binding,
        effect_decl_site,
    );
    let loop_key = draft
        .open_root_loop(context.loop_site().clone())
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;

    // Both locals are seeded from their initialized input values: the
    // carrier is read and rebound; the effect local is rebound inside the
    // loop so the carrier claim keeps its pre-loop seed observable for the
    // zero-iteration case.
    draft
        .carrier_input(loop_key, carrier_key, expr_site(&carrier_decl)?)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    draft
        .carrier_input(loop_key, effect_key, expr_site(&effect_decl)?)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;

    // Predicate block: `carrier < <integer literal>`.
    let condition_block = draft
        .open_condition_block(loop_key)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    let (_, carrier_now) = draft
        .push_read_binding(condition_block, carrier_key, &expr_site(&condition_carrier_read)?)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    let (_, bound_now) = draft
        .push_const_i64(
            condition_block,
            integer_literal(&condition_bound)?,
            &expr_site(&condition_bound)?,
        )
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    let (_, condition_value) = draft
        .push_compare_i64(
            condition_block,
            LoopCompareI64OpV1::Less,
            carrier_now,
            bound_now,
            &expr_site(&condition_operator)?,
        )
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    draft
        .seal_condition(loop_key, condition_block, condition_value)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;

    // Body block: `carrier = carrier + <delta>` then `<effect> = <int>`.
    // The body tail falls through to the loop back-edge; no explicit
    // transfer item exists.
    let body = draft
        .open_body_block(loop_key)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    let (_, step_read_value) = draft
        .push_read_binding(body, carrier_key, &expr_site(&step_read)?)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    let (_, step_delta_value) = draft
        .push_const_i64(body, integer_literal(&step_delta)?, &expr_site(&step_delta)?)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    let (_, step_sum) = draft
        .push_binary_i64(
            body,
            LoopBinaryI64OpV1::Add,
            step_read_value,
            step_delta_value,
            &expr_site(&step_operator)?,
        )
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    draft
        .push_write_binding(body, carrier_key, step_sum, &expr_site(&step_write)?)
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    let (_, effect_literal_value) = draft
        .push_const_i64(
            body,
            integer_literal(&effect_value)?,
            &expr_site(&effect_value)?,
        )
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;
    draft
        .push_write_binding(
            body,
            effect_key,
            effect_literal_value,
            &expr_site(&effect_write)?,
        )
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?;

    let (recipe, bindings, effects, inputs, evidence) = draft
        .finish()
        .map_err(Main0InBodyStepCoSealRejectV1::Draft)?
        .into_parts();

    // Seal: the verifier is the sole semantic authority. The claim, JoinSig,
    // Core, input set, operation evidence, and continuation are all issued
    // from the verified recipe inside this one admission.
    let source_root = bind_resolved_loop_root_v1(loop_source)
        .map_err(Main0InBodyStepCoSealRejectV1::SourceRoot)?;
    let verified_recipe = crate::mir::loop_recipe_contract::LoopRecipeVerifierV1::verify(
        recipe.clone(),
    )
    .map_err(Main0InBodyStepCoSealRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_recipe);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::Main0InBodyStepV1),
        source_binding,
        recipe,
    );
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(Main0InBodyStepCoSealRejectV1::JoinSig)?;
    let after = join_sig
        .require_after_binding(loop_key, carrier_key, LoopValueClassV1::I64)
        .map_err(Main0InBodyStepCoSealRejectV1::JoinSig)?;
    let core = issue_source_bound_core_from_artifact_v1(artifact, join_sig, owner, bindings, effects)
        .map_err(Main0InBodyStepCoSealRejectV1::Recipe)?;
    let input = issue_initialized_local_input_source_set_v1(&core, inputs)
        .map_err(Main0InBodyStepCoSealRejectV1::InputSource)?;
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, evidence)
        .map_err(Main0InBodyStepCoSealRejectV1::OperationEffect)?;
    let continuation = VerifiedLoopContinuationContractV1::from_after(owner, after);

    let (tail_statement, tail_binding) = tail_row
        .target()
        .tail()
        .ok_or(Main0InBodyStepCoSealRejectV1::WrongTarget(
            Main0InBodyStepMapRoleV1::TailReturnRead,
        ))?;
    if tail_binding != carrier_binding {
        return Err(Main0InBodyStepCoSealRejectV1::WrongTarget(
            Main0InBodyStepMapRoleV1::TailReturnRead,
        ));
    }
    let tail = VerifiedMain0InBodyStepTailV1 {
        owner,
        statement: tail_statement.clone(),
        value_site: expr_site(&tail_row)?,
        binding: tail_binding,
    };
    let control = VerifiedMain0InBodyStepControlSourceV1 { owner, effect_site };

    Ok(VerifiedMain0InBodyStepRecipeProductV1 {
        operations,
        input,
        context,
        continuation,
        tail,
        control,
    })
}

fn take_role(
    rows: &mut BTreeMap<Main0InBodyStepMapRoleV1, Main0InBodyStepMapRowV1>,
    role: Main0InBodyStepMapRoleV1,
) -> Result<Main0InBodyStepMapRowV1, Main0InBodyStepCoSealRejectV1> {
    rows.remove(&role)
        .ok_or(Main0InBodyStepCoSealRejectV1::MissingRole(role))
}

fn expr_site(
    row: &Main0InBodyStepMapRowV1,
) -> Result<SourceExprSiteV1, Main0InBodyStepCoSealRejectV1> {
    row.site()
        .expression()
        .cloned()
        .ok_or(Main0InBodyStepCoSealRejectV1::WrongSite(row.role()))
}

fn decl_target(
    row: &Main0InBodyStepMapRowV1,
) -> Result<(BindingRefV1, SourceLiteralShapeV1), Main0InBodyStepCoSealRejectV1> {
    row.target()
        .local_declaration()
        .map(|(binding, literal)| (binding, literal.clone()))
        .ok_or(Main0InBodyStepCoSealRejectV1::WrongTarget(row.role()))
}

fn integer_literal(row: &Main0InBodyStepMapRowV1) -> Result<i64, Main0InBodyStepCoSealRejectV1> {
    match row.target().literal() {
        Some(SourceLiteralShapeV1::Integer(value)) => Ok(*value),
        _ => Err(Main0InBodyStepCoSealRejectV1::UnsupportedLiteral(row.role())),
    }
}

fn binding_label(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: BindingRefV1,
) -> Result<String, Main0InBodyStepCoSealRejectV1> {
    ledger
        .binding(binding)
        .map(|record| record.diagnostic_name().to_owned())
        .ok_or(Main0InBodyStepCoSealRejectV1::MissingBindingRecord)
}

fn declaration_for_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: BindingRefV1,
) -> Result<(SourceBindingSiteV1, SourceStmtSiteV1), Main0InBodyStepCoSealRejectV1> {
    let mut matches = ledger
        .declaration_sites()
        .filter(|site| ledger.declaration_binding(site) == Some(binding));
    let Some(site) = matches.next() else {
        return Err(Main0InBodyStepCoSealRejectV1::MissingDeclaration);
    };
    if matches.next().is_some() {
        return Err(Main0InBodyStepCoSealRejectV1::DuplicateDeclaration);
    }
    match site {
        SourceBindingSiteV1::Local { statement, .. } => Ok((site.clone(), statement.clone())),
        _ => Err(Main0InBodyStepCoSealRejectV1::NonLocalDeclaration),
    }
}
