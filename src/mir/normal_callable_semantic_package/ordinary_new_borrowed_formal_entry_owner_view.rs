//! One borrowed entry view for a global owner or a completed target-static cohort.
//! The target branch borrows the original source-only draft and never mutates
//! the global transport graph.
use super::super::borrowed_formal_uses::{
    BorrowedFormalUsesDraftV1, BorrowedIncomingCallDraftV1, BorrowedIncomingSourceV1,
};
use super::*;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct BorrowedEntryOwnerViewV1
<'a> {
    pub(super) definition: &'a BorrowedFormalUsesDraftV1,
    pub(super) incoming: Box<[&'a BorrowedIncomingCallDraftV1]>,
    pub(super) tagged: bool,
    pub(super) mixed: bool,
}

impl OrdinaryNewClaimLedgerV1 {
    /// Original Instance input selection, shared by finishing and demand.
    /// The entry view is source qualification only; full input gates stay separate.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_source_instance_input_entry_v1<'a>(
        &'a self, source: &'a PreparedBorrowedFormalIngressV1, owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("instance-input/original-catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts.next().ok_or_else(|| freeze("instance-input/contract-missing"))?;
        if contracts.next().is_some() { return Err(freeze("instance-input/contract-duplicate")); }
        if contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod {
            return Ok(None);
        }
        // Reuse global, guarded and mixed source laws; no SourceInstance tag
        // alone qualifies a declaration and no absent incoming is synthesized.
        self.checked_entry_owner_view_v1(source, owner)
    }

