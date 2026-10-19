//! The original Loop owns its conditional prefix groups. This is a destination
//! inside the existing group tree, not a second expression or Home registry.
use super::*;
use std::{cell::{OnceCell, RefCell}, rc::Rc};
use crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2 as Product;

#[derive(Debug)]
pub(crate) struct EmittedSourceLoopProjectionV2 {
    product: Rc<Product>,
    expected: Vec<LifecyclePrefixSourceV1>,
    pub(super) children: RefCell<Vec<RootLifecyclePrefixBindingGroupV1>>,
    // Only a whole source-edge/cleanup closure may install the physical role.
    // Collection alone deliberately provides no completion writer.
    completion: OnceCell<Rc<crate::mir::builder::VerifiedLoopCondAfterPhysicalRoleV1>>,
}

impl EmittedSourceLoopProjectionV2 {
    fn pending(product: Rc<Product>) -> Result<Rc<Self>, String> {
        let expected = product.lifecycle_prefixes_v2();
        if expected.iter().enumerate().any(|(index, row)| expected[..index].iter().any(|old| old.site() == row.site())) {
            return Err(freeze("source-loop/child-site-duplicate"));
        }
        if expected.len() != product.source.body_expressions.len() + 1 + product.schedule().blocks_array_appends_v2().count()
            || !matches!(expected.first(), Some(LifecyclePrefixSourceV1::BoolCondition { frame, .. })
                if Rc::ptr_eq(frame, &product.source.condition)) {
            return Err(freeze("source-loop/child-census"));
        }
        Ok(Rc::new(Self { product, expected, children: RefCell::new(Vec::new()), completion: OnceCell::new() }))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn product(&self) -> &Rc<Product> { &self.product }
    pub(super) fn expected(&self) -> &[LifecyclePrefixSourceV1] { &self.expected }
    pub(super) fn snapshot(&self) -> Vec<RootLifecyclePrefixBindingGroupV1> { self.children.borrow().clone() }
    pub(super) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        self.completion.get().and_then(|role| role.source_proof_v2()).map(|proof| proof.bindings()).unwrap_or(&[])
    }
    pub(super) fn validate_completion_v2(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        bindings: &[(BasicBlockId, MirInstruction)],
        coordinates: Option<&super::super::super::physical_boundary::FinishedBindings>,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let role = self.completion.get().ok_or_else(|| freeze("source-loop/physical-closure-pending"))?;
        let proof = role.source_proof_v2().ok_or_else(|| freeze("source-loop/whole-proof-missing"))?;
        if !Rc::ptr_eq(proof.product(), &self.product) || context.ledger().loop_cond_after_roles.borrow()
            .get(&(role.owner(), role.site().clone())).is_none_or(|original| !Rc::ptr_eq(original, role)) {
            return Err(freeze("source-loop/original-role-membership"));
        }
        proof.validate_v2(context, function, bindings, coordinates, finishing)?;
        self.validate_frames_v2(&proof.graph, context, function, finishing).map(drop)
    }
    pub(super) fn require_completed(&self) -> Result<(), String> {
        self.completion.get().map(|_| ()).ok_or_else(|| freeze("source-loop/physical-closure-pending"))
    }
}

