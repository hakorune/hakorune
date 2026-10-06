//! Single composition fold over source exits; never a Home issuer.
use super::product::SourceResultRowV1;
use super::{
    parameter_contract, OrdinaryNewResultClassClaimsV1, OrdinaryNewResultClassV1, PendingExitV1,
    PendingResultExitV1, ResultExitOriginV1, ResultValueOriginV1,
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
        let mut alternatives = BTreeSet::new();
        match &exit.exit {
            PendingExitV1::New(found) => {
                alternatives.insert(ResultValueOriginV1::Fresh(found.clone()));
            }
            PendingExitV1::Null => {
                alternatives.insert(ResultValueOriginV1::Null);
            }
            PendingExitV1::Formal { ordinal } => {
                alternatives.insert(ResultValueOriginV1::Null);
                alternatives.insert(ResultValueOriginV1::ForwardFormal { ordinal: *ordinal });
            }
            PendingExitV1::Fwd { key, actuals } => {
                let Some(callee_rows) = claims.outcomes(key) else {
                    if pending.contains(key) {
                        waiting = true;
                        continue;
                    }
                    return ExitVerdictV1::Dead;
                };
                // Passive outcomes cannot upgrade the historical ownership projection.
                legacy_eligible &= claims.contains_key(key);
                for origin in callee_rows.iter().flat_map(|row| row.alternatives()) {
                    let origin = match origin {
                        ResultValueOriginV1::ForwardFormal { ordinal } => {
                            let Some(Some(binding)) = actuals.get(*ordinal as usize) else {
                                return ExitVerdictV1::Dead;
                            };
                            let Some(parameter) =
                                parameter_contract(parameter_contracts, batch_slot, *binding)
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
                            ResultValueOriginV1::ForwardFormal {
                                ordinal: parameter.ordinal,
                            }
                        }
                        other => other.clone(),
                    };
                    alternatives.insert(origin);
                }
            }
        }
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
