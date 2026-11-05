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
    declared_type_reference: Option<crate::ast::TypeReferenceV1>,
    writers: Box<[usize]>,
    birth_stores: Box<[Option<BirthStoreSourceRefV1>]>,
    qualified_writers: BTreeSet<usize>,
}
impl DeclaredFieldValueSourceV1 {
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(crate) fn declared_type_reference(&self) -> Option<&crate::ast::TypeReferenceV1> { self.declared_type_reference.as_ref() }
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

    /// The sole untyped field writer initializes its original Map provider in Birth.
    pub(crate) fn map_birth_provider_v1<'a, 'b>(
        &'a self,
        constructors: &'b VerifiedInstanceConstructorSemanticBatchV1,
    ) -> Option<(&'a ObservedFieldWriteV1, &'b ConstructionStoreV1)> {
        let mut writers = self.writers();
        let writer = writers.next()?;
        if writers.next().is_some() || !writer.in_birth() || self.declared_type_reference().is_some() {
            return None;
        }
        let store = self.birth_store(constructors, 0).ok()??;
        (store.mode() == Ok(crate::mir::instruction::FieldSetModeV1::DYNAMIC_MAP_INITIALIZE)
            && matches!(store.rhs(), crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1::ProviderConstruction {
                object: None, class, .. } if class.as_ref() == "MapBox"))
            .then_some((writer, store))
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
        if store.field() != self.field || store.assignment() != Some(write.assignment())
            || store.receiver_site() != Some(write.receiver_site())
            || Some(store.receiver_binding()) != write.receiver()
        { return Err(Issue::StoreIdentity); }
        Ok(Some(store))
    }
}

/// A constant initializer is a field source, never a forwarded formal loan.
#[derive(Debug, Clone)]
pub(crate) struct NullFieldInitializationSourceV1 {
    source: NullFieldValueOriginV1,
    construction: Rc<crate::mir::normal_callable_semantic_package::instance_construction::ConstructionSourceStructureV1>,
    store: usize,
}
#[derive(Debug, Clone)]
enum NullFieldValueOriginV1 {
    Assignment(Rc<DeclaredFieldValueSourceV1>),
    Declaration { inventory: Rc<OriginalFieldWriterInventoryV1>,
        initialization: Rc<crate::mir::normal_callable_semantic_package::instance_constructor_initialization::NullableConstructionInitializationV1> },
}
impl PartialEq for NullFieldValueOriginV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Assignment(a), Self::Assignment(b)) => Rc::ptr_eq(a, b),
            (Self::Declaration { inventory: a, initialization: ai }, Self::Declaration { inventory: b, initialization: bi }) =>
                Rc::ptr_eq(a, b) && Rc::ptr_eq(ai, bi),
            _ => false,
        }
    }
}
impl OriginalFieldWriterInventoryV1 {
    fn corroborates_unwritten_field_v1(&self, class: &str, name: &str) -> bool {
        !self.opaque && !self.value_incomplete && !self.unattributed_fields.contains(name)
            && !self.writes.iter().any(|write| write.field() == name
                && (write.owner_box() == Some(class) || write.owner_box().is_none() || !write.is_attributed()))
    }
}
impl PartialEq for NullFieldInitializationSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && Rc::ptr_eq(&self.construction, &other.construction)
            && self.store == other.store
    }
}
impl Eq for NullFieldInitializationSourceV1 {}
impl NullFieldInitializationSourceV1 {
    pub(crate) fn same_inventory(&self, inventory: &Rc<OriginalFieldWriterInventoryV1>) -> bool {
        match &self.source {
            NullFieldValueOriginV1::Assignment(source) => Rc::ptr_eq(&source.inventory, inventory),
            NullFieldValueOriginV1::Declaration { inventory: original, .. } => Rc::ptr_eq(original, inventory),
        }
    }
    pub(crate) fn recheck(&self, field: CanonicalFieldRefV1, child: hakorune_mir_defs::CanonicalObjectIdV1, name: &str) -> bool {
        let Some(store) = self.construction.stores().get(self.store) else { return false; };
        if store.field() != field || !matches!(store.rhs(),
            crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1::Null { child: actual } if *actual == child) {
            return false;
        }
        match &self.source {
            NullFieldValueOriginV1::Assignment(source) => {
                let mut writers = source.writers();
                let Some(writer) = writers.next() else { return false; };
                writers.next().is_none() && writer.field() == name && source.field() == field
                    && store.assignment() == Some(writer.assignment()) && store.receiver_site() == Some(writer.receiver_site())
                    && Some(store.receiver_binding()) == writer.receiver()
                    && self.construction.constructor().is_some_and(|(id, owner)| writer.birth_source() == Some(id) && *owner == writer.owner())
            }
            NullFieldValueOriginV1::Declaration { inventory, initialization } => {
                let crate::mir::normal_callable_semantic_package::ConstructionInitOriginV1::Declaration { source, field: actual } = store.origin()
                    else { return false; };
                Rc::ptr_eq(source, initialization) && *actual == field
                    && self.construction.constructor().is_some_and(|(_, owner)|
                        initialization.declaration_default(*owner, field, child, name))
                    && inventory.corroborates_unwritten_field_v1(initialization.parent_class(), name)
            }
        }
    }
}
impl DeclaredFieldValueSourceV1 {
    pub(crate) fn null_initialization_v1(self: &Rc<Self>, constructors: &VerifiedInstanceConstructorSemanticBatchV1)
        -> Option<NullFieldInitializationSourceV1>
    {
        let declaration = crate::mir::declared_type_storage::declared_object_field_type_v1(self.declared_type_name()?)?;
        if !declaration.explicitly_nullable() { return None; }
        let mut writers = self.writers();
        let writer = writers.next()?;
        if writers.next().is_some() || !writer.in_birth() { return None; }
        let mut births = constructors.rows().iter().filter(|row|
            row.kind() == ConstructorSourceKindV1::Birth && Some(row.box_name()) == writer.owner_box());
        let birth = births.next()?;
        if births.next().is_some() { return None; }
        let store = self.birth_store(constructors, 0).ok()??;
        let crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1::Null { child } = store.rhs() else { return None; };
        let construction = birth.construction_source().as_ref().ok()?;
        let ordinal = construction.stores().iter().position(|row| std::ptr::eq(row, store))?;
        let proof = NullFieldInitializationSourceV1 { source: NullFieldValueOriginV1::Assignment(Rc::clone(self)), construction: Rc::clone(construction), store: ordinal };
        proof.recheck(self.field(), *child, writer.field()).then_some(proof)
    }
}

