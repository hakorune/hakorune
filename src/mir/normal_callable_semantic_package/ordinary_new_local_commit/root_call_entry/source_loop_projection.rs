//! The original Loop owns its conditional prefix groups. This is a destination
//! inside the existing group tree, not a second expression or Home registry.
use super::*;
use crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2 as Product;
use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

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
        for (index, row) in expected.iter().enumerate() {
            if let Some(old) = expected[..index]
                .iter()
                .find(|old| old.site() == row.site())
            {
                let paired = index > 0
                    && expected.get(index - 1).is_some_and(|prior| {
                        prior.site() == old.site() && source_call_bool_prefix_pair_v2(prior, row)
                    });
                if !paired {
                    return Err(freeze("source-loop/child-site-duplicate"));
                }
            }
        }
        // Verified source coverage binds argument-purpose frames to their
        // append candidate. Its Call packet owns those children; emitting a
        // second prefix here would evaluate the arguments twice.
        let standalone = product
            .source
            .body_expressions
            .iter()
            .filter(|plan| plan.argument_frame().is_none())
            .count();
        if expected.len()
            != standalone
                + 1
                + product.schedule().blocks_array_appends_v2().count()
                + product.schedule().calls_v2().count()
            || !matches!(expected.first(), Some(LifecyclePrefixSourceV1::BoolCondition { frame, .. })
                if Rc::ptr_eq(frame, &product.source.condition))
        {
            return Err(freeze("source-loop/child-census"));
        }
        Ok(Rc::new(Self {
            product,
            expected,
            children: RefCell::new(Vec::new()),
            completion: OnceCell::new(),
        }))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn product(
        &self,
    ) -> &Rc<Product> {
        &self.product
    }
    pub(super) fn expected(&self) -> &[LifecyclePrefixSourceV1] {
        &self.expected
    }
    pub(super) fn snapshot(&self) -> Vec<RootLifecyclePrefixBindingGroupV1> {
        self.children.borrow().clone()
    }
    pub(super) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        self.completion
            .get()
            .and_then(|role| role.source_proof_v2())
            .map(|proof| proof.bindings())
            .unwrap_or(&[])
    }
    /// Lend only transport Copies observed by the SAME source-edge writer.
    /// The enclosing group must validate its whole source/physical completion.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn incoming_copy_bindings_v2(
        &self, function: &MirFunction,
        projection: &super::super::super::physical_boundary::FinishedBindings,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let proof = self.completion.get().and_then(|role| role.source_proof_v2())
            .ok_or_else(|| freeze("source-loop/whole-proof-missing"))?;
        let mut bindings = Vec::new();
        for edge in &proof.graph.edges {
            for incoming in &edge.values {
                incoming.localized.require_completed_v1()?;
                incoming.localized.verify_projected(function, &mut |row|
                    projection.relation_binding(function, row))?;
                incoming.localized.append_observed_bindings_v1(&mut bindings);
            }
        }
        bindings.retain(|(_, row)| matches!(row, MirInstruction::Copy { .. }));
        Ok(bindings)
    }
    pub(super) fn validate_completion_v2(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        bindings: &[(BasicBlockId, MirInstruction)],
        coordinates: Option<&super::super::super::physical_boundary::FinishedBindings>,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let role = self
            .completion
            .get()
            .ok_or_else(|| freeze("source-loop/physical-closure-pending"))?;
        let proof = role
            .source_proof_v2()
            .ok_or_else(|| freeze("source-loop/whole-proof-missing"))?;
        if !Rc::ptr_eq(proof.product(), &self.product)
            || context
                .ledger()
                .loop_cond_after_roles
                .borrow()
                .get(&(role.owner(), role.site().clone()))
                .is_none_or(|original| !Rc::ptr_eq(original, role))
        {
            return Err(freeze("source-loop/original-role-membership"));
        }
        proof.validate_v2(context, function, bindings, coordinates, finishing)?;
        self.validate_frames_v2(&proof.graph, context, function, finishing)
            .map(drop)
    }
    pub(super) fn require_completed(&self) -> Result<(), String> {
        self.completion
            .get()
            .map(|_| ())
            .ok_or_else(|| freeze("source-loop/physical-closure-pending"))
    }
}

fn source_call_bool_prefix_pair_v2(
    call: &LifecyclePrefixSourceV1,
    condition: &LifecyclePrefixSourceV1,
) -> bool {
    let (
        LifecyclePrefixSourceV1::Call {
            site: call_site,
            statement,
        },
        LifecyclePrefixSourceV1::BoolCondition {
            site: condition_site,
            frame,
        },
    ) = (call, condition)
    else {
        return false;
    };
    call_site == condition_site
        && call_site.owner() == frame.source().owner()
        && statement == frame.source().root().statement()
        && frame
            .source()
            .nodes()
            .first()
            .is_some_and(|root| root.site() == call_site.site())
        && frame
            .calls()
            .iter()
            .any(|observation| observation.site() == call_site)
}

