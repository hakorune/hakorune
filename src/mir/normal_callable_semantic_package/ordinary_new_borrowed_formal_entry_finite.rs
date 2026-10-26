//! Finite source-domain selection and alias/entry correspondence.
//! Nominal object inputs keep the existing Uses owner and its actual checks.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// A source domain selects only structural entry work. Unsupported uses
    /// remain unfulfilled; this is never an executable incoming projection.
    pub(super) fn finite_input_entry_structure_v1<'a>(
        source: &'a PreparedBorrowedFormalIngressV1,
        catalog: Option<&[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1]>,
        owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<Option<BorrowedEntryStructureV1<'a>>, String> {
        use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;
        use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1 as Mode;
        let catalog = catalog
            .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
        let mut matching = catalog.iter().filter(|row| row.owner == owner);
        let Some(contract) = matching.next() else { return Ok(None) };
        if matching.next().is_some() { return Err(freeze("borrowed-entry/original-contract-duplicate")); }
        let namespace = match contract.mode {
            Mode::StaticBoxMethod => hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod,
            Mode::InstanceBoxMethod => hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod,
        };
        let borrowed: Vec<_> = contract.parameters.iter()
            .filter(|row| row.kind.is_ordinary_borrowed_handle()).collect();
        // The source read lender uses this SAME full original membership.
        // One Text formal cannot select an ungrounded sibling or grant entry.
        let Some(sums) = source.lend_original_input_sums_v1(&contract.parameters) else { return Ok(None) };
        let text_requested = sums.iter().any(|sum| sum.alternatives().contains(
            &super::super::borrowed_formal_source::SourceValueAlternativeV1::Text));
        // Original Text domains select the same finite source structure even
        // when a legacy Uses definition exists. Full uses and incoming still
        // require their original proof; non-Text owners keep that selection.
        if !text_requested
            && (source.definitions.contains_key(&owner) || source.target_static.contains_key(&owner)) {
            return Ok(None);
        }
        // Preserve the existing Text refusal boundary. New scalar families
        // select only a complete original cohort, never one convenient formal.
        if !text_requested && ((contract.mode == Mode::StaticBoxMethod
            && source.is_scalar_only_static_input_v1(owner)) || sums.is_empty() || !sums.iter().all(|sum| sum.finite_input())) { return Ok(None); }
        let uses = Rc::clone(source.original_formal_uses_v1(owner)?);
        if !text_requested && !source.original_finite_forward_input_request_v1(owner)
            && !uses.scalar_expression_use_requested_v1() { return Ok(None); }
        if parameters.len() != contract.parameters.len()
            || parameters.iter().zip(&contract.parameters).enumerate().any(|(index, ((ordinal, binding, kind), original))|
                *ordinal as usize != index || *ordinal != original.ordinal || *binding != original.binding
                    || binding.owner() != owner || kind != &original.kind)
            || !source.source_incoming.source_registered_owners_v1().contains(&owner)
            || source.source_incoming.sum_vetoed_owners_v1().contains(&owner) {
            return Err(freeze("borrowed-entry/finite-source-contract"));
        }
        if !uses.matches_contract(contract) { return Err(freeze("borrowed-entry/original-use-contract")); }
        let mut domains = BTreeMap::new();
        for (formal, sum) in borrowed.iter().zip(sums) {
            if !sum.finite_input() {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[input-domain] finite owner={owner:?} formal={:?} alternatives={:?}", formal.binding, sum.alternatives());
                }
                return Err(freeze("borrowed-entry/finite-domain-unclosed"));
            }
            domains.insert(formal.binding, sum);
        }
        let incoming: Box<[_]> = source.source_incoming.source_rows_v1()
            .filter(|row| row.callee == owner).collect();
        let Some(first) = incoming.first() else { return Err(freeze("borrowed-entry/finite-incoming-missing")) };
        let first_source = first.source.as_loan()?;
        let mut sites = BTreeSet::new();
        for row in &incoming {
            let original = row.source.as_loan()?;
            if original.call_site() != &row.call || original.callee_owner() != owner
                || original.target_batch_slot() != contract.batch_slot || original.target() != first_source.target()
                || original.namespace() != namespace || original.declaration_mode() != contract.mode
                || original.target().namespace() != namespace
                || original.target().arity() as usize != contract.parameters.len()
                || original.argument_sites().len() != contract.parameters.len()
                || row.arguments.len() != borrowed.len() || !sites.insert(row.call.clone())
                || row.arguments.iter().zip(&borrowed).any(|((ordinal, site, formal), expected)|
                    *ordinal != expected.ordinal || *formal != expected.binding
                        || original.argument_sites().get(*ordinal as usize) != Some(site)) {
                return Err(freeze("borrowed-entry/finite-incoming-identity"));
            }
            match &row.source {
                BorrowedIncomingSourceV1::Static(original) => {
                    if original.parameters().len() != contract.parameters.len()
                        || original.parameters().iter().zip(&contract.parameters).any(|(a, b)|
                            a.ordinal != b.ordinal || a.binding != b.binding || a.kind != b.kind) {
                        return Err(freeze("borrowed-entry/finite-incoming-identity"));
                    }
                }
                // The original target was corroborated before affine lexical
                // issuance consumed its preparation. This retained incoming
                // loan, not the spent preparation slot, owns source identity.
                BorrowedIncomingSourceV1::Instance(_) => {}
                BorrowedIncomingSourceV1::SelectedStored { target, .. } => {
                    if !row.source.bind_selected_v1(target)?.same_source(&row.source) {
                        return Err(freeze("borrowed-entry/finite-incoming-source-drift"));
                    }
                }
                BorrowedIncomingSourceV1::PendingStored(_) => return Err(freeze("borrowed-entry/finite-incoming-kind")),
            }
            // Only the matching namespace can corroborate this source. A
            // failed Static lookup is not contradictory Instance evidence.
            if let Some(retained) = source.source_incoming.static_observations().get(&row.call) {
                match &row.source {
                    BorrowedIncomingSourceV1::Static(original) => {
                        if !Rc::ptr_eq(retained.as_ref().map_err(Clone::clone)?, original) {
                            return Err(freeze("borrowed-entry/finite-incoming-source-drift"));
                        }
                    }
                    _ if retained.is_ok() => return Err(freeze("borrowed-entry/finite-incoming-source-drift")),
                    _ => {},
                }
            }
        }
        Ok(Some(BorrowedEntryStructureV1 {
            owner, source,
            formals: borrowed.iter().map(|row| (row.ordinal, row.binding)).collect(),
            evidence: EntryStructureEvidenceV1::FiniteInput { uses, incoming, domains },
        }))
    }

}