impl NullFieldInitializationSourceV1 {
    pub(crate) fn declaration_default_v1(constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        class: &str, name: &str, field: CanonicalFieldRefV1, child: hakorune_mir_defs::CanonicalObjectIdV1) -> Option<Self> {
        let mut births = constructors.rows().iter().filter(|row| row.kind() == ConstructorSourceKindV1::Birth && row.box_name() == class);
        let birth = births.next()?;
        if births.next().is_some() { return None; }
        let construction = birth.construction_source().as_ref().ok()?;
        let (store, original) = construction.stores().iter().enumerate().find(|(_, row)| row.field() == field)?;
        let crate::mir::normal_callable_semantic_package::ConstructionInitOriginV1::Declaration { source, .. } = original.origin()
            else { return None; };
        if source.parent_class() != class { return None; }
        let proof = Self { source: NullFieldValueOriginV1::Declaration {
            inventory: Rc::clone(constructors.original_field_writers_v1()), initialization: Rc::clone(source),
        }, construction: Rc::clone(construction), store };
        proof.recheck(field, child, name).then_some(proof)
    }
}

pub(super) fn add_declaration_null_residences_v1(
    inventory: &Rc<OriginalFieldWriterInventoryV1>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1, residences: &mut OwnedFieldResidencesV1,
) {
    if !Rc::ptr_eq(inventory, constructors.original_field_writers_v1()) { return; }
    for birth in constructors.rows() {
        let Ok(plan) = birth.construction_source() else { continue; };
        for store in plan.stores() {
            let crate::mir::normal_callable_semantic_package::ConstructionInitOriginV1::Declaration { source, field } = store.origin()
                else { continue; };
            let Some(row) = source.field(*field) else { continue; };
            if NullFieldInitializationSourceV1::declaration_default_v1(constructors, source.parent_class(), &row.name, *field, row.child).is_some() {
                residences.insert((source.parent_class().into(), row.name.clone()), OwnedFieldResidenceV1::Null);
            }
        }
    }
}

