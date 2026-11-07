//! Entry installation: declared parameters and a lent receiver/parameter Home.
use super::*;

impl<'source> PrefixLocalFlow<'source> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_parameters(
        &mut self,
        parameters: impl IntoIterator<
            Item = (
                u32,
                BindingRefV1,
                crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
            ),
        >,
    ) -> bool {
        let mut count = 0;
        for (ordinal, binding, kind) in parameters {
            if binding.owner() != self.input.owner()
                || self
                    .input
                    .function()
                    .declaration_binding(&super::super::SourceBindingSiteV1::Parameter { index: ordinal })
                    != Some(binding)
                || self.locals.contains_key(&binding)
            {
                return false;
            }
            use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
            let is_opaque_formal = matches!(&kind, CallableParameterContractKindV1::OpaqueHandle);
            if matches!(&kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()) {
                self.signed_formals.insert(binding);
            }
            let tagged_dynamic = matches!(
                &kind,
                CallableParameterContractKindV1::OpaqueHandle
            ) && self
                .source_value_class_facts
                .as_ref()
                .is_some_and(|points| points.proves_formal(binding));
            let source_signed_i64 = is_opaque_formal
                && self
                    .source_value_class_facts
                    .as_ref()
                    .is_some_and(|facts| facts.proves_signed_i64_formal(binding));
            if tagged_dynamic && source_signed_i64 {
                return false;
            }
            if source_signed_i64 {
                self.signed_formals.insert(binding);
            }
            let value = if tagged_dynamic {
                StoredLocal::TaggedDynamic
            } else if source_signed_i64 {
                StoredLocal::Trivial(Some(SourceScalarKind::Integer))
            } else {
                match kind {
                CallableParameterContractKindV1::ExactTrivial(abi)
                    if abi.mir_type() == crate::mir::MirType::Integer =>
                {
                    StoredLocal::Trivial(Some(SourceScalarKind::Integer))
                }
                CallableParameterContractKindV1::ExactTrivial(_) => return false,
                CallableParameterContractKindV1::Map => StoredLocal::BorrowedMap,
                CallableParameterContractKindV1::OpaqueHandle
                | CallableParameterContractKindV1::DeclaredHandle
                | CallableParameterContractKindV1::DeclaredObject(_)
                | CallableParameterContractKindV1::ExactText(_) => StoredLocal::Handle(binding),
                }
            };
            self.store(binding, value);
            if is_opaque_formal {
                self.opaque_formals.insert(binding);
            }
            count += 1;
        }
        count
            == self
                .input
                .function()
                .declaration_sites()
                .filter(|site| matches!(site, super::super::SourceBindingSiteV1::Parameter { .. }))
                .count()
    }

    /// Exact unannotated formal identity at this source read. This is a
    /// Dynamic source classification only; the downstream entry-read issuer
    /// still verifies the original formal contract and physical entry.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn opaque_formal_binding(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<BindingRefV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        if !self.opaque_formals.contains(&binding) {
            return None;
        }
        let crate::mir::resolved_semantics::BindingKindV1::Parameter { index } =
            self.input.function().binding(binding)?.kind()
        else {
            return None;
        };
        (self.input.function().declaration_binding(&super::super::SourceBindingSiteV1::Parameter { index })
            == Some(binding))
            .then_some(binding)
    }

    /// Install the receiver and parameters carried by a verified instance
    /// entry loan. The loan is the sole authority for these bindings; the
    /// caller passes the row minted for this exact declaration only. The
    /// receiver installs as a borrowed self-rooted Handle — it is observable
    /// as a handle root but owns no Home and owes no release.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_entry_home(
        &mut self,
        loan: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1,
    ) -> bool {
        // The loan is scoped to one exact declaration: a foreign-owner row
        // never installs, even if its bindings happen to match by accident.
        if loan.owner() != self.input.owner() || self.entry_receiver.is_some() {
            return false;
        }
        let receiver = loan.receiver();
        if !self.install_parameters(
            loan.parameters()
                .iter()
                .map(|row| (row.ordinal(), row.binding(), row.kind())),
        ) {
            return false;
        }
        if receiver.owner() != self.input.owner()
            || self
                .input
                .function()
                .declaration_binding(&super::super::SourceBindingSiteV1::Receiver)
                != Some(receiver)
            || self.locals.contains_key(&receiver)
        {
            return false;
        }
        self.store(receiver, StoredLocal::Handle(receiver));
        self.entry_receiver = Some(PrefixEntryReceiverV1 {
            binding: receiver,
            owner: loan.owner(),
            batch_slot: loan.batch_slot(),
            identity: loan.identity().clone(),
        });
        true
    }
}
