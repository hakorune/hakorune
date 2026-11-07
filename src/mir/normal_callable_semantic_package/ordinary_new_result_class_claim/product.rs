//! Immutable source result outcomes and their legacy safe projection.
//! Only the enclosing source solver constructs rows; lookup is passive.
use super::OrdinaryNewResultClassV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ResultValueOriginV1 {
    Null,
    /// A declared read of this receiver, never a Fresh/owning class.
    BorrowedReceiver { class: Box<str> },
    /// A storage-supported object source; never a Fresh/owning projection.
    MapHeld { field: hakorune_mir_defs::CanonicalFieldRefV1,
        key: crate::mir::resolved_semantics::BindingRefV1,
        object: hakorune_mir_defs::CanonicalObjectIdV1, class: Box<str> },
    Fresh(Box<str>),
    ForwardFormal { ordinal: u32 },
}

#[derive(Debug)]
pub(crate) struct ResultExitOriginV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) alternatives: BTreeSet<ResultValueOriginV1>,
    pub(super) witnesses: Box<[std::rc::Rc<super::ResultOriginWitnessV1>]>,
}
impl ResultExitOriginV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn alternatives(&self) -> &BTreeSet<ResultValueOriginV1> {
        &self.alternatives
    }
    pub(crate) fn witnesses(&self) -> &[std::rc::Rc<super::ResultOriginWitnessV1>] {
        &self.witnesses
    }
}

#[derive(Debug)]
pub(super) struct SourceResultRowV1 {
    pub(super) borrowed_result: Option<std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>,
    pub(super) exits: Box<[ResultExitOriginV1]>,
    pub(super) projection: Option<OrdinaryNewResultClassV1>,
}

