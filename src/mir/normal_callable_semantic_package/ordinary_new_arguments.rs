//! Source-issued argument rows for selected direct-local ordinary `New`.
//!
//! Rows retain source meaning only.  They have no AST, MIR value, ABI, or
//! physical-call authority; the selected New consumer owns materialization.

use super::result_class_claim::SourceConstructionChildRelationV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OrdinaryNewTrivialArgumentKindV1 {
    Integer(i64),
    Bool(bool),
    /// The exact source `null` literal.
    Null,
    Local {
        binding: BindingRefV1,
    },
    Handle {
        binding: BindingRefV1,
    },
    /// A binding produced by an inventoried call expression (source
    /// provenance only; no scalar/rooted-storage class is claimed).
    BoundValue {
        binding: BindingRefV1,
    },
    /// A proven `i64` field read: the exact read site on this row keys
    /// the staged issuer proof; `object` is the receiver expression's
    /// lexical binding.
    I64Field {
        object: BindingRefV1,
    },
    /// SAME qualified original New actual; kept distinct from raw scalar or
    /// object bindings until complete Text/Birth transport is verified.
    Expression { evaluation: Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1> },
    /// A proven `Alias.m(..)` qualified static-box call: the sealed claim
    /// row proves the `StaticBoxMethod` target and its `ExactI64` result
    /// disposition; `arguments` are the callee's sealed literal actuals in
    /// source order. Only the provider-`new` argument arm inside a Birth
    /// plan seals this shape — the `local x = new` argument inventory
    /// never produces it.
    QualifiedStaticCall {
        target: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        arguments: Box<[QualifiedStaticCallArgumentKindV1]>,
    },
}

/// One sealed literal actual of a qualified static-call provider argument.
/// The claim's required-i64 ordinals were discharged at seal time; this
/// row keeps only the value the emitter materializes once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QualifiedStaticCallArgumentKindV1 {
    Integer(i64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub(crate) struct OrdinaryNewTrivialArgumentV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    new_site: OwnedExprSiteV1,
    ordinal: u32,
    site: SourceExprSiteV1,
    kind: OrdinaryNewTrivialArgumentKindV1,
    /// Original actual/formal/field provenance only, never a Home grant.
    child_relations: Box<[Rc<SourceConstructionChildRelationV1>]>,
    field_array_read: Option<Rc<ArgumentFieldArrayReadSourceV1>>,
}

impl PartialEq for OrdinaryNewTrivialArgumentV1 {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.new_site == other.new_site
            && self.ordinal == other.ordinal
            && self.site == other.site
            && match (&self.kind, &other.kind) {
                (OrdinaryNewTrivialArgumentKindV1::Expression { evaluation: a }, OrdinaryNewTrivialArgumentKindV1::Expression { evaluation: b }) => Rc::ptr_eq(a, b),
                _ => self.kind == other.kind,
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
        kind: OrdinaryNewTrivialArgumentKindV1,
    ) -> Self {
        Self {
            owner,
            new_site,
            ordinal,
            site,
            kind,
            child_relations: Box::new([]),
            field_array_read: None,
        }
    }

    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn new_site(&self) -> &OwnedExprSiteV1 {
        &self.new_site
    }
    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    pub(crate) fn kind(&self) -> &OrdinaryNewTrivialArgumentKindV1 {
        &self.kind
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
        if !matches!(self.kind, OrdinaryNewTrivialArgumentKindV1::Local { binding } if binding == read.binding)
            || self.owner != read.read.call().owner() || self.field_array_read.is_some() {
            return Err("[freeze:contract][named-array/argument-source-drift]".into());
        }
        self.field_array_read = Some(read);
        Ok(())
    }
}
