//! Whole source Loop evidence moves into its existing parent and role ledger.
//! A prepared completion owns the SAME Phi transaction, not a parallel seal.
use super::*;
use crate::mir::builder::{EmittedSourceLoopGraphV2, CompletedSourceLoopPhiPointsV2,
    PreparedSourceLoopPhiCompletionV2, VerifiedLoopCondAfterPhysicalRoleV1, MirBuilder};
use std::{cell::RefMut, collections::BTreeMap};
type Binding = (BasicBlockId, MirInstruction);
type Roles = BTreeMap<(FunctionOwnerIdV1, crate::mir::resolved_semantics::SourceStmtSiteV1), Rc<VerifiedLoopCondAfterPhysicalRoleV1>>;

#[derive(Debug)]
pub(in crate::mir) struct SourceLoopPhysicalProofV2 {
    pub(super) graph: EmittedSourceLoopGraphV2,
    points: CompletedSourceLoopPhiPointsV2,
    owned: Vec<Binding>,
}
impl SourceLoopPhysicalProofV2 {
    pub(in crate::mir) fn product(&self) -> &Rc<Product> { &self.graph.product }
    pub(in crate::mir) fn header(&self) -> BasicBlockId { self.graph.header }
    pub(in crate::mir) fn result_binding(&self) -> &(BasicBlockId, MirInstruction) { self.graph.result.original() }
    pub(in crate::mir) fn after(&self) -> BasicBlockId { self.graph.after }
    pub(in crate::mir) fn points(&self) -> &CompletedSourceLoopPhiPointsV2 { &self.points }
    pub(in crate::mir) fn feedback_edges_v2(&self) -> impl Iterator<Item = (BasicBlockId, BasicBlockId)> + '_ {
        self.graph.edges.iter().filter(|edge| matches!(edge.source,
            crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::Backedge
            | crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::Continue(_)))
            .map(|edge| (edge.predecessor, edge.target))
    }
    pub(in crate::mir) fn bindings(&self) -> &[Binding] { &self.owned }
}

pub(in crate::mir) struct PreparedSourceLoopPhysicalCompletionV2<'phi, 'ledger> {
    parent: Rc<EmittedSourceLoopProjectionV2>,
    roles: RefMut<'ledger, Roles>,
    marker: PreparedSourceLoopPhiCompletionV2<'phi>,
    graph: EmittedSourceLoopGraphV2,
    owned: Vec<Binding>,
}
impl PreparedSourceLoopPhysicalCompletionV2<'_, '_> {
    pub(in crate::mir) fn commit(mut self, builder: &mut MirBuilder) -> Result<Rc<VerifiedLoopCondAfterPhysicalRoleV1>, String> {
        // All source/frame/cleanup/census and both slot checks precede this
        // consuming commit. PhiTxn rechecks its actual rows before moving points.
        let points = self.marker.commit(builder)?;
        let proof = Rc::new(SourceLoopPhysicalProofV2 { graph: self.graph, points, owned: self.owned });
        let role = Rc::new(VerifiedLoopCondAfterPhysicalRoleV1::from_source_v2(proof));
        let key = (role.owner(), role.site().clone());
        let previous = self.roles.insert(key, Rc::clone(&role));
        assert!(previous.is_none(), "exclusive source Loop role preparation");
        self.parent.completion.set(Rc::clone(&role)).expect("exclusive source Loop parent preparation");
        Ok(role)
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn prepare_source_loop_physical_v2<'phi>(&self,
        graph: EmittedSourceLoopGraphV2, marker: PreparedSourceLoopPhiCompletionV2<'phi>,
        function: &MirFunction,
    ) -> Result<PreparedSourceLoopPhysicalCompletionV2<'phi, '_>, String> {
        marker.require_source_v2(&graph.product, graph.header, graph.after)?;
        let owner = graph.product.source.source.owner();
        let parent = CallValidationContextV1::Lowering(self).with_local_call_group_tree_v2(owner, |groups| {
            let mut found = groups.iter().filter_map(|group| group.source_loop_rc())
                .filter(|packet| Rc::ptr_eq(packet.product(), &graph.product));
            let parent = found.next().cloned().ok_or_else(|| freeze("source-loop/parent-missing"))?;
            if found.next().is_some() || parent.completion.get().is_some() { return Err(freeze("source-loop/parent-duplicate")); }
            Ok(parent)
        })?;
        let context = CallValidationContextV1::Lowering(self);
        graph.verify_control_v2(function)?;
        let children = parent.validate_frames_v2(&graph, context, function, None)?;
        for edge in &graph.edges {
            if let Some(cleanup) = &edge.cleanup { cleanup.plan.validate_emitted_v2(cleanup, context, function, None)?; }
        }
        let owned = graph.completed_bindings(function)?;
        validate_owned_v2(&graph, &owned, &children, marker.phis(), function)?;
        let roles = self.loop_cond_after_roles.borrow_mut();
        let site = graph.product.source.source.site().clone();
        if self.completion_for_owner(owner).is_none() || roles.contains_key(&(owner, site)) {
            return Err(freeze("source-loop/role-duplicate-or-owner"));
        }
        Ok(PreparedSourceLoopPhysicalCompletionV2 { parent, roles, marker, graph, owned })
    }
}

#[path = "source_loop_physical_validation.rs"]
mod validation;
use validation::validate_owned_v2;
