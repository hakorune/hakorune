//! One source-bound V1 Loop product from the selected variable-bound Mul Facts.
//! The draft allocates every Recipe key and records each source relation once.

use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, VerifiedVariableBoundMulFactsV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1,
    SourceStmtSiteV1,
};

use super::error::LoopRecipeRejectReasonV1;
use super::input_source::{
    issue_initialized_local_input_source_set_v1, LoopInitializedLocalInputSourceSetRejectV1,
    VerifiedLoopInitializedLocalInputSourceSetV1,
};
use super::join_sig::{
    issue_root_carrier_join_closure_v1, LoopJoinClosureRejectV1, VerifiedLoopAfterBindingV1,
};
use super::operation_effect::{LoopOperationEffectRejectV1, VerifiedLoopOperationEffectProductV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::recipe_draft::{LoopRecipeDraftRejectV1, LoopRecipeDraftV1};
use super::schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopRecipeArtifactV1, LoopRecipeProvenanceV1,
    LoopValueClassV1,
};
use super::source_bound_core::issue_source_bound_core_from_artifact_v1;
use super::verify::LoopRecipeVerifierV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VariableBoundMulRecipeRejectV1 {
    SourceBinding,
    Draft(LoopRecipeDraftRejectV1),
    Recipe(LoopRecipeRejectReasonV1),
    After(LoopJoinClosureRejectV1),
    Core(LoopRecipeRejectReasonV1),
    Inputs(LoopInitializedLocalInputSourceSetRejectV1),
    Operations(LoopOperationEffectRejectV1),
}

/// One move-only product. The source/frame identity and complete root After
/// accompany the same verified Core used by input and effect consumers.
#[derive(Debug)]
pub(crate) struct VerifiedVariableBoundMulRecipeProductV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    bindings: [BindingRefV1; 3],
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
    after: Box<[VerifiedLoopAfterBindingV1]>,
}

/// The complete source-bound root After lent to the Home scanner. Only the
/// verified product can construct this move-only capability.
#[derive(Debug)]
pub(crate) struct VerifiedLoopHomeAfterLoanV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    carriers: [BindingRefV1; 2],
}

impl VerifiedLoopHomeAfterLoanV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn carriers(&self) -> [BindingRefV1; 2] {
        self.carriers
    }
}

impl VerifiedVariableBoundMulRecipeProductV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }
    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }
    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }
    pub(crate) const fn bindings(&self) -> [BindingRefV1; 3] {
        self.bindings
    }
    pub(crate) fn operations(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operations
    }
    pub(crate) fn inputs(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.inputs
    }
    pub(crate) fn after(&self) -> &[VerifiedLoopAfterBindingV1] {
        &self.after
    }

    pub(crate) fn home_after_loan(&self) -> Option<VerifiedLoopHomeAfterLoanV1> {
        // The complete JoinSig closure is already verified. Join each logical
        // After identity to its Core source relation; never assume raw keys
        // or infer the result from source names.
        if self.after.len() != 2 || self.bindings[0] == self.bindings[1] {
            return None;
        }
        let relations = self.operations.core().binding_relations();
        let mut seen = [false; 2];
        for after in self.after.iter() {
            if after.class() != LoopValueClassV1::I64 {
                return None;
            }
            let mut matching = relations
                .iter()
                .filter(|relation| relation.recipe_binding() == after.binding());
            let relation = matching.next()?;
            if matching.next().is_some() || relation.class() != LoopValueClassV1::I64 {
                return None;
            }
            let index = self.bindings[..2]
                .iter()
                .position(|binding| *binding == relation.source_binding())?;
            if seen[index] {
                return None;
            }
            seen[index] = true;
        }
        if !seen.into_iter().all(|found| found)
            || self.inputs.rows().len() != 3
            || !self.inputs.rows().iter().any(|row| {
                row.source_binding() == self.bindings[2] && row.class() == LoopValueClassV1::I64
            })
        {
            return None;
        }
        Some(VerifiedLoopHomeAfterLoanV1 {
            owner: self.owner,
            site: self.site.clone(),
            carriers: [self.bindings[0], self.bindings[1]],
        })
    }
}

