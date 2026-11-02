//! Ownership-free nullable call result in the existing prefix state.
use super::*;
use crate::mir::resolved_semantics::SourceBindingSiteV1;
use crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

/// Branch-local read validity retains the same immutable source loan even
/// after interference. Invalidating it must never create a generic BoundValue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BorrowedNullableStateV1 {
    loan: Box<BorrowedNullableLocalSourceLoanV1>,
    read_valid: bool,
}
impl std::ops::Deref for BorrowedNullableStateV1 {
    type Target = BorrowedNullableLocalSourceLoanV1;
    fn deref(&self) -> &Self::Target { &self.loan }
}
impl AsRef<BorrowedNullableLocalSourceLoanV1> for BorrowedNullableStateV1 {
    fn as_ref(&self) -> &BorrowedNullableLocalSourceLoanV1 { &self.loan }
}
impl BorrowedNullableStateV1 {
    pub(super) fn legacy_observation_v1(&self, binding: BindingRefV1) -> Option<OrdinaryObservation> {
        (!self.is_declared_read_v1()).then_some(OrdinaryObservation::BoundValue(binding))
    }
}

/// Compatible branch states meet validity, never loan/source identity.
pub(super) fn meet_read_state_v1(own: &mut StoredLocal, other: &StoredLocal) -> bool {
    let (StoredLocal::BorrowedNullable(own), StoredLocal::BorrowedNullable(other)) = (own, other) else { return false; };
    if !own.is_declared_read_v1() || own.loan != other.loan { return false; }
    own.read_valid &= other.read_valid;
    true
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_received_mixed(
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
        if loan.is_declared_read_v1() {
            // The outer Call completes this single initializer after all child
            // Calls. Multiple LOCAL initializers need finer event ordering.
            let SourceBindingSiteV1::Local { statement, .. } = loan.declaration() else { return false; };
            if self.input.function().expression_source().initializers().filter(|row|
                matches!(row.declaration_site(), SourceBindingSiteV1::Local { statement: original, .. }
                    if original == statement)).count() != 1 { return false; }
        }
        if let Some((root, acquisition, read)) = loan.owned_local_support_v1() {
            if self.home_acquisition(root) != Some(acquisition)
                || self.direct_available_home(read.site()) != Some((root, acquisition)) {
                return false;
            }
        }
        if loan.entry_support_v1().is_some() && !self.borrowed_entry_support_live_v1(&loan) { return false; }
        self.store(
            loan.destination(),
            StoredLocal::BorrowedNullable(BorrowedNullableStateV1 { loan: Box::new(loan), read_valid: true }),
        );
        true
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

    fn borrowed_entry_support_live_v1(&self, loan: &BorrowedNullableLocalSourceLoanV1) -> bool {
        let Some((root, support, actual)) = loan.entry_support_v1() else { return false; };
        match support {
            crate::mir::instruction::BorrowedResultSupportV1::Receiver => self.terminal_receiver_v1(actual.site()) == Some(root),
            crate::mir::instruction::BorrowedResultSupportV1::Formal(_) => loan.root() == Some(root) && self.is_self_rooted_handle(root),
            _ => false,
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_declared_read_alias_v1(
        &mut self, binding: BindingRefV1, site: &SourceExprSiteV1,
    ) -> bool {
        let Some(ResolvedLexicalRefV1::Local(original)) = self.input.function().variable_ref(site) else { return false; };
        let Some(StoredLocal::BorrowedNullable(state)) = self.locals.get(&original) else { return false; };
        if !state.read_valid || super::super::BorrowedResultReadV1::issue_v1(self.input, state.as_ref(), site).is_none() { return false; }
        let live = match state.owned_local_support_v1() {
            Some((root, acquisition, _)) => self.home_acquisition(root) == Some(acquisition),
            None => self.borrowed_entry_support_live_v1(state.as_ref()),
        };
        if !live || binding.owner() != self.input.owner() { return false; }
        let state = state.clone();
        let exclusion = self.null_exclusions.get(&original).cloned();
        let nonnull = self.nonnull.contains(&original);
        self.store(binding, StoredLocal::BorrowedNullable(state));
        if nonnull { self.nonnull.insert(binding); }
        if let Some(exclusion) = exclusion { self.null_exclusions.insert(binding, exclusion); }
        true
    }

    /// One nullable comparison source, with the original read/lifetime law.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn nullable_result_reference_v1(
        &self, site: &SourceExprSiteV1,
    ) -> Option<super::super::NullableResultReferenceV1> {
        use super::super::NullableResultReferenceV1 as Reference;
        if let Some(frame) = self.stored_get_reference_v1(site) { return Some(Reference::StoredMapGet(frame)); }
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else { return None; };
        let StoredLocal::BorrowedNullable(state) = self.locals.get(&binding)? else { return None; };
        let live = match state.owned_local_support_v1() {
            Some((root, acquisition, _)) => self.home_acquisition(root) == Some(acquisition),
            None => self.borrowed_entry_support_live_v1(state.as_ref()),
        };
        (state.is_declared_read_v1() && state.read_valid && live)
            .then(|| super::super::BorrowedResultReadV1::issue_v1(self.input, state.as_ref(), site))
            .flatten().map(Reference::DeclaredFrom)
    }

    /// A returned read reference stays distinct from a general Handle/Home.
    /// Its exact New argument also narrows formal-derived Null, not literal Null.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn declared_read_local_v1(
        &self, binding: BindingRefV1,
    ) -> Option<&BorrowedNullableLocalSourceLoanV1> {
        let StoredLocal::BorrowedNullable(state) = self.locals.get(&binding)? else { return None; };
        let (root, acquisition, _) = state.owned_local_support_v1()?;
        (state.read_valid && state.is_declared_read_v1()
            && self.home_acquisition(root) == Some(acquisition)).then_some(state.as_ref())
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
            let independent = state.owned_local_support_v1().zip(root.as_ref()).is_some_and(
                |((support, acquired, _), (written, site))| support != *written && acquired != site);
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
                && !matches!(state.declaration(), SourceBindingSiteV1::Local { statement: original, .. }
                    if original == statement) {
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
            && matches!(self.locals.get(&loan.destination()), Some(StoredLocal::BorrowedNullable(same)) if same.as_ref() == loan)
    }

    /// Retain exact local Return evidence: legacy needs its narrowed path;
    /// explicit read needs valid support. The result owner separately checks
    /// the original outer From declaration before admitting a read re-return.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn borrowed_nullable_return(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&BorrowedNullableLocalSourceLoanV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        let StoredLocal::BorrowedNullable(loan) = self.locals.get(&binding)? else {
            return None;
        };
        (loan.destination() == binding
            && if loan.is_declared_read_v1() { loan.read_valid } else { self.nonnull.contains(&binding) }
            && self.borrowed_entry_support_live_v1(loan.as_ref()))
        .then_some(loan.as_ref())
    }
}
