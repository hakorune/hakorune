//! Emission observations extend the package-owned source relation. They cannot
//! replace its canonical caller, constructor site, or resolver contract.
use super::SelectedSourceCoreMethodCallV1;
use crate::mir::named_array_obligation::{
    fault, validate_physical_marker, NamedArrayAllocationRefV1, NamedArrayWriteMarkerV1,
};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, NamedArrayRequirementV1, SourceExprSiteV1,
};
use crate::mir::{ArrayWriteSiteId, MirModule, ValueId};

#[derive(Debug)]
pub(crate) struct NamedArrayEmissionDraftV1 {
    source: SelectedSourceCoreMethodCallV1,
    allocation: NamedArrayAllocationRefV1,
    receiver_read: Option<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
}

#[derive(Debug)]
pub(crate) struct EmittedNamedArrayWriteV1 {
    source: SelectedSourceCoreMethodCallV1,
    marker: NamedArrayWriteMarkerV1,
    checked: Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedFieldArrayAppendV1>>,
}

#[derive(Debug)]
pub(crate) enum EmittedNamedArrayRequirementV1 {
    Write(EmittedNamedArrayWriteV1),
    OwnedHolder(crate::mir::normal_callable_semantic_package::EmittedOwnedArrayHolderV1),
    Read(super::field_read::EmittedFieldArrayReadV1),
}
impl EmittedNamedArrayRequirementV1 {
    pub(crate) fn caller(&self) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        match self { Self::Write(row) => row.caller(), Self::Read(row) => row.recipe.caller(), Self::OwnedHolder(row) => row.caller() }
    }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        match self { Self::Write(row) => row.owner(), Self::Read(row) => row.recipe.owner(), Self::OwnedHolder(row) => row.owner() }
    }
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        match self { Self::Write(row) => row.call_site(), Self::Read(row) => row.recipe.call_site(), Self::OwnedHolder(row) => row.construction() }
    }
    pub(crate) fn marker(&self) -> Option<NamedArrayWriteMarkerV1> {
        match self { Self::Write(row) => Some(row.marker()), Self::Read(_) | Self::OwnedHolder(_) => None }
    }
    pub(crate) fn validate<'m>(&self, module: &'m MirModule) -> Result<&'m str, String> {
        match self { Self::Write(row) => row.validate(module), Self::Read(row) => row.validate(module), Self::OwnedHolder(row) => row.validate(module) }
    }
}

impl SelectedSourceCoreMethodCallV1 {
    pub(crate) fn into_named_array_emission(self) -> Result<NamedArrayEmissionDraftV1, String> {
        let Some(requirement) = self.contract().named_array_requirement() else {
            return Err(fault("source-requirement-missing"));
        };
        let allocation = match requirement {
            NamedArrayRequirementV1::Construction(_) => NamedArrayAllocationRefV1::LocalValue(
                self.allocation
                    .ok_or_else(|| fault("allocation-not-emitted"))?,
            ),
            NamedArrayRequirementV1::FieldResidence(requirement) => {
                NamedArrayAllocationRefV1::FieldResidence {
                    field: requirement.field(),
                    provider_caller: requirement.provider_caller().clone(),
                    provider_site: requirement.provider_site().clone(),
                }
            }
        };
        Ok(NamedArrayEmissionDraftV1 {
            allocation,
            source: self,
            receiver_read: None,
        })
    }
}

impl NamedArrayEmissionDraftV1 {
    pub(crate) fn record_write(
        self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
        argument_site: &SourceExprSiteV1,
        write: ArrayWriteSiteId,
        receiver: ValueId,
        argument: ValueId,
    ) -> Result<EmittedNamedArrayRequirementV1, String> {
        let requirement = self
            .source
            .contract()
            .named_array_requirement()
            .ok_or_else(|| fault("source-requirement-missing"))?;
        if requirement.owner() != owner
            || requirement.call() != site
            || requirement.argument() != argument_site
        {
            return Err(fault("write-source-mismatch"));
        }
        let allocation = self.allocation.clone();
        Ok(EmittedNamedArrayRequirementV1::Write(EmittedNamedArrayWriteV1 {
            source: self.source,
            checked: None,
            marker: NamedArrayWriteMarkerV1 {
                owner,
                allocation,
                receiver,
                argument,
                write,
            },
        }))
    }
}

