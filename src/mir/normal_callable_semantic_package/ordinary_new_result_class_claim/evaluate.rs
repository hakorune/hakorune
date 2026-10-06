//! Composition of the existing sealed result-exit draft.
//! This private owner preserves the result solver's original projections;
//! it does not observe source, issue a Home, or lower physical instructions.

use std::collections::BTreeSet;

use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

use super::{
    parameter_contract, OrdinaryNewResultClassClaimsV1, OrdinaryNewResultClassV1, PendingExitV1,
};

/// Whether every exit of this row now composes: `New`/`Null` are self-
/// evident, `Fwd` waits on the callee's claim. Waiting on a key that is
/// neither claimed nor still pending means the callee can never prove a
/// class — the row is dead, not pending.
pub(super) enum ExitVerdictV1 {
    Resolvable(OrdinaryNewResultClassV1),
    Waiting,
    Dead,
}

pub(super) fn evaluate_row(
    exits: &[PendingExitV1],
    claims: &OrdinaryNewResultClassClaimsV1,
    pending: &BTreeSet<CanonicalSameModuleCallableKeyV1>,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    batch_slot: u32,
) -> ExitVerdictV1 {
    use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
    let mut class: Option<Box<str>> = None;
    let mut forwarded: Option<u32> = None;
    let mut nullable = false;
    let mut waiting = false;
    for exit in exits {
        let found_class: Box<str> = match exit {
            PendingExitV1::Null => {
                nullable = true;
                continue;
            }
            PendingExitV1::Formal { ordinal } => {
                // An ordinary formal return is an identity claim —
                // nullable because the caller may pass `null`. It never
                // mints a class of its own and never coexists with one.
                nullable = true;
                match forwarded {
                    None => forwarded = Some(*ordinal),
                    Some(existing) if existing == *ordinal => {}
                    _ => return ExitVerdictV1::Dead,
                }
                continue;
            }
            PendingExitV1::New(found) => found.clone(),
            PendingExitV1::Fwd { key, actuals } => match claims.get(key) {
                Some(claim) => match claim {
                    OrdinaryNewResultClassV1::Object(found)
                    | OrdinaryNewResultClassV1::NullableObject(found) => {
                        nullable |= claim.is_nullable();
                        found.clone()
                    }
                    OrdinaryNewResultClassV1::NullableForwarded { ordinal } => {
                        // An ordinary formal supplies a borrowed identity, not
                        // a new Home, even when its class is declared.
                        nullable = true;
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
                        match forwarded {
                            None => forwarded = Some(parameter.ordinal),
                            Some(existing) if existing == parameter.ordinal => {}
                            _ => return ExitVerdictV1::Dead,
                        }
                        continue;
                    }
                },
                None if pending.contains(key) => {
                    waiting = true;
                    continue;
                }
                None => return ExitVerdictV1::Dead,
            },
        };
        match &class {
            None => class = Some(found_class),
            Some(existing) if existing.as_ref() == found_class.as_ref() => {}
            Some(_) => return ExitVerdictV1::Dead,
        }
    }
    if waiting {
        return ExitVerdictV1::Waiting;
    }
    match (class, forwarded) {
        (Some(class), None) => ExitVerdictV1::Resolvable(if nullable {
            OrdinaryNewResultClassV1::NullableObject(class)
        } else {
            OrdinaryNewResultClassV1::Object(class)
        }),
        (None, Some(ordinal)) => {
            ExitVerdictV1::Resolvable(OrdinaryNewResultClassV1::NullableForwarded { ordinal })
        }
        _ => ExitVerdictV1::Dead,
    }
}
