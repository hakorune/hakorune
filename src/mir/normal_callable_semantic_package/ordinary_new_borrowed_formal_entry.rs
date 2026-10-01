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
}

impl OrdinaryNewClaimLedgerV1 {
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
            .any(|(_, _, kind)| *kind == CallableParameterContractKindV1::OpaqueHandle)
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
            .any(|(_, _, kind)| *kind == CallableParameterContractKindV1::OpaqueHandle)
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
            if *kind == CallableParameterContractKindV1::OpaqueHandle {
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
            if actuals.len() != call.arguments.len() {
                return Err(freeze("borrowed-entry/actuals-cardinality"));
            }
            for (actual, (ordinal, site, formal)) in actuals.iter().zip(call.arguments.iter()) {
                if actual.ordinal != *ordinal
                    || &actual.site != site
                    || actual.formal != *formal
                    || formal.owner() != call.callee
                    || source.definitions[&call.callee].origins.get(formal) != Some(formal)
                {
                    return Err(freeze("borrowed-entry/actuals-identity"));
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
