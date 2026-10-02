//! Original local materialization retained on the same borrowed entry owner.
//! No MIR scan, carrier metadata or DCE permission issues source identity here.
use super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
use super::*;
use crate::mir::builder::LocalProvenanceV1;
use crate::mir::resolved_semantics::{
    ResolvedInitializerRelationV1, SourceBindingSiteV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::{BasicBlockId, MirInstruction};

#[derive(Debug)]
pub(super) struct BorrowedAliasMaterializationV1 {
    site: OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    formal: BindingRefV1,
    source_binding: BindingRefV1,
    value: ValueId,
    proof: LocalProvenanceV1,
}

impl OrdinaryNewClaimLedgerV1 {
    /// The lowering state lends the original initializer and immutable proof.
    /// Non-alias locals do not acquire borrowed authority through this entry.
    pub(in crate::mir) fn record_borrowed_ordinary_alias_v1(
        &self,
        owner: FunctionOwnerIdV1,
        initializer: &ResolvedInitializerRelationV1,
        source_binding: Option<BindingRefV1>,
        value: ValueId,
        proof: Option<&LocalProvenanceV1>,
    ) -> Result<(), String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-alias/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let definition = source
            .definitions
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-alias/owner"))?;
        let binding = initializer.binding();
        if binding.owner() != owner {
            return Err(freeze("borrowed-alias/binding-owner"));
        }
        let Some(formal) = definition.origins.get(&binding).copied() else {
            return Ok(());
        };
        if binding == formal || binding.owner() != owner || formal.owner() != owner {
            return Err(freeze("borrowed-alias/binding"));
        }
        let initializer_site = initializer
            .initializer_site()
            .ok_or_else(|| freeze("borrowed-alias/initializer-missing"))?;
        let site = OwnedExprSiteV1::new(owner, initializer_site.clone());
        if initializer.declared_type_name().is_some() {
            return Err(freeze("borrowed-alias/source-identity"));
        }
        let source_binding =
            source_binding.ok_or_else(|| freeze("borrowed-alias/source-binding"))?;
        self.check_borrowed_alias_source_v1(
            owner,
            &site,
            initializer.declaration_site(),
            binding,
            source_binding,
            formal,
        )?;
        let values = self.borrowed_ordinary_entry_values_v1(owner)?;
        self.check_borrowed_ordinary_entry_values_v1(owner, &Ok(values.clone()))?;
        let entry = values
            .iter()
            .find(|(_, binding, _)| *binding == formal)
            .ok_or_else(|| freeze("borrowed-alias/entry-formal"))?;
        let proof = proof.ok_or_else(|| freeze("borrowed-alias/proof-missing"))?;
        proof.loan_copies(formal, entry.2, value)?;
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries
            .get_mut(&owner)
            .ok_or_else(|| freeze("borrowed-alias/entry-missing"))?;
        if entry.aliases.contains_key(&binding) {
            return Err(freeze("borrowed-alias/duplicate-materialization"));
        }
        entry.aliases.insert(
            binding,
            BorrowedAliasMaterializationV1 {
                site,
                declaration: initializer.declaration_site().clone(),
                formal,
                source_binding,
                value,
                proof: proof.clone(),
            },
        );
        Ok(())
    }

    fn check_borrowed_alias_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
        declaration: &SourceBindingSiteV1,
        binding: BindingRefV1,
        source_binding: BindingRefV1,
        formal: BindingRefV1,
    ) -> Result<(), String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-alias/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let definition = source
            .definitions
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-alias/owner"))?;
        let mut copies = definition
            .uses
            .iter()
            .filter(|row| matches!(row.kind, Use::Copy { destination } if destination == binding));
        let original = copies
            .next()
            .ok_or_else(|| freeze("borrowed-alias/source-copy-missing"))?;
        let SourceBindingSiteV1::Local { statement, ordinal } = declaration else {
            return Err(freeze("borrowed-alias/declaration"));
        };
        let canonical_site = SourcePathV1::from_node(statement.node())
            .child(SourcePathSegmentV1::Initializer(*ordinal))
            .expr();
        if copies.next().is_some()
            || site.owner() != owner
            || binding.owner() != owner
            || formal.owner() != owner
            || source_binding.owner() != owner
            || original.site != *site
            || original.formal != formal
            || original.binding != source_binding
            || site.site() != &canonical_site
            || definition.origins.get(&binding) != Some(&formal)
            || definition.origins.get(&source_binding) != Some(&formal)
        {
            return Err(freeze("borrowed-alias/source-identity"));
        }
        Ok(())
    }

    /// Original physical evidence only. The final consumer still must demand
    /// the same owner's Boundary mapping and check every residual operand use.
    pub(in crate::mir) fn with_borrowed_ordinary_alias_copies_v1(
        &self,
        owner: FunctionOwnerIdV1,
        mut visit: impl FnMut(
            &OwnedExprSiteV1,
            &SourceBindingSiteV1,
            BindingRefV1,
            BindingRefV1,
            ValueId,
            &[(BasicBlockId, MirInstruction)],
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let entries = self.borrowed_entry_values.borrow();
        let entry = entries
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-alias/entry-missing"))?;
        self.check_borrowed_ordinary_entry_values_v1(owner, &entry.values)?;
        let values = entry.values.as_ref().map_err(Clone::clone)?;
        for (binding, alias) in &entry.aliases {
            self.check_borrowed_alias_source_v1(
                owner,
                &alias.site,
                &alias.declaration,
                *binding,
                alias.source_binding,
                alias.formal,
            )?;
            let formal = values
                .iter()
                .find(|(_, formal, _)| *formal == alias.formal)
                .ok_or_else(|| freeze("borrowed-alias/entry-formal"))?;
            let copies = alias
                .proof
                .loan_copies(alias.formal, formal.2, alias.value)?;
            visit(
                &alias.site,
                &alias.declaration,
                *binding,
                alias.formal,
                alias.value,
                copies,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_alias_materialization_tests.rs"]
mod tests;