/// One solver product. Legacy APIs cannot expose mixed/null-only rows.
#[derive(Debug, Default)]
pub(crate) struct VerifiedSourceCallableResultFactsV1 {
    rows: BTreeMap<CanonicalSameModuleCallableKeyV1, SourceResultRowV1>,
    result_catalog: std::cell::OnceCell<std::rc::Rc<crate::mir::normal_callable_semantic_package::callee_result_kind::CalleeResultKindCatalogV1>>,
    field_writers: Option<std::rc::Rc<super::super::field_write_claim::OriginalFieldWriterInventoryV1>>,
    child_relations: BTreeMap<
        (OwnedExprSiteV1, hakorune_mir_defs::CanonicalFieldRefV1),
        std::rc::Rc<super::child_relation::SourceConstructionChildRelationV1>,
    >,
    /// Original construction sites, retained so a consumer cannot mistake a
    /// partial set of child relations for a complete cohort.
    child_construction_sites: BTreeMap<hakorune_mir_defs::CanonicalObjectIdV1, BTreeSet<OwnedExprSiteV1>>,
    child_construction_sites_complete: bool,
    /// Statement-form Map installs, by their original set call site.
    map_installs: BTreeMap<OwnedExprSiteV1, std::rc::Rc<super::witness::MapStoredResultSourceV1>>,
}
impl VerifiedSourceCallableResultFactsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn bind_result_catalog_v1(
        &self, catalog: std::rc::Rc<crate::mir::normal_callable_semantic_package::callee_result_kind::CalleeResultKindCatalogV1>,
    ) -> Result<(), String> {
        self.result_catalog.set(catalog).map_err(|_| "[freeze:contract][callee-result-kind/facts-already-bound]".to_owned())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn result_catalog_v1(&self) -> Option<&crate::mir::normal_callable_semantic_package::callee_result_kind::CalleeResultKindCatalogV1> {
        self.result_catalog.get().map(std::rc::Rc::as_ref)
    }
    pub(super) fn attach_field_inventory_v1(&mut self, fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1) {
        self.field_writers = Some(std::rc::Rc::clone(fields.original_inventory_v1()));
    }
    pub(in crate::mir::normal_callable_semantic_package) fn field_inventory_v1(&self) -> Option<&std::rc::Rc<super::super::field_write_claim::OriginalFieldWriterInventoryV1>> {
        self.field_writers.as_ref()
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(super) fn set_map_installs(&mut self, installs: BTreeMap<OwnedExprSiteV1, std::rc::Rc<super::witness::MapStoredResultSourceV1>>) {
        self.map_installs = installs;
    }
    /// Constructions whose just-built payload a statement-form Map install keeps.
    pub(super) fn map_install_constructions_v1(&self) -> impl Iterator<Item = OwnedExprSiteV1> + '_ {
        self.map_installs.values().flat_map(|source| source.installed_constructions_v1())
    }
    pub(super) fn map_install_v1(&self, at: &OwnedExprSiteV1) -> Option<std::rc::Rc<super::witness::MapStoredResultSourceV1>> {
        self.map_installs.get(at).cloned()
    }
    /// Original explicit result declaration, retained with these SAME roots.
    /// This is declaration transport; consumers still owe return/use validation.
    pub(crate) fn borrowed_result_declaration_v1(
        &self, key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>> {
        self.rows.get(key)?.borrowed_result.as_ref()
    }

    pub(crate) fn get(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&OrdinaryNewResultClassV1> {
        match self.result_catalog.get() {
            Some(catalog) => catalog.object(key),
            None => self.rows.get(key)?.projection.as_ref(),
        }
    }
    pub(crate) fn contains_key(&self, key: &CanonicalSameModuleCallableKeyV1) -> bool {
        self.get(key).is_some()
    }
    pub(crate) fn iter(
        &self,
    ) -> impl Iterator<Item = (&CanonicalSameModuleCallableKeyV1, &OrdinaryNewResultClassV1)> {
        self.rows
            .iter()
            .filter_map(|(key, _)| self.get(key).map(|value| (key, value)))
    }
    pub(crate) fn keys(&self) -> impl Iterator<Item = &CanonicalSameModuleCallableKeyV1> {
        self.iter().map(|(key, _)| key)
    }
    pub(crate) fn outcomes(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&[ResultExitOriginV1]> {
        Some(&self.rows.get(key)?.exits)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn source_rows(
        &self,
    ) -> impl Iterator<Item = (&CanonicalSameModuleCallableKeyV1, &[ResultExitOriginV1])> {
        self.rows.iter().map(|(key, row)| (key, row.exits.as_ref()))
    }
    pub(crate) fn construction_child_relations(
        &self,
    ) -> impl Iterator<Item = &super::child_relation::SourceConstructionChildRelationV1> {
        self.child_relations.values().map(std::rc::Rc::as_ref)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn child_construction_sites_v1(
        &self, parent: hakorune_mir_defs::CanonicalObjectIdV1,
    ) -> Option<&BTreeSet<OwnedExprSiteV1>> {
        self.child_construction_sites_complete.then(|| self.child_construction_sites.get(&parent)).flatten()
    }
    pub(crate) fn construction_child_relation(
        &self,
        outer: &OwnedExprSiteV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
    ) -> Option<&super::child_relation::SourceConstructionChildRelationV1> {
        self.child_relations
            .get(&(outer.clone(), field))
            .map(std::rc::Rc::as_ref)
    }
    /// Lend the SAME relation allocation through argument materialization.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn construction_child_relation_loans(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<super::child_relation::SourceConstructionChildRelationV1>>
    {
        self.child_relations.values()
    }
    pub(super) fn attach_child_relations(
        &mut self,
        relations: BTreeMap<
            (OwnedExprSiteV1, hakorune_mir_defs::CanonicalFieldRefV1),
            super::child_relation::SourceConstructionChildRelationV1,
        >,
        sites: BTreeMap<hakorune_mir_defs::CanonicalObjectIdV1, BTreeSet<OwnedExprSiteV1>>,
        sites_complete: bool,
    ) {
        self.child_construction_sites = sites;
        self.child_construction_sites_complete = sites_complete;
        self.child_relations = relations
            .into_iter()
            .map(|(key, relation)| (key, std::rc::Rc::new(relation)))
            .collect();
    }
    /// Resolve passive receiver projections once, after the SAME child cohort.
    pub(in crate::mir::normal_callable_semantic_package) fn attach_receiver_fields_v1(
        &mut self,
        batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> Result<(), (crate::mir::resolved_semantics::OwnedExprSiteV1, String)> {
        use super::witness::{ResultWitnessStepV1 as Step};
        let mut normalizer = ReceiverFieldNormalizerV1 {
            facts: self, batch, constructors, fields: BTreeMap::new(), roots: BTreeMap::new(),
        };
        let mut replacements = BTreeMap::new();
        let mut invalid = BTreeSet::new();
        let mut dependents = BTreeMap::<_, BTreeSet<_>>::new();
        for (key, row) in &self.rows {
            let mut exits = Vec::new();
            for exit in row.exits.iter() {
                let mut roots = Vec::new();
                for root in exit.witnesses.iter() {
                    if let Step::Call { key: target, .. } = root.step() {
                        dependents.entry(target.clone()).or_default().insert(key.clone());
                    }
                    match normalizer.normalize(key, root)? {
                        Some(root) => roots.push(root),
                        None => { invalid.insert(key.clone()); }
                    }
                }
                exits.push(roots);
            }
            replacements.insert(key.clone(), exits);
        }
        // Replace all rows from one memo before checking canonical membership.
        // A failed alternative invalidates its entire row and all callers.
        for (key, exits) in replacements {
            let row = self.rows.get_mut(&key).expect("enumerated source row");
            for (exit, roots) in row.exits.iter_mut().zip(exits) { exit.witnesses = roots.into_boxed_slice(); }
        }
        for (key, row) in &self.rows {
            let Some(declaration) = row.borrowed_result.as_ref() else { continue; };
            let crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { binding } = declaration.support() else { continue; };
            if row.exits.iter().any(|exit| exit.witnesses.is_empty() || exit.witnesses.iter().any(|root| {
                let canonical = match root.step() {
                    Step::Call { key, callee, .. } => self.outcomes(key).is_some_and(|exits|
                        exits.iter().flat_map(|exit| exit.witnesses()).any(|original| std::rc::Rc::ptr_eq(original, callee))),
                    _ => true,
                };
                !canonical || !root.supports_receiver_v1(binding, declaration.declared_type())
                    || !(root.is_literal_null_v1() || root.recheck_receiver_payload_v1(self))
            })) { invalid.insert(key.clone()); }
        }
        let mut pending: Vec<_> = invalid.iter().cloned().collect();
        while let Some(key) = pending.pop() {
            for caller in dependents.get(&key).into_iter().flatten() {
                if invalid.insert(caller.clone()) { pending.push(caller.clone()); }
            }
        }
        for key in invalid { self.rows.remove(&key); }
        Ok(())
    }
    pub(super) fn insert(&mut self, key: CanonicalSameModuleCallableKeyV1, row: SourceResultRowV1) {
        self.rows.insert(key, row);
    }
}

/// One field-seal pass owns the immutable witness DAG rewrite. The pointer is
/// the original Rc identity, never a source key or an executable identifier.
struct ReceiverFieldNormalizerV1<'a> {
    facts: &'a VerifiedSourceCallableResultFactsV1,
    batch: &'a crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    constructors: &'a crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    fields: BTreeMap<(Box<str>, Box<str>), Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::StoredChildFieldSourceV1>>>,
    roots: BTreeMap<*const super::ResultOriginWitnessV1, Option<std::rc::Rc<super::ResultOriginWitnessV1>>>,
}
impl ReceiverFieldNormalizerV1<'_> {
    fn normalize(&mut self, key: &CanonicalSameModuleCallableKeyV1, root: &std::rc::Rc<super::ResultOriginWitnessV1>)
        -> Result<Option<std::rc::Rc<super::ResultOriginWitnessV1>>, (OwnedExprSiteV1, String)> {
        use super::witness::{ReceiverResultProjectionV1 as Projection, ResultOriginWitnessV1, ResultWitnessStepV1 as Step};
        use std::rc::Rc;
        if !self.facts.outcomes(key).is_some_and(|exits|
            exits.iter().flat_map(|exit| exit.witnesses()).any(|original| Rc::ptr_eq(original, root))) { return Ok(None); }
        let identity = Rc::as_ptr(root);
        if let Some(normalized) = self.roots.get(&identity) { return Ok(normalized.clone()); }
        let normalized = match root.step() {
            Step::Receiver { binding, projection: Projection::PendingField(read) } => {
                let field = (read.parent.clone(), read.field.clone());
                if !self.fields.contains_key(&field) {
                    let source = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue::source_claims::declared_stored_child_field_v1(
                        self.batch, self.constructors, self.facts, &read.parent, &read.field, root.site(), false)
                        .map_err(|issue| (root.site().clone(), issue))?.map(Rc::new);
                    self.fields.insert(field.clone(), source);
                }
                self.fields[&field].as_ref().filter(|source| self.facts.rows.get(key)
                    .and_then(|row| row.borrowed_result.as_ref()).is_some_and(|declaration|
                        declaration.owner() == binding.owner() && declaration.declared_type() == source.child_class()
                            && read.parent.as_ref() == key.owner() && source.recheck_v1(self.facts, &read.field)))
                    .map(|source| Rc::new(ResultOriginWitnessV1 { site: root.site().clone(), origin: root.origin().clone(),
                        step: Step::Receiver { binding: *binding, projection: Projection::Field { read: read.clone(), source: Rc::clone(source) } } }))
            }
            Step::Call { site, key: target, callee, substitution } => {
                self.normalize(target, callee)?.map(|normalized| {
                    if Rc::ptr_eq(callee, &normalized) { return Rc::clone(root); }
                    Rc::new(ResultOriginWitnessV1 { site: root.site().clone(), origin: root.origin().clone(),
                        step: Step::Call { site: site.clone(), key: target.clone(), callee: normalized, substitution: substitution.clone() } })
                })
            }
            _ => Some(Rc::clone(root)),
        };
        self.roots.insert(identity, normalized.clone());
        Ok(normalized)
    }
}
