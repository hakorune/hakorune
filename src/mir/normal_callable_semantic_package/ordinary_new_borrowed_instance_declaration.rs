//! Original mixed declaration frames; body and input execution stay separate.
use super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
use super::*;

/// The same original forwarding frame is used before Completion selection and
/// during Object packet demand. This lends source membership, never a result.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_forwarding_instance_source_v1<'a>(
    source: &'a PreparedBorrowedFormalIngressV1,
    catalog: Option<&[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1]>,
    owner: FunctionOwnerIdV1,
) -> Result<Option<&'a BorrowedFormalUsesDraftV1>, String> {
    if let Some(draft) = checked_guarded_instance_source_v1(source, catalog, owner)? {
        return Ok(Some(draft));
    }
    Ok(checked_mixed_instance_entry_source_v1(source, catalog, owner)?
        .map(|view| view.definition))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Lend the complete original nullable-forward declaration, never a new frame.
    /// All input rows and executable operands remain separate prerequisites.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_borrowed_result_instance_entry_v1<'a>(
        &'a self, source: &'a PreparedBorrowedFormalIngressV1, owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
        let Some(definition) = source.source_only_definitions.get(&owner) else { return Ok(None); };
        if !definition.uses.iter().any(|row| matches!(row.kind, Use::BorrowedFormalReturn { .. })) {
            return Ok(None);
        }
        // Passive Return evidence alone never selects executable input reconstruction.
        // The original retained LOCAL borrower request selects this same declaration.
        let mut demanded = false;
        for row in source.source_incoming.exact_rows().filter(|row| row.callee == owner) {
            let Some(target) = row.source.instance() else { continue; };
            let Some(loan) = target.borrowed_nullable_local_source() else { continue; };
            if loan.callee_owner() != owner || target.callee_owner() != owner
                || loan.call() != &row.call || target.call_site() != &row.call
                || loan.key() != target.target()
            { return Err(freeze("borrowed-result-entry/demand-identity")); }
            let roots = self.callable_result_classes
                .checked_original_source_roots_v1(target.target(), owner)
                .map_err(|issue| freeze(issue))?;
            self.callable_result_classes.check_borrowed_nullable_roots_v1(loan, &roots)
                .map_err(|issue| freeze(issue))?;
            demanded = true;
        }
        if !demanded { return Ok(None); }
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("borrowed-result-entry/catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts.next().ok_or_else(|| freeze("borrowed-result-entry/contract-missing"))?;
        if contracts.next().is_some() { return Err(freeze("borrowed-result-entry/contract-duplicate")); }
        if contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || contract.parameters.iter().any(|row| !row.kind.is_ordinary_borrowed_handle()
                && !matches!(row.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
        { return Ok(None); }
        let roots: BTreeSet<_> = definition.origins.values().copied().collect();
        if roots != contract.parameters.iter().filter(|row| row.kind.is_ordinary_borrowed_handle())
            .map(|row| row.binding).collect()
            || contract.parameters.iter().enumerate().any(|(ordinal, row)|
                row.ordinal as usize != ordinal || row.binding.owner() != owner)
        { return Err(freeze("borrowed-result-entry/formal-identity")); }
        let mut result_key = None;
        for row in &definition.uses {
            if row.site.owner() != owner || definition.origins.get(&row.binding) != Some(&row.formal) {
                return Err(freeze("borrowed-result-entry/use-identity"));
            }
            match &row.kind {
                Use::BorrowedFormalReturn { exit, source: loan } => {
                    self.callable_result_classes.recheck_borrowed_formal_return_source_v1(loan)
                        .map_err(|issue| freeze(issue))?;
                    if loan.value() != &row.site || loan.formal() != row.formal
                        || crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
                            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Value).expr() != *row.site.site()
                        || contract.parameters.get(loan.ordinal() as usize)
                            .is_none_or(|formal| formal.binding != row.formal)
                        || result_key.is_some_and(|key| key != loan.key())
                    { return Err(freeze("borrowed-result-entry/return-source")); }
                    result_key = Some(loan.key());
                }
                Use::FieldWriteReceiver { source: loan, guard } => {
                    self.field_write_claims.recheck_original_write_source_v1(loan).map_err(|issue| freeze(issue))?;
                    if loan.write().owner() != owner || loan.write().receiver_site() != row.site.site()
                        || !definition.uses.iter().any(|other| other.formal == row.formal
                            && matches!(&other.kind, Use::NullCompareOperand { binary }
                                if crate::mir::resolved_semantics::SourcePathV1::from_node(guard)
                                    .child(crate::mir::resolved_semantics::SourcePathSegmentV1::IfCondition).expr() == *binary.site()))
                    { return Err(freeze("borrowed-result-entry/write-receiver")); }
                }
                Use::FieldWriteValue { source: loan, reach } => {
                    self.field_write_claims.recheck_original_write_source_v1(loan).map_err(|issue| freeze(issue))?;
                    if loan.write().owner() != owner || loan.write().assignment().value_site() != row.site.site()
                        || reach.operand() != (row.binding, row.formal, &row.site)
                        || !definition.uses.iter().any(|other| other.formal == row.formal
                            && matches!(&other.kind, Use::CompareOperand { binary, source }
                                if reach.guard().matches_compare(row.formal, binary, source)))
                    { return Err(freeze("borrowed-result-entry/write-value")); }
                }
                _ => {}
            }
        }
        let original = source.source_incoming.project_declaration_v1(owner)
            .map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-result-entry/incoming")))?;
        if original.is_empty() { return Ok(None); }
        if original.iter().any(|row| row.callee != owner
            || row.source.instance().is_none_or(|target| target.callee_owner() != owner
                || target.target_batch_slot() != contract.batch_slot
                || Some(target.target()) != result_key))
        { return Err(freeze("borrowed-result-entry/incoming-identity")); }
        let incoming: Vec<_> = source.source_incoming.exact_rows().filter(|row| row.callee == owner).collect();
        if incoming.len() != original.len() { return Err(freeze("borrowed-result-entry/incoming-coverage")); }
        Ok(Some(BorrowedEntryOwnerViewV1 {
            definition, incoming: incoming.into_boxed_slice(), tagged: false, mixed: false,
        }))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_mixed_instance_entry_v1<
        'a,
    >(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
        checked_mixed_instance_entry_source_v1(
            source,
            self.original_parameter_contracts.as_deref(),
            owner,
        )
    }

    pub(in super::super) fn checked_mixed_instance_actuals_v1<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        view: &BorrowedEntryOwnerViewV1<'a>,
    ) -> Result<Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>, String> {
        if !view.mixed || view.incoming.is_empty() {
            return Err(freeze("mixed-entry/nonempty-required"));
        }
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("mixed-entry/catalog-missing"))?;
        let mut rows = Vec::new();
        for original in &view.incoming {
            if original.callee != owner {
                return Err(freeze("mixed-entry/incoming-owner"));
            }
            let actuals = self
                .borrowed_formal_actuals
                .get(&original.call)
                .ok_or_else(|| freeze("mixed-entry/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            super::super::super::borrowed_formal_actuals::recheck_finished_instance_input_v1(
                self, source, catalog, original, actuals,
            )?;
            actuals.ordered_arguments_for_v1(original)?;
            self.check_entry_actual_domains_v1(source, view.definition, &actuals.opaque_actuals)?;
            for actual in &actuals.opaque_actuals {
                if view.definition.origins.get(&actual.formal) != Some(&actual.formal) {
                    return Err(freeze("mixed-entry/actual-formal"));
                }
            }
            rows.push((&original.call, actuals.opaque_actuals.as_ref()));
        }
        Ok(rows.into_boxed_slice())
    }
}

