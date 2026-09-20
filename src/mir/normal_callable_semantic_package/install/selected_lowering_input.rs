use crate::mir::builder::SelectedNormalCallableKeyV1;

use super::{
    CallablePhysicalHeaderRefV1, CallableResultContractRefV1,
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
        let batch_slot = self
            .selected
            .batch_slot(key)
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)?;
        // Reborrow the package's canonical key instead of trusting a
        // caller-owned spelling. The batch-slot check above proves
        // membership; this clone preserves that exact catalog identity for
        // the later physical-header projection.
        let selected_key = self
            .selected
            .keys()
            .find(|candidate| *candidate == key)
            .cloned()
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)?;
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
                program,
                source,
                ..
            } if *dynamic_slot == batch_slot => {
                SelectedCallableSemanticRefV1::Dynamic { program, source }
            }
            _ => SelectedCallableSemanticRefV1::Ordinary,
        };
        let lend = |result_contract: Option<CallableResultContractRefV1<'_>>,
                    physical_header: Option<CallablePhysicalHeaderRefV1<'_>>| {
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
                        physical_header,
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
            program,
            result,
            ..
        } = &self.dynamic
        {
            if *dynamic_slot == batch_slot {
                let identity = self
                    .selected
                    .identity_for_batch_slot(batch_slot)
                    .ok_or(NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch)?;
                let role = self
                    .selected
                    .role_for_batch_slot(batch_slot)
                    .ok_or(NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch)?;
                return program.with_canonical_session_authority(|authority| {
                    let contract = CallableResultContractRefV1::from_completion(
                        authority.completion().owner(),
                        identity,
                        role,
                        *result,
                        authority.completion(),
                        None,
                    );
                    let header = CallablePhysicalHeaderRefV1::from_result_contract(contract);
                    lend(Some(contract), header)
                });
            }
        }
        let result_contract = match key {
            // The S6C child consumed this row's Completion seed exclusively
            // before the generic retention cohort was assembled. Its selected
            // view remains valid, but the generic result contract is absent by
            // construction; S6C owns the matching completion through its
            // child loan instead.
            SelectedNormalCallableKeyV1::Cataloged(_) if self.selected.is_main_child_key(key) => {
                None
            }
            SelectedNormalCallableKeyV1::Cataloged(_) => {
                let row = self.result_contracts.row(batch_slot).ok_or(
                    NormalCallableSemanticPackageInstallIssueV1::ResultContractUnavailable,
                )?;
                let Some(expected_identity) = self.selected.identity_for_batch_slot(batch_slot)
                else {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                    );
                };
                let Some(expected_role) = self.selected.role_for_batch_slot(batch_slot) else {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                    );
                };
                if !row.identity().same_as(expected_identity) || row.role() != expected_role {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                    );
                }
                Some(row.borrow())
            }
            SelectedNormalCallableKeyV1::TopLevel(_) => None,
        };
        let physical_header = self.physical_header.row(batch_slot, &self.result_contracts);
        lend(result_contract, physical_header)
    }
}