#[derive(Clone)]
pub(super) enum LifecyclePrefixDestinationV2 {
    Root,
    Loop(Rc<EmittedSourceLoopProjectionV2>),
}
impl LifecyclePrefixDestinationV2 {
    pub(super) fn expected(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<LifecyclePrefixSourceV1>, String> {
        match self {
            Self::Root => ledger.expected_local_call_binding_sources(owner),
            Self::Loop(packet) => Ok(packet.expected.clone()),
        }
    }
    pub(super) fn snapshot(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
    ) -> Vec<RootLifecyclePrefixBindingGroupV1> {
        match self {
            Self::Root => ledger
                .root_local_call_bindings
                .borrow()
                .get(&owner)
                .cloned()
                .unwrap_or_default(),
            Self::Loop(packet) => packet.snapshot(),
        }
    }

    pub(super) fn allows_same_site_neighbor_v2(
        &self,
        expected: &[LifecyclePrefixSourceV1],
        groups: &[RootLifecyclePrefixBindingGroupV1],
        group: &RootLifecyclePrefixBindingGroupV1,
    ) -> bool {
        let Self::Loop(_) = self else { return false };
        let Some(previous_index) = groups.len().checked_sub(1) else {
            return false;
        };
        let (Some(previous_source), Some(current_source), Some(previous_group)) = (
            expected.get(previous_index),
            expected.get(groups.len()),
            groups.get(previous_index),
        ) else {
            return false;
        };
        previous_group.site() == group.site()
            && previous_source.site() == group.site()
            && current_source.site() == group.site()
            && previous_group.matches_source(previous_source)
            && group.matches_source(current_source)
            && source_call_bool_prefix_pair_v2(previous_source, current_source)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Borrow the argument frame retained by the active source Loop for one
    /// nested call. Loop conditions keep these frames in the Loop summary,
    /// outside the completed root-flow inventory.
    pub(in crate::mir) fn source_loop_argument_expression_frame_v2(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<Option<Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>>, String> {
        CallValidationContextV1::Lowering(self).with_local_call_group_tree_v2(call.owner(), |groups| {
            let mut selected = None;
            for parent in groups.iter().filter_map(|group| group.source_loop_rc()) {
                let product = parent.product();
                if !product.owns_call_v2(call) { continue; }
                for frame in product.argument_frames_v2() {
                    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                        call: original, ordinal: original_ordinal, ..
                    } = frame.source().root() else { continue };
                    let Some(argument) = original.arguments().get(*original_ordinal as usize) else {
                        return Err(freeze("source-loop/argument-frame-identity"));
                    };
                    let exact = original.owner() == call.owner()
                        && original.site() == call.site()
                        && *original_ordinal == ordinal
                        && argument.ordinal() == ordinal
                        && argument.site() == site
                        && frame.source().owner() == call.owner()
                        && frame.source().nodes().first().is_some_and(|root| root.site() == site)
                        && Rc::ptr_eq(frame.source_loan().plan(), frame.source())
                        && frame.evaluation().reference_coverage_is_exact()
                        && product.owns_expression_v2(frame.source());
                    if !exact { continue; }
                    if selected.is_some() {
                        return Err(freeze("argument-expression/root-flow-ambiguous"));
                    }
                    selected = Some(Rc::clone(frame));
                }
            }
            Ok(selected)
        })
    }

    /// Borrow a CallSlot packet that the same source Loop already emitted for
    /// the direct call used as a BoolCondition value. The frame/product pair,
    /// adjacent lifecycle sources, schedule slot, and child group must all
    /// identify that one source call; nested or unrelated calls keep their own
    /// normal condition emission path.
    pub(in crate::mir) fn source_loop_bool_call_effect_v2(
        &self,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>,
        observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
    ) -> Result<Option<RootLifecyclePrefixBindingGroupV1>, String> {
        let source = frame.source();
        let Some(root) = source.nodes().first() else {
            return Err(freeze("source-loop/bool-call-root-missing"));
        };
        if observation.owner() != source.owner() || observation.site().site() != root.site() {
            return Ok(None);
        }
        CallValidationContextV1::Lowering(self).with_local_call_group_tree_v2(source.owner(), |groups| {
            let mut parents = groups.iter().filter_map(|group| group.source_loop_rc()).filter(|parent| {
                parent.product().schedule().frames().iter().any(|scheduled| {
                    matches!(scheduled.prefix(), LifecyclePrefixSourceV1::BoolCondition { frame: original, .. }
                        if Rc::ptr_eq(original, frame))
                })
            });
            let Some(parent) = parents.next() else { return Ok(None) };
            if parents.next().is_some() {
                return Err(freeze("source-loop/bool-call-parent-duplicate"));
            }
            let statement = source.root().statement();
            let mut call_sources = parent.expected.iter().filter(|candidate| matches!(candidate,
                LifecyclePrefixSourceV1::Call { site, statement: original }
                    if site == observation.site() && original == statement));
            let call_source = call_sources.next();
            if call_sources.next().is_some() {
                return Err(freeze("source-loop/bool-call-source-duplicate"));
            }
            let mut condition_sources = parent.expected.iter().filter(|candidate| matches!(candidate,
                LifecyclePrefixSourceV1::BoolCondition { site, frame: original, .. }
                    if site == observation.site() && Rc::ptr_eq(original, frame)));
            let condition_source = condition_sources.next();
            if condition_sources.next().is_some() {
                return Err(freeze("source-loop/bool-call-frame-duplicate"));
            }
            let (Some(call_source), Some(condition_source)) = (call_source, condition_source) else {
                return Ok(None);
            };
            if !source_call_bool_prefix_pair_v2(call_source, condition_source)
                || !frame.calls().iter().any(|call| call == observation)
            {
                return Err(freeze("source-loop/bool-call-source-drift"));
            }
            let mut scheduled_calls = parent.product().schedule().calls_v2().filter(|step| matches!(step,
                crate::mir::loop_recipe_contract::SourceLoopScheduledStepV2::Call {
                    site: original_statement, observation: original, ..
                } if original_statement == statement && original == observation));
            if scheduled_calls.next().is_none() || scheduled_calls.next().is_some() {
                return Err(freeze("source-loop/bool-call-schedule"));
            }
            let children = parent.snapshot();
            let mut effects = children.iter().filter(|group| group.matches_source(call_source));
            let effect = effects.next().ok_or_else(|| freeze("source-loop/bool-call-effect-missing"))?;
            if effects.next().is_some() {
                return Err(freeze("source-loop/bool-call-effect-duplicate"));
            }
            if effect.lexical().is_none() {
                return Err(freeze("source-loop/bool-call-packet-missing"));
            }
            Ok(Some(effect.clone()))
        })
    }

    /// Borrow the existing child group from this exact open Loop parent. The
    /// sole writer already committed it; no second packet or registry is issued.
    pub(in crate::mir) fn source_loop_effect_group_v2(
        &self,
        product: &Rc<Product>,
        site: &OwnedExprSiteV1,
    ) -> Result<RootLifecyclePrefixBindingGroupV1, String> {
        CallValidationContextV1::Lowering(self).with_local_call_group_tree_v2(
            site.owner(),
            |groups| {
                let mut parents = groups
                    .iter()
                    .filter_map(|group| group.source_loop_rc())
                    .filter(|parent| Rc::ptr_eq(parent.product(), product));
                let parent = parents
                    .next()
                    .ok_or_else(|| freeze("source-loop/parent-missing"))?;
                if parents.next().is_some() {
                    return Err(freeze("source-loop/parent-duplicate"));
                }
                let children = parent.snapshot();
                let mut sources = parent.expected.iter().filter(|source| {
                    source.site() == site
                        && matches!(
                            source,
                            LifecyclePrefixSourceV1::Call { .. }
                                | LifecyclePrefixSourceV1::OwnedArrayAppend { .. }
                        )
                });
                let source = sources
                    .next()
                    .ok_or_else(|| freeze("source-loop/frame-source"))?;
                if sources.next().is_some() {
                    return Err(freeze("source-loop/frame-source"));
                }
                let mut matching = children
                    .iter()
                    .filter(|group| group.site() == site && group.matches_source(source));
                let group = matching
                    .next()
                    .ok_or_else(|| freeze("source-loop/frame-source"))?;
                if matching.next().is_some() {
                    return Err(freeze("source-loop/frame-source"));
                }
                Ok(group.clone())
            },
        )
    }

    /// Open at the sole selected Loop entry, before its condition/body descend.
    /// Reopening or attaching a reissued equal product is never a retry.
    pub(in crate::mir) fn open_source_loop_prefix_v2(
        &self,
        product: Rc<Product>,
    ) -> Result<(), String> {
        let owner = product.source.source.owner();
        let packet = EmittedSourceLoopProjectionV2::pending(product)?;
        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
            eprintln!(
                "[local-call-bindings] open-loop owner={owner:?} expected={}",
                packet.expected.len()
            );
        }
        self.prepare_local_call_binding_group(
            owner,
            RootLifecyclePrefixBindingGroupV1::new_loop_pending(packet),
        )?
        .commit()?;
        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
            let roots = self
                .root_local_call_bindings
                .borrow()
                .get(&owner)
                .cloned()
                .unwrap_or_default();
            eprintln!(
                "[local-call-bindings] roots-after-loop={:?}",
                roots
                    .iter()
                    .map(RootLifecyclePrefixBindingGroupV1::site)
                    .collect::<Vec<_>>()
            );
        }
        Ok(())
    }