/// One original declaration lender, also used before Completion selection.
/// Pure original declaration/domain join, shared by input and body profiles.
/// It never selects a pending phase, reconstructs an actual, or admits a use.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_original_instance_parameter_frame_v1<'a>(
    source: &'a PreparedBorrowedFormalIngressV1,
    catalog: Option<&[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1]>,
    owner: FunctionOwnerIdV1,
) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
    let Some(definition) = source.source_definition_for(owner) else { return Ok(None); };
    let catalog = catalog.ok_or_else(|| freeze("mixed-entry/catalog-missing"))?;
    let mut rows = catalog.iter().filter(|row| row.owner == owner);
    let contract = rows.next().ok_or_else(|| freeze("mixed-entry/contract-missing"))?;
    if rows.next().is_some() { return Err(freeze("mixed-entry/contract-duplicate")); }
    if contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
        || contract.parameters.iter().any(|row| !row.kind.is_ordinary_borrowed_handle()
            && !matches!(row.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
    { return Ok(None); }
    let roots: BTreeSet<_> = definition.origins.values().copied().collect();
    if roots != contract.parameters.iter().filter(|row| row.kind.is_ordinary_borrowed_handle()).map(|row| row.binding).collect()
        || contract.parameters.iter().enumerate().any(|(i, row)| {
            row.ordinal as usize != i
                || row.binding.owner() != owner
                || row.kind.is_ordinary_borrowed_handle() && definition.origins.get(&row.binding) != Some(&row.binding)
        })
        || definition.uses.iter().any(|row| {
            row.site.owner() != owner
                || row.formal.owner() != owner
                || definition.origins.get(&row.binding) != Some(&row.formal)
        })
    {
        return Err(freeze("mixed-entry/formal-identity"));
    }
    for formal in &contract.parameters {
        if let CallableParameterContractKindV1::DeclaredObject(class) = &formal.kind {
            let view = source.source_object_view_v1(formal.binding)?
                .ok_or_else(|| freeze("mixed-entry/declared-domain-missing"))?;
            if !view.is_declared() || view.class() != class.as_ref() {
                return Err(freeze("mixed-entry/declared-domain-drift"));
            }
        }
    }
    let originals = source
        .source_incoming
        .project_declaration_v1(owner)
        .map_err(|issue| format!("{}: {issue:?}", freeze("mixed-entry/incoming")))?;
    let modes = originals.iter().map(|row|
        row.source.as_loan().map(|loan| loan.declaration_mode())
    ).collect::<Result<Vec<_>, String>>()?;
    if originals.iter().any(|row| row.callee != owner)
        || modes.iter().any(|mode| *mode != contract.mode)
    {
        return Err(freeze("mixed-entry/incoming-mode"));
    }
    let incoming: Vec<_> = source
        .source_incoming
        .exact_rows()
        .filter(|row| row.callee == owner)
        .collect();
    if incoming.len() != originals.len() {
        return Err(freeze("mixed-entry/incoming-coverage"));
    }
    Ok(Some(BorrowedEntryOwnerViewV1 {
        definition,
        incoming: incoming.into_boxed_slice(),
        tagged: false,
        mixed: false,
    }))
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_mixed_instance_entry_source_v1<
    'a,
>(
    source: &'a PreparedBorrowedFormalIngressV1,
    catalog: Option<&[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1]>,
    owner: FunctionOwnerIdV1,
) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
    let Some(definition) = source.source_definition_for(owner) else {
        return Ok(None);
    };
    let catalog = catalog.ok_or_else(|| freeze("mixed-entry/catalog-missing"))?;
    let mut rows = catalog.iter().filter(|row| row.owner == owner);
    let contract = rows
        .next()
        .ok_or_else(|| freeze("mixed-entry/contract-missing"))?;
    if rows.next().is_some() {
        return Err(freeze("mixed-entry/contract-duplicate"));
    }
    if contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || !contract.parameters.iter().any(|row| matches!(row.kind, CallableParameterContractKindV1::DeclaredObject(_)))
            || !contract.parameters.iter().any(|row| row.kind == CallableParameterContractKindV1::OpaqueHandle)
            || contract.parameters.iter().any(|row| !row.kind.is_ordinary_borrowed_handle())
        { return Ok(None); }
    let Some(mut frame) = checked_original_instance_parameter_frame_v1(source, Some(catalog), owner)? else {
        return Ok(None);
    };
    for formal in &contract.parameters {
        match &formal.kind {
            CallableParameterContractKindV1::DeclaredObject(_) => {}
            CallableParameterContractKindV1::OpaqueHandle => {
                let compare = definition.uses.iter().any(|row| {
                    row.formal == formal.binding && matches!(row.kind, Use::CompareOperand { .. })
                });
                let mut guarded = false;
                for row in definition
                    .uses
                    .iter()
                    .filter(|row| row.formal == formal.binding)
                {
                    let Use::UnresolvedArgument { call, ordinal } = &row.kind else {
                        continue;
                    };
                    let Some(fact) = source.guarded_actuals.get(&(call.clone(), *ordinal)) else {
                        continue;
                    };
                    if !fact.matches_draft_argument(row, call, *ordinal, definition) {
                        return Err(freeze("mixed-entry/guard-identity"));
                    }
                    let mut incoming = source
                        .source_incoming
                        .exact_rows()
                        .filter(|original| &original.call == call);
                    let original = incoming
                        .next()
                        .ok_or_else(|| freeze("mixed-entry/guard-target-missing"))?;
                    if incoming.next().is_some()
                        || original.call.owner() != owner
                        || original.source.argument_sites().get(*ordinal as usize)
                            != Some(row.site.site())
                        || original
                            .arguments
                            .iter()
                            .filter(|(i, site, root)| {
                                i == ordinal
                                    && site == row.site.site()
                                    && root.owner() == original.callee
                            })
                            .count()
                            != 1
                    {
                        return Err(freeze("mixed-entry/guard-target-drift"));
                    }
                    guarded = true;
                }
                if !compare || !guarded {
                    return Ok(None);
                }
            }
            _ => return Ok(None),
        }
    }
    frame.mixed = true;
    Ok(Some(frame))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Borrow original mixed incoming rows required by the selected reverse graph.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_entry) fn original_static_execution_calls_v1<
        'a,
    >(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        initial: Vec<&'a BorrowedIncomingCallDraftV1>,
    ) -> Result<Vec<&'a BorrowedIncomingCallDraftV1>, String> {
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("borrowed-static/original-catalog-missing"))?;
        let mut calls = initial;
        let mut owners = BTreeSet::from([owner]);
        loop {
            let previous = (owners.len(), calls.len());
            let mut added = BTreeSet::new();
            for forward in source
                .forwards
                .iter()
                .filter(|row| owners.contains(&row.callee_formal.owner()))
            {
                added.insert(forward.source_formal.owner());
            }
            for call in calls.iter().filter(|row| owners.contains(&row.callee)) {
                let actuals = self
                    .borrowed_formal_actuals
                    .get(&call.call)
                    .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                    .as_ref()
                    .map_err(Clone::clone)?;
                if super::super::super::borrowed_formal_actuals::revalidate_guarded_object_incoming_v1(source, catalog, call, actuals,
        Some(self),
    )?
                    || super::super::super::borrowed_formal_actuals::revalidate_guarded_instance_incoming_v1(self, source, catalog, call, actuals)? {
                    added.insert(call.call.owner());
                }
            }
            owners.extend(added);
            for dependency in &owners {
                if let Some(view) = self.checked_mixed_instance_entry_v1(source, *dependency)? {
                    for original in view.incoming.iter().copied() {
                        if !calls.iter().any(|call| call.call == original.call) {
                            calls.push(original);
                        }
                    }
                }
            }
            if previous == (owners.len(), calls.len()) {
                break;
            }
        }
        Ok(calls)
    }
}