    /// Select the original input responsibility without inventing a body draft.
    /// Both finishing and demand still use the whole-input cohort verifier.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn selected_source_instance_input_owner_v1(
        &self, source: &PreparedBorrowedFormalIngressV1, owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
        Ok(Self::original_finite_text_instance_input_request_v1(source, catalog, owner)?
            || self.checked_source_instance_input_entry_v1(source, owner)?.is_some()
            || self.selected_original_instance_input_kind_v1(source, owner)?
                == Some(super::super::borrowed_formal_actuals::object_input_finish::OriginalInstanceInputKindV1::SourceInstance))
    }

    pub(super) fn checked_entry_owner_view_v1<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
        if let Some(cohort) = source.target_static.get(&owner) {
            if source.definitions.contains_key(&owner) || cohort.owner != owner {
                return Err(freeze("borrowed-entry/target-owner-overlap"));
            }
            let definition = source
                .source_only_definitions
                .get(&owner)
                .ok_or_else(|| freeze("borrowed-entry/target-definition-missing"))?;
            if cohort.incoming.len() != 2 {
                return Err(freeze("borrowed-entry/target-cardinality"));
            }
            for call in &cohort.incoming {
                let BorrowedIncomingSourceV1::Static(original) = &call.source else {
                    return Err(freeze("borrowed-entry/target-source-kind"));
                };
                if call.callee != owner
                    || original.call_site() != &call.call
                    || self
                        .checked_completed_static_one_actuals_v1(original)?
                        .is_none()
                {
                    return Err(freeze("borrowed-entry/target-actuals-unfinished"));
                }
            }
            return Ok(Some(BorrowedEntryOwnerViewV1 {
                definition,
                incoming: cohort.incoming.iter().collect(),
                tagged: false,
                mixed: false,
            }));
        }
        if let Some(definition) = source.source_only_definitions.get(&owner) {
            if let Some(view) = self.checked_borrowed_result_instance_entry_v1(source, owner)? {
                return Ok(Some(view));
            }
            if let Some(view) = self.checked_guarded_instance_entry_v1(source, owner)? {
                return Ok(Some(view));
            }
            if let Some(view) = self.checked_mixed_instance_entry_v1(source, owner)? {
                return Ok(Some(view));
            }
            let catalog = self.original_parameter_contracts.as_deref()
                .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
            let mut core_input = false;
            for call in source.source_incoming.exact_rows().filter(|row| row.callee == owner) {
                core_input |= super::super::borrowed_formal_actuals::original_core_argument_input_request_v1(
                    source, catalog, &call.call)?;
            }
            if core_input || self.selected_original_object_input_owner_v1(source, owner)?
                || source.original_returned_field_participant_request_v1(owner)?.is_some() {
                return mixed_instance::checked_original_instance_parameter_frame_v1(
                    source, self.original_parameter_contracts.as_deref(), owner);
            }
            // Static finishers cannot interpret an Instance declaration's empty
            // incoming as a Static execution veto. Mode is original source law.
            let catalog = self
                .original_parameter_contracts
                .as_deref()
                .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
            let mut contracts = catalog.iter().filter(|row| row.owner == owner);
            let contract = contracts
                .next()
                .ok_or_else(|| freeze("borrowed-entry/original-contract-missing"))?;
            if contracts.next().is_some() {
                return Err(freeze("borrowed-entry/original-contract-duplicate"));
            }
            if contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod {
                return Ok(None);
            }
            if let Some(incoming) = self.checked_completed_static_tagged_entry_v1(source, owner)? {
                return Ok(Some(BorrowedEntryOwnerViewV1 {
                    definition,
                    incoming,
                    tagged: true,
                    mixed: false,
                }));
            }
            if let Some(incoming) = self.checked_completed_static_bool_entry_v1(source, owner)? {
                return Ok(Some(BorrowedEntryOwnerViewV1 {
                    definition,
                    incoming,
                    tagged: false,
                    mixed: false,
                }));
            }
            if let Some(incoming) = self.checked_completed_static_scalar_cohort_v1(source, owner)? {
                return Ok(Some(BorrowedEntryOwnerViewV1 {
                    definition,
                    incoming,
                    tagged: false,
                    mixed: false,
                }));
            }
            if let Ok(rows) = source
                .source_incoming
                .project(&std::collections::BTreeSet::from([owner]))
            {
                if let Some(first) = rows.first() {
                    if let BorrowedIncomingSourceV1::Static(original) = &first.source {
                        if let Some(incoming) =
                            self.checked_completed_static_mixed_cohort_v1(source, original)?
                        {
                            return Ok(Some(BorrowedEntryOwnerViewV1 {
                                definition,
                                incoming,
                                tagged: false,
                                mixed: false,
                            }));
                        }
                    }
                }
            }
        }
        let Some(definition) = source.definitions.get(&owner) else {
            return Ok(None);
        };
        // Reconstruction checks complete original/transport correspondence.
        // Equal row projections need no scalar-result graph just to choose
        // their view; entry_actual_loans still demands the selected full owner.
        let original_input = self.original_instance_input_kind_v1(source, owner)?.is_some()
            && self.original_instance_input_requires_reconstruction_v1(source, owner)?;
        let incoming = if original_input {
            source.source_incoming.project_declaration_v1(owner).map_err(|issue| {
                format!("{}: {issue:?}", freeze("borrowed-entry/original-incoming-coverage"))
            })?;
            source.source_incoming.exact_rows().filter(|call| call.callee == owner).collect()
        } else {
            source.incoming.iter().filter(|call| call.callee == owner).collect()
        };
        Ok(Some(BorrowedEntryOwnerViewV1 {
            definition,
            incoming,
            tagged: false,
            mixed: false,
        }))
    }
}

impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    /// Loan the original target, including its source owner and batch identity.
    pub(crate) fn incoming_targets(
        &self,
    ) -> impl Iterator<Item = Result<super::borrowed_formal_uses::BorrowedCallSourceLoanV1<'_>, String>>
    {
        self.incoming_rows
            .iter()
            .filter(move |row| row.callee == self.owner)
            .map(|row| {
                let loan = row.source.as_loan()?;
                if loan.call_site() != &row.call
                    || loan.callee_owner() != row.callee
                    || row.callee != self.owner
                {
                    return Err(freeze("borrowed-entry/incoming-source-identity"));
                }
                if let super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                    &row.source
                {
                    if !original.is_qualified_i64_source_v1()
                        && !matches!(self.incoming, BorrowedEntryActualsV1::SelectedTagged)
                        && !((original.is_current_owner_i64_source_v1()
                            || (original.is_current_owner_bool_source_v1()
                                && self
                                    .source
                                    .source_only_definitions
                                    .contains_key(&self.owner)))
                            && self.incoming().is_ok_and(|rows| {
                                rows.iter()
                                    .any(|(site, actuals)| *site == &row.call && actuals.len() == 1)
                            }))
                    {
                        original.require_qualified_i64_source_v1()?;
                    }
                    let retained = self
                        .source
                        .source_incoming
                        .static_observations()
                        .get(&row.call)
                        .ok_or_else(|| freeze("borrowed-entry/incoming-source-missing"))?
                        .as_ref()
                        .map_err(Clone::clone)?;
                    if !Rc::ptr_eq(retained, original) {
                        return Err(freeze("borrowed-entry/incoming-source-drift"));
                    }
                }
                Ok(loan)
            })
    }
}

