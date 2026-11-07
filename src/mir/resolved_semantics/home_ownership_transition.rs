//! The one ownership-transition type of the caller-prefix walk (S3b-R1..R3, R8).
//! `HomesV1` is the list of responsibilities a Fault must still close; only the
//! operations here and in the other `local_flow` files change it, always together
//! with the stored-local state that says whether a name is still usable.
use super::*;
use std::ops::Deref;

/// Responsibilities still owed on the current path, in acquisition order.
/// The mutators are visible to the `local_flow` files only; observers go through
/// the `PrefixLocalFlow` operations below.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct HomesV1(Vec<LiveHomeV1>);

impl Deref for HomesV1 {
    type Target = [LiveHomeV1];
    fn deref(&self) -> &[LiveHomeV1] {
        &self.0
    }
}

impl HomesV1 {
    pub(crate) fn entry(homes: Vec<LiveHomeV1>) -> Self {
        Self(homes)
    }
    /// A path-local copy of the current list; later changes stay on the copy until `commit`.
    pub(crate) fn snapshot(homes: &[LiveHomeV1]) -> Self {
        Self(homes.to_vec())
    }
    /// S3b-R3a: the list a Fault edge closes, newest acquisition first.
    pub(crate) fn fault_order(&self) -> Box<[LiveHomeV1]> {
        self.0.iter().rev().cloned().collect()
    }
    /// S3b-R3a: the list a Normal edge keeps once `moved` have left the frame.
    pub(crate) fn normal_order_after(&self, moved: &[BindingRefV1]) -> Box<[LiveHomeV1]> {
        self.0.iter().rev().filter(|home| !moved.contains(&home.binding())).cloned().collect()
    }
    /// Ordered intersection of two fall-through paths: a responsibility survives a join
    /// only when both paths still hold it, in the shared order.
    pub(crate) fn join(then: &Self, other: &Self) -> Self {
        Self(then.0.iter().filter(|home| other.0.contains(home)).cloned().collect())
    }
    pub(crate) fn into_boxed_slice(self) -> Box<[LiveHomeV1]> {
        self.0.into_boxed_slice()
    }
    /// Adopt a list computed by a transition on a snapshot or by a join.
    pub(crate) fn commit(&mut self, next: Self) {
        *self = next;
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn install(&mut self, home: LiveHomeV1) {
        self.0.push(home);
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn release(&mut self, binding: BindingRefV1) {
        self.0.retain(|home| home.binding() != binding);
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn release_all(&mut self, bindings: &[BindingRefV1]) {
        self.0.retain(|home| !bindings.contains(&home.binding()));
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn remove_at(&mut self, index: usize) {
        self.0.remove(index);
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn move_at(&mut self, index: usize, moved: LiveHomeV1) {
        self.0[index] = moved;
    }
}

impl<'source> PrefixLocalFlow<'source> {
    /// S3b-R2b: a local move or share keeps its responsibility only when the source,
    /// the destination and the statement sit in one lexical scope.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn same_scope_v1(
        &self,
        destination: BindingRefV1,
        source: BindingRefV1,
        statement: &SourceStmtSiteV1,
    ) -> bool {
        let function = self.input.function();
        let scope = function.binding(destination).map(|row| row.owner_scope());
        scope.is_some()
            && function.binding(source).map(|row| row.owner_scope()) == scope
            && function.exact_scope_containing(statement.node()) == scope
    }

    /// S3b-R2a: a selected `new` acquires its Home on the Normal successor only.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_selected_home(
        &mut self,
        homes: &mut HomesV1,
        binding: BindingRefV1,
        acquisition: OwnedExprSiteV1,
        construction_source: Option<
            std::rc::Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>,
        >,
    ) {
        homes.install(LiveHomeV1::Stable(binding));
        self.install_selected_normal_home(binding, acquisition, construction_source);
    }

    /// S3b-R2a: a received call result is owned by the caller from the call site.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_received_handle(
        &mut self,
        homes: &mut HomesV1,
        binding: BindingRefV1,
        acquisition: &OwnedExprSiteV1,
    ) {
        homes.install(LiveHomeV1::Stable(binding));
        self.install_received_handle(binding, acquisition);
    }

    /// S3b-R2a: a nullable received result is owned only when non-null; the checked release stays.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_received_nullable(
        &mut self,
        homes: &mut HomesV1,
        binding: BindingRefV1,
        acquisition: &OwnedExprSiteV1,
    ) {
        homes.install(LiveHomeV1::Stable(binding));
        self.install_received_nullable(binding, acquisition);
    }

    /// S3b-R2a: a Map built or received on the Normal successor.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_map(
        &mut self,
        homes: &mut HomesV1,
        binding: BindingRefV1,
    ) {
        homes.install(LiveHomeV1::Stable(binding));
        self.install_map(binding);
    }

