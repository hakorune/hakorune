//! Canonical reads demanded by the original selected borrowed entry.
//! Passive Home/index classification remains separate from executable reads.
use super::*;
use super::super::lexical_instance_call::BorrowedOrdinaryEntrySourceRefV1;
use crate::mir::normal_callable_semantic_package::{
    SelectedCallableLoweringInputRefV1,
};
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, ResolvedLexicalRefV1};

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn array_set_receiver_source_v1(
        &self, original: &Rc<crate::mir::normal_callable_semantic_package::DeclaredArrayFieldReadSourceV1>,
    ) -> Result<Rc<LocalFieldReadSourceV1>, String> {
        let rows = self.local_field_reads.borrow();
        let row = rows.get(original.site()).ok_or_else(|| fault("declared-array-not-staged"))?;
        self.recheck_declared_array_read_v1(original.site(), row, original)?;
        Ok(Rc::clone(&row.source))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_declared_array_read_v1(
        &self, site: &OwnedExprSiteV1, row: &LocalFieldRead,
        original: &Rc<crate::mir::normal_callable_semantic_package::DeclaredArrayFieldReadSourceV1>,
    ) -> Result<(), String> {
        let entry = self.finalized_borrowed_ordinary_entry_source_v1(site.owner())?;
        self.recheck_declared_array_entry_read_v1(&entry, site, row, original)
    }

    fn recheck_declared_array_entry_read_v1(&self, entry: &BorrowedOrdinaryEntrySourceRefV1<'_>,
        site: &OwnedExprSiteV1, row: &LocalFieldRead,
        original: &Rc<crate::mir::normal_callable_semantic_package::DeclaredArrayFieldReadSourceV1>,
    ) -> Result<(), String> {
        if original.site() != site || original.receiver().site() != &row.receiver_site
            || original.binding() != row.receiver || row.home != row.receiver
            || row.source.field() != original.field() || row.numeric_source.is_some()
            || !matches!(row.source.result(), crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::DeclaredArray(source)
                if Rc::ptr_eq(source, original))
            || !entry.borrows_array_set_receiver_v1(original)
        { return Err(fault("declared-array-source-drift")); }
        Ok(())
    }

    fn stage_array_set_receiver_v1(
        &self, entry: &BorrowedOrdinaryEntrySourceRefV1<'_>, original: &Rc<crate::mir::normal_callable_semantic_package::DeclaredArrayFieldReadSourceV1>,
    ) -> Result<Rc<LocalFieldReadSourceV1>, String> {
        if !entry.borrows_array_set_receiver_v1(original) { return Err(fault("declared-array-original-missing")); }
        let mut reads = self.local_field_reads.borrow_mut();
        if let Some(row) = reads.get(original.site()) {
            self.recheck_declared_array_entry_read_v1(entry, original.site(), row, original)?;
            return Ok(Rc::clone(&row.source));
        }
        let source = LocalFieldReadSourceV1::from_declaration(original.field(),
            crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::DeclaredArray(Rc::clone(original)),
            "ArrayBox".into());
        reads.insert(original.site().clone(), LocalFieldRead {
            local: None, receiver_site: original.receiver().site().clone(),
            receiver: original.binding(), home: original.binding(), source: Rc::clone(&source),
            numeric_source: None, progress: LocalReadProgress::Pending,
        });
        Ok(source)
    }

    /// Same entry-owned scalar row, qualified by its original read requirement.
    /// Unlike an expression read, this statement-call read has no plan to mint.
    pub(in crate::mir::normal_callable_semantic_package) fn array_set_index_source_v1(
        &self, site: &OwnedExprSiteV1, receiver_site: &OwnedExprSiteV1,
        binding: BindingRefV1, formal: BindingRefV1,
    ) -> Result<Rc<LocalFieldReadSourceV1>, String> {
        let entry = self.finalized_borrowed_ordinary_entry_source_v1(site.owner())?;
        if !entry.field_read_uses().any(|(actual, root, access)| actual == binding && root == formal && access == site) {
            return Err(fault("array-set-index-source-missing"));
        }
        let reads = self.local_field_reads.borrow();
        let row = reads.get(site).ok_or_else(|| fault("array-set-index-not-staged"))?;
        if row.receiver_site != *receiver_site.site() || row.receiver != binding || row.home != formal {
            return Err(fault("array-set-index-source-drift"));
        }
        self.recheck_local_field_source_v1(site, row, crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(self))?;
        Ok(Rc::clone(&row.source))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn array_set_read_emission_v1(
        &self, site: &OwnedExprSiteV1, original: &Rc<LocalFieldReadSourceV1>,
    ) -> Result<(crate::mir::BasicBlockId, crate::mir::MirInstruction), String> {
        let rows = self.local_field_reads.borrow();
        let row = rows.get(site).ok_or_else(|| fault("array-set-read-missing"))?;
        self.recheck_local_field_source_v1(site, row, crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(self))?;
        let LocalReadProgress::Emitted { block, instruction, source } = &row.progress else {
            return Err(fault("array-set-read-unconsumed"));
        };
        if !Rc::ptr_eq(original, &row.source) || !Rc::ptr_eq(original, source) {
            return Err(fault("array-set-read-source-identity"));
        }
        Ok((*block, instruction.clone()))
    }

    pub(in crate::mir) fn stage_borrowed_entry_field_reads_v1(
        &self,
        entry: &BorrowedOrdinaryEntrySourceRefV1<'_>,
        selected: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<(), String> {
        let input = selected.source();
        if entry.owner() != input.owner() {
            return Err(fault("borrowed-entry-owner"));
        }
        for receiver in entry.array_set_receivers_v1() {
            self.stage_array_set_receiver_v1(entry, receiver)?;
        }
        let uses: Vec<_> = entry.field_read_uses().collect();
        if uses.is_empty() { return Ok(()); }
        let shape = input.body_shape().ok_or_else(|| fault("borrowed-entry-shape"))?;
        let mut rows = Vec::with_capacity(uses.len());
        for (binding, formal, site) in uses {
            let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
                shape.expression_shape(site.site()) else {
                return Err(fault("borrowed-entry-read-site"));
            };
            if site.owner() != input.owner() || entry.origins().get(&binding) != Some(&formal)
                || input.function().variable_ref(object) != Some(ResolvedLexicalRefV1::Local(binding))
            { return Err(fault("borrowed-entry-read-origin")); }
            let view = entry.formal_object_view(formal)?
                .ok_or_else(|| fault("borrowed-entry-object-view-missing"))?;
            let source = view.scalar_field_source_v1(field)
                .ok_or_else(|| fault("borrowed-entry-declaration-missing"))?;
            if source.field().object() != view.object() {
                return Err(fault("borrowed-entry-object-drift"));
            }
            rows.push((site.clone(), LocalFieldRead {
                local: None, receiver_site: object.clone(), receiver: binding, home: formal,
                source: Rc::clone(source), numeric_source: None, progress: LocalReadProgress::Pending,
            }));
        }
        // Previously staged expression/terminal reads keep their original
        // descriptor and numeric loan. Never replace them with a declaration.
        let mut reads = self.local_field_reads.borrow_mut();
        let mut missing = BTreeMap::new();
        for (site, row) in rows {
            if entry.formal_object_view(row.home)?.is_none_or(|view| view.object() != row.source.field().object()) {
                return Err(fault("borrowed-entry-object-drift"));
            }
            // Terminal and constructor-argument reads use the same canonical
            // emitter but keep their existing affine ledger families.
            if let Some(original) = self.field_reads.borrow().get(&site) {
                if matches!(original.progress, Progress::Released) {
                    return Err(fault("borrowed-entry-terminal-released"));
                }
                if original.receiver_site != row.receiver_site || original.receiver != row.receiver
                    || original.home != row.home || original.field != row.source.field()
                { return Err(fault("borrowed-entry-terminal-conflict")); }
                self.recheck_terminal_field_value_v1(&site, original, crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(self))?;
                continue;
            }
            if let Some(original) = self.argument_field_reads.borrow().get(&site) {
                if matches!(original.progress, Progress::Released) {
                    return Err(fault("borrowed-entry-argument-released"));
                }
                if original.receiver_site != row.receiver_site || original.object != row.receiver
                    || original.field != row.source.field()
                { return Err(fault("borrowed-entry-argument-conflict")); }
                continue;
            }
            if let Some(original) = reads.get(&site) {
                if original.receiver_site != row.receiver_site || original.receiver != row.receiver
                    || original.home != row.home || original.source != row.source
                { return Err(fault("borrowed-entry-read-conflict")); }
                self.recheck_local_field_source_v1(&site, original, crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(self))?;
            } else {
                if missing.insert(site, row).is_some() {
                    return Err(fault("borrowed-entry-duplicate-stage"));
                }
            }
        }
        // All identities were checked before modifying the same affine map.
        reads.extend(missing);
        Ok(())
    }
}
