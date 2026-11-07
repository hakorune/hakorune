//! Single composition fold over source exits; never a Home issuer.
use super::*;
use super::product::SourceResultRowV1;
use super::{
    OrdinaryNewResultClassClaimsV1, OrdinaryNewResultClassV1, PendingExitV1, PendingResultExitV1,
    ResultExitOriginV1, ResultOriginWitnessV1, ResultValueOriginV1, ResultWitnessStepV1,
};
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::BTreeSet;

pub(super) enum ExitVerdictV1 {
    Resolvable(SourceResultRowV1),
    Waiting,
    Dead,
}

pub(super) fn evaluate_row(
    exits: &[PendingResultExitV1],
    claims: &OrdinaryNewResultClassClaimsV1,
    pending: &BTreeSet<CanonicalSameModuleCallableKeyV1>,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    batch_slot: u32,
    coverage: &ParserOrdinaryBoxSourceCoverageV1,
    borrowed: Option<&crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>,
    context: Option<&super::witness::SourceCompositionContextV1<'_>>,
) -> ExitVerdictV1 {
    if let Some(read) = borrowed {
        use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1 as Support;
        let binding = match read.support() {
            Support::Formal { binding, ordinal } => {
                let exact = parameter_contracts.iter().find(|row| row.batch_slot == batch_slot && row.owner == read.owner())
                    .and_then(|row| row.parameters.iter().find(|row| row.binding == binding && row.ordinal == ordinal));
                if !exact.is_some_and(|parameter| matches!(&parameter.kind,
                    CallableParameterContractKindV1::DeclaredObject(class) if class.as_ref() == read.declared_type())) { return ExitVerdictV1::Dead; }
                binding
            },
            Support::Receiver { binding } => {
                if coverage.row_for(read.declared_type()).ok().flatten().is_none() { return ExitVerdictV1::Dead; }
                binding
            },
        };
        if binding.owner() != read.owner() { return ExitVerdictV1::Dead; }
    }
    let mut rows = Vec::new();
    let mut class: Option<Box<str>> = None;
    let mut forwarded = None;
    let mut nullable = false;
    let mut waiting = false;
    let mut legacy_eligible = true;
    for exit in exits {
        let mut witnesses = Vec::new();
        match &exit.exit {
            PendingExitV1::Receiver { binding, class, projection } => witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                site: exit.site.clone(), origin: ResultValueOriginV1::BorrowedReceiver { class: class.clone() },
                step: ResultWitnessStepV1::Receiver { binding: *binding, projection: projection.clone() },
            })),
            PendingExitV1::MapStored(source) => {
                if !source.returns.contains_key(&exit.site) { return ExitVerdictV1::Dead; }
                legacy_eligible = false;
                witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                    site: exit.site.clone(), origin: source.origin(),
                    step: ResultWitnessStepV1::MapStored { source: std::rc::Rc::clone(source) },
                }));
            }
            PendingExitV1::New(found) => witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                site: exit.site.clone(),
                origin: ResultValueOriginV1::Fresh(found.clone()),
                step: ResultWitnessStepV1::FreshConstruction,
            })),
            PendingExitV1::LocalNew(source) => witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                site: exit.site.clone(),
                origin: ResultValueOriginV1::Fresh(source.construction.class().into()),
                step: ResultWitnessStepV1::LocalConstruction(std::rc::Rc::clone(source)),
            })),
            PendingExitV1::Null => witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                site: exit.site.clone(),
                origin: ResultValueOriginV1::Null,
                step: ResultWitnessStepV1::NullLiteral,
            })),
            PendingExitV1::Formal { binding, ordinal } => {
                for origin in [
                    ResultValueOriginV1::Null,
                    ResultValueOriginV1::ForwardFormal { ordinal: *ordinal },
                ] {
                    witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                        site: exit.site.clone(),
                        origin,
                        step: ResultWitnessStepV1::Formal {
                            binding: *binding,
                            ordinal: *ordinal,
                        },
                    }));
                }
            }
            PendingExitV1::Fwd {
                call_site,
                key,
                actuals,
                receiver,
            } => {
                let Some(call_source) = super::call_witness::CallWitnessSourceV1::verify(
                    &exit.site, call_site, key, actuals,
                ) else {
                    return ExitVerdictV1::Dead;
                };
                let Some(callee_rows) = claims.outcomes(key) else {
                    if pending.contains(key) {
                        waiting = true;
                        continue;
                    }
                    return ExitVerdictV1::Dead;
                };
                legacy_eligible &= claims.contains_key(key);
                witnesses = match call_source.with_receiver(receiver.as_ref())
                    .compose_with_source(callee_rows, parameter_contracts, batch_slot, context) {
                    Ok(Some(witnesses)) => witnesses,
                    Ok(None) => return ExitVerdictV1::Dead,
                    Err(key) if pending.contains(&key) => { waiting = true; continue; },
                    Err(_) => return ExitVerdictV1::Dead,
                };
            }
        }
        if let Some(read) = borrowed {
            if exit.site.owner() != read.owner() || witnesses.is_empty() || witnesses.iter().any(|root| match read.support() {
                crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Formal { binding, ordinal } => !root.supports_formal_v1(binding, ordinal),
                crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { binding } => !root.supports_receiver_draft_v1(binding, read.declared_type()),
            }) { return ExitVerdictV1::Dead; }
        }
        let alternatives: BTreeSet<_> = witnesses
            .iter()
            .map(|witness| witness.origin().clone())
            .collect();
        if alternatives.is_empty() {
            return ExitVerdictV1::Dead;
        }
        for origin in &alternatives {
            match origin {
                ResultValueOriginV1::Null => nullable = true,
                ResultValueOriginV1::MapHeld { .. } | ResultValueOriginV1::BorrowedReceiver { .. } => legacy_eligible = false,
                ResultValueOriginV1::Fresh(found) => {
                    // Check before exposing this row to any dependent composition.
                    if coverage.row_for(found).ok().flatten().is_none() {
                        return ExitVerdictV1::Dead;
                    }
                    match &class {
                        None => class = Some(found.clone()),
                        Some(existing) if existing == found => {}
                        _ => return ExitVerdictV1::Dead,
                    }
                }
                ResultValueOriginV1::ForwardFormal { ordinal } => match forwarded {
                    None => forwarded = Some(*ordinal),
                    Some(existing) if existing == *ordinal => {}
                    _ => return ExitVerdictV1::Dead,
                },
            }
        }
        rows.push(ResultExitOriginV1 {
            site: exit.site.clone(),
            alternatives,
            witnesses: witnesses.into_boxed_slice(),
        });
    }
    if waiting {
        return ExitVerdictV1::Waiting;
    }
    if rows.is_empty() {
        return ExitVerdictV1::Dead;
    }
    if let (Some(found), Some(ordinal)) = (&class, forwarded) {
        let Some(parameter) = parameter_contracts
            .iter()
            .find(|row| row.batch_slot == batch_slot)
            .and_then(|row| {
                row.parameters
                    .iter()
                    .find(|parameter| parameter.ordinal == ordinal)
            })
        else {
            return ExitVerdictV1::Dead;
        };
        if let CallableParameterContractKindV1::DeclaredObject(declared) = &parameter.kind {
            if declared != found {
                return ExitVerdictV1::Dead;
            }
        }
    }
    let projection = if let Some(read) = borrowed {
        match read.support() {
            crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Formal { ordinal, .. } => Some(OrdinaryNewResultClassV1::NullableForwarded { ordinal }),
            crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { .. } => None,
        }
    } else if legacy_eligible {
        match (class, forwarded) {
            (Some(class), None) => Some(if nullable {
                OrdinaryNewResultClassV1::NullableObject(class)
            } else {
                OrdinaryNewResultClassV1::Object(class)
            }),
            (None, Some(ordinal)) => Some(OrdinaryNewResultClassV1::NullableForwarded { ordinal }),
            _ => None,
        }
    } else {
        None
    };
    ExitVerdictV1::Resolvable(SourceResultRowV1 {
        borrowed_result: None,
        exits: rows.into_boxed_slice(),
        projection,
    })
}

