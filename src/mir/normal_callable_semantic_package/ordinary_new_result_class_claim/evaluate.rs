//! Single composition fold over source exits; never a Home issuer.
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
) -> ExitVerdictV1 {
    let support = if let Some(read) = borrowed {
        let crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Formal { binding, ordinal } = read.support()
            else { return ExitVerdictV1::Dead; };
        let exact = parameter_contracts.iter().find(|row| row.batch_slot == batch_slot && row.owner == read.owner())
            .and_then(|row| row.parameters.iter().find(|row| row.binding == binding && row.ordinal == ordinal));
        if binding.owner() != read.owner() || !exact.is_some_and(|parameter| matches!(&parameter.kind,
            CallableParameterContractKindV1::DeclaredObject(class) if class.as_ref() == read.declared_type())) {
            return ExitVerdictV1::Dead;
        }
        Some((binding, ordinal, read.owner()))
    } else { None };
    let mut rows = Vec::new();
    let mut class: Option<Box<str>> = None;
    let mut forwarded = None;
    let mut nullable = false;
    let mut waiting = false;
    let mut legacy_eligible = true;
    for exit in exits {
        let mut witnesses = Vec::new();
        match &exit.exit {
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
                let Some(composed) =
                    call_source.compose(callee_rows, parameter_contracts, batch_slot)
                else {
                    return ExitVerdictV1::Dead;
                };
                witnesses = composed;
            }
        }
        if let Some((binding, ordinal, owner)) = support {
            if exit.site.owner() != owner || witnesses.is_empty() || witnesses.iter().any(|root| !root.supports_formal_v1(binding, ordinal)) {
                return ExitVerdictV1::Dead;
            }
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
                ResultValueOriginV1::MapHeld { .. } => legacy_eligible = false,
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
    let projection = if let Some((_, ordinal, _)) = support {
        Some(OrdinaryNewResultClassV1::NullableForwarded { ordinal })
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
