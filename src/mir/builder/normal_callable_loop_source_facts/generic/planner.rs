//! One pure planner issuance, retained from source through the existing physical issuer.
use super::*;
use crate::mir::builder::control_flow::plan::{LoopFactsPolicyFrameV1, PlanBuildOutcome};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, LoopExecutionFrameKeyV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceV1};
use crate::mir::resolved_value_profile::expression_plan::{
    loop_condition_expression_source_plan_v1, ExpressionPlanRootV1, ExpressionSourcePlanV1,
};
use std::rc::Rc;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopBindingProjectionDispositionV1, CallableLoopBindingSourceV1,
};

#[derive(Debug)]
pub(super) struct PlannedSourceLoopV1 {
    pub(super) outcome: PlanBuildOutcome,
    pub(super) selection: Option<CallableLoopRouteMatchV1>,
}
impl PlannedSourceLoopV1 {
    pub(super) fn issue(condition: &crate::ast::ASTNode, body: &[crate::ast::ASTNode],
        policy: LoopFactsPolicyFrameV1, name: Box<str>, debug: bool) -> Result<Self, String> {
        let outcome = single_planner::try_build_source_outcome(
            CallableLoopFactsPlannerInputV1::new(condition, body, policy, name, debug))?;
        let selection = outcome.facts.as_ref().map(CallableLoopRouteMatchV1::issue);
        Ok(Self { outcome, selection })
    }
}

/// Same original source membership and the sole planner result. No physical ID,
/// Home permission or second planner call can be manufactured by this transport.
#[derive(Debug)]
pub(in crate::mir) struct PreissuedCallableLoopSourcePlanV1 {
    source: Rc<RetainedCallableLoopSourceV1>,
    policy: LoopFactsPolicyFrameV1,
    planned: PlannedSourceLoopV1,
    source_recipe: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
}

/// Original Loop source identity shared by Home and the affine planner consumer.
/// Keeping it alive grants no value class, Home continuation or physical role.
#[derive(Debug)]
pub(in crate::mir) struct RetainedCallableLoopSourceV1 {
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
    let body = source.child_body_from_stmt(&statement, BodyChildRoleV1::LoopBody).map_err(|e| format!("{e:?}"))?;
    let condition_source = loop_condition_expression_source_plan_v1(input, site).map_err(|e| format!("{e:?}"))?;
    if condition_source.nodes().first().is_none_or(|root| root.site() != condition.site()) {
        return Err("[freeze:contract][preissued-loop/condition-source]".into());
    }
    let handoff = CallableLoopBindingProjectionDispositionV1::from_resolved_input(input, site,
        matches!(condition.node(), crate::ast::ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(true), ..
        }))?;
    let bindings = Rc::clone(handoff.source()?);
    let planned = PlannedSourceLoopV1::issue(condition.node(), body.statements(), policy,
        format!("source-owner:{:?}", input.owner()).into_boxed_str(), policy.debug_enabled())?;
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
        policy, planned, source_recipe: None,
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
    /// Carry the same Home-issued source/planner loan into an alternate
    /// source-bound LoopBreak physical owner. That owner may supersede the
    /// generic Loop route only when this exact site has no selected source
    /// Recipe attached; the separate binding handoff must already be consumed.
    pub(in crate::mir::builder) fn validate_loop_break_physical_handoff_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
        origin: crate::mir::resolved_semantics::FunctionOriginV1,
        kind: crate::mir::resolved_semantics::SemanticOwnerSourceKindV1,
        policy: LoopFactsPolicyFrameV1,
    ) -> Result<(), String> {
        if self.policy != policy
            || !self.source.matches_identity_v1(owner, site, origin, kind)
        {
            return Err("[freeze:contract][preissued-loop/alternate-physical-source-drift]".into());
        }
        if self.source_recipe.is_some() {
            return Err("[freeze:contract][preissued-loop/alternate-physical-selected-recipe]".into());
        }
        if self.source.binding_handoff_pending_v1() {
            return Err("[freeze:contract][preissued-loop/alternate-physical-binding-pending]".into());
        }
        Ok(())
    }

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
    pub(super) fn into_planned(self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1,
        origin: Option<crate::mir::resolved_semantics::FunctionOriginV1>,
        kind: Option<crate::mir::resolved_semantics::SemanticOwnerSourceKindV1>, policy: LoopFactsPolicyFrameV1,
    ) -> Result<(PlannedSourceLoopV1, Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>), String> {
        if self.policy != policy || !origin.zip(kind).is_some_and(|(origin, kind)|
            self.source.matches_identity_v1(owner, site, origin, kind)) {
            return Err("[freeze:contract][preissued-loop/physical-source-drift]".into());
        }
        Ok((self.planned, self.source_recipe))
    }

    /// Borrow the original selected LoopCond Recipe with its located body and
    /// analysis/source projection. This is source correspondence only: it grants
    /// no typed operation, portable Recipe/JoinSig, Home After or physical use.
    fn loop_cond_source<'plan, 'source>(
        &'plan self, input: ResolvedFunctionLoweringInputV1<'source>,
    ) -> Option<LoopCondRecipeSourceRefV1<'plan, 'source>> {
        self.condition_source_v1(input)?;
        if self.planned.selection.as_ref()?.sole_family() != Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue) { return None; }
        let canonical = self.planned.outcome.facts.as_ref()?;
        let facts = canonical.facts.loop_cond_break_continue.as_ref()?;
        let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
        let membership = ledger.resolved_loop_source(self.site()).ok()?;
        if !membership.source().frame_key().matches(self.frame()) { return None; }
        let source = input.source();
        let statement = source.stmt_at(&membership).ok()?;
        let body = source.child_body_from_stmt(&statement, BodyChildRoleV1::LoopBody).ok()?;
        let condition = source.child_expr_from_stmt(&statement, ExprChildRoleV1::LoopCondition).ok()?;
        if !corroborate_loop_cond_recipe_source(facts, condition.node(), body.statements()) { return None; }
        Some(LoopCondRecipeSourceRefV1 { input, facts, body, projection: canonical.facts.source_projection() })
    }

    /// Drop the borrowed planner/body view before recursively scanning the
    /// original body. The shared source is observation only, never Home After.
    pub(in crate::mir) fn source_observation_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>)
        -> Option<Rc<RetainedCallableLoopSourceV1>> {
        let selected = self.original_loop_cond_source_matches_v1(input);
        if selected.is_none() && crate::config::env::builder_debug_enabled() {
            eprintln!("[source-loop] owner={:?} site={:?} original planner source unavailable; condition={} family={:?} routes={:?}",
                input.owner(), self.site(), self.condition_source_v1(input).is_some(),
                self.planned.selection.as_ref().and_then(|selection| selection.sole_family()),
                self.planned.selection.as_ref().map(|selection| selection.matched_routes()));
        }
        selected?;
        Some(Rc::clone(&self.source))
    }

    /// Share the original selected condition/body across LoopCond families.
    /// Return stays Return: this loan grants no exit port or physical route.
    fn original_loop_cond_source_matches_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> Option<()> {
        self.condition_source_v1(input)?;
        let canonical = &self.planned.outcome.facts.as_ref()?.facts;
        let (expected_condition, expected_body) = match self.planned.selection.as_ref()?.sole_family()? {
            CallableLoopSoleFamilyV1::LoopCondBreakContinue => {
                let facts = canonical.loop_cond_break_continue.as_ref()?;
                (&facts.condition, facts.recipe.body.body.as_slice())
            }
            CallableLoopSoleFamilyV1::LoopCondReturnInBody => {
                let facts = canonical.loop_cond_return_in_body.as_ref()?;
                (&facts.condition, facts.recipe.body.body.as_slice())
            }
            CallableLoopSoleFamilyV1::LoopTrueBreakContinue => return None,
        };
        let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
        let membership = ledger.resolved_loop_source(self.site()).ok()?;
        if !membership.source().frame_key().matches(self.frame()) { return None; }
        let source = input.source();
        let statement = source.stmt_at(&membership).ok()?;
        let body = source.child_body_from_stmt(&statement, BodyChildRoleV1::LoopBody).ok()?;
        let condition = source.child_expr_from_stmt(&statement, ExprChildRoleV1::LoopCondition).ok()?;
        (expected_condition == condition.node() && expected_body == body.statements()).then_some(())
    }

    /// The existing Array Home consumer requires the complete flat NoExitBody;
    /// sharing source correspondence does not broaden that capability.
    pub(in crate::mir) fn flat_no_exit_statement_sites(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Option<Vec<SourceStmtSiteV1>> {
        self.loop_cond_source(input)?.flat_no_exit_statement_sites()
    }
}

