//! Exact two-caller CurrentOwner source cohort, outside the global transport graph.
//! This is pre-Home eligibility only; the actual finisher grants execution.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::{
    incoming_source::StaticIncomingSourceV1, QualifiedStaticCallClaimIndexV1,
};
use crate::mir::resolved_semantics::SourcePathSegmentV1::{Body, Initializer};
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct TargetStaticSourceCohortV1 {
    pub(in crate::mir::normal_callable_semantic_package) owner: FunctionOwnerIdV1,
    pub(in crate::mir::normal_callable_semantic_package) incoming:
        Box<[BorrowedIncomingCallDraftV1]>,
}

impl TargetStaticSourceCohortV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn sites(
        &self,
    ) -> impl Iterator<Item = &OwnedExprSiteV1> {
        self.incoming.iter().map(|row| &row.call)
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn issue_target_static_source_cohorts_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    inventory: &BorrowedIncomingInventoryV1,
    static_arguments: &BTreeMap<
        (OwnedExprSiteV1, u32),
        super::borrowed_static_argument::StaticArgumentSourceV1,
    >,
    checked_static_inputs: &BTreeSet<BindingRefV1>,
    static_claims: Option<&QualifiedStaticCallClaimIndexV1>,
) -> Result<BTreeMap<FunctionOwnerIdV1, TargetStaticSourceCohortV1>, String> {
    let Some(claims) = static_claims else {
        return Ok(BTreeMap::new());
    };
    let owners: BTreeSet<_> = inventory
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok().map(|row| row.callee_owner()))
        .collect();
    let mut cohorts = BTreeMap::new();
    for owner in owners {
        if !definitions.contains_key(&owner) || inventory.has_unsupported_static_spelling(owner) {
            continue;
        }
        let Ok(incoming) = inventory.project(&BTreeSet::from([owner])) else {
            continue;
        };
        let [first, second] = incoming.as_ref() else {
            continue;
        };
        let mut initializer = false;
        let mut eq_lhs = false;
        for call in [first, second] {
            let BorrowedIncomingSourceV1::Static(source) = &call.source else {
                break;
            };
            if !source_row_agrees(
                source,
                call,
                first,
                inventory,
                static_arguments,
                checked_static_inputs,
            ) {
                break;
            }
            match source.call_site().site().node().segments() {
                [Body(_), Initializer(_)] if !initializer => initializer = true,
                _ if !eq_lhs => {
                    let Some(slot) = selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(
                        source.caller().clone(),
                    )) else {
                        break;
                    };
                    eq_lhs = batch
                        .with_lowering_input(slot, |input| {
                            crate::mir::resolved_semantics::home_new_prefix::contains_static_eq_lhs_source_v1(
                                input,
                                source.call_site(),
                                &mut |site| {
                                    crate::mir::normal_callable_semantic_package::qualified_static_call_claim::claim_for_source_site_v1(
                                        claims,
                                        Some(source.caller()),
                                        &input,
                                        site,
                                        selected,
                                        contracts,
                                        None,
                                    )
                                    .map_err(|_| freeze("borrowed-static/eq-source-claim"))
                                },
                            )
                        })
                        .map_err(|_| freeze("borrowed-static/eq-source-loan"))??;
                }
                _ => break,
            }
        }
        if initializer && eq_lhs {
            cohorts.insert(owner, TargetStaticSourceCohortV1 { owner, incoming });
        }
    }
    Ok(cohorts)
}

fn source_row_agrees(
    source: &Rc<StaticIncomingSourceV1>,
    call: &BorrowedIncomingCallDraftV1,
    first: &BorrowedIncomingCallDraftV1,
    inventory: &BorrowedIncomingInventoryV1,
    static_arguments: &BTreeMap<
        (OwnedExprSiteV1, u32),
        super::borrowed_static_argument::StaticArgumentSourceV1,
    >,
    checked_static_inputs: &BTreeSet<BindingRefV1>,
) -> bool {
    let BorrowedIncomingSourceV1::Static(anchor) = &first.source else {
        return false;
    };
    let [formal] = source.parameters() else {
        return false;
    };
    source.is_current_owner_i64_source_v1()
        && source.call_site() == &call.call
        && source.callee_owner() == call.callee
        && source.callee_owner() == anchor.callee_owner()
        && source.target() == anchor.target()
        && source.target_batch_slot() == anchor.target_batch_slot()
        && source.argument_sites().len() == 1
        && call.arguments.len() == 1
        && call.arguments[0] == (0, source.argument_sites()[0].clone(), formal.binding)
        && formal.kind.is_ordinary_borrowed_handle()
        && checked_static_inputs.contains(&formal.binding)
        && inventory
            .static_observations()
            .get(source.call_site())
            .is_some_and(|row| {
                row.as_ref()
                    .is_ok_and(|retained| Rc::ptr_eq(retained, source))
            })
        && static_arguments
            .get(&(source.call_site().clone(), 0))
            .is_some_and(|argument| {
                Rc::ptr_eq(argument.retained_call_source(), source)
                    && checked_static_inputs.contains(&argument.formal())
                    && argument.target_formal() == formal.binding
            })
}
