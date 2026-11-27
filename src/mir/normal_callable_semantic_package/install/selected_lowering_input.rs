use crate::mir::builder::SelectedNormalCallableKeyV1;

use super::{
    CallableResultContractRefV1,
    InstalledNormalCallableSemanticPackageV1, NormalCallableDynamicProjectionV1,
    NormalCallableSemanticPackageInstallIssueV1, SelectedCallableLoweringInputRefV1,
    SelectedCallableSemanticRefV1,
};

impl InstalledNormalCallableSemanticPackageV1 {
    pub(super) fn with_selected_lowering_input<R>(
        &self,
        key: &SelectedNormalCallableKeyV1,
        callback: impl for<'loan> FnOnce(SelectedCallableLoweringInputRefV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        let row = self
            .selected
            .row_for_key(key)
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)?;
        let batch_slot = row.batch_slot();
        let selected_key = row.key().clone();
        let parameter_declaration = match key {
            SelectedNormalCallableKeyV1::Cataloged(_) => {
                let mut declarations = self
                    .parameter_contracts
                    .iter()
                    .filter(|row| row.batch_slot == batch_slot);
                let declaration = declarations
                    .next()
                    .ok_or(NormalCallableSemanticPackageInstallIssueV1::MissingParameterContract)?;
                if declarations.next().is_some() {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::DuplicateParameterContract,
                    );
                }
                Some(declaration)
            }
            SelectedNormalCallableKeyV1::TopLevel(_) => None,
        };
        let block_expr_expectation = self
            .batch
            .block_expr_expectation(batch_slot)
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)?;
        let semantic = match &self.dynamic {
            NormalCallableDynamicProjectionV1::Selected {
                batch_slot: dynamic_slot,
                completion,
                source,
                ..
            } if *dynamic_slot == batch_slot => {
                SelectedCallableSemanticRefV1::Dynamic { completion, source }
            }
            _ => SelectedCallableSemanticRefV1::Ordinary,
        };
        let lend = |result_contract: Option<CallableResultContractRefV1<'_>>| {
            self.batch
                .with_lowering_input_and_source_identity(batch_slot, |source, source_identity| {
                    if result_contract.is_some_and(|contract| contract.owner() != source.owner()) {
                        return Err(
                            NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                        );
                    }
                    let parameters = match parameter_declaration {
                        Some(declaration) => {
                            if declaration.owner != source.owner() {
                                return Err(
                                NormalCallableSemanticPackageInstallIssueV1::
                                    ParameterContractOwnerMismatch,
                            );
                            }
                            declaration.parameters.as_ref()
                        }
                        None => &[],
                    };
                    Ok(callback(SelectedCallableLoweringInputRefV1 {
                        source,
                        parameter_contracts: parameters,
                        block_expr_expectation,
                        result_contract,
                        semantic,
                        source_identity,
                        selected_key,
                    }))
                })
                .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)?
        };
        if let NormalCallableDynamicProjectionV1::Selected {
            batch_slot: dynamic_slot,
            completion,
            result,
            ..
        } = &self.dynamic
        {
            if *dynamic_slot == batch_slot {
                // This completion has no package-retained per-exit relation map.
                let terminal_relations = std::collections::BTreeMap::new();
                let contract = CallableResultContractRefV1::from_completion(
                    completion.owner(), row.identity(), row.role(), *result,
                    completion, &terminal_relations, None,
                );
                return lend(Some(contract));
            }
        }
        let result_contract = match key {
            SelectedNormalCallableKeyV1::Cataloged(_) if self.selected.is_main_child_key(key) => {
                None
            }
            SelectedNormalCallableKeyV1::Cataloged(_) => {
                let row = self.result_contracts.row(batch_slot).ok_or(
                    NormalCallableSemanticPackageInstallIssueV1::ResultContractUnavailable,
                )?;
                Some(row.borrow())
            }
            SelectedNormalCallableKeyV1::TopLevel(_) => None,
        };
        lend(result_contract)
    }
}
