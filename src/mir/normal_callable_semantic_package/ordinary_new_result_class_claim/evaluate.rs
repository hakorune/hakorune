//! Single composition fold over source exits; never a Home issuer.
use super::product::SourceResultRowV1;
use super::{
    parameter_contract, OrdinaryNewResultClassClaimsV1, OrdinaryNewResultClassV1, PendingExitV1,
    PendingResultExitV1, ResultExitOriginV1, ResultFormalSubstitutionV1, ResultOriginWitnessV1,
    ResultValueOriginV1, ResultWitnessStepV1,
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
) -> ExitVerdictV1 {
    let mut rows = Vec::new();
    let mut class: Option<Box<str>> = None;
    let mut forwarded = None;
    let mut nullable = false;
    let mut waiting = false;
    let mut legacy_eligible = true;
    for exit in exits {
        let mut witnesses = Vec::new();
        match &exit.exit {
            PendingExitV1::New(found) => witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                site: exit.site.clone(),
                origin: ResultValueOriginV1::Fresh(found.clone()),
                step: ResultWitnessStepV1::FreshConstruction,
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
                if call_site.owner() != exit.site.owner() || actuals.len() != key.arity() as usize {
                    return ExitVerdictV1::Dead;
                }
                for (ordinal, actual) in actuals.iter().enumerate() {
                    let mut expected = call_site.site().node().segments().to_vec();
                    expected.push(
                        crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(
                            ordinal as u32,
                        ),
                    );
                    if actual.site.owner() != call_site.owner()
                        || actual.site.site().node().segments() != expected
                    {
                        return ExitVerdictV1::Dead;
                    }
                }
                let Some(callee_rows) = claims.outcomes(key) else {
                    if pending.contains(key) {
                        waiting = true;
                        continue;
                    }
                    return ExitVerdictV1::Dead;
                };
                legacy_eligible &= claims.contains_key(key);
                for callee in callee_rows.iter().flat_map(|row| row.witnesses()) {
                    let substitution = if let Some(ordinal) = callee.formal_ordinal() {
                        let Some(actual) = actuals.get(ordinal as usize) else {
                            return ExitVerdictV1::Dead;
                        };
                        let Some(binding) = actual.binding else {
                            return ExitVerdictV1::Dead;
                        };
                        let Some(parameter) =
                            parameter_contract(parameter_contracts, batch_slot, binding)
                        else {
                            return ExitVerdictV1::Dead;
                        };
                        if !matches!(
                            parameter.kind,
                            CallableParameterContractKindV1::DeclaredObject(_)
                                | CallableParameterContractKindV1::OpaqueHandle
                        ) {
                            return ExitVerdictV1::Dead;
                        }
                        Some(ResultFormalSubstitutionV1 {
                            argument_site: actual.site.clone(),
                            binding,
                            callee_ordinal: ordinal,
                            caller_ordinal: parameter.ordinal,
                        })
                    } else {
                        None
                    };
                    let origin = match (callee.origin(), &substitution) {
                        (ResultValueOriginV1::ForwardFormal { .. }, Some(row)) => {
                            ResultValueOriginV1::ForwardFormal {
                                ordinal: row.caller_ordinal,
                            }
                        }
                        (ResultValueOriginV1::ForwardFormal { .. }, None) => {
                            return ExitVerdictV1::Dead
                        }
                        (other, _) => other.clone(),
                    };
                    witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                        site: exit.site.clone(),
                        origin,
                        step: ResultWitnessStepV1::Call {
                            site: call_site.clone(),
                            key: key.clone(),
                            callee: std::rc::Rc::clone(callee),
                            substitution,
                        },
                    }));
                }
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
    let projection = if legacy_eligible {
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
        exits: rows.into_boxed_slice(),
        projection,
    })
}
