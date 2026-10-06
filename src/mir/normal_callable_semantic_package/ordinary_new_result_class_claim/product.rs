//! Immutable source result outcomes and their legacy safe projection.
//! Only the enclosing source solver constructs rows; lookup is passive.
use super::OrdinaryNewResultClassV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ResultValueOriginV1 {
    Null,
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
    pub(super) exits: Box<[ResultExitOriginV1]>,
    pub(super) projection: Option<OrdinaryNewResultClassV1>,
}

/// One solver product. Legacy APIs cannot expose mixed/null-only rows.
#[derive(Debug, Default)]
pub(crate) struct VerifiedSourceCallableResultFactsV1 {
    rows: BTreeMap<CanonicalSameModuleCallableKeyV1, SourceResultRowV1>,
    child_relations: BTreeMap<
        (OwnedExprSiteV1, hakorune_mir_defs::CanonicalFieldRefV1),
        super::child_relation::SourceConstructionChildRelationV1,
    >,
}
impl VerifiedSourceCallableResultFactsV1 {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn get(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&OrdinaryNewResultClassV1> {
        self.rows.get(key)?.projection.as_ref()
    }
    pub(crate) fn contains_key(&self, key: &CanonicalSameModuleCallableKeyV1) -> bool {
        self.get(key).is_some()
    }
    pub(crate) fn iter(
        &self,
    ) -> impl Iterator<Item = (&CanonicalSameModuleCallableKeyV1, &OrdinaryNewResultClassV1)> {
        self.rows
            .iter()
            .filter_map(|(key, row)| row.projection.as_ref().map(|value| (key, value)))
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
    pub(super) fn source_rows(
        &self,
    ) -> impl Iterator<Item = (&CanonicalSameModuleCallableKeyV1, &[ResultExitOriginV1])> {
        self.rows.iter().map(|(key, row)| (key, row.exits.as_ref()))
    }
    pub(crate) fn construction_child_relations(
        &self,
    ) -> impl Iterator<Item = &super::child_relation::SourceConstructionChildRelationV1> {
        self.child_relations.values()
    }
    pub(crate) fn construction_child_relation(
        &self,
        outer: &OwnedExprSiteV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
    ) -> Option<&super::child_relation::SourceConstructionChildRelationV1> {
        self.child_relations.get(&(outer.clone(), field))
    }
    pub(super) fn attach_child_relations(
        &mut self,
        relations: BTreeMap<
            (OwnedExprSiteV1, hakorune_mir_defs::CanonicalFieldRefV1),
            super::child_relation::SourceConstructionChildRelationV1,
        >,
    ) {
        self.child_relations = relations;
    }
    pub(super) fn insert(&mut self, key: CanonicalSameModuleCallableKeyV1, row: SourceResultRowV1) {
        self.rows.insert(key, row);
    }
}
