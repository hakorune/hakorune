//! Entry projection of the same lexical source/use/actual owner.
//! This borrow is prerequisite evidence, not permission to activate an ABI.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::borrowed_formal_actuals::PreparedBorrowedFormalActualV1;
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::{
    SelectedCallableLoweringInputRefV1, SelectedCallableSemanticRefV1,
};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[path = "ordinary_new_borrowed_formal_return_source.rs"]
mod return_source;
#[path = "ordinary_new_borrowed_field_write_source.rs"]
pub(super) mod field_write_source;
#[path = "ordinary_new_borrowed_array_set_source.rs"]
pub(super) mod array_set_source;

/// Complete canonical alternatives for one original borrowed formal. Equality
/// includes source identity; equal reissued alternatives are not the same loan.
#[derive(Debug, Clone)]
pub(in crate::mir) struct BorrowedFormalDomainV1 {
    ordinal: u32,
    binding: BindingRefV1,
    sum: Rc<super::borrowed_formal_source::SourceValueSumV1>,
}
impl BorrowedFormalDomainV1 {
    pub(super) fn from_source_v1(ordinal: u32, binding: BindingRefV1, sum: Rc<super::borrowed_formal_source::SourceValueSumV1>) -> Self {
        Self { ordinal, binding, sum }
    }
    pub(in crate::mir) fn ordinal(&self) -> u32 { self.ordinal }
    pub(in crate::mir) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(in crate::mir) fn text_only(&self) -> bool { self.sum.text_only() }
    #[cfg(test)]
    pub(super) fn source_sum_v1(&self) -> &Rc<super::borrowed_formal_source::SourceValueSumV1> { &self.sum }
}
impl PartialEq for BorrowedFormalDomainV1 {
    fn eq(&self, other: &Self) -> bool {
        self.ordinal == other.ordinal && self.binding == other.binding && Rc::ptr_eq(&self.sum, &other.sum)
    }
}
impl Eq for BorrowedFormalDomainV1 {}

#[derive(Debug)]
pub(crate) struct BorrowedOrdinaryEntrySourceRefV1<'ledger> {
    owner: FunctionOwnerIdV1,
    domains: Box<[BorrowedFormalDomainV1]>,
    formals: Box<[(u32, BindingRefV1)]>,
    source: &'ledger PreparedBorrowedFormalIngressV1,
    definition: &'ledger super::borrowed_formal_uses::BorrowedFormalUsesDraftV1,
    incoming_rows: Box<[&'ledger super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>,
    incoming: BorrowedEntryActualsV1<'ledger>,
}

impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn formals(&self) -> &[(u32, BindingRefV1)] {
        &self.formals
    }
    pub(in crate::mir) fn formal_domain_v1(&self, ordinal: u32, binding: BindingRefV1) -> Result<Option<&BorrowedFormalDomainV1>, String> {
        if !self.formals.contains(&(ordinal, binding)) { return Err(freeze("borrowed-entry/domain-formal")); }
        Ok(self.domains.iter().find(|domain| domain.ordinal() == ordinal && domain.binding() == binding))
    }
    pub(crate) fn origins(&self) -> &BTreeMap<BindingRefV1, BindingRefV1> {
        &self.definition.origins
    }
    pub(crate) fn incoming(
        &self,
    ) -> Result<&[(&OwnedExprSiteV1, &[PreparedBorrowedFormalActualV1])], String> {
        match &self.incoming {
            BorrowedEntryActualsV1::Executable(rows) => Ok(rows),
            BorrowedEntryActualsV1::SelectedTagged => {
                Err(freeze("borrowed-entry/canonical-packet-actual-required"))
            }
        }
    }
    /// Checked-compare operand admissions this owner's draft proved:
    /// `(binding, formal, binary site)` rows under the operation owner's
    /// `Greater(NormalInteger, NormalInteger)` envelope. Admission evidence
    /// only; publication still counts the exact physical operand uses.
    pub(crate) fn compare_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::CompareOperand {
                    binary,
                    ..
                } => Some((row.binding, row.formal, binary)),
                _ => None,
            })
    }
    /// Exact Return source rows, tied to the SAME original Compare receipt.
    /// This lends source correspondence only; final dominance is checked independently.
    pub(crate) fn integer_return_uses(
        &self,
    ) -> Result<
        Vec<(
            BindingRefV1,
            BindingRefV1,
            crate::mir::resolved_semantics::SourceStmtSiteV1,
            OwnedExprSiteV1,
            OwnedExprSiteV1,
        )>,
        String,
    > {
        use super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
        let draft = self.definition;
        let mut returns = Vec::new();
        for row in &draft.uses {
            let Use::IntegerReturn { exit, guard } = &row.kind else {
                continue;
            };
            if draft.origins.get(&row.binding) != Some(&row.formal)
                || !draft.uses.iter().any(|compare| match &compare.kind {
                    Use::CompareOperand { binary, source } => {
                        compare.formal == row.formal
                            && guard.matches_compare(row.formal, binary, source)
                    }
                    _ => false,
                })
            {
                return Err("ordinary-new/borrowed-return/compare-source-identity".into());
            }
            returns.push((
                row.binding,
                row.formal,
                exit.clone(),
                row.site.clone(),
                guard.binary().clone(),
            ));
        }
        Ok(returns)
    }

    /// Dominated `+` operand admissions this owner's draft proved under the
    /// operation owner's `Add(NormalInteger, NormalInteger)` envelope:
    /// `(binding, formal, binary site)` rows guarded by an admitted compare.
    pub(crate) fn add_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::AddOperand {
                    binary,
                } => Some((row.binding, row.formal, binary)),
                _ => None,
            })
    }
    /// Original Mul source products and exact side; no legacy Add counts.
    /// The consumer must retain the source through materialization/finishing.
    pub(crate) fn mul_source_uses(
        &self,
    ) -> impl Iterator<
        Item = (
            &OwnedExprSiteV1,
            &Rc<super::borrowed_formal_uses::BorrowedMulSourceV1>,
            super::borrowed_formal_uses::BorrowedMulSideV1,
        ),
    > {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand {
                    binary,
                    source,
                    side,
                } => Some((binary, source, *side)),
                _ => None,
            })
    }

    /// Dominated `.set` element-value admissions this owner's draft proved:
    /// `(binding, formal, call site)` rows on a proven `me.<ArrayBox>`
    /// receiver guarded by an admitted compare. Admission evidence only;
    /// publication still counts the exact physical operand uses.
    pub(crate) fn array_element_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::ArrayElementValue {
                    call,
                } => Some((row.binding, row.formal, call)),
                _ => None,
            })
    }
    /// Dominated `new`-argument admissions this owner's draft proved:
    /// `(binding, formal, new site, ordinal)` rows on the sole admitted
    /// argument position, guarded by an admitted compare of the same
    /// formal. Admission evidence only; publication still counts the exact
    /// physical operand uses.
    pub(crate) fn new_argument_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1, u32)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::NewArgument {
                    site,
                    ordinal,
                } => Some((row.binding, row.formal, site, *ordinal)),
                _ => None,
            })
    }
    /// Admitted null-equality operand uses this owner's draft proved:
    /// `(binding, formal, binary site)` rows under the operation owner's
    /// `Equal(Dynamic, Null)` envelope. Admission evidence only;
    /// publication still counts the exact physical operand uses.
    pub(crate) fn null_compare_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::NullCompareOperand {
                    binary,
                } => Some((row.binding, row.formal, binary)),
                _ => None,
            })
    }
    /// Guarded `formal.field` receiver admissions this owner's draft
    /// proved: `(binding, formal, field-access site)` rows dominated by an
    /// admitted null compare of the same formal. Admission evidence only;
    /// publication still counts the exact physical operand uses.
    pub(crate) fn field_read_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::FieldReadOperand {
                    site,
                } => Some((row.binding, row.formal, site)),
                _ => None,
            })
    }
    /// Exact guarded outgoing uses tied to their original Compare source.
    /// This is source correspondence; final consumers prove physical dominance.
    pub(crate) fn guarded_argument_uses(
        &self,
    ) -> Result<
        Vec<(
            OwnedExprSiteV1,
            u32,
            BindingRefV1,
            BindingRefV1,
            OwnedExprSiteV1,
        )>,
        String,
    > {
        use super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
        let mut uses = Vec::new();
        for row in &self.definition.uses {
            let Use::UnresolvedArgument { call, ordinal } = &row.kind else {
                continue;
            };
            let Some(fact) = self.source.guarded_actuals.get(&(call.clone(), *ordinal)) else {
                continue;
            };
            if !fact.matches_draft_argument(row, call, *ordinal, self.definition) {
                return Err(freeze("borrowed-guarded/final-source-identity"));
            }
            uses.push((
                call.clone(),
                *ordinal,
                row.binding,
                row.formal,
                fact.guard_binary().clone(),
            ));
        }
        Ok(uses)
    }

}

