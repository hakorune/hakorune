//! Source-bound S6C TextEq occurrence projection.
//!
//! This is a mechanical view over an already verified source/Recipe cohort.
//! It carries the Needle binding and logical TextEq/If keys, but no physical
//! `ValueId`, runtime slot, generation, handle, or residence capability.

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1;
use crate::mir::callable_semantic_batch::{S6CBinaryRoleV1, S6CLogicalValueClassV1};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CTextEqOccurrenceSourceRejectV1 {
    ForeignOwner,
    MissingNeedle,
    NeedleShape,
    TextEqShape,
    IfShape,
}

/// Source-only occurrence proof retained by the common-V2 envelope.
///
/// The view is intentionally not `Clone` or `Copy`: consumers borrow it from
/// the envelope and must co-seal it with the canonical session before any
/// physical effect.  Its logical keys are not physical SSA identities.
#[derive(Debug)]
pub(crate) struct S6CTextEqOccurrenceSourceViewV1 {
    owner: FunctionOwnerIdV1,
    needle_binding: BindingRefV1,
    needle_input: LoopValueKeyV1,
    text_eq_item: LoopItemKeyV1,
    text_eq_block: LoopBlockKeyV1,
    text_eq_left: LoopValueKeyV1,
    text_eq_right: LoopValueKeyV1,
    text_eq_result: LoopValueKeyV1,
    if_item: LoopItemKeyV1,
    if_block: LoopBlockKeyV1,
    if_condition: LoopValueKeyV1,
    if_then_block: LoopBlockKeyV1,
}

impl S6CTextEqOccurrenceSourceViewV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn needle_binding(&self) -> BindingRefV1 {
        self.needle_binding
    }

    pub(crate) const fn needle_input(&self) -> LoopValueKeyV1 {
        self.needle_input
    }

    pub(crate) const fn needle_ordinal(&self) -> u32 {
        1
    }

    pub(crate) const fn text_eq_item(&self) -> LoopItemKeyV1 {
        self.text_eq_item
    }

    pub(crate) const fn text_eq_block(&self) -> LoopBlockKeyV1 {
        self.text_eq_block
    }

    pub(crate) const fn text_eq_left(&self) -> LoopValueKeyV1 {
        self.text_eq_left
    }

    pub(crate) const fn text_eq_right(&self) -> LoopValueKeyV1 {
        self.text_eq_right
    }

    pub(crate) const fn text_eq_result(&self) -> LoopValueKeyV1 {
        self.text_eq_result
    }

    pub(crate) const fn if_item(&self) -> LoopItemKeyV1 {
        self.if_item
    }

    pub(crate) const fn if_block(&self) -> LoopBlockKeyV1 {
        self.if_block
    }

    pub(crate) const fn if_condition(&self) -> LoopValueKeyV1 {
        self.if_condition
    }

    pub(crate) const fn if_then_block(&self) -> LoopBlockKeyV1 {
        self.if_then_block
    }
}

pub(crate) fn issue_s6c_text_eq_occurrence_source_v1(
    ingress: S6CPrephysicalIngressRefV2<'_, '_, '_>,
    expected_owner: FunctionOwnerIdV1,
) -> Result<S6CTextEqOccurrenceSourceViewV1, S6CTextEqOccurrenceSourceRejectV1> {
    let owner = ingress.source_owner();
    if owner != expected_owner {
        return Err(S6CTextEqOccurrenceSourceRejectV1::ForeignOwner);
    }

    let typed = ingress.typed_input_relation();
    let needle = typed
        .inputs()
        .iter()
        .find(|input| {
            input.role() == crate::mir::callable_semantic_batch::S6CTypedInputRoleV1::Needle
        })
        .ok_or(S6CTextEqOccurrenceSourceRejectV1::MissingNeedle)?;
    let bindings = ingress.input_bindings();
    if needle.class() != S6CLogicalValueClassV1::Text
        || needle.binding() != bindings[1]
        || needle.binding().owner() != owner
    {
        return Err(S6CTextEqOccurrenceSourceRejectV1::NeedleShape);
    }
    let needle_input = ingress.needle_input();

    let _text_equal_binary = typed
        .binaries()
        .iter()
        .find(|binary| binary.role() == S6CBinaryRoleV1::TextEqual)
        .ok_or(S6CTextEqOccurrenceSourceRejectV1::TextEqShape)?;
    let mut text_eq_rows = ingress.logical_items().iter().filter_map(|item| {
        if let S6CLogicalItemV1::TextEq { .. } = item {
            Some(item)
        } else {
            None
        }
    });
    let text_eq_row = text_eq_rows
        .next()
        .ok_or(S6CTextEqOccurrenceSourceRejectV1::TextEqShape)?;
    if text_eq_rows.next().is_some() {
        return Err(S6CTextEqOccurrenceSourceRejectV1::TextEqShape);
    }
    let mut if_rows = ingress.logical_items().iter().filter_map(|item| {
        if let S6CLogicalItemV1::If { .. } = item {
            Some(item)
        } else {
            None
        }
    });
    let if_row = if_rows
        .next()
        .ok_or(S6CTextEqOccurrenceSourceRejectV1::IfShape)?;
    if if_rows.next().is_some() {
        return Err(S6CTextEqOccurrenceSourceRejectV1::IfShape);
    }
    let S6CLogicalItemV1::TextEq {
        item: text_eq_item,
        block: text_eq_block,
        left: text_eq_left,
        right: text_eq_right,
        result: text_eq_result,
    } = *text_eq_row
    else {
        return Err(S6CTextEqOccurrenceSourceRejectV1::TextEqShape);
    };
    if text_eq_right != needle_input {
        return Err(S6CTextEqOccurrenceSourceRejectV1::TextEqShape);
    }
    let S6CLogicalItemV1::If {
        item: if_item,
        block: if_block,
        condition: if_condition,
        then_block: if_then_block,
        else_block: None,
    } = *if_row
    else {
        return Err(S6CTextEqOccurrenceSourceRejectV1::IfShape);
    };
    if if_condition != text_eq_result {
        return Err(S6CTextEqOccurrenceSourceRejectV1::IfShape);
    }
    Ok(S6CTextEqOccurrenceSourceViewV1 {
        owner,
        needle_binding: needle.binding(),
        needle_input,
        text_eq_item,
        text_eq_block,
        text_eq_left,
        text_eq_right,
        text_eq_result,
        if_item,
        if_block,
        if_condition,
        if_then_block,
    })
}
