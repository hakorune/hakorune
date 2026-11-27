//! Checked loop source retained from Resolve through physical emission.
use super::*;
use crate::mir::builder::control_flow::plan::LoopFactsPolicyFrameV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{ExprChildRoleV1, LoopExecutionFrameKeyV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceV1};
use crate::mir::resolved_value_profile::expression_plan::{
    loop_condition_expression_source_plan_v1, ExpressionPlanRootV1, ExpressionSourcePlanV1,
};
use std::rc::Rc;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopBindingProjectionDispositionV1, CallableLoopBindingSourceV1,
};

/// Original source membership and its preissued checked V2 product.
#[derive(Debug)]
pub(in crate::mir) struct PreissuedCallableLoopSourcePlanV1 {
    source: Rc<RetainedCallableLoopSourceV1>,
    policy: LoopFactsPolicyFrameV1,
    source_recipe: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
}

/// Original Loop source identity shared by Home and the affine checked loop consumer.
/// Keeping it alive grants no value class, Home continuation or physical role.
#[derive(Debug)]
pub(crate) struct RetainedCallableLoopSourceV1 {
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    frame: LoopExecutionFrameKeyV1,
    condition: Rc<ExpressionSourcePlanV1>,
    bindings: Rc<CallableLoopBindingSourceV1>,
    binding_handoff: std::cell::RefCell<Option<CallableLoopBindingProjectionDispositionV1>>,
    condition_reads: std::collections::BTreeSet<(SourceNodeSiteV1, crate::mir::resolved_semantics::BindingRefV1)>,
}

pub(in crate::mir) fn issue_preissued_callable_loop_source_plan_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, site: &SourceStmtSiteV1,
    policy: LoopFactsPolicyFrameV1,
) -> Result<PreissuedCallableLoopSourcePlanV1, String> {
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|e| format!("{e:?}"))?;
    let membership = ledger.resolved_loop_source(site).map_err(|e| format!("{e:?}"))?;
    let source = input.source();
    let statement = source.stmt_at(&membership).map_err(|e| format!("{e:?}"))?;
    if !membership.source().matches_identity(input.function().function_origin(), input.function().source_kind(), statement.site()) {
        return Err("[freeze:contract][preissued-loop/source-identity]".into());
    }
    let condition = source.child_expr_from_stmt(&statement, ExprChildRoleV1::LoopCondition).map_err(|e| format!("{e:?}"))?;
    let condition_source = loop_condition_expression_source_plan_v1(input, site).map_err(|e| format!("{e:?}"))?;
    if condition_source.nodes().first().is_none_or(|root| root.site() != condition.site()) {
        return Err("[freeze:contract][preissued-loop/condition-source]".into());
    }
    let handoff = CallableLoopBindingProjectionDispositionV1::from_resolved_input(input, site,
        matches!(condition.node(), crate::ast::ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(true), ..
        }))?;
    let bindings = Rc::clone(handoff.source()?);
    let (source, frame, _) = membership.into_parts();
    // The original ledger includes reads inside opaque children too. Do not
    // replace this complete passive census with only traversable plan leaves.
    let condition_reads = input.function().variable_refs().filter_map(|(site, reference)| {
        let crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) = reference else { return None; };
        site.node().segments().starts_with(condition.site().node().segments())
            .then(|| (site.node().clone(), *binding))
    }).collect();
    Ok(PreissuedCallableLoopSourcePlanV1 {
        source: Rc::new(RetainedCallableLoopSourceV1 { owner: input.owner(), source, frame, condition: condition_source, bindings,
            binding_handoff: std::cell::RefCell::new(Some(handoff)), condition_reads }),
        policy, source_recipe: None,
    })
}

impl RetainedCallableLoopSourceV1 {
    pub(in crate::mir) fn bindings(&self) -> &Rc<CallableLoopBindingSourceV1> { &self.bindings }
    pub(in crate::mir) fn binding_handoff_pending_v1(&self) -> bool { self.binding_handoff.borrow().is_some() }