    /// S3b-R2a: a received Mixed is an outcome-dependent obligation whose own loan must survive End.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_received_mixed(
        &mut self,
        homes: &mut HomesV1,
        loan: crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1,
        calls: &[super::super::LocalCallObservationV1],
        covered: &std::collections::BTreeSet<OwnedExprSiteV1>,
    ) -> bool {
        let binding = loan.destination();
        if !self.install_received_mixed(loan, calls, covered, homes) {
            return false;
        }
        homes.install(LiveHomeV1::Stable(binding));
        true
    }

    /// S3b-R2a: the holder of a named array is a conditional obligation on the construction's Normal successor.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_named_array_holder(
        &mut self,
        homes: &mut HomesV1,
        source: crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1,
    ) {
        let binding = source.binding();
        self.install_named_array_holder(source);
        homes.install(LiveHomeV1::Stable(binding));
    }

    /// S3b-R2a: an opaque Dynamic invocation result keeps its exact frame and lifecycle.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_dynamic_result(
        &mut self,
        homes: &mut HomesV1,
        binding: BindingRefV1,
        frame: std::rc::Rc<super::super::CheckedInitializerExpressionFrameV1>,
    ) {
        self.install_dynamic_invocation_result(binding, frame);
        homes.install(LiveHomeV1::Stable(binding));
    }

    /// S3b-R2a: a Text residence keeps the exact acquisition of its initializer.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn acquire_text_initializer(
        &mut self,
        homes: &mut HomesV1,
        frame: std::rc::Rc<super::super::CheckedInitializerExpressionFrameV1>,
    ) {
        homes.install(LiveHomeV1::Text(frame.text_acquisition_v1()));
        self.install_text_initializer(frame);
    }

    /// S3b-R2b: a rebind replaces the Home of its target; a loop carrier acquires none.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn rebind_text(
        &mut self,
        homes: &mut HomesV1,
        target: BindingRefV1,
        frame: &std::rc::Rc<super::super::CheckedBindingRebindFrameV1>,
    ) {
        homes.release(target);
        if frame.text_loop_carrier_v1().is_none() {
            homes.install(LiveHomeV1::Text(frame.text_acquisition_v1()));
        }
        self.install_text_rebind(std::rc::Rc::clone(frame));
    }

    /// S3b-R2d: arguments of a call or `new` leave the frame on the Normal edge only;
    /// the Fault edge keeps the list it was built from.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn release_to_callee(
        &mut self,
        homes: &mut HomesV1,
        bindings: &[BindingRefV1],
    ) {
        homes.release_all(bindings);
        for binding in bindings {
            self.consume_argument_home(*binding);
        }
    }

    /// S3b-R2f: a Home that is returned or stored leaves the frame at that point.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn transfer_out(
        &mut self,
        homes: &mut HomesV1,
        binding: BindingRefV1,
    ) {
        homes.release(binding);
        self.consume_home(binding);
    }

    /// S3b-R2f with the precondition that the list holds the Home exactly once.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn transfer_out_exactly_one(
        &mut self,
        homes: &mut HomesV1,
        binding: BindingRefV1,
    ) -> Result<(), super::super::HomePrefixUnavailableV1> {
        if homes.iter().filter(|home| home.binding() == binding).count() != 1 {
            return Err(super::super::HomePrefixUnavailableV1::SourceMismatch);
        }
        self.transfer_out(homes, binding);
        Ok(())
    }
}