impl EmittedNamedArrayWriteV1 {
    /// The marker and lifecycle group must retain the very same checked
    /// append packet, not merely equal residence or opcode observations.
    pub(in crate::mir) fn borrows_checked_field_append_v1(
        &self, packet: &crate::mir::normal_callable_semantic_package::EmittedFieldArrayAppendV1,
    ) -> bool {
        self.checked.as_ref().is_some_and(|original| std::ptr::eq(original.as_ref(), packet))
    }
    /// Compare the original residence loan without issuing a replacement.
    pub(in crate::mir) fn borrows_field_residence_v1(
        &self, requirement: &std::rc::Rc<crate::mir::resolved_semantics::NamedArrayFieldResidenceRequirementV1>,
    ) -> bool {
        matches!(self.source.contract().named_array_requirement(),
            Some(NamedArrayRequirementV1::FieldResidence(original))
                if std::rc::Rc::ptr_eq(original, requirement))
    }
    pub(crate) fn caller(&self) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        &self.source.caller
    }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.source.contract().owner()
    }
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        self.source.contract().call_site()
    }

    pub(crate) fn marker(&self) -> NamedArrayWriteMarkerV1 {
        self.marker.clone()
    }

    pub(crate) fn validate<'m>(&self, module: &'m MirModule) -> Result<&'m str, String> {
        if self.marker.owner != self.owner() {
            return Err(fault("marker-owner-mismatch"));
        }
        let symbol = module
            .canonical_callable_definition_symbol(&self.source.caller)
            .ok_or_else(|| fault("canonical-caller-missing"))?;
        let function = module
            .functions
            .get(symbol)
            .ok_or_else(|| fault("physical-caller-missing"))?;
        if function.signature.name != symbol {
            return Err(fault("physical-caller-drift"));
        }
        if let Some(packet) = &self.checked {
            if !packet.source().integer_requirement().is_ok_and(|requirement| self.borrows_field_residence_v1(requirement)) { return Err(fault("checked-source-mismatch")); }
            packet.validate_marker(function, &self.marker)?;
            crate::mir::named_array_obligation::validate_array_allocation(module, function, &self.marker.allocation)?;
        } else { validate_physical_marker(module, function, &self.marker)?; }
        Ok(symbol)
    }
}