/// A borrowed view of the SAME retained planner result and original source.
/// Keeping these loans together prevents a consumer from re-pairing a Recipe
/// with a body or projection obtained by another planner invocation.
struct LoopCondRecipeSourceRefV1<'plan, 'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    facts: &'plan crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakContinueFacts,
    body: crate::mir::compiler::located::LocatedBodyV1<'source>,
    projection: &'plan crate::mir::builder::control_flow::facts::stmt_view::LoopSourceProjectionV1,
}
impl LoopCondRecipeSourceRefV1<'_, '_> {
    fn flat_no_exit_statement_sites(&self) -> Option<Vec<SourceStmtSiteV1>> {
        use crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakAcceptKind;
        use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
        if self.facts.accept_kind != LoopCondBreakAcceptKind::NoExitBody { return None; }
        let length = self.body.statements().len();
        if self.projection.flattened_body_len()? != length || self.facts.recipe.body.len() != length
            || self.facts.recipe.items.len() != length { return None; }
        let mut sites = Vec::with_capacity(length);
        for (index, item) in self.facts.recipe.items.iter().enumerate() {
            let LoopCondBreakContinueItem::Stmt(reference) = item else { return None; };
            if reference.index() != index { return None; }
            let original = self.projection.site_for_flattened_index(reference.index())?;
            if !original.scope_box_children().is_empty() || original.raw_body_index() as usize != index { return None; }
            sites.push(self.input.source().body_stmt(&self.body, original.raw_body_index() as usize).ok()?.site().clone());
        }
        Some(sites)
    }
}

/// Shared source-shape corroboration; Home loans and the physical transfer use
/// the exact same retained condition/body, never a rewritten AST view.
pub(in crate::mir::builder::normal_callable_loop_source_facts) fn corroborate_loop_cond_recipe_source(
    facts: &crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakContinueFacts,
    condition: &crate::ast::ASTNode, body: &[crate::ast::ASTNode],
) -> bool { &facts.condition == condition && facts.recipe.body.body.as_slice() == body }
