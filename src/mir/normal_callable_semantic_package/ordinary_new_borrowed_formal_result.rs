//! Source-result projection for borrowed callers of the existing lexical owner.
use super::borrowed_formal_actuals::PendingBorrowedFormalActualsV1;
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::resolved_semantics::{ResolvedLiteralSourceV1, SourceBindingSiteV1};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct BorrowedI64ResultSourceV1 {
    pub(super) returns: Box<[OwnedExprSiteV1]>,
    pub(super) contract_corroborated: bool,
}

fn source_result(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    owner: FunctionOwnerIdV1,
) -> Result<BorrowedI64ResultSourceV1, String> {
    let mut matches = contracts.iter().filter(|row| row.owner == owner);
    let contract = matches
        .next()
        .ok_or_else(|| freeze("borrowed-result/source-contract-missing"))?;
    if matches.next().is_some() {
        return Err(freeze("borrowed-result/source-contract-duplicate"));
    }
    batch
        .with_lowering_input(contract.batch_slot, |input| {
            let identity_matches = input.owner() == owner
                && contract
                    .parameters
                    .iter()
                    .enumerate()
                    .all(|(index, parameter)| {
                        parameter.ordinal as usize == index
                            && parameter.binding.owner() == owner
                            && input.function().declaration_binding(
                                &SourceBindingSiteV1::Parameter {
                                    index: parameter.ordinal,
                                },
                            ) == Some(parameter.binding)
                    });
            if !identity_matches {
                return Err(freeze("borrowed-result/source-contract-identity"));
            }
            let sites = input
                .body_shape()
                .and_then(|shape| super::super::verified_value_return_sites(input, shape))
                .filter(|sites| !sites.is_empty())
                .ok_or_else(|| freeze("borrowed-result/explicit-value-return-missing"))?;
            for site in &sites {
                let function = input.function();
                let integer = matches!(
                    function.expression_source().literal(site),
                    Some(ResolvedLiteralSourceV1::Integer(_))
                );
                let exact_formal = match function.variable_ref(site) {
                    Some(ResolvedLexicalRefV1::Local(binding)) => {
                        contract.parameters.iter().any(|parameter| {
                            parameter.binding == binding
                                && parameter.kind
                                    == CallableParameterContractKindV1::ExactTrivial(
                                        ExactTrivialParameterAbiV1::I64,
                                    )
                        })
                    }
                    _ => false,
                };
                if !integer && !exact_formal {
                    return Err(freeze("borrowed-result/source-not-i64"));
                }
            }
            Ok(BorrowedI64ResultSourceV1 {
                returns: sites
                    .into_iter()
                    .map(|site| OwnedExprSiteV1::new(owner, site))
                    .collect(),
                contract_corroborated: false,
            })
        })
        .map_err(|_| freeze("borrowed-result/source-loan"))?
}

// Prepare from the same source loan before either prefix walk. Move this
// projection into the existing ledger afterwards; never resolve it again
// from an already completed caller flow.
pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_i64_results_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>> {
    match source {
        Ok(rows) => rows
            .definitions
            .keys()
            .map(|owner| (*owner, source_result(batch, contracts, *owner)))
            .collect(),
        // The source Err itself remains in the ledger and is demanded before
        // any result projection. An empty map does not grant permission.
        Err(_) => BTreeMap::new(),
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn install_borrowed_formal_preparation_v1(
        &mut self,
        source: Result<PreparedBorrowedFormalIngressV1, String>,
        mut actuals: PendingBorrowedFormalActualsV1,
        results: BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
    ) {
        super::borrowed_formal_actuals::finish_borrowed_call_actuals_v1(&source, &mut actuals);
        self.borrowed_i64_results = results;
        self.borrowed_formal_source = Some(source);
        self.borrowed_formal_actuals = actuals;
    }

    pub(super) fn corroborate_borrowed_i64_result_v1(
        &mut self,
        source: &LexicalInstanceCallSourceTargetV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) {
        let Some(pending) = self.borrowed_i64_results.get_mut(&source.callee_owner()) else {
            return;
        };
        let Ok(proof) = pending else {
            return;
        };
        let expected: BTreeSet<_> = proof.returns.iter().cloned().collect();
        let agrees = results.row(source.target_batch_slot()).is_some_and(|row| {
            let borrowed = row.borrow();
            let completion = borrowed.completion();
            let observed: BTreeSet<_> = completion
                .explicit_sites()
                .iter()
                .map(|exit| {
                    let value =
                        crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
                            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Value)
                            .expr();
                    OwnedExprSiteV1::new(completion.owner(), value)
                })
                .collect();
            row.owner() == source.callee_owner()
                && completion.owner() == source.callee_owner()
                && completion.returns_value()
                && expected.len() == proof.returns.len()
                && observed == expected
                && row.result()
                    == Some(crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64)
        });
        if agrees {
            proof.contract_corroborated = true;
        } else {
            *pending = Err(freeze("borrowed-result/result-contract-mismatch"));
        }
    }

    /// Source-phase corroboration precedes old root selection and final result issuance.
    pub(crate) fn borrowed_terminal_arguments_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Result<
        Option<Box<[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>>,
        String,
    > {
        use crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1;
        let Some(crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Call(
            terminal,
        )) = self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        if !terminal
            .arguments()
            .iter()
            .any(|arg| matches!(arg, TerminalCallArgumentV1::Lexical(_)))
        {
            return Ok(None);
        }
        let completion = self
            .completion_for_owner(owner)
            .ok_or_else(|| freeze("borrowed-terminal/completion-missing"))?;
        let direct_value = crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Value)
            .expr();
        if completion.owner() != owner
            || !completion.returns_value()
            || !completion.explicit_sites().contains(exit)
            || terminal.owner() != owner
            || terminal.return_site() != exit
            || terminal.call_site() != &direct_value
        {
            return Err(freeze("borrowed-terminal/return-site"));
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-terminal/source-missing"))?;
        let site = OwnedExprSiteV1::new(owner, terminal.call_site().clone());
        let arguments = super::borrowed_formal_actuals::project_pending_borrowed_i64_arguments_v1(
            source,
            &self.borrowed_formal_actuals,
            &self.borrowed_i64_results,
            &site,
        )?
        .ok_or_else(|| freeze("borrowed-terminal/incoming-missing"))?;
        if terminal.arguments().len() != arguments.len()
            || terminal.arguments().iter().zip(arguments.iter()).any(|(observed, expected)|
                !matches!(observed, TerminalCallArgumentV1::Lexical(arg) if arg == expected)) {
            return Err(freeze("borrowed-terminal/ordered-arguments"));
        }
        Ok(Some(arguments))
    }

    /// Retain the original Taken lexical row; no second root-instance slot exists.
    pub(crate) fn take_borrowed_lexical_call_for_return_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Result<Option<LexicalInstanceCallDispositionRowV1>, String> {
        if self.borrowed_terminal_arguments_v1(owner, exit)?.is_none() {
            return Ok(None);
        }
        let (_, terminal) = self
            .call_source_completion_for_owner_at(owner, exit)
            .ok_or_else(|| freeze("borrowed-terminal/source-missing"))?;
        let row = self
            .take_lexical_instance_call(owner, terminal.call_site())?
            .ok_or_else(|| freeze("borrowed-terminal/disposition-missing"))?;
        self.borrowed_call_actuals_v1(&row)?
            .ok_or_else(|| freeze("borrowed-terminal/actuals-missing"))?;
        Ok(Some(row))
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_result_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_terminal_tests.rs"]
mod terminal_tests;
