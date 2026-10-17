//! Original declaration and full incoming correspondence, before actual readiness.
//! This structure grants no executable actual, object view or compiled marker.
use super::*;
use super::owner_view::BorrowedEntryOwnerViewV1;

pub(super) struct BorrowedEntryStructureV1<'a> {
    owner: FunctionOwnerIdV1,
    source: &'a PreparedBorrowedFormalIngressV1,
    evidence: EntryStructureEvidenceV1<'a>,
    formals: Box<[(u32, BindingRefV1)]>,
}

enum EntryStructureEvidenceV1<'a> {
    Uses(BorrowedEntryOwnerViewV1<'a>),
    FiniteText {
        uses: Rc<super::super::borrowed_formal_uses::OriginalFormalUseSourceV1>,
        incoming: Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>,
        domains: BTreeMap<BindingRefV1, Rc<super::super::borrowed_formal_source::SourceValueSumV1>>,
    },
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn borrowed_entry_structure_for_contract(
        &self,
        owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<Option<BorrowedEntryStructureV1<'_>>, String> {
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
        if let Some(structure) = self.finite_text_entry_structure_v1(source, owner, parameters)? {
            return Ok(Some(structure));
        }
        let Some(view) = self.checked_entry_owner_view_v1(source, owner)? else {
            // A source use outside the closed transport profile was excluded
            // before selection. This is not retry after a selected failure.
            return Ok(None);
        };
        if view.mixed
            || self.selected_instance_input_protocol_v1(source, owner)?.is_some()
            || self
                .checked_guarded_instance_definition_v1(source, owner)?
                .is_some()
        {
            self.check_original_entry_parameters_v1(owner, parameters)?;
        }
        let definition = view.definition;
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
        for call in &view.incoming {
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
        Ok(Some(BorrowedEntryStructureV1 {
            owner, source, evidence: EntryStructureEvidenceV1::Uses(view), formals: formals.into_boxed_slice(),
        }))
    }

    pub(super) fn finish_borrowed_entry_source_v1<'a>(
        &'a self, structure: BorrowedEntryStructureV1<'a>,
    ) -> Result<BorrowedOrdinaryEntrySourceRefV1<'a>, String> {
        let EntryStructureEvidenceV1::Uses(view) = structure.evidence else {
            return Err(freeze("borrowed-entry/source-use-completion-pending"));
        };
        let incoming = self.entry_actual_loans_v1(structure.source, structure.owner, &view)?;
        Ok(BorrowedOrdinaryEntrySourceRefV1 {
            owner: structure.owner, formals: structure.formals, source: structure.source,
            definition: view.definition, incoming_rows: view.incoming, incoming,
        })
    }
    pub(super) fn borrowed_ordinary_entry_structure_v1(
        &self,
        input: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<Option<BorrowedEntryStructureV1<'_>>, String> {
        if !matches!(input.semantic(), SelectedCallableSemanticRefV1::Ordinary) {
            return Ok(None);
        }
        let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) = input.selected_key()
        else {
            return Ok(None);
        };
        let receiver_required = match key.namespace() {
            hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod => true,
            hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod => false,
            _ => return Ok(None),
        };
        let has_receiver = input.source().function().declaration_sites().any(|site| {
            matches!(
                site,
                crate::mir::resolved_semantics::SourceBindingSiteV1::Receiver
            )
        });
        if has_receiver != receiver_required {
            return Err(freeze("borrowed-entry/source-mode-shape"));
        }
        let parameters = input.parameter_contracts().collect::<Vec<_>>();
        if !parameters
            .iter()
            .any(|(_, _, kind)| kind.is_ordinary_borrowed_handle())
        {
            return Ok(None);
        }
        if !self
            .completion_index
            .get(&input.source().owner())
            .is_some_and(|row| {
                row.as_ref()
                    .is_ok_and(|completion| completion.owner() == input.source().owner())
            })
        {
            return Err(freeze("borrowed-entry/foreign-source-loan"));
        }
        self.borrowed_entry_structure_for_contract(input.source().owner(), &parameters)
    }


}