impl PreparedBorrowedFormalIngressV1 {
    /// Lend the existing selected mixed frame for an exact tagged BorrowForCall.
    /// This neither narrows the payload to Integer nor invents a call guard.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn original_mixed_tagged_forward_v1(
        &self, catalog: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
        call: &OwnedExprSiteV1, ordinal: u32, site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        binding: crate::mir::resolved_semantics::BindingRefV1, root: crate::mir::resolved_semantics::BindingRefV1,
    ) -> Result<Option<&super::super::super::borrowed_formal_uses::BorrowedFormalUsesDraftV1>, String> {
        let Some(view) = checked_mixed_instance_entry_source_v1(self, Some(catalog), call.owner())?
            else { return Ok(None); };
        let draft = view.definition;
        let mut originals = self.source_incoming.exact_rows().filter(|row| &row.call == call);
        let original = originals.next().ok_or_else(|| freeze("mixed-forward/original-missing"))?;
        let target = original.source.require_instance()?;
        let mut contracts = catalog.iter().filter(|row| row.owner == original.callee);
        let contract = contracts.next().ok_or_else(|| freeze("mixed-forward/callee-missing"))?;
        if originals.next().is_some() || contracts.next().is_some() || binding.owner() != call.owner()
            || root.owner() != call.owner() || target.call_site() != call || target.callee_owner() != original.callee
            || target.target_batch_slot() != contract.batch_slot || target.argument_sites().get(ordinal as usize) != Some(site)
            || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || target.argument_sites().len() != contract.parameters.len()
            || draft.origins.get(&binding) != Some(&root)
        { return Err(freeze("mixed-forward/original-identity")); }
        let Some(formal) = contract.parameters.get(ordinal as usize) else { return Err(freeze("mixed-forward/ordinal")); };
        if formal.ordinal != ordinal || formal.binding.owner() != original.callee
            || original.arguments.iter().filter(|(i, source, parameter)|
                *i == ordinal && source == site && *parameter == formal.binding).count() != 1
        { return Err(freeze("mixed-forward/callee-formal")); }
        if formal.kind != CallableParameterContractKindV1::OpaqueHandle { return Ok(None); }
        let mut uses = draft.uses.iter().filter(|row| row.site.site() == site);
        let row = uses.next().ok_or_else(|| freeze("mixed-forward/use-missing"))?;
        if uses.next().is_some() || row.site.owner() != call.owner() || row.binding != binding || row.formal != root
            || !matches!(&row.kind, Use::UnresolvedArgument { call: retained, ordinal: retained_ordinal }
                if retained == call && *retained_ordinal == ordinal)
        { return Err(freeze("mixed-forward/use-identity")); }
        Ok(Some(draft))
    }
}
