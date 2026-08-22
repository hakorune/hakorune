//! Source-only initial-index seed relation for the common V2 S6C ingress.
//!
//! This module transports resolver/S6C initializer evidence.  It does not
//! issue a Const, declaration, ValueId, read receipt, or session effect.

use super::ids::LoopValueKeyV1;
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use crate::mir::callable_semantic_batch::S6CTypedInputRoleV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedInitializerRelationV1, ResolvedLiteralSourceV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InitialIndexSeedRelationRejectV1 {
    ForeignOwner,
    MissingIndexInput,
    NonLocalDeclaration,
    MissingInitializerSite,
    WrongType,
    WrongLiteral,
}

/// Non-Clone source-only relation.  The borrowed initializer and literal are
/// still owned by the resolver/S6C typed source product.
#[derive(Debug)]
pub(crate) struct PreparedLoopV2InitialIndexSeedRelationV1<'facts> {
    owner: FunctionOwnerIdV1,
    loop_site: SourceStmtSiteV1,
    initializer: &'facts ResolvedInitializerRelationV1,
    literal: &'facts ResolvedLiteralSourceV1,
    index_carrier_entry: LoopValueKeyV1,
}

impl PreparedLoopV2InitialIndexSeedRelationV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn declaration_site(&self) -> &SourceBindingSiteV1 {
        self.initializer.declaration_site()
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.initializer.binding()
    }

    pub(crate) fn initializer_site(&self) -> Option<&SourceExprSiteV1> {
        self.initializer.initializer_site()
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.initializer.declared_type_name()
    }

    pub(crate) fn literal(&self) -> &ResolvedLiteralSourceV1 {
        self.literal
    }

    pub(crate) const fn index_carrier_entry(&self) -> LoopValueKeyV1 {
        self.index_carrier_entry
    }
}

pub(crate) fn issue_s6c_v2_initial_index_seed_relation_v1<'facts>(
    ingress: S6CPrephysicalIngressRefV2<'_, '_, 'facts>,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2InitialIndexSeedRelationV1<'facts>, InitialIndexSeedRelationRejectV1> {
    if ingress.source_owner() != expected_owner {
        return Err(InitialIndexSeedRelationRejectV1::ForeignOwner);
    }
    let typed = ingress.typed_input_relation();
    let initializer = typed.initializer();
    if !matches!(
        initializer.declaration_site(),
        SourceBindingSiteV1::Local { .. }
    ) {
        return Err(InitialIndexSeedRelationRejectV1::NonLocalDeclaration);
    }
    if initializer.initializer_site().is_none() {
        return Err(InitialIndexSeedRelationRejectV1::MissingInitializerSite);
    }
    if initializer.declared_type_name() != Some("i64") {
        return Err(InitialIndexSeedRelationRejectV1::WrongType);
    }
    if typed.initializer_literal() != &ResolvedLiteralSourceV1::Integer(0) {
        return Err(InitialIndexSeedRelationRejectV1::WrongLiteral);
    }
    let index_binding = typed
        .inputs()
        .iter()
        .find(|input| input.role() == S6CTypedInputRoleV1::Index)
        .map(|input| input.binding())
        .ok_or(InitialIndexSeedRelationRejectV1::MissingIndexInput)?;
    if index_binding != initializer.binding() {
        return Err(InitialIndexSeedRelationRejectV1::MissingIndexInput);
    }
    Ok(PreparedLoopV2InitialIndexSeedRelationV1 {
        owner: expected_owner,
        loop_site: typed.membership().source().site().clone(),
        initializer,
        literal: typed.initializer_literal(),
        index_carrier_entry: ingress.index_carrier_entry(),
    })
}
