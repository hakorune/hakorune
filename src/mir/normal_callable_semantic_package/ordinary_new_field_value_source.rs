//! Original declared field/writer/constructor correspondences, not value classes.
//! Integer preservation and executable receiver/Home/ABI obligations remain later.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    instance_construction::ConstructionStoreV1, VerifiedInstanceConstructorSemanticBatchV1,
};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use crate::parser::ConstructorSourceKindV1;
use hakorune_mir_defs::CanonicalFieldRefV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FieldValueSourceIssueV1 {
    HiddenWriters,
    IncompleteWriterInventory,
    UnattributedWriter,
    Declaration,
    OriginalPlan,
    ConstructorIdentity,
    ConstructionUnavailable,
    MissingWriter,
    StoreIdentity,
}

/// The original constructor identity and source-order store ordinal. Borrowing
/// this correspondence never substitutes a cloned or newly issued store plan.
#[derive(Debug)]
struct BirthStoreSourceRefV1 {
    constructor: ConstructorSourceIdV1,
    store_ordinal: usize,
}

#[derive(Debug)]
pub(crate) struct DeclaredFieldValueSourceV1 {
    inventory: Rc<OriginalFieldWriterInventoryV1>,
    field: CanonicalFieldRefV1,
    declared_type_name: Option<Box<str>>,
    writers: Box<[usize]>,
    birth_stores: Box<[Option<BirthStoreSourceRefV1>]>,
    qualified_writers: BTreeSet<usize>,
}
impl DeclaredFieldValueSourceV1 {
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(crate) fn declared_type_name(&self) -> Option<&str> { self.declared_type_name.as_deref() }
    pub(crate) fn writers(&self) -> impl Iterator<Item = &ObservedFieldWriteV1> {
        self.writers.iter().map(|index| &self.inventory.writes[*index])
    }
    pub(crate) fn same_inventory(&self, claims: &OrdinaryNewFieldWriteClaimsV1) -> bool {
        Rc::ptr_eq(&self.inventory, &claims.writers)
    }

    pub(crate) fn needs_formal_target(&self, ordinal: usize) -> bool {
        self.writers.get(ordinal).is_some_and(|index| self.qualified_writers.contains(index))
    }
    pub(crate) fn corroborates_writer(&self, ordinal: usize, source: &OriginalFieldWriteSourceLoanV1) -> bool {
        Rc::ptr_eq(&self.inventory, &source.inventory) && self.writers.get(ordinal) == Some(&source.index)
    }

    /// Return the SAME constructor's store after independently checking its
    /// source, owner, exact assignment, field and source-order ordinal.
    /// A Parameter RHS stays a Parameter need; metadata grants no Integer.
    pub(crate) fn birth_store<'a>(
        &self,
        constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
        writer_ordinal: usize,
    ) -> Result<Option<&'a ConstructionStoreV1>, FieldValueSourceIssueV1> {
        use FieldValueSourceIssueV1 as Issue;
        let index = *self.writers.get(writer_ordinal).ok_or(Issue::StoreIdentity)?;
        let write = &self.inventory.writes[index];
        let reference = self.birth_stores.get(writer_ordinal).ok_or(Issue::StoreIdentity)?;
        let Some(reference) = reference else {
            return if write.birth_source().is_none() { Ok(None) } else { Err(Issue::ConstructorIdentity) };
        };
        let mut matching = constructors.rows().iter()
            .filter(|row| row.source_id() == &reference.constructor);
        let row = matching.next().ok_or(Issue::ConstructorIdentity)?;
        if matching.next().is_some() || row.kind() != ConstructorSourceKindV1::Birth
            || Some(row.box_name()) != write.owner_box()
            || write.birth_source() != Some(row.source_id())
        { return Err(Issue::ConstructorIdentity); }
        let plan = row.construction_source().as_ref().map_err(|_| Issue::ConstructionUnavailable)?;
        if plan.constructor() != Some(&(reference.constructor.clone(), write.owner()))
            || plan.object() != self.field.object()
        { return Err(Issue::ConstructorIdentity); }
        let store = plan.stores().get(reference.store_ordinal).ok_or(Issue::StoreIdentity)?;
        if store.field() != self.field || store.assignment() != write.assignment()
            || store.receiver_site() != write.receiver_site()
            || Some(store.receiver_binding()) != write.receiver()
        { return Err(Issue::StoreIdentity); }
        Ok(Some(store))
    }
}

