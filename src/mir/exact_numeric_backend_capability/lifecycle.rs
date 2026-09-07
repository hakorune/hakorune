//! Selected V4 numeric capability consumes the already-completed physical input.
//! Coordinates and canonical fields bind obligations; diagnostic names never
//! select declarations. Generic backend capability remains independent.
use crate::mir::compiler::published_backend_view::{
    CompiledEntryFormalKindV1, PublishedLifecyclePhysicalAbiInputV1,
};
use crate::mir::function::{ExactNumericRuntimeCheckContractKind, TypedObjectFieldStorage};
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    BirthFormalDeclarationClassV1, BirthFormalUseCoverageV1,
};
use crate::mir::{MirInstruction, MirModule};
use std::collections::BTreeSet;

pub(super) fn enforce(
    module: &MirModule,
    input: &PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    // Compare borrowed rows, not a name-equivalent clone of a different module.
    for physical in input.program().functions() {
        let function = module
            .functions
            .get(physical.name())
            .ok_or_else(|| fault("foreign-function"))?;
        if !std::ptr::eq(physical.name(), function.signature.name.as_str())
            || physical.blocks().len() != function.blocks.len()
        {
            return Err(fault("foreign-input"));
        }
        for block in physical.blocks() {
            let original = function
                .blocks
                .get(&block.id())
                .ok_or_else(|| fault("foreign-block"))?;
            for row in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                let actual = original
                    .all_instructions()
                    .nth(row.index() as usize)
                    .ok_or_else(|| fault("foreign-instruction"))?;
                if !std::ptr::eq(actual, row.instruction()) {
                    return Err(fault("foreign-input"));
                }
            }
        }
    }
    for (name, function) in &module.functions {
        enforce_contracts(
            module,
            input,
            name,
            &function.metadata.exact_numeric_runtime_check_contracts,
        )?;
    }
    Ok(())
}

fn enforce_contracts(
    module: &MirModule,
    input: &PublishedLifecyclePhysicalAbiInputV1<'_>,
    name: &str,
    contracts: &[crate::mir::function::ExactNumericRuntimeCheckContract],
) -> Result<(), String> {
    let mut consumed = BTreeSet::new();
    for contract in contracts {
        if !consumed.insert((contract.block, contract.instruction_index)) {
            return Err(fault("duplicate-contract"));
        }
        let (ordinal, physical) = input
            .program()
            .functions()
            .iter()
            .enumerate()
            .find(|(_, p)| p.name() == name)
            .ok_or_else(|| fault("uncovered-function"))?;
        let block = physical
            .blocks()
            .iter()
            .find(|b| b.id() == contract.block)
            .ok_or_else(|| fault("uncovered-block"))?;
        let row = block
            .instructions()
            .iter()
            .copied()
            .chain(std::iter::once(block.terminator()))
            .find(|r| r.index() as usize == contract.instruction_index)
            .ok_or_else(|| fault("uncovered-instruction"))?;
        let MirInstruction::Invoke {
            operation: InvokeOperation::FieldSet { field, value, .. },
            ..
        } = row.instruction()
        else {
            return Err(fault("not-field-set"));
        };
        if contract.kind != ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
            || *value != contract.value
        {
            return Err(fault("contract-value"));
        }
        let declaration = module
            .canonical_field_definition(*field)
            .ok_or_else(|| fault("field-missing"))?;
        if declaration.name != contract.field
            || declaration.declared_type_name.as_deref()
                != Some(contract.declared_type_name.as_str())
            || declaration.declared_type_name.as_deref() != Some("i64")
        {
            return Err(fault("field-projection"));
        }
        let object = module
            .canonical_object_definition(field.object())
            .ok_or_else(|| fault("object-missing"))?;
        let issued_layout = object
            .runtime_layout()
            .ok_or_else(|| fault("layout-missing"))?
            .as_ref()
            .map_err(|_| fault("layout-unavailable"))?;
        let layout = input
            .layouts()
            .iter()
            .find(|l| l.object_id() == field.object().declaration_index())
            .ok_or_else(|| fault("uncovered-layout"))?;
        let stored = issued_layout
            .fields
            .get(field.declaration_ordinal() as usize)
            .ok_or_else(|| fault("field-layout-missing"))?;
        let lane = layout
            .fields()
            .iter()
            .find(|f| f.declaration_ordinal() == field.declaration_ordinal())
            .ok_or_else(|| fault("uncovered-field"))?;
        if layout.runtime_type_id() != issued_layout.type_id
            || layout.field_count() != issued_layout.field_count
            || lane.object_id() != layout.object_id()
            || lane.runtime_slot() != stored.slot
            || lane.storage_kind() != 1
            || stored.storage != TypedObjectFieldStorage::I64
        {
            return Err(fault("layout-drift"));
        }
        // Constants already use verifier proofs and do not reach this runtime
        // obligation. Here V4 consumes the retained tagged Birth parameter.
        let birth = input
            .entry()
            .births()
            .iter()
            .find(|b| b.function_index() as usize == ordinal)
            .ok_or_else(|| fault("not-birth"))?;
        let formal = birth
            .formals()
            .iter()
            .find(|f| f.value() == *value)
            .ok_or_else(|| fault("value-coverage-unavailable"))?;
        let source = formal
            .contract()
            .ok_or_else(|| fault("formal-contract-missing"))?;
        if formal.kind() != CompiledEntryFormalKindV1::Parameter
            || source.declaration() != BirthFormalDeclarationClassV1::Unannotated
            || !matches!(
                source.uses(),
                BirthFormalUseCoverageV1::I64FieldStores { .. }
            )
            || physical.params().get(formal.physical_ordinal() as usize) != Some(value)
        {
            return Err(fault("formal-coverage"));
        }
    }
    Ok(())
}

fn fault(reason: &str) -> String {
    format!("[freeze:backend][lifecycle-numeric/{reason}]")
}

#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod tests;
