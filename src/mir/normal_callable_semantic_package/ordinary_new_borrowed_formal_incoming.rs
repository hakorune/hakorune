//! Sole selected-caller incoming scan retains Static/Object transport evidence.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;

pub(in crate::mir::normal_callable_semantic_package) fn exact_bool_value_return_site_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> bool {
    input.body_shape().is_some_and(|shape| {
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::verified_value_return_sites(input, shape)
            .is_some_and(|values| values.as_slice() == std::slice::from_ref(site))
            && shape.statements().iter().any(|statement| match statement {
                crate::mir::resolved_semantics::BodyStatementShapeV1::Return {
                    site: exit, value: Some(value),
                } if value == site => shape.relations().iter().filter(|relation|
                    relation.parent() == exit.node()
                        && relation.role() == &crate::mir::resolved_semantics::SourcePathSegmentV1::Value
                        && relation.child() == site).count() == 1,
                _ => false,
            })
    })
}

#[path = "ordinary_new_borrowed_static_inventory.rs"]
mod static_inventory;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use static_inventory::StaticIncomingContextV1;

#[path = "ordinary_new_borrowed_formal_incoming_inventory.rs"]
mod inventory;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use inventory::{BorrowedIncomingInventoryV1, SourceIncomingRefusalsV1};

/// Visit the complete batch once, but omit only catalog-co-sealed closed-App
/// Static callers from executable incoming obligations. Their original call
/// rows remain in the separate complete Static claim index. Other unselected
/// callers and unresolved selector/arity matches still veto as before.
#[cfg(test)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn inventory_borrowed_incoming_calls_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::VerifiedSelectedCallableBatchMapV1,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::super::LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    static_context: Option<&StaticIncomingContextV1<'_>>,
) -> Result<BorrowedIncomingInventoryV1, BorrowedIncomingDraftErrorV1> {
    inventory_borrowed_incoming_with_stored_dispatch_v1(
        batch,
        selected,
        &std::collections::BTreeMap::new(),
        drafts,
        contracts,
        calls,
        ordinary_callers,
        static_context,
        None,
        None,
        None,
    )
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn inventory_borrowed_incoming_with_stored_dispatch_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &BTreeMap<FunctionOwnerIdV1, crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::super::LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    static_context: Option<&StaticIncomingContextV1<'_>>,
    stored_dispatch: Option<&super::super::source::PreparedSourceNeedsV1>,
    constructors: Option<&crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1>,
    array_sources: Option<&super::super::borrowed_formal_source::OriginalArraySourcesV1>,
) -> Result<BorrowedIncomingInventoryV1, BorrowedIncomingDraftErrorV1> {
    let mut stored = BTreeMap::new();
    if let Some(prepared) = stored_dispatch {
        for need in prepared
            .as_ref()
            .map_err(|_| BorrowedIncomingDraftErrorV1::SourceIdentity)?
        {
            let need = need
                .as_ref()
                .map_err(|_| BorrowedIncomingDraftErrorV1::SourceIdentity)?;
            if let Some(super::super::source::PreparedSourceCallNeedV1::Stored {
                reference,
                receiver,
                result_requirement,
                owning_object_input,
            }) = need
            {
                if stored
                    .insert(&reference.call_site, (reference, receiver, result_requirement, owning_object_input))
                    .is_some()
                {
                    return Err(BorrowedIncomingDraftErrorV1::CallIdentity(
                        reference.call_site.clone(),
                    ));
                }
            }
        }
    }
    let mut stored_seen = std::collections::BTreeSet::new();
    let mut definitions = BTreeMap::new();
    for owner in drafts.keys() {
        let mut matching = contracts.iter().filter(|row| row.owner == *owner);
        let contract = matching
            .next()
            .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        };
        if matching.next().is_some()
            || !matches!(
                (key.namespace(), contract.mode),
                (
                    hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod,
                    CallableParameterDeclarationModeV1::InstanceBoxMethod
                ) | (
                    hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod,
                    CallableParameterDeclarationModeV1::StaticBoxMethod
                )
            )
            || key.arity() as usize != contract.parameters.len()
            || !contract
                .parameters
                .iter()
                .enumerate()
                .all(|(ordinal, formal)| {
                    formal.ordinal as usize == ordinal
                        && formal.binding.owner() == *owner
                        && (!formal.kind.is_ordinary_borrowed_handle()
                            || drafts[owner].origins.get(&formal.binding) == Some(&formal.binding))
                })
            || !contract
                .parameters
                .iter()
                .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        definitions.insert(*owner, (contract, key));
    }
    // Retained result obligations need the same raw caller census. A nullable
    // borrowed LOCAL loan registers original declaration/call identity even when
    // the callee use draft is unavailable; it grants no body/entry/input execution.
    // Index only the already-retained needs; the source scan still occurs once.
    // Stored result requirements register the same callee even without a use
    // draft, before an executable receiver can exist.
    let retained = calls.iter().map(|(site, target)|
        (site, super::super::source::CallTargetReferenceV1::from_target(target), &target.result_requirement, target.owning_object_local_input_v1())
    ).chain(stored.iter().filter(|(site, _)| !calls.contains_key(*site)).map(|(site, (reference, _, requirement, input))|
        (*site, (*reference).clone(), *requirement, input.as_deref())
    ));
    for (site, target, requirement, local_input) in retained {
        let nullable = requirement.borrowed_nullable_local_source();
        let mixed = requirement.mixed_local_source();
        let array_candidate = array_sources.is_some_and(|source| source.contains_candidate_v1(site));
        // A real zero-input scalar call has no borrowed-formal use draft.
        // Register its original declaration in THIS completed source census,
        // without adding a transport definition or fabricating a use draft.
        let empty_scalar_source = matches!(requirement, crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalCallSourceResultRequirementV1::ExistingBorrowedResult
            | crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalCallSourceResultRequirementV1::ScalarReturnSource(_))
            && target.target.arity() == 0 && target.argument_sites.is_empty();
        let literal_scalar_source = match requirement {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalCallSourceResultRequirementV1::ScalarReturnSource(loan)
                if loan.is_literal_scalar_v1() => {
                    if !loan.matches(&target) { return Err(BorrowedIncomingDraftErrorV1::SourceIdentity); }
                    true
                }
            _ => false,
        };
        let unit_source = match requirement {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalCallSourceResultRequirementV1::UnitDiscardSource(loan) => {
                if !loan.is_unit_candidate_v1() || !loan.matches(&target) {
                    return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
                }
                true
            }
            _ => false,
        };
        if !requirement.has_object_source_requirement() && nullable.is_none() && mixed.is_none() && !empty_scalar_source && !literal_scalar_source && !unit_source
            && !array_candidate && local_input.is_none() {
            continue;
        }
        let qualifications = requirement.object_return_sources();
        let mut matching = contracts
            .iter()
            .filter(|row| row.owner == target.callee_owner);
        let contract = matching
            .next()
            .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        };
        if matching.next().is_some()
            || local_input.is_some_and(|loan| loan.call() != site || loan.key() != &target.target
                || loan.callee_owner() != contract.owner || loan.receiver() != Some(&target.receiver_site)
                || loan.destination().owner() != site.owner() || !loan.matches_arguments_v1(&target.argument_sites))
            || mixed.is_some_and(|loan| loan.call() != site || loan.key() != &target.target
                || loan.receiver() != &target.receiver_site || loan.callee_owner() != contract.owner
                || loan.destination().owner() != site.owner() || loan.support().owner() != site.owner())
            || nullable.is_some_and(|loan| {
                loan.call() != site
                    || loan.key() != &target.target
                    || loan.receiver() != &target.receiver_site
                    || loan.callee_owner() != contract.owner
                    || loan.destination().owner() != site.owner()
                    || loan.support_root().owner() != site.owner()
            })
            || qualifications.is_some_and(|rows| {
                rows.is_empty()
                    || rows
                        .iter()
                        .any(|loan| loan.call() != site || loan.key() != &target.target)
            })
            || requirement
                .object_producer_dependencies()
                .is_some_and(|rows| rows.is_empty())
            || &target.call_site != site
            || &target.target != key
            || target.target_batch_slot != contract.batch_slot
            || key.namespace()
                != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || key.arity() as usize != contract.parameters.len()
            || target.argument_sites.len() != contract.parameters.len()
            || contract
                .parameters
                .iter()
                .enumerate()
                .any(|(ordinal, formal)| {
                    formal.ordinal as usize != ordinal || formal.binding.owner() != contract.owner
                })
        {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        if nullable.is_none() && !array_candidate && local_input.is_none() && !unit_source && !literal_scalar_source
            && contract
                .parameters
                .iter()
                .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            continue;
        }
        if !contract.parameters.iter().all(|formal| (nullable.is_some() || array_candidate || local_input.is_some() || unit_source || literal_scalar_source) && formal.kind.is_ordinary_borrowed_handle()
            || matches!(formal.kind,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(abi)
                if abi == crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64)) {
            continue;
        }
        definitions.insert(contract.owner, (contract, key));
    }
    // Zero-input Static owners have no borrowed draft. Register only an
    // original ExactI64 target in this SAME raw census, not transport owners
    // or Prepared definitions. Every matching caller/error stays below.
    if let Some(context) = static_context {
        for contract in contracts.iter().filter(|contract| {
            contract.mode == CallableParameterDeclarationModeV1::StaticBoxMethod
                && contract.parameters.is_empty()
        }) {
            let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
                selected.key_for_batch_slot(contract.batch_slot)
            else {
                continue;
            };
            if !context.claims.contains_zeroarg_i64_input_target(key) {
                continue;
            }
            if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                || key.arity() != 0
                || contracts
                    .iter()
                    .filter(|row| row.owner == contract.owner)
                    .count()
                    != 1
                || definitions
                    .insert(contract.owner, (contract, key))
                    .is_some()
            {
                return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
            }
        }
    }
    let legacy_owners: std::collections::BTreeSet<_> = definitions.keys().copied().collect();
    // Source identity never depends on a usable transport/use draft. Keep the
    // legacy set frozen above, and feed both namespaces to the SAME scan.
    for contract in contracts {
        if matches!(selected.key_for_batch_slot(contract.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod)
            && contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
        { return Err(BorrowedIncomingDraftErrorV1::SourceIdentity); }
        let namespace = match contract.mode {
            CallableParameterDeclarationModeV1::InstanceBoxMethod => hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod,
            CallableParameterDeclarationModeV1::StaticBoxMethod if static_context.is_some() => hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod,
            _ => continue,
        };
        let key = match selected.key_for_batch_slot(contract.batch_slot) {
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) => Some(key),
            _ if namespace == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod => omitted_static_callers.get(&contract.owner),
            _ => None,
        };
        let Some(key) = key else { continue; };
        let declaration = batch.declarations().find(|row| row.batch_slot() == contract.batch_slot)
            .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
        let same_mode = matches!((contract.mode, declaration.mode()),
            (CallableParameterDeclarationModeV1::InstanceBoxMethod, crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::InstanceBoxMethod)
            | (CallableParameterDeclarationModeV1::StaticBoxMethod, crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::StaticBoxMethod));
        let original_parameters = batch.with_lowering_input(contract.batch_slot, |input|
            input.owner() == contract.owner && contract.parameters.iter().all(|formal|
                input.function().declaration_binding(&crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter { index: formal.ordinal }) == Some(formal.binding)))
            .map_err(|_| BorrowedIncomingDraftErrorV1::BatchLoan)?;
        if key.namespace() != namespace || declaration.owner() != contract.owner || !same_mode
            || key.arity() as usize != contract.parameters.len() || !original_parameters
            || contracts.iter().filter(|row| row.owner == contract.owner).count() != 1
            || !contract.parameters.iter().enumerate().all(|(ordinal, formal)|
                formal.ordinal as usize == ordinal && formal.binding.owner() == contract.owner)
        { return Err(BorrowedIncomingDraftErrorV1::SourceIdentity); }
        definitions.entry(contract.owner).or_insert((contract, key));
    }
    let mut legacy_seen = std::collections::BTreeSet::new();
    let mut static_observations = BTreeMap::new();
    let mut static_seen = std::collections::BTreeSet::new();
    let mut observations = Vec::new();
    let mut unsupported_static_spelling = std::collections::BTreeSet::new();
    let mut unsupported_static_context = std::collections::BTreeSet::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut unvisited_omitted: std::collections::BTreeSet<_> = omitted_static_callers.keys().copied().collect();
    use crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1;
    let mut collect = |caller: Option<OriginalCallSourceCallerV1<'_>>, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>| -> Result<(), BorrowedIncomingDraftErrorV1> {
            for (site, call) in input.function().method_calls() {
                let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                if let Some(caller) = caller {
                    caller.constructor_key_for_call(&owned, call)
                        .map_err(|_| BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()))?;
                }
                let method = input.function().method_call_source_v1(site)
                    .filter(|row| std::ptr::eq(row.as_ref(), call)
                        && row.owner() == input.owner() && row.site() == site)
                    .ok_or_else(|| BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()))?;
                let first_observation = observations.len();
                let source_only = caller.is_none() || omitted_static_callers.contains_key(&input.owner());
                static_seen.insert(owned.clone());
                let mut source_static_observation = None;
                if let (Some(context), Some(caller)) = (static_context, caller) {
                    if let Some(row) = context.observe(
                        caller,
                        &owned,
                        call,
                        input,
                        omitted_static_callers,
                        selected,
                        contracts,
                        drafts.get(&input.owner())
                            .map(|draft| draft.static_operand_call_source_at(input, &owned))
                            .transpose()
                            .map_err(|_| BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()))?
                            .flatten(),
                    )? {
                        source_static_observation = Some(row.clone());
                        // Preserve the old selected-target requirement on the
                        // existing legacy port without repeating source issuance.
                        let legacy_row = row.and_then(|source| {
                            if selected.batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(source.target().clone())) == Some(source.target_batch_slot()) {
                                Ok(source)
                            } else { Err(super::super::freeze("borrowed-static/incoming-source-identity")) }
                        });
                        if !source_only
                            && static_observations.insert(owned.clone(), legacy_row).is_some() {
                            return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned));
                        }
                    }
                }
                let instance = caller.and_then(|_| calls.get(&owned));
                let stored_target = if let (Some(caller), Some((reference, receiver, _, _))) = (caller, stored.get(&owned)) {
                    corroborate_stored_dispatch_v1(caller, input, call, reference, receiver, selected, contracts)?;
                    if instance.is_some_and(|row| super::super::source::CallTargetReferenceV1::from_target(row) != **reference) {
                        return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()));
                    }
                    stored_seen.insert(owned.clone());
                    Some(&reference.target)
                } else { None };
                for (callee, (contract, key)) in &definitions {
                    let legacy = legacy_owners.contains(callee) && !source_only;
                    let scope = if legacy { inventory::IncomingObservationScopeV1::Both } else { inventory::IncomingObservationScopeV1::SourceOnly };
                    let static_observation = if legacy { static_observations.get(&owned) } else { source_static_observation.as_ref() };
                    if call.selector() != key.name() || call.arity() != key.arity() {
                        continue;
                    }
                    let exact = if caller.is_none() { None } else { match contract.mode {
                        CallableParameterDeclarationModeV1::InstanceBoxMethod => {
                            // A qualified original Static loan belongs to a different
                            // namespace. Missing/failed authority remains a veto.
                            if let Some(Ok(original)) = static_observation {
                                let mut target_contracts = contracts.iter().filter(|row|
                                    row.owner == original.callee_owner());
                                let target_contract = target_contracts.next()
                                    .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
                                if instance.is_some()
                                    || original.call_site() != &owned
                                    || original.target().namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                                    || original.target().name() != call.selector()
                                    || original.target().arity() != call.arity()
                                    || original.argument_sites().len() != call.arguments().len()
                                    || target_contracts.next().is_some()
                                    || target_contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                                    || target_contract.batch_slot != original.target_batch_slot()
                                    || legacy && !matches!(selected.key_for_batch_slot(target_contract.batch_slot),
                                        Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(target)) if target == original.target())
                                    || original.parameters().len() != target_contract.parameters.len()
                                    || original.parameters().iter().zip(&target_contract.parameters).any(|(source, formal)|
                                        source.ordinal != formal.ordinal || source.binding != formal.binding || source.kind != formal.kind)
                                    || !call.arguments().iter().enumerate().all(|(ordinal, argument)|
                                        argument.ordinal() as usize == ordinal
                                            && original.argument_sites()[ordinal] == *argument.site())
                                {
                                    return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()));
                                }
                                continue;
                            }
                            // Passive dispatch excludes only a proved different callee.
                            // The same callee retains a Pending original source row;
                            // selected projection still requires its real issued receiver.
                            if stored_target.is_some_and(|target| target != *key) {
                                continue;
                            }
                            if let Some((reference, receiver, result, input)) = stored.get(&owned) {
                                Some(BorrowedIncomingSourceV1::pending_stored_v1(
                                    batch,
                                    constructors.ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?,
                                    reference, receiver, result, (*input).clone(),
                                ).map_err(|_| BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()))?)
                            } else {
                                instance.map(|row| BorrowedIncomingSourceV1::Instance((*row).clone()))
                            }
                        }
                        CallableParameterDeclarationModeV1::StaticBoxMethod => {
                            // Passive source identity is also exact about the namespace.
                            if stored_target.is_some() {
                                continue;
                            }
                            // The original proved Instance source is another namespace.
                            if let Some(original) = instance {
                                if original.call_site() != &owned
                                    || original.target().namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                                    || original.target().name() != call.selector()
                                    || original.target().arity() != call.arity()
                                    || original.argument_sites().len() != call.arguments().len()
                                    || !call.arguments().iter().enumerate().all(|(ordinal, argument)|
                                        argument.ordinal() as usize == ordinal
                                        && original.argument_sites()[ordinal] == *argument.site())
                                {
                                    return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()));
                                }
                                continue;
                            }
                            static_observation
                                .and_then(|row| row.as_ref().ok())
                                .map(|row| {
                                    BorrowedIncomingSourceV1::Static(std::rc::Rc::clone(
                                        row,
                                    ))
                                })
                        }
                        _ => return Err(BorrowedIncomingDraftErrorV1::SourceIdentity),
                    }};
                    let Some(exact) = exact else {
                        let refusal_scope = if legacy && source_static_observation.as_ref()
                            .and_then(|row| row.as_ref().ok()).is_some_and(|source| source.target() != *key)
                        { inventory::IncomingObservationScopeV1::LegacyOnly } else { scope };
                        observations.push(inventory::IncomingObservationV1::new(refusal_scope,
                            Some(*callee),
                            Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(
                                owned.clone(),
                            )),
                        ));
                        continue;
                    };
                    // A proven different receiver class is not an incoming
                    // edge of this definition, despite the same method name.
                    if exact.target() != *key {
                        continue;
                    }
                    if legacy && caller.and_then(|caller| caller.declaration_slot()).is_some() && !ordinary_callers.contains(&input.owner()) {
                        observations.push(inventory::IncomingObservationV1::new(scope,
                            Some(*callee),
                            Err(BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(
                                owned.clone(),
                            )),
                        ));
                        continue;
                    }
                    if exact.call_site() != &owned
                        || exact.callee_owner() != *callee
                        || exact.target_batch_slot() != contract.batch_slot
                        || call.owner() != input.owner()
                        || call.site() != site
                        || call.arguments().len() != contract.parameters.len()
                        || exact.argument_sites().len() != call.arguments().len()
                        || !call
                            .arguments()
                            .iter()
                            .enumerate()
                            .all(|(ordinal, argument)| {
                                argument.ordinal() as usize == ordinal
                                    && exact.argument_sites()[ordinal] == *argument.site()
                                    && contract.parameters[ordinal].ordinal as usize == ordinal
                            })
                    {
                        observations.push(inventory::IncomingObservationV1::new(scope,
                            Some(*callee),
                            Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone())),
                        ));
                        continue;
                    }
                    // Capability exclusions apply to the whole callee. Retain
                    // the original row below, including every caller and veto.
                    if contract.mode == CallableParameterDeclarationModeV1::StaticBoxMethod {
                        let exact_bool_return = matches!(&exact,
                            BorrowedIncomingSourceV1::Static(original)
                                if original.is_current_owner_bool_source_v1())
                            && exact_bool_value_return_site_v1(input, site);
                        // A provider argument is real source incoming, but
                        // the existing provider scalar ABI is not a LOCAL
                        // borrowed-transport capability. Decline the whole
                        // generic cohort, retaining every source observation.
                        if legacy && !input.function().expression_source().initializers().any(|initializer|
                            initializer.initializer_site() == Some(site)) && !exact_bool_return {
                            unsupported_static_context.insert(*callee);
                        }
                        if legacy && call.arguments().iter().zip(&contract.parameters).any(|(argument, formal)|
                            formal.kind.is_ordinary_borrowed_handle()
                                && crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, argument.site()).is_none()) {
                            unsupported_static_spelling.insert(*callee);
                        }
                    }
                    let arguments = call
                        .arguments()
                        .iter()
                        .zip(&contract.parameters)
                        .filter(|(_, formal)| formal.kind.is_ordinary_borrowed_handle())
                        .map(|(argument, formal)| {
                            (argument.ordinal(), argument.site().clone(), formal.binding)
                        })
                        .collect();
                    observations.push(inventory::IncomingObservationV1::new(scope,
                        Some(*callee),
                        Ok(BorrowedIncomingCallDraftV1 {
                            source: exact,
                            call: owned.clone(),
                            callee: *callee,
                            arguments,
                        }),
                    ));
                    seen.insert(*callee);
                    if legacy { legacy_seen.insert(*callee); }
                }
                for observation in &mut observations[first_observation..] {
                    observation.method_source = Some(std::sync::Arc::clone(method));
                }
            }
            Ok(())
    };
    let mut source_global_failures = Vec::new();
    for declaration in batch.declarations() {
        if omitted_static_callers.contains_key(&declaration.owner()) {
            if declaration.mode()
                != crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::StaticBoxMethod
                || selected.key_for_batch_slot(declaration.batch_slot()).is_some()
                || !unvisited_omitted.remove(&declaration.owner())
            {
                return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
            }
        }
        let loan = batch.with_lowering_input(declaration.batch_slot(), |input| {
            collect(Some(OriginalCallSourceCallerV1::Declaration { batch_slot: declaration.batch_slot() }), input)?;
            for (owner, _) in input.forest().semantic_owners().filter(|(owner, _)| *owner != input.owner()) {
                match batch.with_owner_lowering_input(declaration.batch_slot(), owner, |nested| collect(None, nested)) {
                    Ok(result) => result?,
                    Err(_) => source_global_failures.push(inventory::IncomingObservationV1::new(
                        inventory::IncomingObservationScopeV1::SourceOnly, None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))),
                }
            }
            Ok(())
        });
        match loan {
            Ok(result) => result?,
            Err(_) if omitted_static_callers.contains_key(&declaration.owner()) => source_global_failures.push(
                inventory::IncomingObservationV1::new(inventory::IncomingObservationScopeV1::SourceOnly,
                    None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))),
            Err(_) => return Err(BorrowedIncomingDraftErrorV1::BatchLoan),
        }
    }
    if let Some(constructors) = constructors {
        for row in constructors.rows().iter().filter(|row| row.kind() == crate::parser::ConstructorSourceKindV1::Birth) {
            let input = row.lowering_input(batch.source_ast()).map_err(|_| BorrowedIncomingDraftErrorV1::BatchLoan)?;
            collect(Some(OriginalCallSourceCallerV1::Constructor { row, input }), input)?;
            for (owner, _) in input.forest().semantic_owners().filter(|(owner, _)| *owner != input.owner()) {
                match row.lowering_input_for_owner(batch.source_ast(), owner) {
                    Ok(nested) => collect(None, nested)?,
                    Err(_) => source_global_failures.push(inventory::IncomingObservationV1::new(
                        inventory::IncomingObservationScopeV1::SourceOnly, None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))),
                }
            }
        }
    }
    observations.extend(source_global_failures);
    if !unvisited_omitted.is_empty() {
        return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
    }
    for owner in definitions.keys() {
        let legacy_empty = legacy_owners.contains(owner) && !legacy_seen.contains(owner);
        let source_empty = !seen.contains(owner);
        let scope = match (legacy_empty, source_empty) {
            (true, true) => inventory::IncomingObservationScopeV1::Both,
            (true, false) => inventory::IncomingObservationScopeV1::LegacyOnly,
            (false, true) => inventory::IncomingObservationScopeV1::SourceOnly,
            (false, false) => continue,
        };
        observations.push(inventory::IncomingObservationV1::new(scope, Some(*owner),
            Err(BorrowedIncomingDraftErrorV1::NoIncoming(*owner))));
    }
    let inventory = BorrowedIncomingInventoryV1 {
        owners: legacy_owners,
        source_owners: definitions.keys().copied().collect(),
        unsupported_static_spelling,
        unsupported_static_context,
        observations,
        static_observations,
    };
    inventory.corroborate_scan_completeness(
        &static_seen,
        static_context,
        stored_seen.len() == stored.len(),
    )?;
    Ok(inventory)
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_source_graph_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_source_domain_tests.rs"]
mod static_source_domain_tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_capability_tests.rs"]
mod static_capability_tests;

