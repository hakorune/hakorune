//! Named Array source state stays distinct from ordinary object New and aliases.
use super::super::named_array::NamedArrayHomeObservationV1;
use super::*;
use crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1;

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn has_field_array_count_source_v1(
        &self, source: &crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1,
    ) -> bool {
        let Some(root) = source.receiver_root_binding_v1(self.input) else { return false; };
        self.field_root_is_live(root) && source.receiver_alias_v1().is_none_or(|alias|
            matches!(self.locals.get(&alias.binding()), Some(StoredLocal::BorrowedArray { source: current, root: parent })
                if *parent == root && (current == alias || current.integer_requirement_v1().is_some_and(|operation|
                    alias.fixed_source_v1().is_some_and(|(field, initializer)| operation.owner() == self.input.owner()
                        && operation.binding() == initializer.binding() && operation.field_site() == field.site().site()
                        && operation.field() == source.field() && operation.field_name() == source.field_name()
                        && operation.provider_caller() == source.provider() && operation.provider_site() == source.provider_site())))))
    }
    pub(super) fn live_borrowed_array_sources(
        &self,
    ) -> BTreeMap<BindingRefV1, super::super::field_read::BorrowedArrayFieldSourceV1> {
        self.locals
            .iter()
            .filter_map(|(binding, value)| match value {
                StoredLocal::BorrowedArray { source, root } if self.field_root_is_live(*root) => {
                    Some((*binding, source.clone()))
                }
                _ => None,
            })
            .collect()
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_no_exit_array_after(
        &mut self,
        prestate: LoopI64PreStateRequestV1,
        loan: std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>,
    ) -> bool {
        let current_arrays = self.live_borrowed_array_sources();
        if !loan.matches_prestate(&prestate)
            || loan.owner() != self.input.owner()
            || prestate.owner() != loan.owner()
            || loan.site() != prestate.site()
            || loan.updated().iter().any(|binding| {
                !prestate.proves_integer(*binding)
                    || !matches!(
                        self.locals.get(binding),
                        Some(StoredLocal::Trivial(Some(SourceScalarKind::Integer)))
                    )
            })
            || loan.arrays().iter().any(|(binding, source)| {
                !prestate
                    .borrowed_array(*binding)
                    .is_some_and(|alias| alias.integer_requirement_v1().is_some_and(|original| std::rc::Rc::ptr_eq(original, source)))
                    || !current_arrays
                        .get(binding)
                        .is_some_and(|alias| alias.integer_requirement_v1().is_some_and(|original| std::rc::Rc::ptr_eq(original, source)))
            })
        {
            return false;
        }
        // The same canonical Home walk retains cleanup participation. The
        // Recipe row shares only this immutable proof, never emission state.
        self.retain_named_array_observation(NamedArrayHomeObservationV1::field_loop(
            std::rc::Rc::clone(&loan),
        ));
        for binding in loan.updated() {
            self.store(
                *binding,
                StoredLocal::Trivial(Some(SourceScalarKind::Integer)),
            );
        }
        true
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_borrowed_array_field(
        &mut self,
        binding: BindingRefV1,
        source: super::super::field_read::BorrowedArrayFieldSourceV1,
        root: BindingRefV1,
    ) -> bool {
        if source.binding() != binding
            || binding.owner() != root.owner()
            || !self.field_root_is_live(root)
        {
            return false;
        }
        self.store(binding, StoredLocal::BorrowedArray { source, root });
        true
    }

    /// Direct live owner only. A Handle/alias never lends the root's transfer.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn available_named_array_holder(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<NamedArrayHolderSourceLoanV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        let StoredLocal::NamedArrayHolder(source) = self.locals.get(&binding)? else {
            return None;
        };
        (source.binding() == binding).then(|| source.clone())
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_named_array_holder(
        &self,
        binding: BindingRefV1,
    ) -> bool {
        matches!(
            self.locals.get(&binding),
            Some(StoredLocal::NamedArrayHolder(_))
        )
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_named_array_holder(
        &mut self,
        source: NamedArrayHolderSourceLoanV1,
    ) {
        self.store(source.binding(), StoredLocal::NamedArrayHolder(source));
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn has_named_array_holder(
        &self,
        source: &NamedArrayHolderSourceLoanV1,
    ) -> bool {
        matches!(self.locals.get(&source.binding()), Some(StoredLocal::NamedArrayHolder(current)) if current == source)
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn retain_named_array_observation(
        &mut self,
        observation: NamedArrayHomeObservationV1,
    ) {
        self.named_arrays.borrow_mut().push(observation);
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn named_array_observations(
        &self,
    ) -> Vec<NamedArrayHomeObservationV1> {
        self.named_arrays.borrow().clone()
    }
}