/// Canonical source correspondence does not grant an executable actual loan.
#[derive(Debug)]
pub(super) enum BorrowedEntryActualsV1<'a> {
    Executable(Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>),
    SelectedTagged,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn entry_actual_loans_v1<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        view: &BorrowedEntryOwnerViewV1<'a>,
    ) -> Result<BorrowedEntryActualsV1<'a>, String> {
        if view.mixed {
            if view.incoming.is_empty() {
                return Ok(BorrowedEntryActualsV1::SelectedTagged);
            }
            return self
                .checked_mixed_instance_actuals_v1(source, owner, view)
                .map(BorrowedEntryActualsV1::Executable);
        }
        if view.tagged {
            return Ok(BorrowedEntryActualsV1::SelectedTagged);
        }
        if self.selected_instance_input_protocol_v1(source, owner)?.is_some() {
            let (selected, signatures) = self.original_instance_input_issuance.as_ref()
                .ok_or_else(|| freeze("borrowed-entry/original-issuance-missing"))?;
            let catalog = self.original_parameter_contracts.as_deref()
                .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
            let first = view.incoming.first()
                .ok_or_else(|| freeze("borrowed-entry/incoming-missing"))?;
            if self.checked_completed_original_instance_target_arguments_v1(
                first.source.require_instance()?, selected, catalog, signatures,
            )?.is_none() {
                return Err(format!("{}: {}", freeze("borrowed-entry/original-input-not-ready"),
                    self.typed_object_input_completion_diagnostic_v1(owner)?));
            }
            let mut rows = Vec::new();
            for call in &view.incoming {
                if call.callee != owner {
                    return Err(freeze("borrowed-entry/incoming-identity"));
                }
                let actuals = self.borrowed_formal_actuals.get(&call.call)
                    .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                    .as_ref().map_err(Clone::clone)?;
                actuals.ordered_arguments_for_v1(call)?;
                self.check_entry_actual_domains_v1(source, view.definition, &actuals.opaque_actuals)?;
                rows.push((&call.call, actuals.opaque_actuals.as_ref()));
            }
            return Ok(BorrowedEntryActualsV1::Executable(rows.into_boxed_slice()));
        }
        self.checked_borrowed_entry_incoming(source, owner)
            .map(BorrowedEntryActualsV1::Executable)
    }
}

