//! Exact LoopCond Phi completion at the sole physical writer boundary.
use super::PlanLowerer;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopCondPhysicalInputV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::cell::{OnceCell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

/// Only the selected source entry can issue this exact completed physical role.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedLoopCondAfterPhysicalRoleV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    block: BasicBlockId,
    phi_prefix: Box<[MirInstruction]>,
    loop_phis: BTreeMap<BasicBlockId, Box<[MirInstruction]>>,
}
impl VerifiedLoopCondAfterPhysicalRoleV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(in crate::mir) fn site(&self) -> &SourceStmtSiteV1 { &self.site }
    pub(in crate::mir) const fn block(&self) -> BasicBlockId { self.block }
    pub(in crate::mir) fn loop_phis(&self) -> &BTreeMap<BasicBlockId, Box<[MirInstruction]>> { &self.loop_phis }
}

/// One stack-scoped completion of the SAME selected plan. Nested plans cannot
/// satisfy it, and a destination census alone cannot seal incoming values.
#[derive(Debug)]
pub(super) struct LoopCondPhysicalCompletionV1<'phi> {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    header: BasicBlockId,
    after: BasicBlockId,
    destinations: BTreeSet<(BasicBlockId, ValueId)>,
    finished: OnceCell<VerifiedLoopCondAfterPhysicalRoleV1>,
    phi_session: Option<&'phi crate::mir::builder::normal_callable_loop_source_facts::SourceLoopPhiSessionV1>,
}
impl<'phi> LoopCondPhysicalCompletionV1<'phi> {
    pub(super) fn new(owner: FunctionOwnerIdV1, site: SourceStmtSiteV1, plan: &CoreLoopPlan) -> Result<Self, String> {
        let destinations: BTreeSet<_> = plan.phis.iter().map(|phi| (phi.block, phi.dst)).collect();
        let values: BTreeSet<_> = plan.phis.iter().map(|phi| phi.dst).collect();
        if plan.header_bb == plan.after_bb || destinations.len() != plan.phis.len() || values.len() != plan.phis.len() {
            return Err(freeze("plan-destination-duplicate"));
        }
        Ok(Self { owner, site, header: plan.header_bb, after: plan.after_bb, destinations, finished: OnceCell::new(), phi_session: None })
    }
    pub(super) fn matching_phi_session(&self, plan: &CoreLoopPlan)
        -> Result<Option<&crate::mir::builder::normal_callable_loop_source_facts::SourceLoopPhiSessionV1>, String> {
        if plan.header_bb != self.header && plan.after_bb != self.after { return Ok(None); }
        if plan.header_bb != self.header || plan.after_bb != self.after { return Err(freeze("phi-session-loop-drift")); }
        Ok(self.phi_session)
    }
    pub(super) fn observe_completed_loop(&self, plan: &CoreLoopPlan, function: &MirFunction) -> Result<(), String> {
        // Recursive nested loops use the same effect port, but own other blocks.
        if plan.header_bb != self.header && plan.after_bb != self.after { return Ok(()); }
        if plan.header_bb != self.header || plan.after_bb != self.after || self.finished.get().is_some() {
            return Err(freeze("completion-source-or-repeat"));
        }
        let actual: BTreeSet<_> = plan.phis.iter().map(|phi| (phi.block, phi.dst)).collect();
        if actual != self.destinations || actual.len() != plan.phis.len() { return Err(freeze("plan-destination-drift")); }
        let rows: super::LoopPhiInputRowsV2 = plan.phis.iter().map(|phi| (phi.block, phi.dst, phi.inputs.clone())).collect();
        let loop_phis = super::capture_exact_loop_phis_v2(function, &rows)?;
        let phi_prefix = loop_phis.get(&self.after).cloned().unwrap_or_default();
        self.finished.set(VerifiedLoopCondAfterPhysicalRoleV1 {
            owner: self.owner, site: self.site.clone(), block: self.after, phi_prefix, loop_phis,
        }).map_err(|_| freeze("completion-repeat"))
    }
    pub(super) fn require_finished(&self) -> Result<(), String> {
        self.finished.get().map(|_| ()).ok_or_else(|| freeze("completion-missing"))
    }
    fn into_role(self) -> Result<Option<VerifiedLoopCondAfterPhysicalRoleV1>, String> {
        let role = self.finished.into_inner().ok_or_else(|| freeze("completion-missing"))?;
        Ok((!role.phi_prefix.is_empty()).then_some(role))
    }
}
pub(super) fn freeze(reason: &str) -> String { format!("[freeze:contract][loop-cond-after-role/{reason}]") }

impl PlanLowerer {
    pub(in crate::mir::builder) fn lower_loop_cond_with_source_publication(
        builder: &mut MirBuilder, plan: LoweredRecipe, ctx: &dyn PlanLoweringContext,
        ledger: Rc<RefCell<CallableSemanticLoweringState>>, source: &SourceLoopCondPhysicalInputV1<'_, '_>,
    ) -> Result<(Option<ValueId>, Option<VerifiedLoopCondAfterPhysicalRoleV1>), String> {
        let result = (|| {
            let CorePlan::Loop(loop_plan) = &plan else { return Err(freeze("non-loop-plan")); };
            let mut completion = LoopCondPhysicalCompletionV1::new(source.owner(),
                SourceStmtSiteV1::from_node(source.parent_site().clone()), loop_plan)?;
            completion.phi_session = Some(source.phi_session_v1());
            let after = loop_plan.after_bb;
            let mut port = super::emission_port::CorePlanEffectEmissionPortV1::source_publication_with_loop_completion(ledger, &completion);
            let result = Self::lower_with_emission_port(builder, plan, ctx, &mut port)?;
            port.finish()?;
            if builder.function_state.current_block != Some(after) { return Err(freeze("after-block-drift")); }
            let role = completion.into_role()?;
            source.phi_session_v1().commit(builder)?;
            Ok((result, role))
        })();
        result.map_err(|error| source.phi_session_v1().abort_error(builder, error))
    }
}