pub(crate) fn produce_variable_bound_mul_recipe_v1(
    facts: VerifiedVariableBoundMulFactsV1,
) -> Result<VerifiedVariableBoundMulRecipeProductV1, VariableBoundMulRecipeRejectV1> {
    use VariableBoundMulRecipeRejectV1 as Reject;
    let (
        owner,
        source,
        frame,
        scope_region,
        condition,
        condition_operands,
        operations,
        operation_operands,
        bindings,
        inputs,
    ) = facts.into_parts();
    let site = source.site().clone();
    let root_source = bind_resolved_loop_root_v1(source).map_err(|_| Reject::SourceBinding)?;
    let mut draft = LoopRecipeDraftV1::new(owner);
    let [scale, induction, bound] = std::array::from_fn(|index| {
        draft.declare_local_binding(
            ["scale", "induction", "bound"][index],
            LoopValueClassV1::I64,
            bindings[index],
            inputs[index].0.clone(),
        )
    });
    let root = draft.open_root_loop(site.clone()).map_err(Reject::Draft)?;
    for (binding, input) in [(scale, &inputs[0]), (induction, &inputs[1])] {
        draft
            .carrier_input(root, binding, input.1.clone())
            .map_err(Reject::Draft)?;
    }
    draft
        .read_only_input(root, bound, inputs[2].1.clone())
        .map_err(Reject::Draft)?;
    let predicate = draft.open_condition_block(root).map_err(Reject::Draft)?;
    let (_, left) = draft
        .push_read_binding(predicate, induction, &condition_operands[0])
        .map_err(Reject::Draft)?;
    let (_, right) = draft
        .push_read_binding(predicate, bound, &condition_operands[1])
        .map_err(Reject::Draft)?;
    let (_, comparison) = draft
        .push_compare_i64(predicate, LoopCompareI64OpV1::Less, left, right, &condition)
        .map_err(Reject::Draft)?;
    draft
        .seal_condition(root, predicate, comparison)
        .map_err(Reject::Draft)?;
    let body = draft.open_body_block(root).map_err(Reject::Draft)?;
    for (index, (binding, op, literal)) in [
        (scale, LoopBinaryI64OpV1::Mul, 2),
        (induction, LoopBinaryI64OpV1::Add, 1),
    ]
    .into_iter()
    .enumerate()
    {
        let [target, lhs, rhs] = &operation_operands[index];
        let (_, left) = draft
            .push_read_binding(body, binding, lhs)
            .map_err(Reject::Draft)?;
        let (_, right) = draft
            .push_const_i64(body, literal, rhs)
            .map_err(Reject::Draft)?;
        let (_, result) = draft
            .push_binary_i64(body, op, left, right, &operations[index].1)
            .map_err(Reject::Draft)?;
        draft
            .push_write_binding(body, binding, result, target)
            .map_err(Reject::Draft)?;
    }
    let (recipe, source_bindings, effects, input_rows, operation_rows) =
        draft.finish().map_err(Reject::Draft)?.into_parts();
    let verified = LoopRecipeVerifierV1::verify(recipe.clone()).map_err(Reject::Recipe)?;
    let source_binding = root_source.into_root_claim(&verified);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::VariableBoundMulV1),
        source_binding,
        recipe,
    );
    let closure = issue_root_carrier_join_closure_v1(&verified).map_err(Reject::After)?;
    let (join_sig, after) = closure.into_parts();
    let core = issue_source_bound_core_from_artifact_v1(
        artifact,
        join_sig,
        owner,
        source_bindings,
        effects,
    )
    .map_err(Reject::Core)?;
    let input_set =
        issue_initialized_local_input_source_set_v1(&core, input_rows).map_err(Reject::Inputs)?;
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, operation_rows)
        .map_err(Reject::Operations)?;
    Ok(VerifiedVariableBoundMulRecipeProductV1 {
        owner,
        site,
        frame,
        scope_region,
        bindings,
        operations,
        inputs: input_set,
        after,
    })
}
