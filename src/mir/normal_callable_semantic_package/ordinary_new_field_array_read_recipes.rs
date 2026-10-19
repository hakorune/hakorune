use super::*;
// Issue checked field-array reads beside the original ledger-owned field relations.
use crate::mir::normal_callable_semantic_package::FieldArrayReadRecipeV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::named_array_obligation::fault as array_fault;
impl OrdinaryNewClaimLedgerV1 {
    fn field_array_read_sources_for_owner_v1(&self, owner: FunctionOwnerIdV1)
        -> Result<Vec<Rc<super::super::CheckedFieldArrayAccessSourceV1>>, String> {
        let mut sources = BTreeMap::<OwnedExprSiteV1, Rc<super::super::CheckedFieldArrayAccessSourceV1>>::new();
        let mut insert = |source: &Rc<super::super::CheckedFieldArrayAccessSourceV1>| -> Result<(), String> {
            source.recheck_census(owner, &self.array_i64_fields)?;
            if let Some(original) = sources.get(source.call()) {
                if !Rc::ptr_eq(original, source) { return Err(array_fault("read-source-conflict")); }
            } else { sources.insert(source.call().clone(), Rc::clone(source)); }
            Ok(())
        };
        for read in self.field_array_return_reads_v1(owner)? { insert(read.read())?; }
        let mut arguments = |rows: &[OrdinaryNewTrivialArgumentV1]| -> Result<(), String> {
            for row in rows.iter().filter(|row| row.owner() == owner) {
                if let Some(source) = row.field_array_read() { insert(source.read())?; }
            }
            Ok(())
        };
        for row in self.claims.borrow().values() {
            if let Ok(rows) = &row.core.argument_rows { arguments(rows)?; }
        }
        for row in self.result_claims.borrow().values() {
            if let Ok(rows) = &row.core.argument_rows { arguments(rows)?; }
        }
        for row in self.local_commits.borrow().values() {
            if let Some(Ok(rows)) = row.new_argument_rows() { arguments(rows)?; }
        }
        Ok(sources.into_values().collect())
    }
    fn field_array_read_recipe_from_source_v1(&self, owner: FunctionOwnerIdV1,
        source: Rc<super::super::CheckedFieldArrayAccessSourceV1>,
    ) -> Result<Rc<FieldArrayReadRecipeV1>, String> {
        let (selected, _) = self.original_instance_input_issuance.as_ref().ok_or_else(|| array_fault("read-original-issuance-missing"))?;
        let catalog = self.original_parameter_contracts.as_deref().ok_or_else(|| array_fault("read-original-catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts.next().ok_or_else(|| array_fault("read-original-contract-missing"))?;
        if contracts.next().is_some() { return Err(array_fault("read-original-contract-duplicate")); }
        let Some(SelectedNormalCallableKeyV1::Cataloged(caller)) = selected.key_for_batch_slot(contract.batch_slot) else { return Err(array_fault("read-original-caller-missing")) };
        if source.caller() != caller { return Err(array_fault("read-original-caller-drift")); }
        Ok(Rc::new(FieldArrayReadRecipeV1::new(source)))
    }
    pub(crate) fn field_array_read_recipe_v1(&self, owner: FunctionOwnerIdV1, site: &OwnedExprSiteV1)
        -> Result<Option<Rc<FieldArrayReadRecipeV1>>, String> {
        let source = self.field_array_read_sources_for_owner_v1(owner)?.into_iter().find(|source| source.call() == site);
        source.map(|source| self.field_array_read_recipe_from_source_v1(owner, source)).transpose()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn array_field_original_caller_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Option<&CanonicalSameModuleCallableKeyV1>, String> {
        let (selected, _) = self.original_instance_input_issuance.as_ref().ok_or_else(|| array_fault("read-original-issuance-missing"))?;
        let catalog = self.original_parameter_contracts.as_deref().ok_or_else(|| array_fault("read-original-catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let Some(contract) = contracts.next() else { return Ok(None); };
        if contracts.next().is_some() { return Err(array_fault("read-original-contract-duplicate")); }
        let Some(SelectedNormalCallableKeyV1::Cataloged(caller)) = selected.key_for_batch_slot(contract.batch_slot) else { return Ok(None); };
        Ok((caller.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod).then_some(caller))
    }
    /// Issue once from the original input; the existing field-read row retains
    /// the immutable receiver facts and the same canonical field-class Rc.
    pub(in crate::mir::normal_callable_semantic_package) fn array_field_receiver_source_v1(&self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &OwnedExprSiteV1, name: &str,
    ) -> Result<Option<Rc<super::super::CanonicalArrayFieldReadSourceV1>>, String> {
        if input.owner() != site.owner() { return Err(array_fault("receiver-source-owner")); }
        let Some(caller) = self.array_field_original_caller_v1(input.owner())? else { return Ok(None); };
        let Some(source) = self.field_write_claims.array_source(caller.owner(), name) else { return Ok(None); };
        super::super::CanonicalArrayFieldReadSourceV1::fixed_receiver_v1(input, caller.clone(), site, name, source)
            .map(|source| source.map(Rc::new))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn field_array_read_recipes_v1(&self) -> Result<Vec<Rc<FieldArrayReadRecipeV1>>, String> {
        let Some(catalog) = &self.original_parameter_contracts else { return Ok(Vec::new()) };
        let mut recipes = Vec::new();
        for contract in catalog.iter() {
            for source in self.field_array_read_sources_for_owner_v1(contract.owner)? {
                recipes.push(self.field_array_read_recipe_from_source_v1(contract.owner, source)?);
            }
        }
        Ok(recipes)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Reuse the exact local/expression field-read owner; no second index ledger.
    pub(in crate::mir::normal_callable_semantic_package) fn stage_field_array_index_reads_v1(
        &self, recipes: &[Rc<FieldArrayReadRecipeV1>],
    ) -> Result<(), String> {
        let mut reads = self.local_field_reads.borrow_mut();
        let mut staged = BTreeMap::new();
        for recipe in recipes {
            let Some((binding, receiver_site, source)) = recipe.index_field() else { continue };
            let site = OwnedExprSiteV1::new(recipe.owner(), recipe.index_site().ok_or_else(|| array_fault("index-field-site"))?.clone());
            if binding.owner() != recipe.owner() { return Err(array_fault("index-field-owner")); }
            if let Some(existing) = reads.get(&site) {
                if existing.receiver != *binding || existing.home != *binding || existing.source.as_ref() != source.as_ref()
                    || existing.receiver_site != *receiver_site
                    || !matches!(existing.source.result(), crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar)
                    || !matches!(existing.progress, LocalReadProgress::Pending)
                { return Err(array_fault("index-field-conflict")); }
                continue;
            }
            if staged.insert(site, LocalFieldRead { receiver_site: receiver_site.clone(), receiver: *binding,
                home: *binding, source: Rc::clone(source), numeric_source: None, local: None,
                progress: LocalReadProgress::Pending }).is_some()
            { return Err(array_fault("index-field-duplicate")); }
        }
        reads.extend(staged);
        Ok(())
    }
}