/// Check both directions: neither markers without source nor dropped markers
/// on a retained row can pass. Shared allocation is allowed; writes are unique.
/// Field-resident rows also claim their birth-side provider recording so an
/// unconsumed provider site cannot escape validation.
pub(crate) fn validate_named_array_coverage(
    module: &MirModule,
    rows: &[EmittedNamedArrayRequirementV1],
) -> Result<(), String> {
    let mut covered = std::collections::BTreeSet::new();
    let mut source_calls = std::collections::BTreeSet::new();
    let mut constructions = std::collections::BTreeMap::new();
    let mut allocations = std::collections::BTreeMap::new();
    let mut read_covered = std::collections::BTreeSet::new();
    let mut claimed_providers = std::collections::BTreeSet::new();
    for emitted in rows {
        let symbol = emitted.validate(module)?;
        if let EmittedNamedArrayRequirementV1::OwnedHolder(row) = emitted {
            if !constructions.insert((symbol, row.construction().clone()), row.array_value()).is_none() {
                return Err(fault("owned-holder-duplicate-physical"));
            }
            continue;
        }
        let EmittedNamedArrayRequirementV1::Write(row) = emitted else {
            let EmittedNamedArrayRequirementV1::Read(read) = emitted else { unreachable!() };
            if !read_covered.insert((symbol.to_owned(), read.read)) { return Err(fault("read-duplicate-physical")); }
            claimed_providers.insert((module.canonical_callable_definition_symbol(read.recipe.provider()).ok_or_else(|| fault("provider-symbol-missing"))?.to_owned(), read.recipe.provider_site().clone()));
            continue;
        };
        let requirement = row
            .source
            .contract()
            .named_array_requirement()
            .ok_or_else(|| fault("source-requirement-missing"))?;
        if !source_calls.insert((symbol, requirement.call())) {
            return Err(fault("duplicate-source-write"));
        }
        match requirement {
            NamedArrayRequirementV1::Construction(requirement) => {
                let NamedArrayAllocationRefV1::LocalValue(allocation) =
                    &row.marker.allocation
                else {
                    return Err(fault("allocation-ref-mismatch"));
                };
                let construction = requirement.construction();
                if constructions
                    .insert((symbol, construction.clone()), *allocation)
                    .is_some_and(|previous| previous != *allocation)
                    || allocations
                        .insert((symbol, *allocation), construction.clone())
                        .is_some_and(|previous| previous != *construction)
                {
                    return Err(fault("construction-allocation-mismatch"));
                }
            }
            NamedArrayRequirementV1::FieldResidence(requirement) => {
                let NamedArrayAllocationRefV1::FieldResidence {
                    field,
                    provider_caller,
                    provider_site,
                } = &row.marker.allocation
                else {
                    return Err(fault("allocation-ref-mismatch"));
                };
                if *field != requirement.field()
                    || provider_caller != requirement.provider_caller()
                    || provider_site != requirement.provider_site()
                {
                    return Err(fault("provider-binding-mismatch"));
                }
                let provider_symbol = module
                    .canonical_callable_definition_symbol(provider_caller)
                    .ok_or_else(|| fault("provider-symbol-missing"))?;
                claimed_providers
                    .insert((provider_symbol.to_owned(), provider_site.clone()));
            }
        }
        if !covered.insert((symbol, row.marker.write.0)) {
            return Err(fault("duplicate-retained-write"));
        }
        let markers = &module.functions[symbol]
            .metadata
            .named_array_write_obligations;
        if markers
            .iter()
            .filter(|marker| **marker == row.marker)
            .count()
            != 1
        {
            return Err(fault("marker-source-mismatch"));
        }
    }
    let marker_count: usize = module
        .functions
        .iter()
        .map(|(symbol, function)| {
            function
                .metadata
                .named_array_write_obligations
                .iter()
                .map(|marker| {
                    if !covered.contains(&(symbol.as_str(), marker.write.0)) {
                        return Err(fault("uncovered-write-marker"));
                    }
                    Ok(1usize)
                })
                .sum::<Result<usize, String>>()
        })
        .sum::<Result<usize, String>>()?;
    if marker_count != rows.iter().filter(|row| matches!(row, EmittedNamedArrayRequirementV1::Write(_))).count() {
        return Err(fault("marker-cardinality"));
    }
    let read_count: usize = module.functions.iter().map(|(symbol, function)| {
        function.metadata.named_array_read_obligations.iter().map(|marker| {
            if !read_covered.contains(&(symbol.clone(), marker.read)) { return Err(fault("read-uncovered-marker")); }
            Ok(1usize)
        }).sum::<Result<usize, String>>()
    }).sum::<Result<usize, String>>()?;
    if read_count != read_covered.len() { return Err(fault("read-marker-domain-drift")); }
    for (symbol, function) in &module.functions {
        for site in function.metadata.named_array_field_allocations.keys() {
            if !claimed_providers.contains(&(symbol.clone(), site.clone())) {
                return Err(fault("unclaimed-provider-binding"));
            }
        }
    }
    Ok(())
}

impl SelectedSourceCoreMethodCallV1 {
    pub(crate) fn record_named_allocation(
        &mut self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
        destination: ValueId,
    ) -> Result<bool, String> {
        let Some(requirement) = self.contract().named_array_requirement() else {
            return Ok(false);
        };
        if requirement.construction() != Some(site) {
            return Ok(false);
        }
        if requirement.owner() != owner {
            return Err(fault("allocation-source-mismatch"));
        }
        if self.allocation.is_some() {
            return Err(fault("duplicate-allocation"));
        }
        self.allocation = Some(destination);
        Ok(true)
    }
}

/// Recipe clones share one consumption cell; they never clone the contract.
#[derive(Debug, Clone)]
pub(crate) struct NamedArrayWriteEmissionPortV1(std::rc::Rc<std::cell::RefCell<WriteState>>);

#[derive(Debug)]
enum WriteState {
    Pending(NamedArrayEmissionDraftV1),
    Emitted(EmittedNamedArrayRequirementV1),
    Taken,
}

