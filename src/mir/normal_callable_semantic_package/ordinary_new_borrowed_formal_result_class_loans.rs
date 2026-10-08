//! Conditional declaration loans schedule results; only strict ingress issues views.
use super::super::super::borrowed_formal_source::{classify_actual_seed, FormalActualSeedV1};
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;

fn collect_opaque_forward_identities_v1(
    need: &PreparedSourceCallNeedV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
) -> Option<Vec<ForwardIdentityV1>> {
    let reference = need.reference();
    let draft = drafts.get(&reference.call_site.owner())?;
    let mut contracts = contracts
        .iter()
        .filter(|row| row.owner == reference.callee_owner);
    let contract = contracts.next()?;
    let callee = drafts.get(&contract.owner)?;
    if contracts.next().is_some()
        || contract.batch_slot != reference.target_batch_slot
        || contract.parameters.len() != reference.argument_sites.len()
        || reference.argument_sites.len() != reference.target.arity() as usize
    {
        return None;
    }
    let mut forwards = Vec::new();
    for formal in contract
        .parameters
        .iter()
        .filter(|row| row.kind.is_ordinary_borrowed_handle())
    {
        let site = reference.argument_sites.get(formal.ordinal as usize)?;
        let mut rows = draft.uses.iter().filter(|row| row.site.site() == site
            && matches!(&row.kind, BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal }
                if *call == reference.call_site && *ordinal == formal.ordinal));
        let Some(row) = rows.next() else {
            continue;
        };
        if rows.next().is_some()
            || row.site.owner() != reference.call_site.owner()
            || row.binding.owner() != row.site.owner()
            || row.formal.owner() != row.site.owner()
            || draft.origins.get(&row.binding) != Some(&row.formal)
            || formal.binding.owner() != reference.callee_owner
            || callee.origins.get(&formal.binding) != Some(&formal.binding)
        {
            return None;
        }
        forwards.push(ForwardIdentityV1 {
            site: row.site.clone(),
            binding: row.binding,
            source_formal: row.formal,
            call: reference.call_site.clone(),
            target: reference.target.clone(),
            ordinal: formal.ordinal,
            callee_formal: formal.binding,
        });
    }
    Some(forwards)
}

/// Source-only subset validates all original argument coordinates while only
/// opaque ordinals lend identities. Legacy consumers keep their former law.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn collect_observed_forward_identities_v1(
    need: &PreparedSourceCallNeedV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
) -> Option<Vec<ForwardIdentityV1>> {
    let forwards = collect_opaque_forward_identities_v1(need, contracts, drafts)?;
    let reference = need.reference();
    let draft = drafts.get(&reference.call_site.owner())?;
    let contract = contracts
        .iter()
        .find(|row| row.owner == reference.callee_owner)?;
    let mut seen = std::collections::BTreeSet::new();
    let mut opaque = 0;
    for row in &draft.uses {
        let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &row.kind else {
            continue;
        };
        if *call != reference.call_site {
            continue;
        }
        let formal = contract.parameters.get(*ordinal as usize)?;
        if formal.ordinal != *ordinal
            || formal.binding.owner() != reference.callee_owner
            || row.site.owner() != reference.call_site.owner()
            || reference.argument_sites.get(*ordinal as usize) != Some(row.site.site())
            || !seen.insert(*ordinal)
        {
            return None;
        }
        if formal.kind.is_ordinary_borrowed_handle() {
            opaque += 1;
        }
    }
    (opaque == forwards.len()).then_some(forwards)
}

pub(super) fn forward_identities_v1(
    need: &PreparedSourceCallNeedV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
) -> Option<Vec<ForwardIdentityV1>> {
    let forwards = collect_opaque_forward_identities_v1(need, contracts, drafts)?;
    let reference = need.reference();
    let contract = contracts
        .iter()
        .find(|row| row.owner == reference.callee_owner)?;
    (forwards.len()
        == contract
            .parameters
            .iter()
            .filter(|formal| formal.kind.is_ordinary_borrowed_handle())
            .count())
    .then_some(forwards)
}

pub(super) fn conditional_class_loans_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    needs: &[Result<Option<PreparedSourceCallNeedV1>, String>],
    callable_result_classes: &super::super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> BTreeMap<BindingRefV1, ConditionalClassLoanV1> {
    // These are conditional declaration requirements, never all-incoming views.
    let mut proposals: BTreeMap<_, BTreeMap<Box<str>, ClassLenderV1>> = contracts
        .iter()
        .filter(|row| {
            drafts.contains_key(&row.owner)
                && row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
        })
        .flat_map(|row| row.parameters.iter())
        .filter_map(|row| match &row.kind {
            CallableParameterContractKindV1::DeclaredObject(class) => Some((
                row.binding,
                BTreeMap::from([(class.clone(), ClassLenderV1::Declared(row.binding))]),
            )),
            _ => None,
        })
        .collect();
    loop {
        let mut progressed = false;
        for need in needs.iter().filter_map(|row| row.as_ref().ok()?.as_ref()) {
            if !contracts.iter().any(|row| {
                row.owner == need.reference().call_site.owner()
                    && row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
            }) {
                continue;
            }
            let Some(forwards) = forward_identities_v1(need, contracts, drafts) else {
                continue;
            };
            for forward in forwards {
                let Some(slot) = batch
                    .declarations()
                    .find(|row| row.owner() == forward.call.owner())
                    .map(|row| row.batch_slot())
                else {
                    continue;
                };
                let seed = batch.with_lowering_input(slot, |input| {
                    classify_actual_seed(
                        input,
                        batch,
                        selected,
                        callable_result_classes,
                        input.owner(),
                        &[],
                        None,
                        drafts,
                        contracts,
                        forward.site.site(),
                    )
                });
                let classes: Vec<_> = match seed {
                    Ok(FormalActualSeedV1::Class(class)) => vec![class],
                    Ok(FormalActualSeedV1::Forward(formal)) => proposals
                        .get(&formal)
                        .map(|rows| rows.keys().cloned().collect())
                        .unwrap_or_default(),
                    _ => Vec::new(),
                };
                for class in classes {
                    let rows = proposals.entry(forward.callee_formal).or_default();
                    if let std::collections::btree_map::Entry::Vacant(entry) = rows.entry(class) {
                        entry.insert(ClassLenderV1::Forward(forward.clone()));
                        progressed = true;
                    }
                }
            }
        }
        if !progressed {
            break;
        }
    }
    // A competing class remains a conditional conflict. It never becomes an
    // executable view or an eager error for a caller outside the selected profile.
    proposals
        .into_iter()
        .filter_map(|(formal, rows)| {
            if rows.len() != 1 {
                return None;
            }
            let (class, lender) = rows.into_iter().next()?;
            Some((formal, ConditionalClassLoanV1 { class, lender }))
        })
        .collect()
}

#[cfg(test)]
#[path = "ordinary_new_forward_identity_loan_tests.rs"]
mod forward_identity_loan_tests;
