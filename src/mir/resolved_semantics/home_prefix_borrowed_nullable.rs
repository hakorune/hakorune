//! Ownership-free nullable call result in the existing prefix state.
use super::*;
use crate::mir::resolved_semantics::SourceBindingSiteV1;
use crate::mir::normal_callable_semantic_package::{BorrowedNullableLocalSourceLoanV1, BorrowedLifetimeAnchorV1};
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

/// Branch-local read validity retains the same immutable source loan even
/// after interference. Invalidating it must never create a generic BoundValue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BorrowedNullableStateV1 {
    origin: BorrowedReadOriginV1,
    read_valid: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum BorrowedReadOriginV1 {
    Call(Box<BorrowedNullableLocalSourceLoanV1>),
    Flow(std::rc::Rc<super::borrowed_flow::BorrowedReadFlowV1>),
}
impl BorrowedNullableStateV1 {
    pub(super) fn single_loan(&self) -> Option<&BorrowedNullableLocalSourceLoanV1> {
        match &self.origin { BorrowedReadOriginV1::Call(loan) => Some(loan), BorrowedReadOriginV1::Flow(_) => None }
    }
    pub(super) fn flow_source(&self) -> Option<&super::borrowed_flow::BorrowedReadFlowV1> {
        match &self.origin { BorrowedReadOriginV1::Flow(flow) => Some(flow.as_ref()), _ => None }
    }
    pub(super) fn loans(&self) -> impl Iterator<Item = &BorrowedNullableLocalSourceLoanV1> {
        self.single_loan().into_iter().chain(self.flow_source().into_iter().flat_map(|source| source.loans()))
    }
    pub(super) fn from_flow(source: super::borrowed_flow::BorrowedReadFlowV1, read_valid: bool) -> Self {
        Self { origin: BorrowedReadOriginV1::Flow(std::rc::Rc::new(source)), read_valid }
    }
    pub(super) fn read_valid(&self) -> bool { self.read_valid }
    fn is_declared_read_v1(&self) -> bool { self.loans().all(|loan| loan.is_declared_read_v1()) }
    pub(super) fn legacy_observation_v1(&self, binding: BindingRefV1) -> Option<OrdinaryObservation> {
        self.single_loan().filter(|loan| !loan.is_declared_read_v1()).map(|_| OrdinaryObservation::BoundValue(binding))
    }
}