/// Execution dependency is determined by original forwards, never pending phase.
impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn checked_static_execution_owners_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        calls: &[&BorrowedIncomingCallDraftV1],
    ) -> Result<BTreeSet<FunctionOwnerIdV1>, String> {
        use super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1;
        let mut owners = BTreeSet::from([owner]);
        loop {
            let mut added = BTreeSet::new();
            for forward in source
                .forwards
                .iter()
                .filter(|row| owners.contains(&row.callee_formal.owner()))
            {
                let matching: Vec<_> = calls
                    .iter()
                    .filter(|call| call.call == forward.call)
                    .collect();
                let [call] = matching.as_slice() else {
                    return Err(freeze("borrowed-static/forward-incoming-identity"));
                };
                let actuals = self
                    .borrowed_formal_actuals
                    .get(&call.call)
                    .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                    .as_ref()
                    .map_err(Clone::clone)?;
                let actual: Vec<_> = actuals
                    .opaque_actuals
                    .iter()
                    .filter(|actual| actual.ordinal == forward.ordinal)
                    .collect();
                let [actual] = actual.as_slice() else {
                    return Err(freeze("borrowed-static/forward-actual-identity"));
                };
                if call.callee != forward.callee_formal.owner()
                    || call.call.owner() != forward.source_formal.owner()
                    || call.source.target() != &forward.target
                    || forward.site.owner() != call.call.owner()
                    || actual.site != *forward.site.site()
                    || actual.formal != forward.callee_formal
                    || !call.arguments.iter().any(|(ordinal, site, formal)| {
                        *ordinal == forward.ordinal
                            && site == forward.site.site()
                            && *formal == forward.callee_formal
                    })
                    || actual.source
                        != (BorrowedFormalActualSourceV1::Forwarded {
                            binding: forward.binding,
                            formal: forward.source_formal,
                        })
                    || source
                        .definitions
                        .get(&forward.source_formal.owner())
                        .and_then(|draft| draft.origins.get(&forward.binding))
                        != Some(&forward.source_formal)
                    || source
                        .definitions
                        .get(&forward.callee_formal.owner())
                        .and_then(|draft| draft.origins.get(&forward.callee_formal))
                        != Some(&forward.callee_formal)
                {
                    return Err(freeze("borrowed-static/forward-source-drift"));
                }
                added.insert(forward.source_formal.owner());
            }
            // Source-only guarded edges were intentionally excluded from transport
            // forwards. Reconstruct their exact original input before adding the
            // caller dependency; a phase alone cannot supply an edge.
            let catalog = self
                .original_parameter_contracts
                .as_deref()
                .ok_or_else(|| freeze("borrowed-static/original-catalog-missing"))?;
            for call in calls.iter().filter(|row| owners.contains(&row.callee)) {
                let actuals = self
                    .borrowed_formal_actuals
                    .get(&call.call)
                    .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                    .as_ref()
                    .map_err(Clone::clone)?;
                if super::super::borrowed_formal_actuals::revalidate_guarded_object_incoming_v1(
                    source, catalog, call, actuals,
                    Some(self),
                )? || super::super::borrowed_formal_actuals::revalidate_guarded_instance_incoming_v1(
                    self, source, catalog, call, actuals,
                )? {
                    added.insert(call.call.owner());
                }
            }
            let old_len = owners.len();
            owners.extend(added);
            if owners.len() == old_len {
                break;
            }
        }
        let mut executable = owners.clone();
        for dependency in owners.iter().filter(|dependency| **dependency != owner) {
            if !source.definitions.contains_key(dependency)
                || !calls.iter().any(|call| call.callee == *dependency)
            {
                if let Some(view) = self.checked_mixed_instance_entry_v1(source, *dependency)? {
                    if view.incoming.is_empty() {
                        executable.remove(dependency);
                    } else {
                        self.checked_mixed_instance_actuals_v1(source, *dependency, &view)?;
                    }
                    continue;
                }
                let view = self
                    .checked_guarded_instance_entry_v1(source, *dependency)?
                    .ok_or_else(|| freeze("borrowed-static/forward-entry-missing"))?;
                if !view.tagged || !view.incoming.is_empty() {
                    return Err(freeze("borrowed-static/forward-entry-missing"));
                }
                executable.remove(dependency);
            }
        }
        let originals = source
            .source_incoming
            .project(&executable)
            .map_err(|error| {
                format!("{}: {error:?}", freeze("borrowed-static/incoming-coverage"))
            })?;
        let selected: Vec<_> = calls
            .iter()
            .filter(|call| executable.contains(&call.callee))
            .copied()
            .collect();
        if selected.len() != originals.len() {
            return Err(freeze("borrowed-static/incoming-coverage"));
        }
        for original in &originals {
            let matching: Vec<_> = selected
                .iter()
                .filter(|call| call.call == original.call)
                .collect();
            let [call] = matching.as_slice() else {
                return Err(freeze("borrowed-static/incoming-coverage"));
            };
            let source_agrees = call.source.same_source(&original.source);
            if call.callee != original.callee
                || call.arguments != original.arguments
                || !source_agrees
            {
                return Err(freeze("borrowed-static/incoming-source-drift"));
            }
            let actuals = self
                .borrowed_formal_actuals
                .get(&call.call)
                .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let catalog = self
                .original_parameter_contracts
                .as_deref()
                .ok_or_else(|| freeze("borrowed-static/original-catalog-missing"))?;
            let guarded =
                super::super::borrowed_formal_actuals::revalidate_guarded_object_incoming_v1(
                    source, catalog, call, actuals,
                    Some(self),
                )? || super::super::borrowed_formal_actuals::revalidate_guarded_instance_incoming_v1(
                    self, source, catalog, call, actuals,
                )?;
            for actual in &actuals.opaque_actuals {
                if let BorrowedFormalActualSourceV1::Forwarded { binding, formal } = &actual.source
                {
                    if source
                        .forwards
                        .iter()
                        .filter(|row| {
                            row.call == call.call
                                && row.target == *call.source.target()
                                && row.ordinal == actual.ordinal
                                && row.site.site() == &actual.site
                                && row.binding == *binding
                                && row.source_formal == *formal
                                && row.callee_formal == actual.formal
                        })
                        .count()
                        != 1
                        && !guarded
                    {
                        return Err(freeze("borrowed-static/forward-source-drift"));
                    }
                }
            }
        }
        Ok(owners)
    }
}

