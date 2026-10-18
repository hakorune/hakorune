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
}

/// Original Loop source identity shared by Home and the affine planner consumer.
/// Keeping it alive grants no value class, Home continuation or physical role.
#[derive(Debug)]
pub(in crate::mir) struct RetainedCallableLoopSourceV1 {
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    frame: LoopExecutionFrameKeyV1,
    condition: Rc<ExpressionSourcePlanV1>,
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
        source: Rc::new(RetainedCallableLoopSourceV1 { owner: input.owner(), source, frame, condition: condition_source, condition_reads }),
        policy, planned,
    })
}

impl RetainedCallableLoopSourceV1 {
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
    ) -> Result<PlannedSourceLoopV1, String> {
        if self.policy != policy || !origin.zip(kind).is_some_and(|(origin, kind)|
            self.source.matches_identity_v1(owner, site, origin, kind)) {
            return Err("[freeze:contract][preissued-loop/physical-source-drift]".into());
        }
        Ok(self.planned)
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
