//! Source-issued argument rows for selected direct-local ordinary `New`.
//!
//! Rows retain source meaning only.  They have no AST, MIR value, ABI, or
//! physical-call authority; the selected New consumer owns materialization.

use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OrdinaryNewTrivialArgumentKindV1 {
    Integer(i64),
    Bool(bool),
    /// The exact source `null` literal.
    Null,
    Local { binding: BindingRefV1 },
    Handle { binding: BindingRefV1 },
    /// A binding produced by an inventoried call expression (source
    /// provenance only; no scalar/rooted-storage class is claimed).
    BoundValue { binding: BindingRefV1 },
    /// A proven `i64` field read: the exact read site on this row keys
    /// the staged issuer proof; `object` is the receiver expression's
    /// lexical binding.
    I64Field { object: BindingRefV1 },
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrdinaryNewTrivialArgumentV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    new_site: OwnedExprSiteV1,
    ordinal: u32,
    site: SourceExprSiteV1,
    kind: OrdinaryNewTrivialArgumentKindV1,
}

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
}