impl OrdinaryNewClaimLedgerV1 {
    /// Finalization borrows the original source and recorded correspondence;
    /// it must not reconstruct a lowering input or issue new formal contracts.
    pub(crate) fn finalized_borrowed_ordinary_entry_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<BorrowedOrdinaryEntrySourceRefV1<'_>, String> {
        let values = self.borrowed_ordinary_entry_values_v1(owner);
        self.check_borrowed_ordinary_entry_values_v1(owner, &values)?;
        let values = values?;
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let view = self
            .checked_entry_owner_view_v1(source, owner)?
            .ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
        let incoming = self.entry_actual_loans_v1(source, owner, &view)?;
        Ok(BorrowedOrdinaryEntrySourceRefV1 {
            owner,
            domains: self.checked_entry_domains_v1(source, owner, &view, &incoming)?,
            formals: values
                .iter()
                .map(|(ordinal, binding, _)| (*ordinal, *binding))
                .collect(),
            source,
            definition: view.definition,
            incoming_rows: view.incoming,
            incoming,
        })
    }

    /// Only the installed Ordinary source input can request this projection.
    /// Nonopaque and Dynamic callers do not demand unrelated pending errors.
    #[cfg(test)]
    pub(crate) fn borrowed_ordinary_entry_source_v1(
        &self, input: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<Option<BorrowedOrdinaryEntrySourceRefV1<'_>>, String> {
        self.borrowed_ordinary_entry_structure_v1(input)?
            .map(|structure| self.finish_borrowed_entry_source_v1(structure)).transpose()
    }

    /// Original incoming namespace determines the entry's receiver law.
    /// This does not infer mode from the physical receiver's presence.
    pub(crate) fn borrowed_ordinary_entry_receiver_required_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if self.finite_text_entry_for_owner_v1(owner)?.is_some() {
            let mut contracts = self.original_parameter_contracts.as_deref()
                .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?
                .iter().filter(|row| row.owner == owner);
            let contract = contracts.next().ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
            if contracts.next().is_some() { return Err(freeze("borrowed-entry/original-contract-duplicate")); }
            return match contract.mode {
                crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod => Ok(false),
                crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod => Ok(true),
            };
        }
        let view = self
            .checked_entry_owner_view_v1(source, owner)?
            .ok_or_else(|| freeze("borrowed-entry/incoming-missing"))?;
        if view.mixed
            || self.selected_instance_input_protocol_v1(source, owner)?.is_some()
            || self
                .checked_guarded_instance_definition_v1(source, owner)?
                .is_some()
        {
            return Ok(true); // SAME original Instance declaration, checked by the lender.
        }
        let mut modes = view
            .incoming
            .iter()
            .map(|row| row.source.as_loan().map(|loan| loan.declaration_mode()))
            .collect::<Result<Vec<_>, String>>()?.into_iter();
        let mode = modes
            .next()
            .ok_or_else(|| freeze("borrowed-entry/incoming-missing"))?;
        if modes.any(|other| other != mode) {
            return Err(freeze("borrowed-entry/incoming-mode-drift"));
        }
        match mode {
            crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod => Ok(true),
            crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod => Ok(false),
        }
    }

    /// Original final opaque entry and argument join for completed return acquisition.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_opaque_object_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        loan: &crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        contract: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1,
    ) -> Result<
        Option<&[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>,
        String,
    > {
        if contract.owner != target.callee_owner()
            || contract.batch_slot != target.target_batch_slot()
            || loan.call() != target.call_site()
            || loan.key() != target.target()
            || self
                .callable_result_classes
                .object_return_qualification(loan.value())
                .as_ref()
                != Some(loan)
            || !target
                .object_return_sources()
                .is_some_and(|rows| rows.contains(loan))
        {
            return Err(freeze("object-return/opaque-input-identity"));
        }
        self.checked_completed_opaque_object_target_arguments_v1(target, contract)
    }

    /// Shares the original whole-final-incoming input check without a caller result loan.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_opaque_object_target_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        contract: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1,
    ) -> Result<
        Option<&[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>,
        String,
    > {
        if contract.owner != target.callee_owner()
            || contract.batch_slot != target.target_batch_slot()
        {
            return Err(freeze("object-return/opaque-input-identity"));
        }
        let Some(source) = &self.borrowed_formal_source else {
            return Ok(None);
        };
        let ingress = source.as_ref().map_err(Clone::clone)?;
        // The original QualifiedObject family owns its complete input cohort.
        // Its acquisition must not depend on an unrelated Array consumer being
        // executable before the sole producer Normal seal. The global opaque
        // family below retains its existing whole-final-incoming contract.
        let source_input = self.has_original_object_source_input_v1(ingress, target)?;
        let qualified = ingress.source_incoming.has_object_input_callee_v1(target);
        if (qualified || source_input)
            && self.selected_object_input_contract_v1(ingress, contract)?
        {
            let Some((selected, signatures)) = &self.original_instance_input_issuance else {
                return Ok(None);
            };
            let catalog = self.original_parameter_contracts.as_deref()
                .ok_or_else(|| freeze("object-return/original-input-catalog-missing"))?;
            let mut original = catalog.iter().filter(|row| row.owner == contract.owner);
            let original = original.next()
                .filter(|_| original.next().is_none())
                .ok_or_else(|| freeze("object-return/original-input-contract"))?;
            if original.batch_slot != contract.batch_slot || original.mode != contract.mode
                || original.parameters.len() != contract.parameters.len()
                || original.parameters.iter().zip(&contract.parameters).any(|(a, b)| {
                    a.ordinal != b.ordinal || a.binding != b.binding || a.kind != b.kind
                })
            { return Err(freeze("object-return/original-input-contract-drift")); }
            return if !qualified && source_input {
                self.checked_completed_original_instance_target_arguments_v1(
                    target, selected, catalog, signatures,
                )
            } else {
                self.checked_completed_typed_object_target_arguments_v1(
                    target, selected, catalog, signatures,
                )
            };
        }
        if !ingress.definitions.contains_key(&contract.owner) {
            return Ok(None);
        }
        let projected = ingress
            .source_incoming
            .project(&[contract.owner].into_iter().collect())
            .map_err(|issue| {
                format!("{}: {issue:?}", freeze("borrowed-formal/incoming-coverage"))
            })?;
        for call in &projected {
            if !ingress
                .incoming
                .iter()
                .any(|final_call| final_call.call == call.call)
            {
                return Ok(None);
            }
        }
        // Preserve the existing whole-final-incoming checker failure scope.
        let mut missing = false;
        for call in &ingress.incoming {
            match self.borrowed_formal_actuals.get(&call.call) {
                None => missing = true,
                Some(Err(issue)) => return Err(issue.clone()),
                Some(Ok(actuals)) => {
                    if actuals.require_executable_v1().is_err() {
                        missing = true;
                    }
                }
            }
        }
        if missing {
            return Ok(None);
        }
        let parameters: Vec<_> = contract
            .parameters
            .iter()
            .map(|row| (row.ordinal, row.binding, row.kind.clone()))
            .collect();
        if self
            .borrowed_entry_source_for_contract(contract.owner, &parameters)?
            .is_none()
        {
            return Ok(None);
        }
        let Some((call, arguments)) =
            super::borrowed_formal_actuals::lend_pending_borrowed_arguments_v1(
                source,
                &self.borrowed_formal_actuals,
                target.call_site(),
            )?
        else {
            return Ok(None);
        };
        if call.source.require_instance()? != target || call.callee != contract.owner {
            return Err(freeze("object-return/opaque-target"));
        }
        Ok(Some(arguments))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_opaque_receiver_argument_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        kind: &crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1,
    ) -> Result<bool, String> {
        use super::borrowed_formal_actuals::BorrowedFormalActualSourceV1 as Source;
        use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1 as Kind;
        let Some(actuals) = self.borrowed_formal_actuals.get(call) else {
            return Ok(false);
        };
        let actuals = actuals.as_ref().map_err(Clone::clone)?;
        actuals.require_executable_v1()?;
        let mut rows = actuals
            .opaque_actuals
            .iter()
            .filter(|row| row.ordinal == ordinal);
        let Some(row) = rows.next() else {
            return Ok(false);
        };
        if rows.next().is_some() {
            return Err(freeze("object-return/receiver-ordinal"));
        }
        Ok(match (&row.source, kind) {
            (Source::Integer(a), Kind::Integer(b)) => a == b,
            (Source::Bool(a), Kind::Bool(b)) => a == b,
            (Source::Null, Kind::Null) => true,
            (
                Source::ReceivedNullable { binding, .. }
                | Source::ReceivedMixed { binding, .. }
                | Source::Scalar { binding, .. }
                | Source::TypedHome { binding, .. }
                | Source::EntryReceiver { binding, .. }
                | Source::DeclaredFormal { binding, .. }
                | Source::Forwarded { binding, .. },
                Kind::Local { binding: actual } | Kind::Handle { binding: actual },
            ) => binding == actual,
            _ => false,
        })
    }

    fn borrowed_entry_source_for_contract(
        &self, owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<Option<BorrowedOrdinaryEntrySourceRefV1<'_>>, String> {
        self.borrowed_entry_structure_for_contract(owner, parameters)?
            .map(|structure| self.finish_borrowed_entry_source_v1(structure)).transpose()
    }
    /// Borrow the original actual rows only for a consumed, owned lexical row.
    pub(crate) fn borrowed_call_actuals_v1(
        &self,
        row: &super::LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        self.borrowed_call_actuals_using_v1(row, CallValidationContextV1::Lowering(self))
    }

    pub(in crate::mir) fn borrowed_call_actuals_using_v1(
        &self, row: &super::LexicalInstanceCallDispositionRowV1, context: CallValidationContextV1<'_>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        context.require_ledger(self)?;
        if !matches!(
            self.lexical_instance_calls.borrow().get(row.call_site()),
            Some(super::LexicalInstanceCallDispositionSlotV1::Taken)
        ) {
            return Err(freeze("borrowed-call/disposition-not-owned-and-taken"));
        }
        if self.uses_object_packet_protocol_v1(row)? {
            return row.checked_object_packet_actuals_v1(self).map(Some);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-call/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let Some(view) = self.checked_entry_owner_view_v1(source, row.callee_owner())? else {
            if row.result() == Some(crate::mir::instruction::InvokeCallResultKind::Bool) {
                return Err(freeze("borrowed-call/bool-input-incomplete"));
            }
            if source.incoming.iter().any(|call| &call.call == row.call_site()) {
                return Err(freeze("borrowed-entry/incoming-identity"));
            }
            return if source.definitions.contains_key(&row.callee_owner()) {
                Err(freeze("borrowed-call/incoming-missing"))
            } else { Ok(None) };
        };
        let mut incoming = view.incoming.iter().copied()
            .filter(|call| &call.call == row.call_site());
        let Some(call) = incoming.next() else {
            if row.result() == Some(crate::mir::instruction::InvokeCallResultKind::Bool) {
                return Err(freeze("borrowed-call/bool-input-incomplete"));
            }
            return if source.definitions.contains_key(&row.callee_owner()) {
                Err(freeze("borrowed-call/incoming-missing"))
            } else {
                Ok(None)
            };
        };
        if incoming.next().is_some()
            || call.callee != row.callee_owner()
            || row.source_target() != call.source.require_instance()?
            || row.argument_sites().len() != row.target().arity() as usize
            || call
                .arguments
                .iter()
                .any(|(ordinal, site, _)| row.argument_sites().get(*ordinal as usize) != Some(site))
        {
            return Err(freeze("borrowed-call/disposition-identity"));
        }
        let inputs = self.entry_actual_loans_v1(source, call.callee, &view)?;
        let BorrowedEntryActualsV1::Executable(inputs) = inputs else {
            return Err(freeze("borrowed-entry/canonical-packet-actual-required"));
        };
        let mut exact = inputs.iter().filter(|(site, _)| *site == row.call_site());
        let actuals = exact.next().map(|(_, actuals)| *actuals)
            .ok_or_else(|| freeze("borrowed-call/incoming-missing"))?;
        if exact.next().is_some() {
            return Err(freeze("borrowed-call/disposition-identity"));
        }
        if row.source_target().borrowed_nullable_local_source().is_some() {
            self.corroborate_borrowed_nullable_call_v1(row)?;
            return Ok(Some(actuals));
        }
        if row.source_target().mixed_local_source().is_some() {
            self.corroborate_mixed_call_using_v1(row, context)?;
            return Ok(Some(actuals));
        }
        let proof = self
            .borrowed_i64_results
            .get(&call.callee)
            .ok_or_else(|| freeze("borrowed-call/result-source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        proof.require_source_sealed_v1()?;
        if !proof.contract_corroborated
            || proof.returns.is_empty()
            || proof.returns.iter().any(|site| site.owner() != call.callee)
        {
            return Err(freeze("borrowed-call/result-not-corroborated"));
        }
        // The source-proved result class names the one invoke result kind
        // this call may carry: scalar i64 transport or the checked
        // nullable-handle transport — never a third shape.
        let expected = match proof.class {
            super::borrowed_formal_result::BorrowedResultClassV1::I64 => {
                crate::mir::instruction::InvokeCallResultKind::I64
            }
            super::borrowed_formal_result::BorrowedResultClassV1::Bool => {
                let loan = row.scalar_result_source_v1().ok_or_else(|| freeze("borrowed-call/scalar-source-missing"))?;
                super::borrowed_formal_result::validate_source_scalar_result_loan_v1(
                    self.borrowed_formal_source.as_ref().unwrap(), &self.borrowed_i64_results, loan, row.source_target(),
                )?;
                if self.callable_scalar_result_source_v1(call.callee)?.is_none()
                    || loan.kind() != crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool
                { return Err(freeze("borrowed-call/scalar-source-drift")); }
                crate::mir::instruction::InvokeCallResultKind::Bool
            }
            super::borrowed_formal_result::BorrowedResultClassV1::UnresolvedScalar => {
                return Err(freeze("borrowed-result/source-class-unresolved"));
            }
            super::borrowed_formal_result::BorrowedResultClassV1::Nullable => {
                crate::mir::instruction::InvokeCallResultKind::NullableHandle
            }
        };
        if row.result() != Some(expected) {
            return Err(freeze("borrowed-call/result-mismatch"));
        }
        Ok(Some(actuals))
    }

    pub(super) fn checked_borrowed_entry_incoming<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>, String> {
        self.checked_borrowed_entry_incoming_core_v1(source, owner, false)
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_entry_tests.rs"]
mod tests;

#[path = "ordinary_new_borrowed_formal_entry_values.rs"]
mod entry_values;
#[path = "ordinary_new_borrowed_formal_entry_incoming.rs"]
mod incoming;
#[path = "ordinary_new_borrowed_formal_entry_owner_view.rs"]
mod owner_view;
pub(super) use owner_view::checked_forwarding_instance_source_v1;
use owner_view::BorrowedEntryActualsV1;

pub(in crate::mir) use entry_values::BorrowedCompareCarrierOperandLoanV1;
pub(crate) use entry_values::BorrowedCompareIntegerLiteralLoanV1;
pub(in crate::mir) use entry_values::BorrowedCompareIntegerLiteralMaterializationV1;
pub(in crate::mir) use entry_values::BorrowedCompareMaterializationV1;
pub(crate) use entry_values::BorrowedCompareSourceLoanV1;
pub(in crate::mir::normal_callable_semantic_package) use entry_values::BorrowedOrdinaryEntryPhysicalV1;
pub(in crate::mir) use entry_values::{BorrowedMulMaterializationV1, BorrowedMulSourceLoanV1};

#[cfg(test)]
#[path = "ordinary_new_borrowed_incoming_kind_tests.rs"]
mod incoming_kind_tests;

#[path = "ordinary_new_borrowed_static_packet_entry.rs"]
mod static_packet_entry;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) use owner_view::original_borrowed_instance_input_completion_needed_v1;

#[path = "ordinary_new_borrowed_text_entry_read.rs"]
mod text_read;
pub(crate) use text_read::BorrowedTextEntryReadRequirementV1;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use text_read::issue_text_entry_read_requirement_v1;

#[path = "ordinary_new_borrowed_formal_entry_contract.rs"]
mod entry_contract;
