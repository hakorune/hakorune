//! Ownership-free nullable call result in the existing prefix state.
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

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
    ) {
        self.store(frame.binding(), StoredLocal::StoredMapGet(frame));
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
            && frame.matches_input(self.input)
            && self.is_self_rooted_handle(frame.source().receiver().receiver()))
        .then(|| std::rc::Rc::clone(frame))
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_borrowed_nullable(
        &mut self,
        loan: BorrowedNullableLocalSourceLoanV1,
    ) {
        self.store(
            loan.destination(),
            StoredLocal::BorrowedNullable(Box::new(loan)),
        );
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

    /// Only the exact result binding on this non-null path lends its loan.
    /// General argument/field observation stays BoundValue, even after narrowing.
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
            && self.nonnull.contains(&binding)
            && loan
                .root()
                .is_some_and(|root| self.is_self_rooted_handle(root)))
        .then_some(loan.as_ref())
    }
}