/// Check original passive field dispatch against the exact source call.
/// This issues no receiver inventory, incoming actual or physical permission.
fn corroborate_stored_dispatch_v1(
    caller: crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1<'_>,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    reference: &super::super::source::CallTargetReferenceV1,
    receiver: &super::super::source::StoredReceiverSourceV1,
    selected: &super::super::VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> Result<(), BorrowedIncomingDraftErrorV1> {
    use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1};
    let invalid = || BorrowedIncomingDraftErrorV1::CallIdentity(reference.call_site.clone());
    if reference.call_site.owner() != input.owner()
        || call.owner() != input.owner()
        || reference.call_site.site() != call.site()
        || reference.receiver_site != *call.receiver_site()
        || reference.target.namespace()
            != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
        || reference.target.owner() != receiver.child_class.as_ref()
        || reference.target.name() != call.selector()
        || reference.target.arity() != call.arity()
        || reference.argument_sites.len() != call.arguments().len()
        || !call
            .arguments()
            .iter()
            .enumerate()
            .all(|(ordinal, actual)| {
                actual.ordinal() as usize == ordinal
                    && actual.site() == &reference.argument_sites[ordinal]
            })
        || receiver.parent_binding.owner() != input.owner()
    {
        return Err(invalid());
    }
    let shape = input.body_shape().ok_or_else(invalid)?;
    if !matches!(shape.expression_shape(&reference.receiver_site),
        Some(BodyExpressionShapeV1::FieldAccess { object, field, .. })
            if object == &receiver.parent_site && field == &receiver.field_name)
    {
        return Err(invalid());
    }
    let local_new = match caller.declaration_slot() {
        Some(slot) => receiver.local_new_root_in_input_v1(input, slot).map_err(|_| invalid())?.is_some(),
        None if receiver.parent_new.is_some() => return Err(invalid()),
        None => false,
    };
    if !local_new && (input.function().binding(receiver.parent_binding)
            .is_none_or(|row| row.kind() != crate::mir::resolved_semantics::BindingKindV1::Receiver)
        || !matches!(shape.expression_shape(&receiver.parent_site),
            Some(BodyExpressionShapeV1::Me { receiver: BodyMeReceiverV1::Lexical(binding), .. })
                if binding == &receiver.parent_binding)) { return Err(invalid()); }
    let mut targets = contracts
        .iter()
        .filter(|row| row.owner == reference.callee_owner);
    let target = targets
        .next()
        .filter(|row| {
            row.batch_slot == reference.target_batch_slot
                && row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
                && row.parameters.len() == reference.argument_sites.len()
        })
        .ok_or_else(invalid)?;
    if targets.next().is_some()
        || !matches!(selected.key_for_batch_slot(target.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key == &reference.target)
        || match caller {
            crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1::Declaration { batch_slot } =>
                !matches!(receiver.origin, super::super::source::StoredReceiverSourceOriginV1::Ordinary)
                || !contracts.iter().any(|row| row.owner == input.owner() && row.batch_slot == batch_slot
                    && (local_new || matches!(selected.key_for_batch_slot(row.batch_slot),
                        Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                            if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                                && key.owner() == receiver.parent_class.as_ref()))),
            crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1::Constructor { row, .. } => {
                let key = caller.constructor_key_for_call(&reference.call_site, call).map_err(|_| invalid())?.ok_or_else(invalid)?;
                let super::super::source::StoredReceiverSourceOriginV1::Constructor(loan) = &receiver.origin else { return Err(invalid()); };
                key.owner() != receiver.parent_class.as_ref()
                    || !row.construction_source().as_ref().is_ok_and(|source| std::rc::Rc::ptr_eq(source, loan.source()))
                    || loan.call().site() != &reference.call_site
                    || loan.call().receiver_site() != &reference.receiver_site
                    || loan.call().receiver_binding() != receiver.parent_binding
                    || loan.call().class() != receiver.child_class.as_ref()
                    || loan.call().arguments() != reference.argument_sites.as_ref()
            }
        }
    { return Err(invalid()); }
    Ok(())
}
