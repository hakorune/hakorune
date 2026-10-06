//! Shared source-call identity and witness composition; no Home or pending policy.
use super::{
    parameter_contract, ResultActualSourceV1, ResultExitOriginV1, ResultFormalSubstitutionV1,
    ResultOriginWitnessV1, ResultValueOriginV1, ResultWitnessStepV1,
};
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::rc::Rc;

/// Ephemeral corroboration of existing source edges, not an admission receipt.
pub(super) struct CallWitnessSourceV1<'a> {
    output_site: &'a OwnedExprSiteV1,
    call_site: &'a OwnedExprSiteV1,
    key: &'a CanonicalSameModuleCallableKeyV1,
    actuals: &'a [ResultActualSourceV1],
}
impl<'a> CallWitnessSourceV1<'a> {
    pub(super) fn verify(
        output_site: &'a OwnedExprSiteV1,
        call_site: &'a OwnedExprSiteV1,
        key: &'a CanonicalSameModuleCallableKeyV1,
        actuals: &'a [ResultActualSourceV1],
    ) -> Option<Self> {
        if call_site.owner() != output_site.owner() || actuals.len() != key.arity() as usize {
            return None;
        }
        for (ordinal, actual) in actuals.iter().enumerate() {
            let mut expected = call_site.site().node().segments().to_vec();
            expected.push(
                crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(ordinal as u32),
            );
            if actual.site.owner() != call_site.owner()
                || actual.site.site().node().segments() != expected
            {
                return None;
            }
        }
        Some(Self {
            output_site,
            call_site,
            key,
            actuals,
        })
    }

    pub(super) fn compose(
        self,
        callee_rows: &[ResultExitOriginV1],
        parameter_contracts: &[OwnedCallableParameterContractDeclarationV1],
        batch_slot: u32,
    ) -> Option<Vec<Rc<ResultOriginWitnessV1>>> {
        let Self {
            output_site,
            call_site,
            key,
            actuals,
        } = self;
        let mut witnesses = Vec::new();
        for callee in callee_rows.iter().flat_map(|row| row.witnesses()) {
            let substitution = if let Some(ordinal) = callee.formal_ordinal() {
                let Some(actual) = actuals.get(ordinal as usize) else {
                    return None;
                };
                let Some(binding) = actual.binding else {
                    return None;
                };
                let Some(parameter) = parameter_contract(parameter_contracts, batch_slot, binding)
                else {
                    return None;
                };
                if !matches!(
                    parameter.kind,
                    CallableParameterContractKindV1::DeclaredObject(_)
                        | CallableParameterContractKindV1::OpaqueHandle
                ) {
                    return None;
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
                (ResultValueOriginV1::ForwardFormal { .. }, None) => return None,
                (other, _) => other.clone(),
            };
            witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                site: output_site.clone(),
                origin,
                step: ResultWitnessStepV1::Call {
                    site: call_site.clone(),
                    key: key.clone(),
                    callee: std::rc::Rc::clone(callee),
                    substitution,
                },
            }));
        }
        Some(witnesses)
    }
}
