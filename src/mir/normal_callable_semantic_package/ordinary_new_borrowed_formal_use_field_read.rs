//! Guarded `formal.field` receiver use inside the ordinary lexical-call
//! owner.
//!
//! A `FieldAccess` whose receiver is the exact formal binding is admissible
//! only when an admitted null-compare of the same formal dominates the
//! read in the same enclosing sequence — the non-null successor is the
//! only path where the borrowed object is live. The draft pins the exact
//! FieldAccess site; class authority, field declaration and emission all
//! stay with the package issuer's sealed object view, never this draft.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
};

use super::{use_dominated_by_if, BorrowedFormalUseDraftErrorV1, BorrowedFormalUseDraftKindV1};

/// `Some(FieldReadOperand)` when `site` is the receiver of exactly one
/// `FieldAccess` shape row dominated by an admitted `formal == null` if —
/// the arm that terminates proves the surviving sequence carries a live
/// borrowed object. Unguarded reads, reads inside the guard's own arms,
/// and ambiguous receivers (the same ref claimed by two FieldAccess rows)
/// stay unsupported.
pub(super) fn field_read_operand_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    formal: BindingRefV1,
    null_guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let mut matching = shape.expressions().iter().filter_map(|row| match row {
        BodyExpressionShapeV1::FieldAccess {
            object, site: access, ..
        } if object == site => Some(access),
        _ => None,
    });
    let Some(access) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some() {
        return Ok(None);
    }
    let dominated = null_guards
        .get(&formal)
        .into_iter()
        .flatten()
        .any(|guard| use_dominated_by_if(access.node(), guard));
    if !dominated {
        return Ok(None);
    }
    Ok(Some(BorrowedFormalUseDraftKindV1::FieldReadOperand {
        site: OwnedExprSiteV1::new(input.owner(), access.clone()),
    }))
}