    pub(super) fn prefix_destination_v2(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
    ) -> Result<LifecyclePrefixDestinationV2, String> {
        let roots = self
            .root_local_call_bindings
            .borrow()
            .get(&owner)
            .cloned()
            .unwrap_or_default();
        let mut matching = roots
            .iter()
            .filter_map(|group| group.source_loop_rc())
            .filter(|packet| packet.expected.iter().any(|source| source.site() == site));
        let destination = matching.next().cloned();
        if matching.next().is_some() {
            return Err(freeze("source-loop/child-owner-duplicate"));
        }
        Ok(destination
            .map(LifecyclePrefixDestinationV2::Loop)
            .unwrap_or(LifecyclePrefixDestinationV2::Root))
    }

    pub(super) fn next_prefix_source_v2(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<LifecyclePrefixSourceV1>, String> {
        let destination = self.prefix_destination_v2(owner, site)?;
        let count = destination.snapshot(self, owner).len();
        Ok(destination.expected(self, owner)?.get(count).cloned())
    }

    pub(super) fn prepare_prefix_at_destination_v2(
        &self,
        owner: FunctionOwnerIdV1,
        group: RootLifecyclePrefixBindingGroupV1,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        let site = group.site();
        if site.owner() != owner {
            return Err(freeze("local-call-binding-owner-drift"));
        }
        let destination = self.prefix_destination_v2(owner, site)?;
        let expected = destination.expected(self, owner)?;
        let groups = destination.snapshot(self, owner);
        if groups.iter().any(|recorded| recorded.site() == site)
            && !destination.allows_same_site_neighbor_v2(&expected, &groups, &group)
        {
            return Err(freeze("duplicate-local-call-bindings"));
        }
        if groups.len() >= expected.len() {
            return Err(freeze("local-call-bindings-overflow"));
        }
        let next_source = expected.get(groups.len());
        if next_source.is_none_or(|source| !group.matches_source(source)) {
            return Err(freeze("local-call-binding-site-order"));
        }
        Ok(PreparedLifecyclePrefixBindingGroupV1 {
            ledger: self,
            owner,
            group,
            destination,
            preceding_sites: groups
                .iter()
                .map(|recorded| recorded.site().clone())
                .collect(),
        })
    }
}

/// Snapshot shared packet references before callbacks. Neither the root pool
/// nor a child's collection RefCell remains borrowed during recursive checks.
pub(super) fn snapshot_group_tree_v2(
    roots: &[RootLifecyclePrefixBindingGroupV1],
) -> Vec<RootLifecyclePrefixBindingGroupV1> {
    let mut output = Vec::new();
    let mut pending: Vec<_> = roots.iter().rev().cloned().collect();
    while let Some(group) = pending.pop() {
        if let Some(packet) = group.source_loop_rc() {
            pending.extend(packet.snapshot().into_iter().rev());
        }
        output.push(group);
    }
    output
}

impl CallValidationContextV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn require_source_loop_parent_v2(
        self,
        product: &std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        completed: bool,
    ) -> Result<(), String> {
        self.with_local_call_group_tree_v2(product.source.source.owner(), |groups| {
            let mut packets = groups
                .iter()
                .filter_map(|group| group.source_loop_rc())
                .filter(|packet| std::rc::Rc::ptr_eq(packet.product(), product));
            let packet = packets.next().ok_or_else(|| {
                format!(
                    "{} owner={:?} phase={} groups={}",
                    freeze("source-loop/original-group-membership"),
                    product.source.source.owner(),
                    if matches!(self, Self::Finalized(_)) {
                        "Finalized"
                    } else {
                        "Lowering"
                    },
                    groups.len()
                )
            })?;
            if packets.next().is_some() {
                return Err(freeze("source-loop/original-group-membership"));
            }
            if completed {
                packet.require_completed()?;
            }
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
pub(in crate::mir) use physical::{
    PreparedSourceLoopPhysicalCompletionV2, SourceLoopPhysicalProofV2,
};