/// Declaration-bound tagged entry for the existing guarded Object forward shape.
/// Empty original incoming is not an executable actual cohort.
impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn checked_guarded_instance_entry_v1<'a>(
        &self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
        let Some(definition) = checked_guarded_instance_source_v1(
            source,
            self.original_parameter_contracts.as_deref(),
            owner,
        )?
        else {
            return Ok(None);
        };
        Ok(Some(BorrowedEntryOwnerViewV1 {
            definition,
            incoming: source
                .source_incoming
                .exact_rows()
                .filter(|row| row.callee == owner)
                .collect(),
            tagged: true,
            mixed: false,
        }))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_guarded_instance_definition_v1<
        'a,
    >(
        &self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<&'a BorrowedFormalUsesDraftV1>, String> {
        Ok(self
            .checked_guarded_instance_entry_v1(source, owner)?
            .map(|view| view.definition))
    }

    /// Supplied entry parameters cannot forge a kind while retaining its roots.
    pub(super) fn check_original_entry_parameters_v1(
        &self,
        owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<(), String> {
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts
            .next()
            .ok_or_else(|| freeze("borrowed-entry/original-contract-missing"))?;
        if contracts.next().is_some()
            || parameters.len() != contract.parameters.len()
            || parameters.iter().zip(&contract.parameters).any(
                |((ordinal, binding, kind), original)| {
                    *ordinal != original.ordinal
                        || *binding != original.binding
                        || *kind != original.kind
                },
            )
        {
            return Err(freeze("borrowed-entry/original-parameter-drift"));
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Complete-empty incoming borrows the same qualified Instance entry law.
    /// No executable actual or physical Call is manufactured for a declaration.
    pub(crate) fn checked_borrowed_declaration_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<BorrowedOrdinaryEntrySourceRefV1<'_>, String> {
        let source = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("borrowed-declaration/source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let incoming = source.source_incoming.project_declaration_v1(owner)
            .map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-declaration/incoming")))?;
        if !incoming.is_empty() {
            let calls: Vec<_> = incoming.iter().map(|row| &row.call).collect();
            return Err(format!("{}: {calls:?}", freeze("borrowed-declaration/missing-retained-call")));
        }
        let view = self.checked_source_instance_input_entry_v1(source, owner)?
            .ok_or_else(|| freeze("borrowed-declaration/profile"))?;
        if !view.incoming.is_empty() {
            return Err(freeze("borrowed-declaration/nonempty-incoming"));
        }
        self.finalized_borrowed_ordinary_entry_source_v1(owner)
    }

}

/// Shared original declaration/use lender; no executable or physical authority.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_guarded_instance_source_v1<
    'a,
>(
    source: &'a PreparedBorrowedFormalIngressV1,
    catalog: Option<&[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1]>,
    owner: FunctionOwnerIdV1,
) -> Result<Option<&'a BorrowedFormalUsesDraftV1>, String> {
    use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
    use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1 as Mode;
    let Some(draft) = source.source_only_definitions.get(&owner) else {
        return Ok(None);
    };
    if !source
        .guarded_actuals
        .keys()
        .any(|(call, _)| call.owner() == owner)
    {
        return Ok(None);
    }
    let catalog = catalog.ok_or_else(|| freeze("borrowed-guarded/original-catalog-missing"))?;
    let mut contracts = catalog.iter().filter(|row| row.owner == owner);
    let contract = contracts
        .next()
        .ok_or_else(|| freeze("borrowed-guarded/declaration-missing"))?;
    if contracts.next().is_some() {
        return Err(freeze("borrowed-guarded/declaration-duplicate"));
    }
    if contract.mode != Mode::InstanceBoxMethod
        || contract.parameters.is_empty()
        || contract
            .parameters
            .iter()
            .any(|row| row.kind != CallableParameterContractKindV1::OpaqueHandle)
    {
        return Ok(None);
    }
    let roots: BTreeSet<_> = draft.origins.values().copied().collect();
    if roots != contract.parameters.iter().map(|row| row.binding).collect()
        || contract.parameters.iter().enumerate().any(|(index, row)| {
            row.ordinal as usize != index
                || row.binding.owner() != owner
                || draft.origins.get(&row.binding) != Some(&row.binding)
        })
    {
        return Err(freeze("borrowed-guarded/declaration-formals"));
    }
    let mut compares = 0;
    let mut arguments = 0;
    for row in &draft.uses {
        if row.site.owner() != owner
            || row.formal.owner() != owner
            || draft.origins.get(&row.binding) != Some(&row.formal)
        {
            return Err(freeze("borrowed-guarded/use-identity"));
        }
        match &row.kind {
            Use::CompareOperand { .. } => compares += 1,
            Use::UnresolvedArgument { call, ordinal } => {
                let Some(fact) = source.guarded_actuals.get(&(call.clone(), *ordinal)) else {
                    return Ok(None);
                };
                if !fact.matches_draft_argument(row, call, *ordinal, draft) {
                    return Err(freeze("borrowed-guarded/argument-guard"));
                }
                let mut calls = source
                    .source_incoming
                    .exact_rows()
                    .filter(|original| original.call == *call);
                let original = calls
                    .next()
                    .ok_or_else(|| freeze("borrowed-guarded/call-missing"))?;
                if calls.next().is_some() {
                    return Err(freeze("borrowed-guarded/call-duplicate"));
                }
                let Some(target) = original.source.instance() else {
                    return Ok(None);
                };
                if !source.source_incoming.has_object_input_callee_v1(target) {
                    return Ok(None);
                }
                if call.owner() != owner
                    || target.call_site() != call
                    || original.callee != target.callee_owner()
                    || target.argument_sites().get(*ordinal as usize) != Some(row.site.site())
                    || original
                        .arguments
                        .iter()
                        .filter(|(index, site, formal)| {
                            *index == *ordinal
                                && site == row.site.site()
                                && formal.owner() == original.callee
                        })
                        .count()
                        != 1
                {
                    return Err(freeze("borrowed-guarded/call-identity"));
                }
                arguments += 1;
            }
            _ => return Ok(None),
        }
    }
    if compares == 0 || arguments == 0 {
        return Ok(None);
    }
    let projected = source
        .source_incoming
        .project_declaration_v1(owner)
        .map_err(|error| {
            format!(
                "{}: {error:?}",
                freeze("borrowed-guarded/incoming-coverage")
            )
        })?;
    let incoming: Vec<_> = source
        .source_incoming
        .exact_rows()
        .filter(|row| row.callee == owner)
        .collect();
    let modes = incoming.iter().map(|row|
        row.source.as_loan().map(|loan| loan.declaration_mode())
    ).collect::<Result<Vec<_>, String>>()?;
    if incoming.len() != projected.len()
        || modes.iter().any(|mode| *mode != contract.mode)
    {
        return Err(freeze("borrowed-guarded/incoming-mode"));
    }
    Ok(Some(draft))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Same qualified complete-empty Instance entries, without exposing drafts.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_declaration_owners_v1(
        &self,
    ) -> Result<Vec<FunctionOwnerIdV1>, String> {
        let Some(source) = &self.borrowed_formal_source else {
            return Ok(Vec::new());
        };
        let source = source.as_ref().map_err(Clone::clone)?;
        let mut owners = Vec::new();
        for owner in source.source_only_definitions.keys().chain(source.definitions.keys())
            .copied().collect::<BTreeSet<_>>() {
            if self
                .checked_source_instance_input_entry_v1(source, owner)?
                .is_some()
                && source
                    .source_incoming
                    .project_declaration_v1(owner)
                    .map_err(|issue| {
                        format!("{}: {issue:?}", freeze("borrowed-declaration/incoming"))
                    })?
                    .is_empty()
            {
                owners.push(owner);
            }
        }
        Ok(owners)
    }
}

#[path = "ordinary_new_borrowed_instance_declaration.rs"]
mod mixed_instance;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
use mixed_instance::checked_forwarding_instance_source_v1;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn original_object_input_parameter_frame_v1(
        &self, source: &PreparedBorrowedFormalIngressV1, owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        Ok(mixed_instance::checked_original_instance_parameter_frame_v1(
            source, self.original_parameter_contracts.as_deref(), owner)?.is_some())
    }
}


pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal)
use mixed_instance::original_borrowed_instance_input_completion_needed_v1;
