//! Source-issued argument rows for selected direct-local ordinary `New`.
//!
//! Rows retain source meaning only.  They have no AST, MIR value, ABI, or
//! physical-call authority; the selected New consumer owns materialization.

use super::result_class_claim::SourceConstructionChildRelationV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1};
use std::rc::Rc;

pub(crate) use crate::mir::resolved_semantics::home_new_prefix::{
    QualifiedStaticCallArgumentKindV1, SelectedNewArgumentKindV1,
};
use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentV1;

#[derive(Debug, Clone)]
pub(crate) struct OrdinaryNewTrivialArgumentV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    new_site: OwnedExprSiteV1,
    source: SelectedNewArgumentV1,
    /// Original actual/formal/field provenance only, never a Home grant.
    child_relations: Box<[Rc<SourceConstructionChildRelationV1>]>,
    field_array_read: Option<Rc<ArgumentFieldArrayReadSourceV1>>,
}

impl PartialEq for OrdinaryNewTrivialArgumentV1 {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.new_site == other.new_site
            && self.ordinal() == other.ordinal()
            && self.site() == other.site()
            && match (self.kind(), other.kind()) {
                (SelectedNewArgumentKindV1::Expression { evaluation: a }, SelectedNewArgumentKindV1::Expression { evaluation: b }) => Rc::ptr_eq(a, b),
                _ => self.kind() == other.kind(),
            }
            && match (&self.field_array_read, &other.field_array_read) {
                (None, None) => true, (Some(a), Some(b)) => Rc::ptr_eq(a, b), _ => false,
            }
            && self.child_relations.len() == other.child_relations.len()
            && self
                .child_relations
                .iter()
                .zip(other.child_relations.iter())
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
impl Eq for OrdinaryNewTrivialArgumentV1 {}

impl OrdinaryNewTrivialArgumentV1 {
    pub(crate) fn new(
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        new_site: OwnedExprSiteV1,
        ordinal: u32,
        site: SourceExprSiteV1,
        kind: SelectedNewArgumentKindV1,
    ) -> Self {
        Self::from_resolved(new_site, owner, SelectedNewArgumentV1::new(ordinal, site, kind))
    }

    pub(super) fn from_resolved(
        new_site: OwnedExprSiteV1,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        source: SelectedNewArgumentV1,
    ) -> Self {
        Self {
            owner,
            new_site,
            source,
            child_relations: Box::new([]),
            field_array_read: None,
        }
    }

    pub(crate) fn with_explicit_move(mut self, explicit_move: bool) -> Self {
        self.source = self.source.with_explicit_move(explicit_move);
        self
    }

    pub(crate) const fn explicit_move(&self) -> bool {
        self.source.explicit_move()
    }

    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn new_site(&self) -> &OwnedExprSiteV1 {
        &self.new_site
    }
    pub(crate) const fn ordinal(&self) -> u32 {
        self.source.ordinal()
    }
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        self.source.site()
    }
    pub(crate) fn kind(&self) -> &SelectedNewArgumentKindV1 {
        self.source.kind()
    }

    pub(crate) fn child_relations(&self) -> &[Rc<SourceConstructionChildRelationV1>] {
        &self.child_relations
    }

    pub(super) fn retain_child_relations(
        &mut self,
        relations: Box<[Rc<SourceConstructionChildRelationV1>]>,
    ) {
        self.child_relations = relations;
    }
}

/// A use of the SAME checked read by one original local; New site/ordinal stay
/// on the containing argument. This is source meaning, never a physical value.
#[derive(Debug)]
pub(crate) struct ArgumentFieldArrayReadSourceV1 {
    read: Rc<super::CheckedFieldArrayAccessSourceV1>,
    binding: BindingRefV1,
    declaration: crate::mir::resolved_semantics::SourceBindingSiteV1,
}
impl ArgumentFieldArrayReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn new(
        read: Rc<super::CheckedFieldArrayAccessSourceV1>, binding: BindingRefV1,
        declaration: crate::mir::resolved_semantics::SourceBindingSiteV1,
    ) -> Self { Self { read, binding, declaration } }
    pub(crate) fn read(&self) -> &Rc<super::CheckedFieldArrayAccessSourceV1> { &self.read }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn declaration(&self) -> &crate::mir::resolved_semantics::SourceBindingSiteV1 { &self.declaration }
}
impl OrdinaryNewTrivialArgumentV1 {
    pub(crate) fn field_array_read(&self) -> Option<&Rc<ArgumentFieldArrayReadSourceV1>> { self.field_array_read.as_ref() }
    pub(in crate::mir::normal_callable_semantic_package) fn retain_field_array_read(
        &mut self, read: Rc<ArgumentFieldArrayReadSourceV1>,
    ) -> Result<(), String> {
        if !matches!(self.kind(), SelectedNewArgumentKindV1::Local { binding } if *binding == read.binding)
            || self.owner != read.read.call().owner() || self.field_array_read.is_some() {
            return Err("[freeze:contract][named-array/argument-source-drift]".into());
        }
        self.field_array_read = Some(read);
        Ok(())
    }
}
