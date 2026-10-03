//! `new <Child>(...)` argument admission for the borrowed-view draft.
//!
//! This classifier is not ABI authority. It consumes the selected-New source
//! inventory and the operation owner's checked-compare dominance; the
//! physical closure still proves the emitted BirthConstructor coordinate and
//! the call transport spells the tagged actual only for this exact ordinal.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1};

use super::{
    use_dominated_by_if, BorrowedFormalUseDraftErrorV1, BorrowedFormalUseDraftKindV1,
};

/// `site` is a `new <Child>(...)` argument only when an admitted checked
/// compare of the same formal owns a region dominating this use. The
/// (new site, ordinal) pair pins the sole admitted argument position; an
/// undominated use or a `new` outside the source inventory stays
/// `UnsupportedUse` — there is no lexical-call join for a construction.
pub(super) fn new_argument_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    formal: BindingRefV1,
    guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let mut containing = input
        .function()
        .expression_source()
        .constructions()
        .filter(|construction| construction.arguments().iter().any(|argument| argument == site));
    let Some(construction) = containing.next() else {
        return Ok(None);
    };
    if containing.next().is_some() {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    let ordinal = construction
        .arguments()
        .iter()
        .position(|argument| argument == site)
        .expect("the containing check above fixes membership");
    let ordinal =
        u32::try_from(ordinal).map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    let dominated = guards
        .get(&formal)
        .into_iter()
        .flatten()
        .any(|guard| use_dominated_by_if(site.node(), guard));
    if !dominated {
        return Ok(None);
    }
    Ok(Some(BorrowedFormalUseDraftKindV1::NewArgument {
        site: OwnedExprSiteV1::new(input.owner(), construction.site().clone()),
        ordinal,
    }))
}