pub(super) fn issue_value_sources_v1(
    inventory: &Rc<OriginalFieldWriterInventoryV1>,
    coverage: &ParserOrdinaryBoxSourceCoverageV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> FieldValueSourcesV1 {
    let mut fields: BTreeMap<(Box<str>, Box<str>), Vec<usize>> = BTreeMap::new();
    // Project only the retained original observations, never visit source again.
    for (index, write) in inventory.writes.iter().enumerate() {
        if let Some(class) = write.owner_box() {
            fields.entry((class.into(), write.field().into())).or_default().push(index);
        }
    }
    fields.into_iter().map(|((class, name), writers)| {
        let source = issue_one(inventory, coverage, constructors, &class, &name, writers, &BTreeMap::new());
        ((class, name), source.map(Rc::new))
    }).collect()
}

fn issue_one(
    inventory: &Rc<OriginalFieldWriterInventoryV1>,
    coverage: &ParserOrdinaryBoxSourceCoverageV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    class: &str,
    name: &str,
    writers: Vec<usize>,
    qualified: &BTreeMap<usize, crate::mir::resolved_semantics::BindingRefV1>,
) -> Result<DeclaredFieldValueSourceV1, FieldValueSourceIssueV1> {
    use FieldValueSourceIssueV1 as Issue;
    if inventory.opaque { return Err(Issue::HiddenWriters); }
    if inventory.value_incomplete { return Err(Issue::IncompleteWriterInventory); }
    if inventory.unattributed_fields.contains(name) && (qualified.is_empty()
        || inventory.writes.iter().enumerate().any(|(index, write)| write.field() == name
            && (!write.is_attributed() || write.owner_box().is_none()) && !qualified.contains_key(&index)))
    { return Err(Issue::UnattributedWriter); }
    let box_source = coverage.row_for(class).map_err(|_| Issue::Declaration)?
        .ok_or(Issue::Declaration)?;
    let first = &inventory.writes[*writers.first().ok_or(Issue::MissingWriter)?];
    let (field, declared) = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::source_declared_field(
        constructors, box_source,
        &OwnedExprSiteV1::new(first.owner(), first.assignment().target_site().clone()), name,
    ).map_err(|_| Issue::Declaration)?.ok_or(Issue::Declaration)?;
    let mut birth_stores = Vec::new();
    for index in &writers {
        let write = &inventory.writes[*index];
        let attributed = match qualified.get(index) {
            Some(binding) => !write.in_birth() && !write.is_attributed() && binding.owner() == write.owner(),
            None => write.is_attributed() && write.receiver().is_some() && write.owner_box() == Some(class),
        };
        if !attributed || write.field() != name { return Err(Issue::UnattributedWriter); }
        let plan = write.source().map_err(|_| Issue::OriginalPlan)?;
        if plan.owner() != write.owner()
            || plan.root().statement() != write.assignment().statement_site()
            || plan.nodes().first().is_none_or(|node| node.site() != write.assignment().value_site())
        { return Err(Issue::OriginalPlan); }
        let reference = if let Some(source) = write.birth_source() {
            let mut constructors = constructors.rows().iter().filter(|row| row.source_id() == source);
            let constructor = constructors.next().ok_or(Issue::ConstructorIdentity)?;
            if constructors.next().is_some() || constructor.kind() != ConstructorSourceKindV1::Birth
                || constructor.box_name() != class
            { return Err(Issue::ConstructorIdentity); }
            let construction = constructor.construction_source().as_ref().map_err(|_| Issue::ConstructionUnavailable)?;
            if construction.constructor() != Some(&(source.clone(), write.owner()))
                || construction.object() != field.object()
            { return Err(Issue::ConstructorIdentity); }
            let mut stores = construction.stores().iter().enumerate()
                .filter(|(_, store)| store.assignment() == write.assignment());
            let (ordinal, store) = stores.next().ok_or(Issue::StoreIdentity)?;
            if stores.next().is_some() || store.field() != field
                || store.receiver_site() != write.receiver_site()
            || Some(store.receiver_binding()) != write.receiver()
            { return Err(Issue::StoreIdentity); }
            Some(BirthStoreSourceRefV1 { constructor: source.clone(), store_ordinal: ordinal })
        } else { None };
        birth_stores.push(reference);
    }
    // An observed update cannot hide an omitted original Birth store. Every
    // store for this field must have exactly one original observed writer.
    for constructor in constructors.rows().iter()
        .filter(|row| row.box_name() == class && row.kind() == ConstructorSourceKindV1::Birth)
    {
        let plan = constructor.construction_source().as_ref().map_err(|_| Issue::ConstructionUnavailable)?;
        for store in plan.stores().iter().filter(|store| store.field() == field) {
            if writers.iter().filter(|index| {
                let write = &inventory.writes[**index];
                write.birth_source() == Some(constructor.source_id())
                    && write.assignment() == store.assignment()
            }).count() != 1 { return Err(Issue::MissingWriter); }
        }
    }
    Ok(DeclaredFieldValueSourceV1 {
        inventory: Rc::clone(inventory), field, declared_type_name: declared.map(Into::into),
        writers: writers.into_boxed_slice(), birth_stores: birth_stores.into_boxed_slice(),
        qualified_writers: qualified.keys().copied().collect(),
    })
}

impl OrdinaryNewFieldWriteClaimsV1 {
    /// Qualify only the deferred formal-target writers on the SAME inventory.
    /// Successful original sources are never reissued. All unknown same-name
    /// observations participate; a single failed target preserves the refusal.
    pub(crate) fn qualify_unattributed_value_source_v1<T>(
        &self, class: &str, name: &str,
        coverage: &ParserOrdinaryBoxSourceCoverageV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        mut target: impl FnMut(&OriginalFieldWriteSourceLoanV1, CanonicalFieldRefV1)
            -> Result<(crate::mir::resolved_semantics::BindingRefV1, T), String>,
    ) -> Result<(Rc<DeclaredFieldValueSourceV1>, Box<[(usize, T)]>), String> {
        let fail = |issue: FieldValueSourceIssueV1| format!("field-value/source/{issue:?}");
        if !matches!(self.value_source(class, name), Some(Err(FieldValueSourceIssueV1::UnattributedWriter))) {
            return Err("field-value/source/not-deferred-target".into());
        }
        if self.writers.opaque { return Err(fail(FieldValueSourceIssueV1::HiddenWriters)); }
        if self.writers.value_incomplete { return Err(fail(FieldValueSourceIssueV1::IncompleteWriterInventory)); }
        let mut writers: Vec<_> = self.writers.writes.iter().enumerate().filter(|(_, write)|
            write.field() == name && ((write.is_attributed() && write.owner_box() == Some(class))
                || !write.is_attributed() || write.owner_box().is_none())).map(|(index, _)| index).collect();
        writers.sort_unstable();
        writers.dedup();
        let first = writers.iter().map(|index| &self.writers.writes[*index])
            .find(|write| write.is_attributed() && write.owner_box() == Some(class))
            .ok_or_else(|| fail(FieldValueSourceIssueV1::MissingWriter))?;
        let box_source = coverage.row_for(class).ok().flatten().ok_or_else(|| fail(FieldValueSourceIssueV1::Declaration))?;
        let (field, _) = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::source_declared_field(
            constructors, box_source, &OwnedExprSiteV1::new(first.owner(), first.assignment().target_site().clone()), name)
            .map_err(|_| fail(FieldValueSourceIssueV1::Declaration))?.ok_or_else(|| fail(FieldValueSourceIssueV1::Declaration))?;
        let mut qualified = BTreeMap::new();
        let mut targets = Vec::new();
        for (ordinal, index) in writers.iter().enumerate() {
            let write = &self.writers.writes[*index];
            if write.is_attributed() && write.owner_box() == Some(class) { continue; }
            if write.in_birth() { return Err(fail(FieldValueSourceIssueV1::UnattributedWriter)); }
            let original = OriginalFieldWriteSourceLoanV1 { inventory: Rc::clone(&self.writers), index: *index };
            self.recheck_original_write_source_v1(&original).map_err(str::to_owned)?;
            let (binding, proof) = target(&original, field)?;
            qualified.insert(*index, binding);
            targets.push((ordinal, proof));
        }
        let source = issue_one(&self.writers, coverage, constructors, class, name, writers, &qualified).map_err(fail)?;
        Ok((Rc::new(source), targets.into_boxed_slice()))
    }
}