impl OrdinaryNewClaimLedgerV1 {
    /// A source domain selects only structural entry work. Unsupported uses
    /// remain unfulfilled; this is never an executable incoming projection.
    fn finite_text_entry_structure_v1<'a>(
        &'a self, source: &'a PreparedBorrowedFormalIngressV1, owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<Option<BorrowedEntryStructureV1<'a>>, String> {
        use super::super::borrowed_formal_source::SourceValueAlternativeV1 as Alternative;
        use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;
        use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1 as Mode;
        // Existing admitted owners keep their original use/actual authority.
        if source.definitions.contains_key(&owner) || source.target_static.contains_key(&owner) {
            return Ok(None);
        }
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
        let mut matching = catalog.iter().filter(|row| row.owner == owner);
        let Some(contract) = matching.next() else { return Ok(None) };
        if matching.next().is_some() { return Err(freeze("borrowed-entry/original-contract-duplicate")); }
        if contract.mode != Mode::StaticBoxMethod { return Ok(None); }
        let borrowed: Vec<_> = contract.parameters.iter()
            .filter(|row| row.kind.is_ordinary_borrowed_handle()).collect();
        let sums: Vec<_> = borrowed.iter().map(|row| source.lend_source_value_sum_v1(row.binding)).collect();
        if !sums.iter().flatten().any(|sum| sum.alternatives().contains(&Alternative::Text)) {
            return Ok(None);
        }
        if parameters.len() != contract.parameters.len()
            || parameters.iter().zip(&contract.parameters).enumerate().any(|(index, ((ordinal, binding, kind), original))|
                *ordinal as usize != index || *ordinal != original.ordinal || *binding != original.binding
                    || binding.owner() != owner || kind != &original.kind)
            || !source.source_incoming.source_registered_owners_v1().contains(&owner)
            || source.source_incoming.sum_vetoed_owners_v1().contains(&owner) {
            return Err(freeze("borrowed-entry/finite-source-contract"));
        }
        let uses = Rc::clone(source.original_formal_uses_v1(owner)?);
        if !uses.matches_contract(contract) { return Err(freeze("borrowed-entry/original-use-contract")); }
        let mut domains = BTreeMap::new();
        for (formal, sum) in borrowed.iter().zip(sums) {
            let sum = sum.ok_or_else(|| freeze("borrowed-entry/finite-domain-unclosed"))?;
            if sum.alternatives().is_empty() || sum.alternatives().iter().any(|alternative|
                !matches!(alternative, Alternative::Text | Alternative::SignedI64 | Alternative::Null)) {
                return Err(freeze("borrowed-entry/finite-domain-unclosed"));
            }
            domains.insert(formal.binding, sum);
        }
        let incoming: Box<[_]> = source.source_incoming.source_rows_v1()
            .filter(|row| row.callee == owner).collect();
        let Some(first) = incoming.first() else { return Err(freeze("borrowed-entry/finite-incoming-missing")) };
        let BorrowedIncomingSourceV1::Static(first_source) = &first.source else {
            return Err(freeze("borrowed-entry/finite-incoming-kind"));
        };
        let mut sites = BTreeSet::new();
        for row in &incoming {
            let BorrowedIncomingSourceV1::Static(original) = &row.source else {
                return Err(freeze("borrowed-entry/finite-incoming-kind"));
            };
            if original.call_site() != &row.call || original.callee_owner() != owner
                || original.target_batch_slot() != contract.batch_slot || original.target() != first_source.target()
                || original.target().namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                || original.target().arity() as usize != contract.parameters.len()
                || original.parameters().len() != contract.parameters.len()
                || original.parameters().iter().zip(&contract.parameters).any(|(a, b)|
                    a.ordinal != b.ordinal || a.binding != b.binding || a.kind != b.kind)
                || original.argument_sites().len() != contract.parameters.len()
                || row.arguments.len() != borrowed.len() || !sites.insert(row.call.clone())
                || row.arguments.iter().zip(&borrowed).any(|((ordinal, site, formal), expected)|
                    *ordinal != expected.ordinal || *formal != expected.binding
                        || original.argument_sites().get(*ordinal as usize) != Some(site)) {
                return Err(freeze("borrowed-entry/finite-incoming-identity"));
            }
            // Legacy rows, when present, must be the same canonical source.
            if let Some(retained) = source.source_incoming.static_observations().get(&row.call) {
                if !Rc::ptr_eq(retained.as_ref().map_err(Clone::clone)?, original) {
                    return Err(freeze("borrowed-entry/finite-incoming-source-drift"));
                }
            }
        }
        Ok(Some(BorrowedEntryStructureV1 {
            owner, source,
            formals: borrowed.iter().map(|row| (row.ordinal, row.binding)).collect(),
            evidence: EntryStructureEvidenceV1::FiniteText { uses, incoming, domains },
        }))
    }

    pub(super) fn finite_text_entry_for_owner_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryStructureV1<'_>>, String> {
        let source = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?.as_ref().map_err(Clone::clone)?;
        let Some(contract) = self.original_parameter_contracts.as_deref()
            .and_then(|rows| rows.iter().find(|row| row.owner == owner)) else { return Ok(None) };
        let parameters: Vec<_> = contract.parameters.iter()
            .map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
        self.finite_text_entry_structure_v1(source, owner, &parameters)
    }

    /// Only source-scope staging consumes the structure. Object/field consumers
    /// continue to require their original completed use/actual products.
    pub(crate) fn stage_borrowed_ordinary_entry_structure_v1(
        &self, input: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<Option<Box<[(u32, BindingRefV1)]>>, String> {
        let Some(structure) = self.borrowed_ordinary_entry_structure_v1(input)? else { return Ok(None) };
        let formals = structure.formals.clone();
        if matches!(structure.evidence, EntryStructureEvidenceV1::Uses(_)) {
            let entry = self.finish_borrowed_entry_source_v1(structure)?;
            self.stage_borrowed_entry_field_reads_v1(&entry, input)?;
        }
        Ok(Some(formals))
    }
}

