//! Independent physical String lifetimes. No source admission or type inference.
//! Owned pairs may be lent only at source-proved Invoke argument slots; End reverses actual Normal
//! acquisition order, including every later Fault prefix.
use crate::mir::instruction::{
    InvokeNormalResultKind, InvokeOperation, StringInvokeOperand as Operand,
    StringInvokeOperation as Op,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Structural role/lifetime checks are also used before the final source
/// handoff. Module verification separately demands every exact Call occurrence.
pub(super) fn check_source_uses(
    function: &MirFunction,
    original: impl Fn(BasicBlockId, usize, usize, ValueId) -> bool,
) -> Result<(), &'static str> {
    let owned: std::collections::BTreeSet<_> = function.blocks.values().flat_map(|block| block.all_instructions())
        .filter_map(|row| match row {
            MirInstruction::InvokeNormalResult { dst, invoke_block }
                if function.blocks.get(invoke_block).and_then(|block| block.terminator.as_ref()).is_some_and(|row|
                    matches!(row, MirInstruction::Invoke { operation, .. } if operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString))) => Some(*dst),
            _ => None,
        }).collect();
    for (block, body) in &function.blocks {
        for (index, row) in body.all_instructions().enumerate() {
            let call = match row {
                MirInstruction::Call(call)
                | MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, .. },
                    ..
                } => call,
                _ => continue,
            };
            for (ordinal, value) in call.args.iter().enumerate() {
                if owned.contains(value) && !original(*block, index, ordinal, *value) {
                    return Err("string-original-borrow-slot-missing");
                }
            }
        }
    }
    Ok(())
}

fn is_call_argument(instruction: &MirInstruction, value: ValueId) -> bool {
    matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }
        if call.args.contains(&value) && call.callee.receiver() != Some(value)
            && matches!(call.callee, crate::mir::Callee::Global(_) | crate::mir::Callee::SameModuleInstance { .. }))
}

fn text_operands(operation: &InvokeOperation) -> Vec<&Operand> {
    match operation {
        InvokeOperation::String(operation) => operation.operands(),
        InvokeOperation::Birth(call) => call
            .arguments
            .iter()
            .filter_map(|argument| match argument {
                crate::mir::instruction::BirthArgumentV1::Text(operand) => Some(operand),
                _ => None,
            })
            .collect(),
        InvokeOperation::ArrayElementWrite {
            value: crate::mir::instruction::ArrayElementWriteValueV1::Text(value),
            ..
        } => vec![value],
        InvokeOperation::Map(
            crate::mir::instruction::MapInvokeOperation::FieldHas { key, .. }
            | crate::mir::instruction::MapInvokeOperation::FieldGetIndexed { key, .. }
            | crate::mir::instruction::MapInvokeOperation::FieldInstallIndexed { key, .. }
            | crate::mir::instruction::MapInvokeOperation::FieldDeleteIndexed { key, .. },
        ) => vec![key],
        _ => Vec::new(),
    }
}

fn is_tagged_dynamic_formal(function: &MirFunction, value: ValueId) -> bool {
    let Some(index) = function
        .params
        .iter()
        .position(|parameter| *parameter == value)
    else {
        return false;
    };
    function.metadata.physical_param_carriers.as_deref()
        .and_then(|carriers| carriers.get(index))
        == Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue)
}

#[path = "invoke_string_lifetime.rs"]
mod lifetime;
#[path = "invoke_string_source.rs"]
mod source;
pub(super) use lifetime::read::{check_available, has_scalar_read_origin};

pub(super) fn check(function: &MirFunction) -> Result<(), &'static str> {
    check_with_geometry(&super::FunctionVerificationGeometry::new(function))
}

pub(super) fn check_with_geometry(
    function: &super::FunctionVerificationGeometry<'_>,
) -> Result<(), &'static str> {
    lifetime::check(function, None).map_err(|(_, reason, _)| reason)
}

pub(super) fn check_module_with_source(
    module: &crate::mir::MirModule,
    root: Option<&crate::mir::finalized_root_handoff::FinalizedRootHandoffV1>,
    geometries: &super::ModuleGeometry<'_>,
) -> Result<(), Vec<crate::mir::verification_types::VerificationError>> {
    let images = source::images_for_module(module, root).map_err(|reason| {
        vec![super::error(
            module
                .functions
                .values()
                .next()
                .map(|function| function.entry_block)
                .unwrap_or(BasicBlockId(0)),
            &reason,
        )]
    })?;
    let mut errors = Vec::new();
    for function in module.functions.values() {
        if let Err((block, reason, outstanding)) = lifetime::check(
            &geometries[&(function as *const MirFunction)],
            images.get(&function.signature.name),
        ) {
            let mut failure = super::error(block, reason);
            if let crate::mir::verification_types::VerificationError::ControlFlowError {
                reason,
                ..
            } = &mut failure
            {
                reason.push_str(&format!(" function={}", function.signature.name));
                if let Some(outstanding) = outstanding {
                    reason.push_str(&format!(" {outstanding}"));
                }
                match function
                    .blocks
                    .get(&block)
                    .and_then(|body| body.terminator.as_ref())
                {
                    Some(MirInstruction::Return { value }) => {
                        reason.push_str(&format!(" terminator=Return({value:?})"))
                    }
                    Some(MirInstruction::ReturnFault { .. }) => {
                        reason.push_str(" terminator=ReturnFault")
                    }
                    _ => {}
                }
            }
            errors.push(failure);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Physical evidence only: source/domain/use closure is checked independently.
/// A readonly descriptor must retain its exact marked formal through Copies.
fn check_borrowed_operand(
    function: &super::FunctionVerificationGeometry<'_>,
    mut value: ValueId,
) -> Result<(), &'static str> {
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    let mut seen = std::collections::BTreeSet::new();
    loop {
        if !seen.insert(value) {
            return Err("string-borrowed-copy-cycle");
        }
        let parameter = function.params.iter().position(|old| *old == value);
        if let Some(index) = parameter {
            if function.ambiguous(value)
                || function
                    .metadata
                    .physical_param_carriers
                    .as_deref()
                    .and_then(|rows| rows.get(index))
                    != Some(&Carrier::BorrowedTaggedValue)
                || function
                    .metadata
                    .declared_param_decls
                    .get(index)
                    .is_none_or(|row| row.implicit_receiver)
                || !matches!(
                    function.signature.params.get(index),
                    Some(crate::mir::MirType::Integer | crate::mir::MirType::Unknown)
                )
            {
                return Err("string-borrowed-formal");
            }
            return Ok(());
        }
        let Some((_, _, MirInstruction::Copy { src, .. })) = function.definition(value) else {
            return Err("string-borrowed-origin");
        };
        if function.ambiguous(value) {
            return Err("string-borrowed-copy-duplicate");
        }
        value = *src;
    }
}

#[cfg(test)]
#[path = "invoke_string_tests.rs"]
mod tests;