#[derive(Clone)]
pub(super) enum LifecyclePrefixDestinationV2 {
    Root,
    Loop(Rc<EmittedSourceLoopProjectionV2>),
}
impl LifecyclePrefixDestinationV2 {
    pub(super) fn expected(&self, ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1)
        -> Result<Vec<LifecyclePrefixSourceV1>, String> {
        match self { Self::Root => ledger.expected_local_call_binding_sources(owner), Self::Loop(packet) => Ok(packet.expected.clone()) }
    }
    pub(super) fn snapshot(&self, ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1)
        -> Vec<RootLifecyclePrefixBindingGroupV1> {
        match self { Self::Root => ledger.root_local_call_bindings.borrow().get(&owner).cloned().unwrap_or_default(),
            Self::Loop(packet) => packet.snapshot() }
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Open at the sole selected Loop entry, before its condition/body descend.
    /// Reopening or attaching a reissued equal product is never a retry.
    pub(in crate::mir) fn open_source_loop_prefix_v2(&self, product: Rc<Product>) -> Result<(), String> {
        let owner = product.source.source.owner();
        let packet = EmittedSourceLoopProjectionV2::pending(product)?;
        self.prepare_local_call_binding_group(owner, RootLifecyclePrefixBindingGroupV1::new_loop_pending(packet))?.commit()
    }

    pub(super) fn prefix_destination_v2(&self, owner: FunctionOwnerIdV1, site: &OwnedExprSiteV1)
        -> Result<LifecyclePrefixDestinationV2, String> {
        let roots = self.root_local_call_bindings.borrow().get(&owner).cloned().unwrap_or_default();
        let mut matching = roots.iter().filter_map(|group| group.source_loop_rc())
            .filter(|packet| packet.expected.iter().any(|source| source.site() == site));
        let destination = matching.next().cloned();
        if matching.next().is_some() { return Err(freeze("source-loop/child-owner-duplicate")); }
        Ok(destination.map(LifecyclePrefixDestinationV2::Loop).unwrap_or(LifecyclePrefixDestinationV2::Root))
    }

    pub(super) fn next_prefix_source_v2(&self, owner: FunctionOwnerIdV1, site: &OwnedExprSiteV1)
        -> Result<Option<LifecyclePrefixSourceV1>, String> {
        let destination = self.prefix_destination_v2(owner, site)?;
        let count = destination.snapshot(self, owner).len();
        Ok(destination.expected(self, owner)?.get(count).cloned())
    }

    pub(super) fn prepare_prefix_at_destination_v2(&self, owner: FunctionOwnerIdV1,
        group: RootLifecyclePrefixBindingGroupV1,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        let site = group.site();
        if site.owner() != owner { return Err(freeze("local-call-binding-owner-drift")); }
        let destination = self.prefix_destination_v2(owner, site)?;
        let expected = destination.expected(self, owner)?;
        let groups = destination.snapshot(self, owner);
        if groups.iter().any(|recorded| recorded.site() == site) { return Err(freeze("duplicate-local-call-bindings")); }
        if groups.len() >= expected.len() { return Err(freeze("local-call-bindings-overflow")); }
        if expected.get(groups.len()).is_none_or(|source| !group.matches_source(source)) {
            return Err(freeze("local-call-binding-site-order"));
        }
        Ok(PreparedLifecyclePrefixBindingGroupV1 { ledger: self, owner, group, destination,
            preceding_sites: groups.iter().map(|recorded| recorded.site().clone()).collect() })
    }
}

/// Snapshot shared packet references before callbacks. Neither the root pool
/// nor a child's collection RefCell remains borrowed during recursive checks.
pub(super) fn snapshot_group_tree_v2(roots: &[RootLifecyclePrefixBindingGroupV1])
    -> Vec<RootLifecyclePrefixBindingGroupV1> {
    let mut output = Vec::new();
    let mut pending: Vec<_> = roots.iter().rev().cloned().collect();
    while let Some(group) = pending.pop() {
        if let Some(packet) = group.source_loop_rc() { pending.extend(packet.snapshot().into_iter().rev()); }
        output.push(group);
    }
    output
}

impl CallValidationContextV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn require_source_loop_parent_v2(self,
        product: &std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        completed: bool,
    ) -> Result<(), String> {
        self.with_local_call_group_tree_v2(product.source.source.owner(), |groups| {
            let mut packets = groups.iter().filter_map(|group| group.source_loop_rc())
                .filter(|packet| std::rc::Rc::ptr_eq(packet.product(), product));
            let packet = packets.next().ok_or_else(|| freeze("source-loop/original-group-membership"))?;
            if packets.next().is_some() { return Err(freeze("source-loop/original-group-membership")); }
            if completed { packet.require_completed()?; }
            Ok(())
        })
    }
}

#[path = "source_loop_cleanup.rs"]
mod cleanup;
pub(in crate::mir) use cleanup::SourceLoopCleanupPlanV2;

#[path = "source_loop_frame_validation.rs"]
mod frames;
#[path = "source_loop_physical.rs"]
mod physical;
pub(in crate::mir) use physical::{SourceLoopPhysicalProofV2, PreparedSourceLoopPhysicalCompletionV2};