    pub(in crate::mir::builder) fn take_binding_handoff_v1(&self)
        -> Result<CallableLoopBindingProjectionDispositionV1, String> {
        let handoff = self.binding_handoff.borrow_mut().take()
            .ok_or("[freeze:contract][preissued-loop/binding-handoff-already-taken]")?;
        if !Rc::ptr_eq(handoff.source()?, &self.bindings) {
            return Err("[freeze:contract][preissued-loop/binding-source-drift]".into());
        }
        Ok(handoff)
    }

    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(in crate::mir) fn site(&self) -> &SourceStmtSiteV1 { self.source.site() }
    pub(in crate::mir) fn frame(&self) -> &LoopExecutionFrameKeyV1 { &self.frame }
    /// Reborrow one canonical condition plan, including its original purpose.
    /// This source loan cannot promote an uncovered Home prefix.
    pub(in crate::mir) fn condition_source_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> Option<&Rc<ExpressionSourcePlanV1>> {
        if input.owner() != self.owner || !self.source.matches_identity(input.function().function_origin(), input.function().source_kind(), self.site())
            || !self.condition_matches_source_v1() { return None; }
        let ExpressionPlanRootV1::LoopCondition { bundle, .. } = self.condition.root() else { return None };
        (input.function().loop_region_bundle(self.site()).ok() == Some(bundle)).then_some(&self.condition)
    }
    fn condition_matches_source_v1(&self) -> bool {
        matches!(self.condition.root(), ExpressionPlanRootV1::LoopCondition { statement, condition, frame, .. }
            if self.condition.owner() == self.owner && statement == self.site() && frame.matches(&self.frame)
                && self.condition.nodes().first().is_some_and(|root| root.site() == condition))
    }
    pub(in crate::mir) fn matches_identity_v1(&self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1,
        origin: crate::mir::resolved_semantics::FunctionOriginV1,
        kind: crate::mir::resolved_semantics::SemanticOwnerSourceKindV1,
    ) -> bool {
        owner == self.owner && site == self.site().node()
            && self.source.matches_identity(origin, kind, self.site())
            && self.frame.matches(&self.source.frame_key()) && self.condition_matches_source_v1()
    }
    /// Exact source correspondence, including opaque/receiver reads. Duplicate,
    /// missing or extra receipts cannot be hidden by a set conversion.
    pub(in crate::mir) fn matches_condition_reads_v1(&self, condition: &SourceNodeSiteV1,
        reads: impl IntoIterator<Item = (SourceNodeSiteV1, crate::mir::resolved_semantics::BindingRefV1)>,
    ) -> bool {
        if self.condition.nodes().first().is_none_or(|root| root.site().node() != condition) { return false; }
        let mut observed = std::collections::BTreeSet::new();
        for row in reads { if !observed.insert(row) { return false; } }
        observed == self.condition_reads
    }
}

impl PreissuedCallableLoopSourcePlanV1 {

    /// Validate the SAME retained source and condition before the ledger takes
    /// this affine plan. Sharing logical evidence grants no physical execution.
    pub(in crate::mir) fn validate_source_recipe_attachment_v2(&self,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    ) -> Result<(), String> {
        if self.source_recipe.is_some() || !Rc::ptr_eq(&self.source, &product.source.source)
            || !Rc::ptr_eq(&self.source.condition, product.source.condition.source()) {
            return Err("[freeze:contract][source-loop/plan-product-identity]".into());
        }
        Ok(())
    }
    pub(in crate::mir) fn attach_source_recipe_v2(&mut self,
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    ) -> Result<(), String> {
        self.validate_source_recipe_attachment_v2(&product)?;
        self.source_recipe = Some(product);
        Ok(())
    }
    #[cfg(test)]
    pub(in crate::mir) fn source_recipe_for_test_v2(&self)
        -> Option<&Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>> { self.source_recipe.as_ref() }
    pub(in crate::mir) fn original_source_v1(&self) -> &Rc<RetainedCallableLoopSourceV1> { &self.source }
    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 { self.source.owner() }
    pub(in crate::mir) fn site(&self) -> &SourceStmtSiteV1 { self.source.site() }
    pub(in crate::mir) fn frame(&self) -> &LoopExecutionFrameKeyV1 { self.source.frame() }
    pub(in crate::mir) fn condition_source_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> Option<&Rc<ExpressionSourcePlanV1>> {
        self.source.condition_source_v1(input)
    }
    pub(super) fn into_issuance(self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1,
        origin: Option<crate::mir::resolved_semantics::FunctionOriginV1>,
        kind: Option<crate::mir::resolved_semantics::SemanticOwnerSourceKindV1>, policy: LoopFactsPolicyFrameV1,
    ) -> Result<Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>, String> {
        if self.policy != policy || !origin.zip(kind).is_some_and(|(origin, kind)|
            self.source.matches_identity_v1(owner, site, origin, kind)) {
            return Err("[freeze:contract][preissued-loop/physical-source-drift]".into());
        }
        Ok(self.source_recipe)
    }

}