/// Source class for the same complete original writer cohort. This grants
/// neither a borrowed field payload nor an owned Text result.
#[derive(Debug, Clone)]
pub(crate) struct TextFieldValueSourceV1 {
    source: Rc<DeclaredFieldValueSourceV1>,
    sum: Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>,
}
impl PartialEq for TextFieldValueSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) && Rc::ptr_eq(&self.sum, &other.sum)
    }
}
impl Eq for TextFieldValueSourceV1 {}
impl TextFieldValueSourceV1 {
    /// Untyped, or a declared Text slot (`String` / `StringBox` / `String?`): any other declared type is not a Text class.
    pub(in crate::mir::normal_callable_semantic_package) fn declaration_admits_v1(declared: Option<&str>) -> bool {
        declared.is_none_or(crate::mir::declared_type_storage::is_declared_text_type_name_v1)
    }
    /// A Print root admits the read at any node; a Return admits only a value that IS this read.
    pub(in crate::mir::normal_callable_semantic_package) fn role_admits_v1(
        plan: &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        read: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1 as Role;
        match plan.root() {
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement { role: Role::PrintValue, .. } => true,
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement { role: Role::ReturnValue, .. } =>
                plan.nodes().first().is_some_and(|node| node.site() == read),
            _ => false,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn lend(
        ingress: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        claims: &OrdinaryNewFieldWriteClaimsV1, source: &Rc<DeclaredFieldValueSourceV1>,
    ) -> Option<Self> {
        if !Self::declaration_admits_v1(source.declared_type_name()) { return None; }
        let sum = ingress.source_field_value_sum_v1(claims, source);
        if crate::config::env::builder_debug_enabled() { eprintln!("[text-lend] sum={:?}", sum); }
        let sum = sum?;
        sum.text_only().then(|| Self { source: Rc::clone(source), sum })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(
        &self,
        ingress: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        claims: &OrdinaryNewFieldWriteClaimsV1, name: &str,
    ) -> bool {
        self.source.writers().all(|writer| writer.field() == name)
            && Self::lend(ingress, claims, &self.source).as_ref().is_some_and(|current| current == self)
    }
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.source.field() }
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
                .filter(|(_, store)| store.assignment() == Some(write.assignment()));
            let (ordinal, store) = stores.next().ok_or(Issue::StoreIdentity)?;
            if stores.next().is_some() || store.field() != field
                || store.receiver_site() != Some(write.receiver_site())
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
                    && store.assignment() == Some(write.assignment())
            }).count() != 1 { return Err(Issue::MissingWriter); }
        }
    }
    let declared_type_reference = declared.as_deref().map(crate::ast::TypeReferenceV1::parse)
        .transpose().map_err(|_| Issue::Declaration)?;
    Ok(DeclaredFieldValueSourceV1 {
        inventory: Rc::clone(inventory), field, declared_type_name: declared.map(Into::into),
        declared_type_reference,
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

impl OrdinaryNewFieldWriteClaimsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn object_sources_with_sums_v1(
        &self, coverage: &ParserOrdinaryBoxSourceCoverageV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        mut sum_for: impl FnMut(&Rc<DeclaredFieldValueSourceV1>) -> Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>>,
    ) -> ObjectFieldClassSourcesV1 {
        let mut additional = BTreeMap::new();
        for (class, name, source) in self.value_sources() {
            let Ok(source) = source else { continue };
            let Some(sum) = sum_for(source) else { continue };
            let key = (Box::<str>::from(class), Box::<str>::from(name));
            if !self.object_classes.contains_key(&key) {
                if let Some(proof) = self.object_source_from_sum_v1(class, name, source, &sum, coverage, constructors) {
                    additional.insert(key, proof);
                }
            }
        }
        if additional.is_empty() { return Rc::clone(&self.object_classes); }
        let mut all = self.object_classes.as_ref().clone();
        all.extend(additional);
        Rc::new(all)
    }
    /// A declared child class from the exact closed writer sum, not an owned
    /// child permission. The sole Birth store still retains its original RHS.
    pub(in crate::mir::normal_callable_semantic_package) fn object_source_from_sum_v1(
        &self, parent: &str, name: &str, source: &Rc<DeclaredFieldValueSourceV1>,
        sum: &Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>,
        coverage: &ParserOrdinaryBoxSourceCoverageV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    ) -> Option<Rc<ObjectFieldClassSourceV1>> {
        use crate::mir::normal_callable_semantic_package::{SourceValueAlternativeV1 as Value, ConstructionStoreRhsV1};
        if !source.same_inventory(self) || !self.value_source(parent, name)?.as_ref().ok()
            .is_some_and(|original| Rc::ptr_eq(original, source)) { return None; }
        let declared = source.declared_type_name()?;
        let nominal = crate::mir::declared_type_storage::declared_object_field_type_v1(declared)?;
        let class = nominal.class_name();
        let child_source = coverage.row_for(class).ok().flatten()?;
        let child = constructors.object_for(&child_source).ok()?;
        let mut writers = source.writers();
        let writer = writers.next()?;
        if writers.next().is_some() || !writer.in_birth() || !writer.is_attributed()
            || writer.owner_box() != Some(parent) || writer.field() != name { return None; }
        let store = source.birth_store(constructors, 0).ok().flatten()?;
        if !matches!(store.rhs(), ConstructionStoreRhsV1::Parameter { provided: Some(object), .. } if *object == child)
            || sum.alternatives().is_empty()
            || !sum.alternatives().iter().all(|value| matches!(value, Value::Null)
                || matches!(value, Value::Object(actual) if actual.as_ref() == class)) { return None; }
        Some(Rc::new(ObjectFieldClassSourceV1 {
            inventory: Rc::clone(&self.writers), class: class.into(),
            declaration: Some((source.field(), Some(declared.to_owned()))), array_qualified: false,
            source_sum: Some((Rc::clone(source), Rc::clone(sum))),
        }))
    }
}
