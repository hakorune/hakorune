//! Installed-package accounting for exact source obligations and their emissions.
use super::{EmittedNamedArrayRequirementV1, SelectedSourceCoreMethodCallV1};
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
use crate::mir::named_array_obligation::fault;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use std::{cell::RefCell, collections::BTreeMap};

type Key = (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1);

#[derive(Debug)]
enum Expected {
    OwnedHolder(std::rc::Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>),
    Write(FunctionOwnerIdV1),
    FieldAppend(super::FieldArrayAppendSourceLoanV1),
    Read(std::rc::Rc<super::field_read::FieldArrayReadRecipeV1>),
}
#[derive(Debug)]
pub(crate) struct NamedArrayEmissionCollectorV1 {
    expected: Result<BTreeMap<Key, Expected>, String>,
    emitted: RefCell<BTreeMap<Key, EmittedNamedArrayRequirementV1>>,
}

impl NamedArrayEmissionCollectorV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn from_source_rows(
        rows: &BTreeMap<
            SelectedNormalCallableKeyV1,
            BTreeMap<SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
        >,
    ) -> Self {
        let expected = rows
            .values()
            .flat_map(|rows| rows.values())
            .filter_map(|row| {
                row.contract().named_array_requirement().map(|requirement| {
                    (
                        (row.caller.clone(), requirement.call().clone()),
                        Expected::Write(requirement.owner()),
                    )
                })
            })
            .collect();
        Self {
            expected: Ok(expected),
            emitted: RefCell::new(BTreeMap::new()),
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn with_field_appends(mut self,
        sources: Result<Vec<super::FieldArrayAppendSourceLoanV1>, String>,
        providers: &mut super::NamedArrayFieldProviderTableV1,
    ) -> Self {
        self.expected = self.expected.and_then(|mut expected| {
            for source in sources? {
                let frame = source.text_frame().ok_or_else(|| fault("checked-source-mismatch"))?;
                let fields = providers.entry(frame.source().provider().clone()).or_default();
                if fields.get(frame.source().provider_site()).is_some_and(|field| *field != frame.source().field()) {
                    return Err(fault("read-provider-conflict"));
                }
                fields.insert(frame.source().provider_site().clone(), frame.source().field());
                let key = (frame.source().caller().clone(), source.site().site().clone());
                if expected.insert(key, Expected::FieldAppend(source)).is_some() { return Err(fault("read-source-conflict")); }
            }
            Ok(expected)
        });
        self
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_field_appends(&self,
        sources: Vec<super::FieldArrayAppendSourceLoanV1>,
    ) -> Result<(), String> {
        let expected = self.expected.as_ref().map_err(Clone::clone)?;
        if expected.values().filter(|row| matches!(row, Expected::FieldAppend(_))).count() != sources.len() {
            return Err(fault("read-source-domain-drift"));
        }
        for source in sources {
            let frame = source.text_frame().ok_or_else(|| fault("checked-source-mismatch"))?;
            let Some(Expected::FieldAppend(original)) = expected.get(&(frame.source().caller().clone(), source.site().site().clone()))
                else { return Err(fault("read-source-domain-drift")) };
            if !original.same_source(&source) { return Err(fault("read-original-source-drift")); }
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn with_field_reads(
        mut self,
        reads: Result<Vec<std::rc::Rc<super::field_read::FieldArrayReadRecipeV1>>, String>,
        providers: &mut super::NamedArrayFieldProviderTableV1,
    ) -> Self {
        self.expected = self.expected.and_then(|mut expected| {
            for recipe in reads? {
                let fields = providers.entry(recipe.provider().clone()).or_default();
                if fields.get(recipe.provider_site()).is_some_and(|field| *field != recipe.field()) {
                    return Err(fault("read-provider-conflict"));
                }
                fields.insert(recipe.provider_site().clone(), recipe.field());
                let key = (recipe.caller().clone(), recipe.call_site().clone());
                if expected.insert(key, Expected::Read(recipe)).is_some() { return Err(fault("read-source-conflict")); }
            }
            Ok(expected)
        });
        self
    }
    /// A genuine source-owned holder is neither a numeric write nor a field
    /// read. Its full lifecycle remains mandatory even before emission exists.
    pub(in crate::mir::normal_callable_semantic_package) fn with_owned_holders(
        mut self,
        recipes: Result<Vec<std::rc::Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>>, String>,
    ) -> Self {
        self.expected = self.expected.and_then(|mut expected| {
            for recipe in recipes? {
                let key = (recipe.caller().clone(), recipe.construction().clone());
                if expected.insert(key, Expected::OwnedHolder(recipe)).is_some() {
                    return Err(fault("owned-holder-source-conflict"));
                }
            }
            Ok(expected)
        });
        self
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_owned_holders(
        &self,
        current: Vec<std::rc::Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>>,
    ) -> Result<(), String> {
        let expected = self.expected.as_ref().map_err(Clone::clone)?;
        if expected.values().filter(|row| matches!(row, Expected::OwnedHolder(_))).count() != current.len() {
            return Err(fault("owned-holder-source-domain-drift"));
        }
        for recipe in current {
            let Some(Expected::OwnedHolder(original)) = expected.get(&(recipe.caller().clone(), recipe.construction().clone()))
                else { return Err(fault("owned-holder-source-domain-drift")); };
            if !std::rc::Rc::ptr_eq(original, &recipe) {
                return Err(fault("owned-holder-original-source-drift"));
            }
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_reads(
        &self, current: Vec<std::rc::Rc<super::field_read::FieldArrayReadRecipeV1>>,
    ) -> Result<(), String> {
        let expected = self.expected.as_ref().map_err(Clone::clone)?;
        if expected.values().filter(|row| matches!(row, Expected::Read(_))).count() != current.len() { return Err(fault("read-source-domain-drift")); }
        for recipe in current {
            let Some(Expected::Read(original)) = expected.get(&(recipe.caller().clone(), recipe.call_site().clone())) else { return Err(fault("read-source-domain-drift")) };
            if !original.same_source(&recipe) { return Err(fault("read-original-source-drift")); }
        }
        Ok(())
    }

    pub(crate) fn hand_back(
        &self,
        rows: Vec<EmittedNamedArrayRequirementV1>,
    ) -> Result<(), String> {
        let mut emitted = self.emitted.borrow_mut();
        for row in rows {
            let key = (row.caller().clone(), row.call_site().clone());
            let expected = self.expected.as_ref().map_err(Clone::clone)?;
            let matches = match (expected.get(&key), &row) {
                (Some(Expected::Write(owner)), EmittedNamedArrayRequirementV1::Write(_)) => *owner == row.owner(),
                (Some(Expected::FieldAppend(original)), EmittedNamedArrayRequirementV1::Write(write)) => write.borrows_append_source_v1(original),
                (Some(Expected::Read(original)), EmittedNamedArrayRequirementV1::Read(read)) => original.same_source(&read.recipe),
                (Some(Expected::OwnedHolder(original)), EmittedNamedArrayRequirementV1::OwnedHolder(holder)) => std::rc::Rc::ptr_eq(original, holder.recipe()),
                _ => false,
            };
            if !matches {
                return Err(fault("foreign-emission-handback"));
            }
            if emitted.contains_key(&key) {
                return Err(fault("duplicate-emission-handback"));
            }
            emitted.insert(key, row);
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish(
        self,
    ) -> Result<Box<[EmittedNamedArrayRequirementV1]>, String> {
        let expected = self.expected?;
        let emitted = self.emitted.into_inner();
        if emitted.len() != expected.len()
            || expected.keys().any(|key| !emitted.contains_key(key))
        {
            return Err(fault("unconsumed-package-obligations"));
        }
        Ok(emitted.into_values().collect())
    }
}
