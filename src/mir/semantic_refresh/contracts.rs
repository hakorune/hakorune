use crate::mir::MirModule;

use super::refresh_module_semantic_metadata;

pub const CONTRACT_REFRESH_REBUILD_FAILED_TAG: &str = "[type/contract_refresh_rebuild_failed]";
pub const CONTRACT_CARRIER_MISSING_AFTER_REFRESH_TAG: &str =
    "[type/contract_carrier_missing_after_refresh]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractRefreshBoundary {
    Verifier,
    MirJsonExport,
    VmExecution,
    BackendPreflight,
    ToolDirectVerify,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ContractCarrierSummary {
    pub box_field_writes: usize,
    pub parameter_entries: usize,
    pub return_exits: usize,
    pub local_slots: usize,
}

impl ContractCarrierSummary {
    pub fn total(self) -> usize {
        self.box_field_writes + self.parameter_entries + self.return_exits + self.local_slots
    }
}

/// Boundary-scoped proof that semantic contract carriers were rebuilt and
/// validated against the current module. Fields stay private so consumers
/// cannot manufacture a successful refresh locally.
pub struct RefreshedContractBundle<'a> {
    module: &'a MirModule,
    boundary: ContractRefreshBoundary,
    carriers: ContractCarrierSummary,
}

impl<'a> RefreshedContractBundle<'a> {
    pub fn module(&self) -> &'a MirModule {
        self.module
    }

    pub fn boundary(&self) -> ContractRefreshBoundary {
        self.boundary
    }

    pub fn carriers(&self) -> ContractCarrierSummary {
        self.carriers
    }
}

/// Sole public owner for rebuilding semantic contract carriers before a
/// verifier, export, execution, or backend boundary consumes the module.
pub fn refresh_and_validate_for_boundary(
    module: &mut MirModule,
    boundary: ContractRefreshBoundary,
) -> Result<RefreshedContractBundle<'_>, String> {
    refresh_module_semantic_metadata(module);
    validate_refreshed_contracts(module)?;
    let carriers = collect_carrier_summary(module);
    Ok(RefreshedContractBundle {
        module,
        boundary,
        carriers,
    })
}

fn validate_refreshed_contracts(module: &MirModule) -> Result<(), String> {
    validate_box_field_contracts(module)?;
    for function in module.functions.values() {
        crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts(function)
            .map_err(|reason| carrier_validation_error("parameter_entry", function, reason))?;
        crate::mir::type_contracts::return_exit::validate_return_exit_contract(function)
            .map_err(|reason| carrier_validation_error("return_exit", function, reason))?;
        crate::mir::type_contracts::local_slot::validate_local_slot_contracts(function)
            .map_err(|reason| carrier_validation_error("local_slot", function, reason))?;
    }
    Ok(())
}

fn carrier_validation_error(
    family: &str,
    function: &crate::mir::MirFunction,
    reason: String,
) -> String {
    format!(
        "{} family={} function={} reason={}",
        CONTRACT_CARRIER_MISSING_AFTER_REFRESH_TAG, family, function.signature.name, reason
    )
}

fn validate_box_field_contracts(module: &MirModule) -> Result<(), String> {
    let findings =
        crate::mir::exact_numeric_field_contracts::collect_exact_numeric_field_assignment_findings(
            module,
        );
    if findings.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} family=box_field findings={}",
        CONTRACT_REFRESH_REBUILD_FAILED_TAG,
        findings.len()
    ))
}

fn collect_carrier_summary(module: &MirModule) -> ContractCarrierSummary {
    ContractCarrierSummary {
        box_field_writes: module
            .functions
            .values()
            .map(|function| {
                function.metadata.exact_numeric_field_contract_proofs.len()
                    + function
                        .metadata
                        .exact_numeric_runtime_check_contracts
                        .len()
            })
            .sum(),
        parameter_entries: module
            .functions
            .values()
            .map(|function| function.metadata.parameter_entry_contracts.len())
            .sum(),
        return_exits: module
            .functions
            .values()
            .filter(|function| function.metadata.return_exit_contract.is_some())
            .count(),
        local_slots: module
            .functions
            .values()
            .map(|function| function.metadata.local_slot_contracts.len())
            .sum(),
    }
}

#[cfg(test)]
mod tests;