impl NamedArrayEmissionDraftV1 {
    pub(crate) fn with_receiver_read(mut self, read: crate::mir::builder::ExactLexicalReadV1) -> Result<Self, String> {
        let contract = self.source.contract();
        let requirement = contract.named_array_requirement().ok_or_else(|| fault("source-requirement-missing"))?;
        read.value_for(contract.owner(), contract.receiver_site().node(), requirement.binding())
            .map_err(|issue| issue.to_string())?;
        self.receiver_read = Some(std::rc::Rc::new(read));
        Ok(self)
    }
    pub(crate) fn into_write_port(self) -> NamedArrayWriteEmissionPortV1 {
        NamedArrayWriteEmissionPortV1(std::rc::Rc::new(std::cell::RefCell::new(
            WriteState::Pending(self),
        )))
    }
}

impl NamedArrayWriteEmissionPortV1 {
    pub(crate) fn is_field_residence_v1(&self) -> Result<bool, String> {
        let state = self.0.borrow();
        let WriteState::Pending(draft) = &*state else { return Err(fault("duplicate-physical-write")); };
        Ok(matches!(draft.source.contract().named_array_requirement(), Some(NamedArrayRequirementV1::FieldResidence(_))))
    }
    /// Canonical source family selection; absence of a required source proof
    /// is an error after FieldResidence selection, never raw-route fallback.
    pub(crate) fn field_append_source_v1(&self, ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1)
        -> Result<(super::FieldArrayAppendSourceLoanV1, std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>), String> {
        let state = self.0.borrow();
        let WriteState::Pending(draft) = &*state else { return Err(fault("duplicate-physical-write")); };
        if !matches!(draft.source.contract().named_array_requirement(), Some(NamedArrayRequirementV1::FieldResidence(_))) { return Err(fault("field-residence-required")); }
        let source = ledger.field_array_append_source_v1(draft.source.contract())?;
        let receiver = draft.receiver_read.as_ref().ok_or_else(|| fault("receiver-read-missing"))?;
        Ok((source, std::rc::Rc::clone(receiver)))
    }
    pub(crate) fn record_write(
        &self,
        write: ArrayWriteSiteId,
        receiver: ValueId,
        argument: ValueId,
    ) -> Result<NamedArrayWriteMarkerV1, String> {
        let mut state = self.0.borrow_mut();
        let WriteState::Pending(draft) = std::mem::replace(&mut *state, WriteState::Taken) else {
            return Err(fault("duplicate-physical-write"));
        };
        let requirement = draft
            .source
            .contract()
            .named_array_requirement()
            .ok_or_else(|| fault("source-requirement-missing"))?;
        let (owner, site, argument_site) = (
            requirement.owner(),
            requirement.call().clone(),
            requirement.argument().clone(),
        );
        let row = draft.record_write(owner, &site, &argument_site, write, receiver, argument)?;
        let marker = row.marker().ok_or_else(|| fault("write-marker-missing"))?;
        *state = WriteState::Emitted(row);
        Ok(marker)
    }

    pub(crate) fn record_checked_write_v1(&self, packet: std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedFieldArrayAppendV1>)
        -> Result<NamedArrayWriteMarkerV1, String> {
        {
            let state = self.0.borrow();
            let WriteState::Pending(draft) = &*state else { return Err(fault("duplicate-physical-write")); };
            if !matches!(draft.source.contract().named_array_requirement(), Some(NamedArrayRequirementV1::FieldResidence(original))
                if packet.source().integer_requirement().is_ok_and(|requirement| std::rc::Rc::ptr_eq(original, requirement)))
                || draft.receiver_read.as_ref().is_none_or(|original| !packet.borrows_receiver_read_v1(original)) {
                return Err(fault("checked-source-mismatch"));
            }
        }
        let crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ArrayElementWrite { site_id, receiver, value, .. }, .. } = &packet.invoke_binding().1
            else { return Err(fault("checked-invoke-missing")); };
        let marker = self.record_write(*site_id, *receiver, *value)?;
        let mut state = self.0.borrow_mut();
        let WriteState::Emitted(EmittedNamedArrayRequirementV1::Write(row)) = &mut *state else { return Err(fault("checked-state-drift")); };
        row.checked = Some(packet);
        Ok(marker)
    }
    pub(crate) fn take_completed(&self) -> Result<EmittedNamedArrayRequirementV1, String> {
        match std::mem::replace(&mut *self.0.borrow_mut(), WriteState::Taken) {
            WriteState::Emitted(row) => Ok(row),
            WriteState::Pending(_) => Err(fault("write-not-emitted")),
            WriteState::Taken => Err(fault("duplicate-emission-consumption")),
        }
    }
}
