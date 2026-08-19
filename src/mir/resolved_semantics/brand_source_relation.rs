//! Exact source relations for explicit Brand construction and unwrap.
//!
//! The shadow traversal issues drafts while it still owns exact AST paths and
//! a loan of the effective Brand declaration catalog. Canonicalization only
//! attaches the already-issued rows to one semantic owner; it never re-pairs
//! names or callable lookup failures.

use std::collections::{BTreeMap, BTreeSet};

use crate::analysis::brand_program_declaration_catalog::{
    BrandDeclarationSourceIdV1, VerifiedBrandProgramDeclarationV1,
};

use super::{FunctionOwnerIdV1, SourceExprSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BrandCallSourceRelationKindV1 {
    Constructor,
    Unwrap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrandCallSourceRelationDraftV1 {
    kind: BrandCallSourceRelationKindV1,
    declaration: BrandDeclarationSourceIdV1,
    name: Box<str>,
    underlying_type: Box<str>,
    call_site: SourceExprSiteV1,
    receiver_site: Option<SourceExprSiteV1>,
    operand_site: SourceExprSiteV1,
}

impl BrandCallSourceRelationDraftV1 {
    pub(crate) fn from_catalog_row(
        kind: BrandCallSourceRelationKindV1,
        declaration: &VerifiedBrandProgramDeclarationV1,
        call_site: SourceExprSiteV1,
        receiver_site: Option<SourceExprSiteV1>,
        operand_site: SourceExprSiteV1,
    ) -> Self {
        Self {
            kind,
            declaration: declaration.source(),
            name: declaration.name().into(),
            underlying_type: declaration.underlying_type().into(),
            call_site,
            receiver_site,
            operand_site,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedBrandCallSourceRelationV1 {
    owner: FunctionOwnerIdV1,
    kind: BrandCallSourceRelationKindV1,
    declaration: BrandDeclarationSourceIdV1,
    name: Box<str>,
    underlying_type: Box<str>,
    call_site: SourceExprSiteV1,
    receiver_site: Option<SourceExprSiteV1>,
    operand_site: SourceExprSiteV1,
}

impl VerifiedBrandCallSourceRelationV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn kind(&self) -> BrandCallSourceRelationKindV1 {
        self.kind
    }

    pub(crate) const fn declaration(&self) -> BrandDeclarationSourceIdV1 {
        self.declaration
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn underlying_type(&self) -> &str {
        &self.underlying_type
    }

    pub(crate) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(crate) const fn receiver_site(&self) -> Option<&SourceExprSiteV1> {
        self.receiver_site.as_ref()
    }

    pub(crate) const fn operand_site(&self) -> &SourceExprSiteV1 {
        &self.operand_site
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BrandCallSourceRelationSealErrorV1 {
    DuplicateCallSite(SourceExprSiteV1),
    MissingCallSite(SourceExprSiteV1),
    MissingOperandSite(SourceExprSiteV1),
    MissingReceiverSite(SourceExprSiteV1),
    ConstructorHasReceiver(SourceExprSiteV1),
    UnwrapMissingReceiver(SourceExprSiteV1),
}

pub(crate) fn seal_brand_call_source_relations_v1(
    owner: FunctionOwnerIdV1,
    drafts: BTreeMap<SourceExprSiteV1, BrandCallSourceRelationDraftV1>,
    expression_sites: &BTreeSet<SourceExprSiteV1>,
) -> Result<
    BTreeMap<SourceExprSiteV1, VerifiedBrandCallSourceRelationV1>,
    BrandCallSourceRelationSealErrorV1,
> {
    let mut rows = BTreeMap::new();
    for (key, draft) in drafts {
        if key != draft.call_site {
            return Err(BrandCallSourceRelationSealErrorV1::MissingCallSite(
                draft.call_site,
            ));
        }
        if !expression_sites.contains(&key) {
            return Err(BrandCallSourceRelationSealErrorV1::MissingCallSite(key));
        }
        if !expression_sites.contains(&draft.operand_site) {
            return Err(BrandCallSourceRelationSealErrorV1::MissingOperandSite(
                draft.operand_site,
            ));
        }
        match (draft.kind, draft.receiver_site.as_ref()) {
            (BrandCallSourceRelationKindV1::Constructor, Some(_)) => {
                return Err(BrandCallSourceRelationSealErrorV1::ConstructorHasReceiver(
                    key,
                ))
            }
            (BrandCallSourceRelationKindV1::Unwrap, None) => {
                return Err(BrandCallSourceRelationSealErrorV1::UnwrapMissingReceiver(
                    key,
                ))
            }
            (_, Some(receiver)) if !expression_sites.contains(receiver) => {
                return Err(BrandCallSourceRelationSealErrorV1::MissingReceiverSite(
                    receiver.clone(),
                ))
            }
            _ => {}
        }
        let row = VerifiedBrandCallSourceRelationV1 {
            owner,
            kind: draft.kind,
            declaration: draft.declaration,
            name: draft.name,
            underlying_type: draft.underlying_type,
            call_site: key.clone(),
            receiver_site: draft.receiver_site,
            operand_site: draft.operand_site,
        };
        if rows.insert(key.clone(), row).is_some() {
            return Err(BrandCallSourceRelationSealErrorV1::DuplicateCallSite(key));
        }
    }
    Ok(rows)
}
