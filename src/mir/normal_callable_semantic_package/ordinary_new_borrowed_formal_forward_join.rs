//! Join exact statically resolved ordinary borrowed forwarding edges.
use super::*;

/// Join every unresolved argument to the existing exact lexical disposition
/// and the callee's source-owned opaque binding. The complete finite draft
/// set permits cycles without assuming a recursive edge succeeds. This
/// result still does not prove incoming coverage, availability or carrier ABI.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn join_borrowed_forward_uses_v1(
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, BorrowedIncomingCallLoanV1<'_>>,
) -> Result<Box<[BorrowedForwardUseDraftRowV1]>, BorrowedForwardJoinDraftErrorV1> {
    let mut rows = Vec::new();
    for (owner, draft) in drafts {
        for use_row in &draft.uses {
            let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &use_row.kind
            else {
                continue;
            };
            let error_site = use_row.site.clone();
            let exact_call = calls
                .get(call)
                .ok_or_else(|| BorrowedForwardJoinDraftErrorV1::MissingCall(error_site.clone()))?;
            if use_row.site.owner() != *owner
                || call.owner() != *owner
                || use_row.binding.owner() != *owner
                || use_row.formal.owner() != *owner
                || draft.origins.get(&use_row.binding) != Some(&use_row.formal)
                || exact_call.call_site() != call
                || exact_call.target().namespace() != exact_call.namespace()
                || exact_call.argument_sites().get(*ordinal as usize) != Some(use_row.site.site())
            {
                return Err(BorrowedForwardJoinDraftErrorV1::CallIdentity(error_site));
            }
            let mut matching = contracts
                .iter()
                .filter(|contract| contract.owner == exact_call.callee_owner());
            let contract = matching.next().ok_or_else(|| {
                BorrowedForwardJoinDraftErrorV1::MissingCallee(error_site.clone())
            })?;
            let callee_draft = drafts.get(&contract.owner).ok_or_else(|| {
                BorrowedForwardJoinDraftErrorV1::MissingCallee(error_site.clone())
            })?;
            if matching.next().is_some()
                || contract.batch_slot != exact_call.target_batch_slot()
                || contract.mode != exact_call.declaration_mode()
                || contract.parameters.len() != exact_call.argument_sites().len()
                || contract.parameters.len() != exact_call.target().arity() as usize
            {
                return Err(BorrowedForwardJoinDraftErrorV1::FormalIdentity(error_site));
            }
            let formal = contract.parameters.get(*ordinal as usize).ok_or_else(|| {
                BorrowedForwardJoinDraftErrorV1::FormalIdentity(error_site.clone())
            })?;
            if formal.ordinal != *ordinal
                || formal.binding.owner() != contract.owner
                || !formal.kind.is_ordinary_borrowed_handle()
                || callee_draft.origins.get(&formal.binding) != Some(&formal.binding)
            {
                return Err(BorrowedForwardJoinDraftErrorV1::FormalIdentity(error_site));
            }
            rows.push(BorrowedForwardUseDraftRowV1 {
                site: use_row.site.clone(),
                binding: use_row.binding,
                source_formal: use_row.formal,
                call: call.clone(),
                target: exact_call.target().clone(),
                ordinal: *ordinal,
                callee_formal: formal.binding,
            });
        }
    }
    Ok(rows.into_boxed_slice())
}