// The source collector retains exits; this owner solves their composition.
impl OrdinaryNewResultClassClaimDraftV1 {
    /// Pass B: resolve forwarded exits against the batch's own sealed
    /// facts, then compose by monotone fixpoint — every iteration either
    /// mints newly-provable claims or removes rows whose forwarded callee
    /// can never claim, so iteration is bounded by the pending-key count.
    /// Fresh classes require ordinary source coverage before insertion;
    /// forwarded identities carry no acquired Home. Passive mixed/null-only
    /// rows remain absent from the legacy class projection.
    pub(crate) fn finish(
        self,
        ordinary_box_coverage: &ParserOrdinaryBoxSourceCoverageV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        app_main: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedAppMainSourceLoanV1<'_>>,
        field_write_claims: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        entry_home: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1>,
    ) -> OrdinaryNewResultClassClaimsV1 {
        use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
        let mut pending: BTreeMap<
            CanonicalSameModuleCallableKeyV1,
            (u32, Vec<PendingResultExitV1>, Option<Arc<ResolvedCallableBorrowedResultDeclarationV1>>, Option<Box<str>>),
        > = BTreeMap::new();
        for row in self.rows {
            let main = app_main.filter(|main| main.batch_slot() == row.batch_slot);
            if let Some(main) = main {
                if main.catalog_key() != &row.key {
                    continue;
                }
            } else if !matches!(selected.key_for_batch_slot(row.batch_slot),
                Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == &row.key)
            {
                continue;
            }
            let resolved = batch.with_lowering_input(row.batch_slot, |input| {
                if main.is_some_and(|main| {
                    !main.matches_function(input.owner(), row.batch_slot, &row.key)
                }) {
                    return None;
                }
                let function = input.function();
                let Some(body_shape) = input.body_shape() else {
                    return None;
                };
                let stored = entry_home.and_then(|entries| witness::map_stored_result_source_v1(
                    input, &row, batch, selected, constructors, entries, field_write_claims));
                row.exits
                    .iter()
                    .map(|(site, exit)| {
                        if site.owner() != input.owner() {
                            return None;
                        }
                        if let Some(source) = &stored {
                            return Some(PendingResultExitV1 { site: site.clone(),
                                exit: PendingExitV1::MapStored(std::rc::Rc::clone(source)) });
                        }
                        let exit = match exit {
                            ResultClassExitDraftV1::New(class) => {
                                Some(PendingExitV1::New(class.clone()))
                            }
                            ResultClassExitDraftV1::Null => Some(PendingExitV1::Null),
                            ResultClassExitDraftV1::ForwardCall(site) => resolve_call_key(
                                site,
                                function,
                                body_shape,
                                &row.key,
                                batch,
                                selected,
                                field_write_claims,
                            )
                            .map(|(key, actuals)| PendingExitV1::Fwd {
                                call_site: crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    input.owner(),
                                    site.clone(),
                                ),
                                key,
                                actuals,
                                receiver: super::call_witness::original_receiver_actual_v1(function, body_shape, site),
                            }),
                            ResultClassExitDraftV1::ForwardLocal(binding) => resolve_forward_local(
                                *binding,
                                site,
                                input,
                                body_shape,
                                &row.key,
                                batch,
                                selected,
                                field_write_claims,
                                row.borrowed_result.is_some(),
                            ),
                            ResultClassExitDraftV1::Receiver { binding, class, projection } => Some(PendingExitV1::Receiver { binding: *binding, class: class.clone(), projection: projection.clone() }),
                            ResultClassExitDraftV1::ForwardFormal { binding, ordinal } => {
                                // Both ordinary kinds borrow their input. A type
                                // annotation proves class, not a moved-in Home.
                                parameter_contract(parameter_contracts, row.batch_slot, *binding)
                                    .filter(|parameter| parameter.ordinal == *ordinal)
                                    .filter(|parameter| {
                                        matches!(
                                            parameter.kind,
                                            CallableParameterContractKindV1::DeclaredObject(_)
                                                | CallableParameterContractKindV1::OpaqueHandle
                                        )
                                    })
                                    .map(|_| PendingExitV1::Formal {
                                        binding: *binding,
                                        ordinal: *ordinal,
                                    })
                            }
                        }?;
                        Some(PendingResultExitV1 {
                            site: site.clone(),
                            exit,
                        })
                    })
                    .collect::<Option<Vec<_>>>()
            });
            if let Ok(Some(exits)) = resolved {
                pending.insert(row.key, (row.batch_slot, exits, row.borrowed_result, row.declared_box));
            }
        }
        let mut claims = OrdinaryNewResultClassClaimsV1::new();
        loop {
            let pending_keys: BTreeSet<CanonicalSameModuleCallableKeyV1> =
                pending.keys().cloned().collect();
            let mut inserts = Vec::new();
            let mut deads = Vec::new();
            for (key, (batch_slot, exits, borrowed_result, declared_box)) in &pending {
                let context = super::witness::SourceCompositionContextV1 {
                    batch, selected, fields: field_write_claims, claims: &claims, pending: &pending_keys,
                    parameters: parameter_contracts, caller: key, batch_slot: *batch_slot,
                };
                match evaluate_row(
                    exits,
                    &claims,
                    &pending_keys,
                    parameter_contracts,
                    *batch_slot,
                    ordinary_box_coverage,
                    borrowed_result.as_deref(),
                    borrowed_result.as_ref().map(|_| &context),
                ) {
                    // O3: a declared box fixes the class; the body must return exactly that box.
                    ExitVerdictV1::Resolvable(claim)
                        if declared_box.as_deref().is_some_and(|declared| !matches!(&claim.projection,
                            Some(OrdinaryNewResultClassV1::Object(class)) if class.as_ref() == declared)) =>
                    {
                        deads.push(key.clone());
                    }
                    ExitVerdictV1::Resolvable(mut claim) => {
                        claim.borrowed_result = borrowed_result.clone();
                        inserts.push((key.clone(), claim));
                    },
                    ExitVerdictV1::Dead => deads.push(key.clone()),
                    ExitVerdictV1::Waiting => {}
                }
            }
            if inserts.is_empty() && deads.is_empty() {
                break;
            }
            for key in deads {
                pending.remove(&key);
            }
            for (key, claim) in inserts {
                pending.remove(&key);
                claims.insert(key, claim);
            }
        }
        if let Some(entries) = entry_home {
            claims.set_map_installs(witness::map_install_sources_v1(batch, selected, constructors, entries, field_write_claims));
        }
        claims
    }
}
