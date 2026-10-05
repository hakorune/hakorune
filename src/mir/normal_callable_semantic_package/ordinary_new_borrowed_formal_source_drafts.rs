//! Original borrowed source drafts, collected before transport profile closure.
use super::*;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn collect_borrowed_source_drafts_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    app_main_slot: Option<u32>,
    dynamic_slot: Option<u32>,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<(
    BTreeSet<FunctionOwnerIdV1>,
    BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    BTreeSet<OwnedExprSiteV1>,
), String> {
    let ordinary_callers: BTreeSet<_> = batch
        .declarations()
        .filter(|row| {
            Some(row.batch_slot()) != dynamic_slot
                && (selected.role_for_batch_slot(row.batch_slot()).is_some()
                    || Some(row.batch_slot()) == app_main_slot)
        })
        .map(|row| row.owner())
        .collect();
    let mut definitions = BTreeMap::new();
    let mut dominated_view_sites = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for contract in contracts {
        if !seen.insert(contract.owner) {
            return Err(freeze("borrowed-formal/duplicate-source-contract"));
        }
        if !ordinary_callers.contains(&contract.owner)
            || contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || !contract
                .parameters
                .iter()
                .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            continue;
        }
        let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
            selected,
            batch,
            entry_home_loans.for_batch_slot(contract.batch_slot),
            contract.batch_slot,
        );
        let draft = batch
            .with_lowering_input(contract.batch_slot, |input| {
                draft_borrowed_formal_uses_v1(input, contract, instance_constructors, receiver)
            })
            .map_err(|_| freeze("borrowed-formal/batch-loan"))?;
        match draft {
            Ok(draft) => {
                for row in draft.uses.iter() {
                    if matches!(
                        row.kind,
                        BorrowedFormalUseDraftKindV1::ArrayElementValue { .. }
                            | BorrowedFormalUseDraftKindV1::AddOperand { .. }
                            | BorrowedFormalUseDraftKindV1::NewArgument { .. }
                    ) {
                        dominated_view_sites.insert(row.site.clone());
                    }
                }
                definitions.insert(contract.owner, draft);
            }
            Err(
                BorrowedFormalUseDraftErrorV1::UnsupportedUse(_)
                | BorrowedFormalUseDraftErrorV1::Rebound(_)
                | BorrowedFormalUseDraftErrorV1::Captured(_)
                | BorrowedFormalUseDraftErrorV1::AnnotatedAlias(_),
            ) => {}
            Err(error) => {
                return Err(format!(
                    "{}: {error:?}",
                    freeze("borrowed-formal/source-identity")
                ));
            }
        }
    }
    Ok((ordinary_callers, definitions, dominated_view_sites))
}
