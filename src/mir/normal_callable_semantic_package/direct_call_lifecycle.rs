//! Co-seal existing source products in the affine AppMain Call inventory.
//! No target lookup by name, second Completion, or physical continuation is issued.
use super::super::model::OwnedCallableParameterContractDeclarationV1;
use super::super::ordinary_new_coseal::OrdinaryNewClaimLedgerV1;
use super::super::result_contract::VerifiedCallableResultContractCohortV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, SourceBindingSiteV1};

fn map_owned(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    row: &AppMainDirectCallDispositionRowV1,
) -> bool {
    let owner = row.emission.target().callable().owner();
    batch.declarations().any(|declaration| {
        declaration.owner() == owner
            && declaration
                .body_shape()
                .expressions()
                .iter()
                .any(|expression| matches!(expression, BodyExpressionShapeV1::MapLiteral { .. }))
    })
}

fn exact_formals(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameters: &[OwnedCallableParameterContractDeclarationV1],
    row: &AppMainDirectCallDispositionRowV1,
) -> bool {
    let target = row.emission.target();
    if !target.published_key().is_some_and(|key| {
        key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
    }) || target.signature().result() != ExactTrivialScalarAbiV1::I64
    {
        return false;
    }
    let mut matches = parameters
        .iter()
        .filter(|row| row.owner == target.callable().owner());
    let Some(contract) = matches.next() else {
        return false;
    };
    if matches.next().is_some()
        || contract.parameters.len() != row.argument_sites.len()
        || target.signature().arity() != row.argument_sites.len()
    {
        return false;
    }
    batch
        .with_lowering_input(contract.batch_slot, |input| {
            input.owner() == contract.owner
                && contract
                    .parameters
                    .iter()
                    .enumerate()
                    .all(|(index, parameter)| {
                        parameter.ordinal as usize == index
                            && parameter.kind
                                == CallableParameterContractKindV1::ExactTrivial(
                                    ExactTrivialParameterAbiV1::I64,
                                )
                            && target.signature().params()[index] == ExactTrivialScalarAbiV1::I64
                            && input.function().declaration_binding(
                                &SourceBindingSiteV1::Parameter {
                                    index: parameter.ordinal,
                                },
                            ) == Some(parameter.binding)
                    })
        })
        .unwrap_or(false)
}

impl AppMainDirectCallDispositionLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn has_map_target(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
    ) -> bool {
        self.rows.values().any(|slot| match slot {
            AppMainDirectCallDispositionSlotV1::Ready(row) => map_owned(batch, row),
            AppMainDirectCallDispositionSlotV1::Taken => false,
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn is_map_i64_call(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &OwnedExprSiteV1,
    ) -> bool {
        if site.owner() != self.owner || input.owner() != self.owner {
            return false;
        }
        match self.rows.get(site) {
            Some(AppMainDirectCallDispositionSlotV1::Ready(row)) => {
                map_owned(batch, row)
                    && exact_formals(batch, parameters, row)
                    && input
                        .function()
                        .direct_call_target(site.site())
                        .is_some_and(|target| target.callable() == row.emission.target().callable())
                    && input.function().direct_call_observations().any(
                        |(observed_site, observation)| {
                            observed_site == site.site()
                                && observation.argument_sites() == row.argument_sites()
                        },
                    )
            }
            _ => false,
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn co_seal_lifecycle(
        &mut self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        results: &VerifiedCallableResultContractCohortV1,
        root: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), AppMainDirectCallLoanErrorV1> {
        let reject = AppMainDirectCallLoanErrorV1::LifecycleSourceMismatch;
        for (site, slot) in &mut self.rows {
            let AppMainDirectCallDispositionSlotV1::Ready(row) = slot else {
                return Err(reject);
            };
            // Unavailable Map coverage is still Map-owned, never Scalar evidence.
            if !map_owned(batch, row) {
                continue;
            }
            if !exact_formals(batch, parameters, row) {
                return Err(reject);
            }
            let (caller, terminal) = root.call_source_completion().ok_or(reject)?;
            if terminal.owner() != self.owner
                || caller.owner() != self.owner
                || site.site() != terminal.call_site()
                || caller.explicit_site() != Some(terminal.return_site())
                || !caller.returns_value()
                || !matches!(caller.cleanup().terminal_homes(), Some(Ok(_)))
                || terminal.arguments().len() != row.argument_sites.len()
            {
                return Err(reject);
            }
            let owner = row.emission.target().callable().owner();
            let mut matches = results.rows().filter(|result| result.owner() == owner);
            let callee = matches.next().ok_or(reject)?.borrow();
            if matches.next().is_some()
                || owner == self.owner
                || callee.result() != Some(ExactTrivialScalarAbiV1::I64)
                || !callee.completion().returns_value()
            {
                return Err(reject);
            }
            let Some(TerminalRelationV1::IntegerLiteral(value)) = callee.terminal_relation() else {
                return Err(reject);
            };
            if value.owner() != owner
                || callee.completion().explicit_site() != Some(value.return_site())
            {
                return Err(reject);
            }
            let flow = callee.completion().cleanup().root_flow().ok_or(reject)?;
            if flow.terminal_homes().is_err()
                || flow.maps().is_empty()
                || flow.maps().iter().any(|map| map.complete().is_none())
            {
                return Err(reject);
            }
            row.execution = AppMainCallExecutionV1::Lifecycle;
        }
        Ok(())
    }
}