/// Identical original provenance meets validity; distinct Calls require the
/// original flow join and explicit caller-support correspondence.
pub(super) fn meet_read_state_v1(own: &mut StoredLocal, other: &StoredLocal) -> bool {
    let (StoredLocal::BorrowedNullable(own), StoredLocal::BorrowedNullable(other)) = (own, other) else { return false; };
    if !own.is_declared_read_v1() || own.origin != other.origin { return false; }
    own.read_valid &= other.read_valid;
    true
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn install_received_mixed(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1,
        calls: &[super::super::LocalCallObservationV1],
        covered: &std::collections::BTreeSet<OwnedExprSiteV1>,
        homes: &[LiveHomeV1],
    ) -> bool {
        if !self.mixed_anchor_live_v1(&loan, Some((calls, covered, homes))) {
            return false;
        }
        self.store(
            loan.destination(),
            StoredLocal::ReceivedMixed(Box::new(loan)),
        );
        true
    }

    /// A received Mixed is observable only while its original support chain
    /// remains live. Rebinding/consuming any anchor invalidates later uses.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_received_mixed_v1(
        &self,
        binding: BindingRefV1,
    ) -> bool {
        let Some(StoredLocal::ReceivedMixed(loan)) = self.locals.get(&binding) else {
            return false;
        };
        loan.destination() == binding && self.mixed_anchor_live_v1(loan, None)
    }

    fn mixed_anchor_live_v1(
        &self,
        loan: &crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1,
        original_calls: Option<(
            &[super::super::LocalCallObservationV1],
            &std::collections::BTreeSet<OwnedExprSiteV1>,
            &[LiveHomeV1],
        )>,
    ) -> bool {
        let mut current = loan;
        let mut seen = std::collections::BTreeSet::new();
        loop {
            let binding = current.support();
            if binding.owner() != self.input.owner() || !seen.insert(binding) {
                return false;
            }
            if current.formal_support_v1().is_some() {
                return self.is_self_rooted_handle(binding);
            }
            let (Some(acquisition), Some(declaration)) = (
                current.acquired_call_v1(),
                current.acquired_declaration_v1(),
            ) else {
                return false;
            };
            let (kind, parent) = match self.locals.get(&binding) {
                Some(StoredLocal::ReceivedNullable {
                    acquisition: original,
                }) if original == acquisition => {
                    (super::super::LocalCallResultClassV1::Nullable, None)
                }
                Some(StoredLocal::ReceivedMixed(parent))
                    if parent.call() == acquisition
                        && parent.destination() == binding
                        && parent.declaration() == declaration =>
                {
                    (
                        super::super::LocalCallResultClassV1::MixedObject,
                        Some(parent.as_ref()),
                    )
                }
                _ => return false,
            };
            if let Some((calls, covered, homes)) = original_calls {
                let mut rows = calls.iter().filter(|row| {
                    row.site() == acquisition
                        && row.owner() == self.input.owner()
                        && row.local_binding() == Some((declaration, binding))
                });
                let Some(row) = rows.next() else {
                    return false;
                };
                if rows.next().is_some()
                    || row.result() != kind
                    || !covered.contains(acquisition)
                    || !homes.iter().any(|home| home.binding() == binding)
                    || parent.is_some_and(|parent| row.mixed() != Some(parent))
                {
                    return false;
                }
            }
            match parent {
                Some(parent) => current = parent,
                None => {
                    return self
                        .received_call_acquisition(binding)
                        .is_some_and(|(original, nullable)| nullable && original == acquisition)
                }
            }
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_stored_get_v1(
        &mut self,
        frame: std::rc::Rc<super::super::MapFieldGetLocalV1>,
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> bool {
        if !std::ptr::eq(input.forest(), self.input.forest())
            || !std::ptr::eq(input.function(), self.input.function())
            || !matches!((input.body_shape(), self.input.body_shape()), (Some(a), Some(b)) if std::ptr::eq(a, b))
            || !frame.matches_input(input) { return false; }
        self.store(frame.binding(), StoredLocal::StoredMapGet(frame));
        true
    }

    /// Borrow the same original stored get on the current path, without
    /// inventing an entry/scalar view or granting a non-null observation.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn stored_get_reference_v1(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<std::rc::Rc<super::super::MapFieldGetLocalV1>> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        let StoredLocal::StoredMapGet(frame) = self.locals.get(&binding)? else {
            return None;
        };
        (binding == frame.binding()
            && self.is_self_rooted_handle(frame.source().receiver().receiver()))
        .then(|| std::rc::Rc::clone(frame))
    }

    /// Borrow a Dynamic local only through the exact initializer frame that
    /// produced it. This preserves the opaque Dynamic class and its lifecycle
    /// owner; it does not make the value a scalar, handle, or tagged SUM.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn dynamic_invocation_result_reference_v1(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<std::rc::Rc<super::super::CheckedInitializerExpressionFrameV1>> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        let StoredLocal::DynamicInvocationResult { frame } = self.locals.get(&binding)? else {
            return None;
        };
        let flow = self.input.function().expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let definition = frame.source().nodes().first()?.site();
        let origin = flow.alias_origin_with_expression_boundary(value,
            |site| self.input.function().variable_ref(site), |site| site == definition)?;
        if !matches!(flow.origin(origin), Some(crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(site))
            if site == definition) { return None; }
        let owner = frame.dynamic_home_binding_v1()?;
        if !matches!(self.locals.get(&owner), Some(StoredLocal::DynamicInvocationResult { frame })
            if frame.binding() == owner && frame.is_dynamic_invocation_result()) {
            return None;
        }
        (frame.is_dynamic_value()
            && frame.source().owner() == binding.owner()
            && frame.binding() == binding)
            .then(|| std::rc::Rc::clone(frame))
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_borrowed_nullable(
        &mut self,
        loan: BorrowedNullableLocalSourceLoanV1,
    ) -> bool {
        if !self.borrowed_call_support_available_v1(&loan) { return false; }
        if loan.is_declared_read_v1() {
            // The outer Call completes this single initializer after all child
            // Calls. Multiple LOCAL initializers need finer event ordering.
            let SourceBindingSiteV1::Local { statement, .. } = loan.declaration() else { return false; };
            if self.input.function().expression_source().initializers().filter(|row|
                matches!(row.declaration_site(), SourceBindingSiteV1::Local { statement: original, .. }
                    if original == statement)).count() != 1 { return false; }
        }
        self.store(
            loan.destination(),
            StoredLocal::BorrowedNullable(BorrowedNullableStateV1 { origin: BorrowedReadOriginV1::Call(Box::new(loan)), read_valid: true }),
        );
        true
    }

    /// Current read authority is separate from the retained lifetime anchor.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn borrowed_call_support_available_v1(
        &self, loan: &crate::mir::normal_callable_semantic_package::BorrowedCallResultSourceLoanV1,
    ) -> bool {
        if let Some(actual) = loan.reborrowed_read_v1() {
            if !matches!(self.nullable_result_reference_v1(actual.read().site()),
                Some(super::super::NullableResultReferenceV1::DeclaredFrom(current)) if actual.matches_value(&current)) { return false; }
        }
        if let Some(actual) = loan.reborrowed_read_v1() {
            if !actual.loans().all(|inner| self.declared_read_support_live_v1(inner)) { return false; }
        } else {
            if let Some((root, acquisition, read)) = loan.owned_local_support_v1() {
                if self.home_acquisition(root) != Some(acquisition)
                    || self.direct_available_home(read.site()) != Some((root, acquisition)) { return false; }
            }
            if loan.entry_support_v1().is_some() && !self.borrowed_entry_support_live_v1(&loan) { return false; }
        }
        true
    }

    /// A receiver observation consumes the current prefix read permission;
    /// signature/class discovery alone cannot lend this capability.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn declared_read_call_receiver_v1(
        &self, call: &OwnedExprSiteV1, prefix_known: bool,
    ) -> Option<super::super::DeclaredReadReceiverV1> {
        if !prefix_known || call.owner() != self.input.owner() { return None; }
        let receiver = self.input.function().method_call_source_v1(call.site())?.receiver_site();
        let super::super::NullableResultReferenceV1::DeclaredFrom(view) = self.nullable_result_reference_v1(receiver)? else { return None; };
        let exclusion = if view.payload_nonnull() { None } else {
            if !self.nonnull.contains(&view.binding()) { return None; }
            let proof = self.null_exclusions.get(&view.binding()).or_else(|| self.null_exclusions.get(&view.original_binding()))?;
            if !proof.corroborates_result_read_v1(self.input, &view) { return None; }
            Some(std::rc::Rc::clone(proof))
        };
        Some(super::super::DeclaredReadReceiverV1::issue_v1(view, exclusion))
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn terminal_receiver_v1(&self, site: &SourceExprSiteV1) -> Option<BindingRefV1> {
        let crate::mir::resolved_semantics::BodyExpressionShapeV1::Me { receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding), .. } = self.input.body_shape()?.expression_shape(site)? else { return None; };
        let entry = self.entry_receiver.as_ref()?;
        (entry.binding == *binding && entry.owner == self.input.owner() && self.is_self_rooted_handle(*binding)
            && self.input.function().declaration_binding(&SourceBindingSiteV1::Receiver) == Some(*binding)).then_some(*binding)
    }

    /// Select original read provenance even after interference; selection does
    /// not grant current liveness or non-null permission.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn has_declared_read_origin_v1(&self, binding: BindingRefV1) -> bool {
        matches!(self.locals.get(&binding), Some(StoredLocal::BorrowedNullable(state)) if state.is_declared_read_v1())
    }

    fn borrowed_entry_support_live_v1(&self, loan: &crate::mir::normal_callable_semantic_package::BorrowedCallResultSourceLoanV1) -> bool {
        let Some((root, support, actual)) = loan.entry_support_v1() else { return false; };
        match support {
            crate::mir::instruction::BorrowedResultSupportV1::Receiver => self.terminal_receiver_v1(actual.site()) == Some(root),
            crate::mir::instruction::BorrowedResultSupportV1::Formal(_) => loan.root() == Some(root) && self.is_self_rooted_handle(root),
            _ => false,
        }
    }

    /// A read keeps the original support kind; a caller entry is not a Home.
    pub(super) fn declared_read_support_live_v1(&self, loan: &crate::mir::normal_callable_semantic_package::BorrowedCallResultSourceLoanV1) -> bool {
        if let Some(actual) = loan.reborrowed_read_v1() {
            return actual.loans().all(|inner| self.declared_read_support_live_v1(inner));
        }
        match loan.lifetime_anchor_v1() {
            Some(BorrowedLifetimeAnchorV1::Owned { root, acquisition }) => self.home_acquisition(root) == Some(&acquisition),
            _ => self.borrowed_entry_support_live_v1(loan),
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_declared_read_alias_v1(
        &mut self, binding: BindingRefV1, site: &SourceExprSiteV1,
    ) -> bool {
        let Some(ResolvedLexicalRefV1::Local(original)) = self.input.function().variable_ref(site) else { return false; };
        let Some(StoredLocal::BorrowedNullable(state)) = self.locals.get(&original) else { return false; };
        let matches = match &state.origin {
            BorrowedReadOriginV1::Call(loan) => super::super::BorrowedResultReadV1::issue_v1(self.input, loan, site).is_some(),
            BorrowedReadOriginV1::Flow(source) => source.matches_read(self.input, site),
        };
        if !state.read_valid || !matches { return false; }
        let live = state.loans().all(|loan| self.declared_read_support_live_v1(loan));
        if !live || binding.owner() != self.input.owner() { return false; }
        let state = state.clone();
        let exclusion = self.null_exclusions.get(&original).cloned();
        let nonnull = self.nonnull.contains(&original);
        self.store(binding, StoredLocal::BorrowedNullable(state));
        if nonnull { self.nonnull.insert(binding); }
        if let Some(exclusion) = exclusion { self.null_exclusions.insert(binding, exclusion); }
        true
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn declared_read_rebind_v1(
        &self, binding: BindingRefV1, site: &SourceExprSiteV1, homes: &[LiveHomeV1],
    ) -> Option<super::super::BorrowedResultReadV1> {
        let StoredLocal::BorrowedNullable(previous) = self.locals.get(&binding)? else { return None; };
        if binding.owner() != self.input.owner() || !previous.is_declared_read_v1()
            || homes.iter().any(|home| home.binding() == binding) { return None; }
        let super::super::NullableResultReferenceV1::DeclaredFrom(read) = self.nullable_result_reference_v1(site)? else { return None; };
        let read = read.single()?.clone();
        (previous.single_loan()?.declared_read_result_v1()?.declared_type() == read.source().declared_read_result_v1()?.declared_type()).then_some(read)
    }

    /// One nullable comparison source, with the original read/lifetime law.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn nullable_result_reference_v1(
        &self, site: &SourceExprSiteV1,
    ) -> Option<super::super::NullableResultReferenceV1> {
        use super::super::NullableResultReferenceV1 as Reference;
        if let Some(frame) = self.stored_get_reference_v1(site) { return Some(Reference::StoredMapGet(frame)); }
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else { return None; };
        let StoredLocal::BorrowedNullable(state) = self.locals.get(&binding)? else { return None; };
        if !state.read_valid || !state.is_declared_read_v1()
            || !state.loans().all(|loan| self.declared_read_support_live_v1(loan)) { return None; }
        let read = match &state.origin {
            BorrowedReadOriginV1::Call(loan) => super::super::DeclaredReadValueV1::Single(
                super::super::BorrowedResultReadV1::issue_v1(self.input, loan, site)?),
            BorrowedReadOriginV1::Flow(source) => super::super::DeclaredReadValueV1::Flow(
                super::borrowed_flow::BorrowedFlowReadV1::issue(self.input, std::rc::Rc::clone(source), site)?),
        };
        Some(Reference::DeclaredFrom(read))
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_declared_read_binding_v1(&self, binding: BindingRefV1) -> bool {
        matches!(self.locals.get(&binding), Some(StoredLocal::BorrowedNullable(state)) if state.is_declared_read_v1())
    }

    /// Apply an admitted write after RHS evaluation. Independent live New
    /// roots are disjoint; fields/unknown receivers are not guessed disjoint.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn invalidate_declared_read_write_v1(
        &mut self, receiver: &SourceExprSiteV1,
    ) {
        let root = self.input.function().variable_ref(receiver).and_then(|reference| match reference {
            ResolvedLexicalRefV1::Local(binding) => match self.locals.get(&binding) {
                Some(StoredLocal::Home { acquisition, .. }) => Some((binding, acquisition.clone())),
                Some(StoredLocal::Handle(root)) => self.home_acquisition(*root).map(|site| (*root, site.clone())),
                _ => None,
            },
            _ => None,
        });
        for value in self.locals.values_mut() {
            let StoredLocal::BorrowedNullable(state) = value else { continue; };
            if !state.is_declared_read_v1() { continue; }
            let independent = state.loans().all(|loan| matches!((loan.lifetime_anchor_v1(), root.as_ref()),
                (Some(BorrowedLifetimeAnchorV1::Owned { root: support, acquisition }), Some((written, site))) if support != *written && acquisition != *site));
            if !independent { state.read_valid = false; }
        }
    }

    /// New LOCAL loans become live only after their initializer's outer Call.
    /// Other current loans cannot retain permission across an unknown effect.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn invalidate_declared_read_call_v1(
        &mut self, statement: &SourceStmtSiteV1,
    ) {
        for value in self.locals.values_mut() {
            let StoredLocal::BorrowedNullable(state) = value else { continue; };
            if state.is_declared_read_v1()
                && !state.loans().all(|loan| matches!(loan.declaration(), SourceBindingSiteV1::Local { statement: original, .. }
                    if original == statement)) {
                state.read_valid = false;
            }
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn mark_borrowed_nullable_nonnull(
        &mut self,
        binding: BindingRefV1,
    ) {
        if matches!(
            self.locals.get(&binding),
            Some(StoredLocal::BorrowedNullable(_) | StoredLocal::StoredMapGet(_))
        ) {
            self.mark_nonnull(binding);
        }
    }

    /// Exact stored Map borrower on the current source path. This lends no
    /// generic Handle observation and never substitutes the holder as payload.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn borrowed_map_receiver_v1(
        &self,
        site: &SourceExprSiteV1,
        loan: &BorrowedNullableLocalSourceLoanV1,
    ) -> bool {
        self.input.function().variable_ref(site)
            == Some(ResolvedLexicalRefV1::Local(loan.destination()))
            && loan.is_map_slot()
            && loan.destination().owner() == self.input.owner()
            && self.is_self_rooted_handle(loan.support_root())
            && matches!(self.locals.get(&loan.destination()), Some(StoredLocal::BorrowedNullable(same)) if same.single_loan() == Some(loan))
    }

    /// Retain exact local Return evidence: legacy needs its narrowed path;
    /// explicit read needs valid support. The result owner separately checks
    /// the original outer From declaration before admitting a read re-return.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn borrowed_nullable_return(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<super::super::BorrowedNullableReturnV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else { return None; };
        let StoredLocal::BorrowedNullable(state) = self.locals.get(&binding)? else { return None; };
        let loan = state.single_loan()?;
        if loan.is_declared_read_v1() {
            if !state.read_valid || !matches!(loan.lifetime_anchor_v1(), Some(BorrowedLifetimeAnchorV1::Entry { .. }))
                || !self.declared_read_support_live_v1(loan) { return None; }
            return super::super::BorrowedResultReadV1::issue_v1(self.input, loan, site)
                .map(super::super::BorrowedNullableReturnV1::Declared);
        }
        if !self.borrowed_entry_support_live_v1(loan) { return None; }
        (loan.destination() == binding && self.nonnull.contains(&binding))
            .then(|| super::super::BorrowedNullableReturnV1::Legacy(Box::new(loan.clone())))
    }
}
