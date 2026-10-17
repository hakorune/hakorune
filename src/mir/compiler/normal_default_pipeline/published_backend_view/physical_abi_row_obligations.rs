//! Runtime obligations attached to exact physical function/block/row coordinates.
//! These independent walks retain the physical ABI owner's ordering and faults.
use super::{
    fault, PublishedLifecycleCheckedOperationKindV1, PublishedLifecycleExactNumericCheckV1,
    PublishedLifecycleOperationDiagnosticSiteV1, PublishedLifecyclePhysicalProgramV1,
};
use crate::mir::MirInstruction;
use std::collections::BTreeSet;

/// Binds every issued dynamic-integer-range contract to its physical row.
/// Bare routed stores and canonical usize Invokes carry range obligations.
/// i64 Invoke contracts retain the existing separate tagged-type/Birth law.
pub(super) fn issue_exact_numeric_checks(
    module: &crate::mir::MirModule,
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<Vec<PublishedLifecycleExactNumericCheckV1>, String> {
    let mut checks = Vec::new();
    let mut coordinates = BTreeSet::new();
    for (name, function) in &module.functions {
        for contract in &function.metadata.exact_numeric_runtime_check_contracts {
            if contract.kind
                != crate::mir::function::ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
            {
                continue;
            }
            let (ordinal, physical) = program
                .functions()
                .iter()
                .enumerate()
                .find(|(_, p)| p.name() == name.as_str())
                .ok_or_else(|| fault("check-uncovered-function"))?;
            let block = physical
                .blocks()
                .iter()
                .find(|b| b.id() == contract.block)
                .ok_or_else(|| fault("check-uncovered-block"))?;
            let row = block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
                .find(|r| r.index() as usize == contract.instruction_index)
                .ok_or_else(|| fault("check-uncovered-instruction"))?;
            let (field, value) = match row.instruction() {
                MirInstruction::FieldSet { value, .. } =>
                    (row.field_ref().ok_or_else(|| fault("check-field-missing"))?, *value),
                MirInstruction::Invoke {
                    operation: crate::mir::instruction::InvokeOperation::FieldSet { field, value, .. }, ..
                } => {
                    if contract.declared_type_name != "usize" { continue; }
                    (*field, *value)
                },
                _ => return Err(fault("check-not-field-set")),
            };
            let declaration = module.canonical_field_definition(field)
                .ok_or_else(|| fault("check-field-missing"))?;
            if value != contract.value || declaration.name != contract.field
                || declaration.declared_type_name.as_deref() != Some(contract.declared_type_name.as_str())
            { return Err(fault("check-field-contract")); }
            let function_ordinal =
                u32::try_from(ordinal).map_err(|_| fault("check-function-overflow"))?;
            if !coordinates.insert((function_ordinal, contract.block.as_u32(), row.index())) {
                return Err(fault("check-coordinate-duplicate"));
            }
            checks.push(PublishedLifecycleExactNumericCheckV1 {
                function: function_ordinal,
                block: contract.block.as_u32(),
                instruction: row.index(),
                declared_type_name: contract.declared_type_name.clone(),
            });
        }
    }
    Ok(checks)
}

pub(super) fn issue_diagnostic_sites(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<Vec<PublishedLifecycleOperationDiagnosticSiteV1>, String> {
    let mut sites = Vec::new();
    let mut coordinates = BTreeSet::new();
    for (function_ordinal, physical_function) in program.functions().iter().enumerate() {
        let function =
            u32::try_from(function_ordinal).map_err(|_| fault("site-function-overflow"))?;
        for block in physical_function.blocks() {
            for row in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                let Some(kind) =
                    PublishedLifecycleCheckedOperationKindV1::from_instruction(row.instruction())
                else {
                    continue;
                };
                let coordinate = (function, block.id().0, row.index());
                if !coordinates.insert(coordinate) {
                    return Err(fault("site-coordinate-duplicate"));
                }
                let site = u64::try_from(sites.len()).map_err(|_| fault("site-overflow"))?;
                sites.push(PublishedLifecycleOperationDiagnosticSiteV1 {
                    function,
                    block: block.id().0,
                    instruction: row.index(),
                    kind,
                    site,
                });
            }
        }
    }
    // Ordinary Calls may require this ABI without any checked operation.
    // The root epilogue still owns process-result site zero in that case.
    Ok(sites)
}
