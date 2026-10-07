//! Original borrowed source drafts, collected before transport profile closure.
use super::*;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn collect_borrowed_source_drafts_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    static_claims: Option<&crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1>,
    app_main_slot: Option<u32>,
    dynamic_slot: Option<u32>,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<
    (
        BTreeSet<FunctionOwnerIdV1>,
        BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        BTreeSet<OwnedExprSiteV1>,
    ),
    String,
> {
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
            || !contract
                .parameters
                .iter()
                .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            continue;
        }
        let receiver = match contract.mode {
            CallableParameterDeclarationModeV1::InstanceBoxMethod =>
                crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                    selected, batch, entry_home_loans.for_batch_slot(contract.batch_slot), contract.batch_slot,
                ),
            CallableParameterDeclarationModeV1::StaticBoxMethod => {
                let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
                    selected.key_for_batch_slot(contract.batch_slot) else { continue; };
                if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                    || !static_claims.is_some_and(|index| index.contains_exact_i64_target(key)) {
                    continue;
                }
                if key.arity() as usize != contract.parameters.len() {
                    return Err(freeze("borrowed-formal/static-source-contract"));
                }
                None
            }
            _ => continue,
        };
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

/// Borrow the original qualified incoming certification, then close ALL callers
/// with the existing graph and inventory projection. This issues no new claim.
pub(super) fn seed_static_transport_owners_v1(
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    inventory: &BorrowedIncomingInventoryV1,
    owners: &mut BTreeSet<FunctionOwnerIdV1>,
) -> Result<(), String> {
    for source in inventory
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok())
    {
        if !definitions.contains_key(&source.callee_owner())
            || inventory.has_unsupported_static_spelling(source.callee_owner())
            || inventory.has_unsupported_static_context(source.callee_owner())
        {
            continue;
        }
        let mut contracts = contracts
            .iter()
            .filter(|row| row.owner == source.callee_owner());
        let contract = contracts
            .next()
            .ok_or_else(|| freeze("borrowed-static/seed-contract-missing"))?;
        if contracts.next().is_some()
            || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
            || contract.batch_slot != source.target_batch_slot()
            || !matches!(selected.key_for_batch_slot(contract.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                    if key == source.target())
            || source.target().namespace()
                != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
            || source.parameters().len() != contract.parameters.len()
            || source
                .parameters()
                .iter()
                .zip(contract.parameters.iter())
                .any(|(original, formal)| {
                    original.ordinal != formal.ordinal
                        || original.binding != formal.binding
                        || original.kind != formal.kind
                })
        {
            return Err(freeze("borrowed-static/seed-source-identity"));
        }
        owners.insert(contract.owner);
    }
    Ok(())
}
