//! Entry projection of the same lexical source/use/actual owner.
//! This borrow is prerequisite evidence, not permission to activate an ABI.
use super::borrowed_formal_actuals::PreparedBorrowedFormalActualV1;
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::{
    SelectedCallableLoweringInputRefV1, SelectedCallableSemanticRefV1,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(crate) struct BorrowedOrdinaryEntrySourceRefV1<'ledger> {
    owner: FunctionOwnerIdV1,
    formals: Box<[(u32, BindingRefV1)]>,
    source: &'ledger PreparedBorrowedFormalIngressV1,
    incoming: Box<
        [(
            &'ledger OwnedExprSiteV1,
            &'ledger [PreparedBorrowedFormalActualV1],
        )],
    >,
}

impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn formals(&self) -> &[(u32, BindingRefV1)] {
        &self.formals
    }
    pub(crate) fn origins(&self) -> &BTreeMap<BindingRefV1, BindingRefV1> {
        &self.source.definitions[&self.owner].origins
    }
    pub(crate) fn incoming(&self) -> &[(&OwnedExprSiteV1, &[PreparedBorrowedFormalActualV1])] {
        &self.incoming
    }
    /// Checked-compare operand admissions this owner's draft proved:
    /// `(binding, formal, binary site)` rows under the operation owner's
    /// `Greater(NormalInteger, NormalInteger)` envelope. Admission evidence
    /// only; publication still counts the exact physical operand uses.
    pub(crate) fn compare_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.source.definitions[&self.owner]
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::CompareOperand {
                    binary,
                } => Some((row.binding, row.formal, binary)),
                _ => None,
            })
    }
    /// Dominated `+` operand admissions this owner's draft proved under the
    /// operation owner's `Add(NormalInteger, NormalInteger)` envelope:
    /// `(binding, formal, binary site)` rows guarded by an admitted compare.
    pub(crate) fn add_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.source.definitions[&self.owner]
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::AddOperand {
                    binary,
                } => Some((row.binding, row.formal, binary)),
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
        self.source.definitions[&self.owner]
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
        self.source.definitions[&self.owner]
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
        self.source.definitions[&self.owner]
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
        self.source.definitions[&self.owner]
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::FieldReadOperand {
                    site,
                } => Some((row.binding, row.formal, site)),
                _ => None,
            })
    }
    /// The co-sealed class view for one of this owner's formals — `Some`
    /// only when every incoming actual agreed on one ordinary class.
    pub(crate) fn formal_object_view(
        &self,
        formal: BindingRefV1,
    ) -> Option<&super::borrowed_formal_source::BorrowedFormalObjectViewV1> {
        self.source.formal_object_view(formal)
    }
    /// Loan the original target, including its source owner and batch identity.
    pub(crate) fn incoming_targets(
        &self,
    ) -> impl Iterator<Item = &super::LexicalInstanceCallSourceTargetV1> {
        self.source
            .incoming
            .iter()
            .filter(move |row| row.callee == self.owner)
            .map(|row| &row.source)
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
        let incoming = self.checked_borrowed_entry_incoming(source, owner)?;
        Ok(BorrowedOrdinaryEntrySourceRefV1 {
            owner,
            formals: values
                .iter()
                .map(|(ordinal, binding, _)| (*ordinal, *binding))
                .collect(),
            source,
            incoming,
        })
    }

    /// Only the installed Ordinary source input can request this projection.
    /// Nonopaque and Dynamic callers do not demand unrelated pending errors.
    pub(crate) fn borrowed_ordinary_entry_source_v1(
        &self,
        input: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<Option<BorrowedOrdinaryEntrySourceRefV1<'_>>, String> {
        if !matches!(input.semantic(), SelectedCallableSemanticRefV1::Ordinary) {
            return Ok(None);
        }
        if !matches!(input.selected_key(), crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)
            if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod)
        {
            return Ok(None);
        }
        let parameters = input.parameter_contracts().collect::<Vec<_>>();
        if !parameters
            .iter()
            .any(|(_, _, kind)| kind.is_ordinary_borrowed_handle())
        {
            return Ok(None);
        }
        if !self.completion_index.contains_key(&input.source().owner()) {
            return Err(freeze("borrowed-entry/foreign-source-loan"));
        }
        self.borrowed_entry_source_for_contract(input.source().owner(), &parameters)
    }

    fn borrowed_entry_source_for_contract(
        &self,
        owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<Option<BorrowedOrdinaryEntrySourceRefV1<'_>>, String> {
        if !parameters
            .iter()
            .any(|(_, _, kind)| kind.is_ordinary_borrowed_handle())
        {
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let Some(definition) = source.definitions.get(&owner) else {
            // A source use outside the closed transport profile was excluded
            // before selection. This is not retry after a selected failure.
            return Ok(None);
        };
        let mut formals = Vec::new();
        for (index, (ordinal, binding, kind)) in parameters.iter().enumerate() {
            if *ordinal as usize != index || binding.owner() != owner {
                return Err(freeze("borrowed-entry/formal-identity"));
            }
            if kind.is_ordinary_borrowed_handle() {
                if definition.origins.get(binding) != Some(binding) {
                    return Err(freeze("borrowed-entry/formal-origin"));
                }
                formals.push((*ordinal, *binding));
            }
        }
        let roots: BTreeSet<_> = definition.origins.values().copied().collect();
        if roots != formals.iter().map(|(_, binding)| *binding).collect() {
            return Err(freeze("borrowed-entry/formal-cardinality"));
        }
        let incoming = self.checked_borrowed_entry_incoming(source, owner)?;
        for call in source.incoming.iter().filter(|call| call.callee == owner) {
            if call.arguments.len() != formals.len()
                || call.arguments.iter().zip(formals.iter()).any(
                    |((ordinal, _, formal), (expected, binding))| {
                        ordinal != expected || formal != binding
                    },
                )
            {
                return Err(freeze("borrowed-entry/incoming-formals"));
            }
        }
        Ok(Some(BorrowedOrdinaryEntrySourceRefV1 {
            owner,
            formals: formals.into_boxed_slice(),
            source,
            incoming,
        }))
    }
    /// Borrow the original actual rows only for a consumed, owned lexical row.
    pub(crate) fn borrowed_call_actuals_v1(
        &self,
        row: &super::LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        if !matches!(
            self.lexical_instance_calls.borrow().get(row.call_site()),
            Some(super::LexicalInstanceCallDispositionSlotV1::Taken)
        ) {
            return Err(freeze("borrowed-call/disposition-not-owned-and-taken"));
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-call/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let mut incoming = source
            .incoming
            .iter()
            .filter(|call| &call.call == row.call_site());
        let Some(call) = incoming.next() else {
            return if source.definitions.contains_key(&row.callee_owner()) {
                Err(freeze("borrowed-call/incoming-missing"))
            } else {
                Ok(None)
            };
        };
        if incoming.next().is_some()
            || call.callee != row.callee_owner()
            || row.source_target() != &call.source
            || row.argument_sites().len() != row.target().arity() as usize
            || call
                .arguments
                .iter()
                .any(|(ordinal, site, _)| row.argument_sites().get(*ordinal as usize) != Some(site))
        {
            return Err(freeze("borrowed-call/disposition-identity"));
        }
        self.checked_borrowed_entry_incoming(source, call.callee)?;
        let proof = self
            .borrowed_i64_results
            .get(&call.callee)
            .ok_or_else(|| freeze("borrowed-call/result-source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
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
            super::borrowed_formal_result::BorrowedResultClassV1::Nullable => {
                crate::mir::instruction::InvokeCallResultKind::NullableHandle
            }
        };
        if row.result() != Some(expected) {
            return Err(freeze("borrowed-call/result-mismatch"));
        }
        self.borrowed_formal_actuals
            .get(row.call_site())
            .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
            .as_ref()
            .map(|rows| Some(rows.opaque_actuals.as_ref()))
            .map_err(Clone::clone)
    }

    fn checked_borrowed_entry_incoming<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>, String> {
        let mut seen = BTreeSet::new();
        let mut incoming = Vec::new();
        for call in &source.incoming {
            if !seen.insert(&call.call) || !source.definitions.contains_key(&call.callee) {
                return Err(freeze("borrowed-entry/incoming-identity"));
            }
            let target_roots: BTreeSet<_> = source.definitions[&call.callee]
                .origins
                .values()
                .copied()
                .collect();
            let argument_roots: BTreeSet<_> = call
                .arguments
                .iter()
                .map(|(_, _, formal)| *formal)
                .collect();
            if target_roots != argument_roots
                || argument_roots.len() != call.arguments.len()
                || call.arguments.windows(2).any(|rows| rows[0].0 >= rows[1].0)
            {
                return Err(freeze("borrowed-entry/incoming-cardinality"));
            }
            let actuals = self
                .borrowed_formal_actuals
                .get(&call.call)
                .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            actuals.ordered_arguments_for_v1(call)?;
            let actuals = &actuals.opaque_actuals;
            for actual in actuals.iter() {
                if source.definitions[&call.callee].origins.get(&actual.formal)
                    != Some(&actual.formal)
                {
                    return Err(freeze("borrowed-entry/actuals-identity"));
                }
                // A minted object view binds every incoming actual to one
                // agreed class — a sealed actual naming a different class
                // (or no class at all) is drift, never a second authority.
                if let Some(view) = source.object_views.get(&actual.formal) {
                    let agrees = match &actual.source {
                        super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::ReceivedNullable { class, .. }
                        | super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::TypedHome { class, .. }
                        | super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::EntryReceiver { class, .. } => {
                            class.as_ref() == view.class()
                        }
                        super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Null => true,
                        super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Forwarded { formal, .. } => {
                            source
                                .object_views
                                .get(formal)
                                .is_some_and(|origin| origin.class() == view.class())
                        }
                        _ => false,
                    };
                    if !agrees {
                        return Err(freeze("borrowed-entry/object-view-drift"));
                    }
                }
            }
            if call.callee == owner {
                incoming.push((&call.call, actuals.as_ref()));
            }
        }
        if incoming.is_empty() {
            return Err(freeze("borrowed-entry/incoming-missing"));
        }
        Ok(incoming.into_boxed_slice())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_entry_tests.rs"]
mod tests;

#[path = "ordinary_new_borrowed_formal_entry_values.rs"]
mod entry_values;

pub(in crate::mir::normal_callable_semantic_package) use entry_values::BorrowedOrdinaryEntryPhysicalV1;
