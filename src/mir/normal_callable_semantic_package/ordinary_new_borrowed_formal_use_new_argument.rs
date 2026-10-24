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


/// Source-only nonowning child use. The existing constructor relation issuer
/// already joined the returned New, Birth parameter and canonical field store.
/// This does not activate a store, borrowed ABI, or an owning move.
pub(super) fn construction_child_argument_kind_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &super::OwnedCallableParameterContractDeclarationV1,
    context: Option<&super::BorrowedFormalSourceUseContextV1<'_>>,
    formal: BindingRefV1,
    binding: BindingRefV1,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    use super::result_class_claim::{ResultValueOriginV1, ResultWitnessStepV1};
    use std::rc::Rc;
    let Some(context) = context else { return Ok(None); };
    let relations: Vec<_> = context.results.construction_child_relation_loans()
        .filter(|relation| !relation.is_unqualified_dynamic_v1()
            && relation.actual().owner() == input.owner() && relation.actual().site() == site)
        .collect();
    if relations.is_empty() { return Ok(None); }
    let mut parameters = contract.parameters.iter().filter(|row| row.binding == formal);
    let parameter = parameters.next().ok_or(BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    if parameters.next().is_some() || contract.owner != input.owner() || binding != formal
        || !matches!(parameter.kind, super::CallableParameterContractKindV1::DeclaredObject(_))
    { return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity); }
    for relation in &relations {
        let mut constructions = input.function().expression_source().constructions()
            .filter(|row| row.site() == relation.outer().site());
        let construction = constructions.next().ok_or(BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
        if constructions.next().is_some() || relation.outer().owner() != input.owner()
            || construction.arguments().get(relation.formal_ordinal() as usize) != Some(site)
            || relation.witnesses().len() != 2
            || relation.witnesses().iter().any(|witness| witness.site() != relation.actual()
                || !matches!(witness.step(), ResultWitnessStepV1::Formal { binding: source, ordinal }
                    if *source == formal && *ordinal == parameter.ordinal))
            || relation.witnesses().iter().filter(|witness|
                matches!(witness.origin(), ResultValueOriginV1::Null)).count() != 1
            || relation.witnesses().iter().filter(|witness|
                matches!(witness.origin(), ResultValueOriginV1::ForwardFormal { ordinal }
                    if *ordinal == parameter.ordinal)).count() != 1
        { return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity); }
    }
    Ok(Some(BorrowedFormalUseDraftKindV1::ConstructionChildArgument {
        relations: relations.into_iter().map(Rc::clone).collect(),
    }))
}