impl BorrowedEntryStructureV1<'_> {
    /// This is a canonical source-domain loan, never completed actual/use proof.
    pub(super) fn text_domain_for_formal_v1(&self, ordinal: u32, formal: BindingRefV1)
        -> Result<&Rc<super::super::borrowed_formal_source::SourceValueSumV1>, String> {
        use super::super::borrowed_formal_source::SourceValueAlternativeV1 as Alternative;
        let EntryStructureEvidenceV1::FiniteText { domains, .. } = &self.evidence else {
            return Err(freeze("borrowed-text-entry/finite-structure-required"));
        };
        if !self.formals.contains(&(ordinal, formal)) { return Err(freeze("borrowed-text-entry/structural-formal")); }
        let domain = domains.get(&formal).ok_or_else(|| freeze("borrowed-text-entry/domain-missing"))?;
        if domain.alternatives().len() != 1 || !domain.alternatives().contains(&Alternative::Text) {
            return Err(freeze("borrowed-text-entry/domain-not-text-only"));
        }
        Ok(domain)
    }

    pub(super) fn check_finite_entry_values_v1(
        &self, rows: &[(u32, BindingRefV1, crate::mir::ValueId)],
    ) -> Result<(), String> {
        let EntryStructureEvidenceV1::FiniteText { uses, incoming, domains } = &self.evidence else {
            return Err(freeze("borrowed-entry/finite-structure-required"));
        };
        if rows.len() != self.formals.len()
            || rows.iter().zip(self.formals.iter()).any(|((ordinal, binding, _), formal)| (*ordinal, *binding) != *formal)
            || rows.iter().map(|(_, _, value)| *value).collect::<BTreeSet<_>>().len() != rows.len()
            || domains.len() != rows.len() || incoming.is_empty()
            || uses.roots().keys().copied().collect::<BTreeSet<_>>() != rows.iter().map(|(_, binding, _)| *binding).collect()
            || rows.iter().any(|(_, binding, _)| !domains.contains_key(binding)) {
            return Err(freeze("borrowed-entry/finite-entry-values-identity"));
        }
        Ok(())
    }
}
