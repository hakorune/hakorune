//! One pure planner issuance, retained from source through the existing physical issuer.
use super::*;
use crate::mir::builder::control_flow::plan::{LoopFactsPolicyFrameV1, PlanBuildOutcome};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, LoopExecutionFrameKeyV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceV1};

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
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    frame: LoopExecutionFrameKeyV1,
    policy: LoopFactsPolicyFrameV1,
    planned: PlannedSourceLoopV1,
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
    let planned = PlannedSourceLoopV1::issue(condition.node(), body.statements(), policy,
        format!("source-owner:{:?}", input.owner()).into_boxed_str(), policy.debug_enabled())?;
    let (source, frame, _) = membership.into_parts();
    Ok(PreissuedCallableLoopSourcePlanV1 { owner: input.owner(), source, frame, policy, planned })
}

impl PreissuedCallableLoopSourcePlanV1 {
    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(in crate::mir) fn site(&self) -> &SourceStmtSiteV1 { self.source.site() }
    pub(in crate::mir) fn frame(&self) -> &LoopExecutionFrameKeyV1 { &self.frame }
    pub(super) fn into_planned(self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1,
        origin: Option<crate::mir::resolved_semantics::FunctionOriginV1>,
        kind: Option<crate::mir::resolved_semantics::SemanticOwnerSourceKindV1>, policy: LoopFactsPolicyFrameV1,
    ) -> Result<PlannedSourceLoopV1, String> {
        if owner != self.owner || site != self.site().node() || self.policy != policy
            || origin != Some(self.source.function_origin()) || kind != Some(self.source.source_kind())
            || !self.frame.matches(&self.source.frame_key()) {
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
        if input.owner() != self.owner || !self.source.matches_identity(input.function().function_origin(), input.function().source_kind(), self.site())
            || self.planned.selection.as_ref()?.sole_family() != Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue) { return None; }
        let canonical = self.planned.outcome.facts.as_ref()?;
        let facts = canonical.facts.loop_cond_break_continue.as_ref()?;
        let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
        let membership = ledger.resolved_loop_source(self.site()).ok()?;
        if !membership.source().frame_key().matches(&self.frame) { return None; }
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
