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
    let reachable = program.reachable_symbols_v1(module);
    for (name, function) in &module.functions {
        for contract in &function.metadata.exact_numeric_runtime_check_contracts {
            if contract.kind
                != crate::mir::function::ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
            {
                continue;
            }
            // A function no resolved call chain reaches from the Root AND with no
            // physical function has nothing to run; its contracts are inert. A
            // physical function keeps its checks even if no edge reaches it.
            // Unresolved calls leave `reachable` empty and keep today's check.
            if reachable.as_ref().is_some_and(|reached| !reached.contains(name.as_str()))
                && !program.functions().iter().any(|physical| physical.name() == name.as_str())
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

/// The original take declaration owns this attribute; neither a type name nor
/// the one-word carrier can grant ownership on its own.
pub(crate) fn take_parameter_objects_v1(
    function: &super::super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    input: Option<&super::PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<std::collections::BTreeMap<usize, hakorune_mir_defs::CanonicalObjectIdV1>, String> {
    let Some(input) = input else { return Ok(std::collections::BTreeMap::new()); };
    let handoff = input.program().handoff();
    let Some(source) = handoff.root_source() else { return Ok(std::collections::BTreeMap::new()); };
    let take_owners = source.take_entry_owners_v1();
    if take_owners.is_empty() { return Ok(std::collections::BTreeMap::new()); }
    let Some(key) = function.role().ordinary_target() else { return Ok(std::collections::BTreeMap::new()); };
    let owner = handoff.callables().and_then(|catalog| catalog.owner_for_canonical_key(key))
        .ok_or_else(|| fault("borrowed-carrier-source-missing"))?;
    if !take_owners.contains(&owner) { return Ok(std::collections::BTreeMap::new()); }
    let (_, owned) = source.move_parameter_abi_v1(owner, key, function.role().receiver_object(), function.params().len())?;
    Ok(owned)
}
