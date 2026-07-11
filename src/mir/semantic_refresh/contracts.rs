use crate::mir::MirModule;

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
    pub record_values: usize,
    pub static_tables: usize,
    pub array_writes: usize,
    pub typed_arrays: usize,
    pub weak_fields: usize,
}

impl ContractCarrierSummary {
    pub fn total(self) -> usize {
        self.box_field_writes
            + self.parameter_entries
            + self.return_exits
            + self.local_slots
            + self.record_values
            + self.static_tables
            + self.array_writes
            + self.typed_arrays
            + self.weak_fields
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

pub struct OwnedRefreshedContractBundle {
    module: MirModule,
    boundary: ContractRefreshBoundary,
    carriers: ContractCarrierSummary,
}

impl OwnedRefreshedContractBundle {
    pub fn module(&self) -> &MirModule {
        &self.module
    }

    pub fn boundary(&self) -> ContractRefreshBoundary {
        self.boundary
    }

    pub fn carriers(&self) -> ContractCarrierSummary {
        self.carriers
    }
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
    refresh_active_contract_carriers(module)?;
    validate_refreshed_contracts(module)?;
    let carriers = collect_carrier_summary(module);
    Ok(RefreshedContractBundle {
        module,
        boundary,
        carriers,
    })
}

fn refresh_active_contract_carriers(module: &mut MirModule) -> Result<(), String> {
    crate::mir::type_contracts::weak_field::refresh_module_specs(module)?;
    let weak_specs = module.metadata.weak_field_contract_specs.clone();
    let record_decls = module.metadata.record_decls.clone();
    for function in module.functions.values_mut() {
        crate::mir::type_contracts::weak_field::canonicalize_function(function, &weak_specs)?;
        crate::mir::type_contracts::weak_field::refresh_function(function, &weak_specs)?;
        crate::mir::type_contracts::typed_array::refresh_source_rows(function)
            .map_err(|reason| carrier_validation_error("typed_array", function, reason))?;
        crate::mir::array_element_write::canonicalize_legacy_array_write_calls(function)
            .map_err(|reason| carrier_validation_error("array_write", function, reason))?;
        crate::mir::array_element_write::refresh_function_array_write_witnesses(function)
            .map_err(|reason| carrier_validation_error("array_write", function, reason))?;
        crate::mir::type_contracts::typed_array::refresh_function(function)
            .map_err(|reason| carrier_validation_error("typed_array", function, reason))?;
        crate::mir::type_contracts::parameter_entry::refresh_function_parameter_entry_contracts(
            function,
        );
        crate::mir::type_contracts::return_exit::refresh_function_return_exit_contract(function);
        crate::mir::type_contracts::local_slot::refresh_function_local_identity_evidence(function);
        // Rebuild from canonical operations plus module-owned declaration truth.
        // Validation below reports the same stable family boundary on failure.
        crate::mir::type_contracts::record_value::refresh_function_record_value_contracts(
            function,
            &record_decls,
        )
        .map_err(|reason| carrier_validation_error("record_value", function, reason))?;
    }
    crate::mir::exact_numeric_field_contracts::refresh_module_exact_numeric_runtime_check_contracts(
        module,
    );
    crate::mir::type_contracts::static_table::refresh_module_static_table_contracts(module)?;
    Ok(())
}

/// Compatibility bridge for immutable public APIs. The cloned module is
/// refreshed by the same owner and remains owned by the returned bundle.
pub fn refresh_owned_for_boundary(
    module: &MirModule,
    boundary: ContractRefreshBoundary,
) -> Result<OwnedRefreshedContractBundle, String> {
    let mut refreshed = module.clone();
    let bundle = refresh_and_validate_for_boundary(&mut refreshed, boundary)?;
    let carriers = bundle.carriers();
    Ok(OwnedRefreshedContractBundle {
        module: refreshed,
        boundary,
        carriers,
    })
}

fn validate_refreshed_contracts(module: &MirModule) -> Result<(), String> {
    crate::mir::extern_call_route_plan::validate_semantic_outcome_routes(module)?;
    crate::mir::type_contracts::weak_field::validate_module_specs(module)?;
    validate_box_field_contracts(module)?;
    crate::mir::type_contracts::static_table::validate_static_table_contracts(module)?;
    for function in module.functions.values() {
        crate::mir::type_contracts::weak_field::validate_function(
            function,
            &module.metadata.weak_field_contract_specs,
        )?;
        crate::mir::array_element_write::validate_function_array_write_witnesses(function)
            .map_err(|reason| carrier_validation_error("array_write", function, reason))?;
        crate::mir::type_contracts::typed_array::validate_function(function)
            .map_err(|reason| carrier_validation_error("typed_array", function, reason))?;
        crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts(function)
            .map_err(|reason| carrier_validation_error("parameter_entry", function, reason))?;
        crate::mir::type_contracts::return_exit::validate_return_exit_contract(function)
            .map_err(|reason| carrier_validation_error("return_exit", function, reason))?;
        crate::mir::type_contracts::local_slot::validate_local_slot_contracts(function)
            .map_err(|reason| carrier_validation_error("local_slot", function, reason))?;
        crate::mir::type_contracts::record_value::validate_record_value_contracts(
            function,
            &module.metadata.record_decls,
        )
        .map_err(|reason| carrier_validation_error("record_value", function, reason))?;
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
        record_values: module
            .functions
            .values()
            .map(|function| function.metadata.record_value_contracts.len())
            .sum(),
        static_tables: module.metadata.verified_static_table_contracts.len(),
        array_writes: module
            .functions
            .values()
            .map(|function| function.metadata.array_element_write_witnesses.len())
            .sum(),
        typed_arrays: module
            .functions
            .values()
            .map(|function| function.metadata.typed_array_element_contracts.len())
            .sum(),
        weak_fields: module.metadata.weak_field_contract_specs.len(),
    }
}

#[cfg(test)]
mod tests;
