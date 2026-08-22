//! Parent-retaining, source-bound TextEq site contract.
//!
//! This product does not mint Recipe keys or revalidate the source.  The
//! prephysical ingress has already co-sealed the source binary, Recipe row,
//! TextEq If, and exact typed `StringBox` inputs.  This layer only retains
//! that cohort and lends a narrow per-site view together with the passive
//! language-law projection.

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::s6c_prephysical_ingress::VerifiedS6CPrephysicalIngressV2;
use super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1;
use crate::mir::callable_semantic_batch::S6CBinaryRelationV1;

/// Reusable language rule.  It has no source site, Recipe key, route, or
/// physical ownership and is therefore safe to copy as a passive projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextEqualityLawV1 {
    ExactUnicodeScalarSequence,
}

/// Narrow source/Recipe/control view for one already-sealed TextEq site.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LoopTextEqSiteRefV1<'a> {
    law: TextEqualityLawV1,
    source: &'a S6CBinaryRelationV1,
    item: LoopItemKeyV1,
    block: LoopBlockKeyV1,
    left: LoopValueKeyV1,
    right: LoopValueKeyV1,
    result: LoopValueKeyV1,
    if_item: LoopItemKeyV1,
    if_block: LoopBlockKeyV1,
    if_condition: LoopValueKeyV1,
    if_then_block: LoopBlockKeyV1,
}

impl<'a> LoopTextEqSiteRefV1<'a> {
    pub(crate) const fn law(self) -> TextEqualityLawV1 {
        self.law
    }

    pub(crate) const fn source(self) -> &'a S6CBinaryRelationV1 {
        self.source
    }

    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn left(self) -> LoopValueKeyV1 {
        self.left
    }

    pub(crate) const fn right(self) -> LoopValueKeyV1 {
        self.right
    }

    pub(crate) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }

    pub(crate) const fn if_item(self) -> LoopItemKeyV1 {
        self.if_item
    }

    pub(crate) const fn if_block(self) -> LoopBlockKeyV1 {
        self.if_block
    }

    pub(crate) const fn if_condition(self) -> LoopValueKeyV1 {
        self.if_condition
    }

    pub(crate) const fn if_then_block(self) -> LoopBlockKeyV1 {
        self.if_then_block
    }
}

/// One non-Clone site binding.  The original ingress remains the sole cohort
/// owner; no detached keys or source ledger are stored here.
#[derive(Debug)]
pub(crate) struct VerifiedS6CTextEqSourceBindingV1 {
    ingress: VerifiedS6CPrephysicalIngressV2,
    law: TextEqualityLawV1,
}

impl VerifiedS6CTextEqSourceBindingV1 {
    pub(crate) fn with_site<R>(
        &self,
        callback: impl for<'site> FnOnce(LoopTextEqSiteRefV1<'site>) -> R,
    ) -> R {
        self.ingress.with_text_eq_leaf(|leaf| {
            let S6CLogicalItemV1::TextEq {
                item,
                block,
                left,
                right,
                result,
            } = leaf.row()
            else {
                unreachable!("sealed TextEq site row")
            };
            let S6CLogicalItemV1::If {
                item: if_item,
                block: if_block,
                condition: if_condition,
                then_block: if_then_block,
                else_block: None,
            } = leaf.if_row()
            else {
                unreachable!("sealed TextEq If row")
            };
            callback(LoopTextEqSiteRefV1 {
                law: self.law,
                source: leaf.binary(),
                item,
                block,
                left,
                right,
                result,
                if_item,
                if_block,
                if_condition,
                if_then_block,
            })
        })
    }
}

/// Consume the complete ingress cohort once.  All semantic rejection belongs
/// to the upstream ingress issuer; this function does not create a second
/// source/Recipe authority or a new failure policy.
pub(crate) fn issue_s6c_text_eq_source_binding_v1(
    ingress: VerifiedS6CPrephysicalIngressV2,
) -> VerifiedS6CTextEqSourceBindingV1 {
    VerifiedS6CTextEqSourceBindingV1 {
        ingress,
        law: TextEqualityLawV1::ExactUnicodeScalarSequence,
    }
}