impl BorrowedEntryStructureV1<'_> {
    /// This is a canonical source-domain loan, never completed actual/use proof.
    pub(in super::super) fn finite_domain_for_formal_v1(&self, ordinal: u32, formal: BindingRefV1)
        -> Result<&Rc<super::super::borrowed_formal_source::SourceValueSumV1>, String> {
        let EntryStructureEvidenceV1::FiniteInput { domains, .. } = &self.evidence else {
            return Err(freeze("borrowed-text-entry/finite-structure-required"));
        };
        if !self.formals.contains(&(ordinal, formal)) { return Err(freeze("borrowed-text-entry/structural-formal")); }
        domains.get(&formal).ok_or_else(|| freeze("borrowed-text-entry/domain-missing"))
    }

    /// Borrow the selected structure's original domain without completing its
    /// actuals. Existing Uses owners retain their full use/actual final proof.
    pub(in super::super) fn source_domain_for_formal_v1(&self, ordinal: u32, formal: BindingRefV1)
        -> Result<Rc<super::super::borrowed_formal_source::SourceValueSumV1>, String> {
        if matches!(self.evidence, EntryStructureEvidenceV1::FiniteInput { .. }) {
            return self.finite_domain_for_formal_v1(ordinal, formal).map(Rc::clone);
        }
        if !self.formals.contains(&(ordinal, formal))
            || !self.source.source_incoming.source_registered_owners_v1().contains(&self.owner)
            || self.source.source_incoming.sum_vetoed_owners_v1().contains(&self.owner) {
            return Err(freeze("borrowed-text-entry/structural-formal"));
        }
        let sum = self.source.lend_source_value_sum_v1(formal)
            .ok_or_else(|| freeze("borrowed-text-entry/domain-missing"))?;
        if !sum.finite_input() {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[input-domain] uses owner={:?} formal={formal:?} alternatives={:?}", self.owner, sum.alternatives());
            }
            return Err(freeze("borrowed-entry/finite-domain-unclosed"));
        }
        Ok(sum)
    }

    pub(in super::super) fn finite_alias_formal_v1(&self,
        initializer: &crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
        source_binding: Option<BindingRefV1>,
    ) -> Result<Option<BindingRefV1>, String> {
        let EntryStructureEvidenceV1::FiniteInput { uses, .. } = &self.evidence else { return Err(freeze("borrowed-alias/finite-structure-required")); };
        let formal = uses.initializer_entry_alias_v1(initializer, source_binding)
            .map_err(|_| freeze("borrowed-alias/source-identity"))?;
        if formal.is_some_and(|formal| !self.formals.iter().any(|(_, root)| *root == formal)) {
            return Err(freeze("borrowed-alias/entry-formal"));
        }
        Ok(formal)
    }
    pub(in super::super) fn check_finite_alias_source_v1(&self, site: &OwnedExprSiteV1,
        declaration: &crate::mir::resolved_semantics::SourceBindingSiteV1,
        binding: BindingRefV1, source_binding: BindingRefV1, formal: BindingRefV1,
    ) -> Result<(), String> {
        let EntryStructureEvidenceV1::FiniteInput { uses, .. } = &self.evidence else { return Err(freeze("borrowed-alias/finite-structure-required")); };
        let mut originals = uses.initializers().iter().filter(|row| row.binding() == binding && row.declaration_site() == declaration);
        let original = originals.next().ok_or_else(|| freeze("borrowed-alias/source-copy-missing"))?;
        if originals.next().is_some() || site.owner() != self.owner || original.initializer_site() != Some(site.site())
            || self.finite_alias_formal_v1(original, Some(source_binding))? != Some(formal) {
            return Err(freeze("borrowed-alias/source-identity"));
        }
        Ok(())
    }

    pub(in super::super) fn check_finite_entry_values_v1(
        &self, rows: &[(u32, BindingRefV1, crate::mir::ValueId)],
    ) -> Result<(), String> {
        if rows.len() != self.formals.len()
            || rows.iter().zip(self.formals.iter()).any(|((ordinal, binding, _), formal)| (*ordinal, *binding) != *formal)
            || rows.iter().map(|(_, _, value)| *value).collect::<BTreeSet<_>>().len() != rows.len() {
            return Err(freeze("borrowed-entry/finite-entry-values-identity"));
        }
        let roots = match &self.evidence {
            EntryStructureEvidenceV1::FiniteInput { uses, incoming, domains } => {
                if domains.len() != rows.len() || incoming.is_empty()
                    || rows.iter().any(|(_, binding, _)| !domains.contains_key(binding)) {
                    return Err(freeze("borrowed-entry/finite-entry-values-identity"));
                }
                uses.roots().keys().copied().collect::<BTreeSet<_>>()
            }
            EntryStructureEvidenceV1::Uses(view) => {
                if view.incoming.iter().any(|call| call.arguments.len() != rows.len()
                    || call.arguments.iter().zip(rows).any(|((ordinal, _, formal), (expected, binding, _))|
                        ordinal != expected || formal != binding)) {
                    return Err(freeze("borrowed-entry/finite-entry-values-identity"));
                }
                view.definition.origins.values().copied().collect()
            }
        };
        if roots != rows.iter().map(|(_, binding, _)| *binding).collect() {
            return Err(freeze("borrowed-entry/finite-entry-values-identity"));
        }
        Ok(())
    }
}
