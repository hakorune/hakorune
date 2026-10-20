//! Source-issued Array writes joined to exact final function coordinates.
use super::*;
use crate::mir::{ArrayElementWriteKind, ArrayWriteProducerKind, BasicBlockId, ConstValue};

type Coordinate = (u32, BasicBlockId, u32);

pub(super) fn issue<'module>(
    view: &PublishedMirBackendView<'module>,
    program: &PublishedLifecyclePhysicalProgramV1<'module>,
) -> Result<(Vec<CompiledEntryArrayClaimV1<'module>>, Vec<CompiledEntryArrayWriteV1>), String> {
    let mut claims = Vec::new();
    let mut writes = Vec::new();
    if let Some(script) = program.handoff().script_array() {
        for (array, contract_id, spec) in script.claims() {
            claims.push(CompiledEntryArrayClaimV1 { array, contract_id, spec });
        }
        for (site, array, value, definition) in script.writes() {
            let MirInstruction::Const { dst, value: constant } = definition else {
                return Err(fault("compiled-entry-array-definition"));
            };
            if *dst != value { return Err(fault("compiled-entry-array-value")); }
            let kind = match constant {
                ConstValue::Integer(_) => CompiledEntryArrayValueKindV1::I64,
                ConstValue::Bool(_) => CompiledEntryArrayValueKindV1::Bool,
                ConstValue::Float(_) => CompiledEntryArrayValueKindV1::F64,
                _ => return Err(fault("compiled-entry-array-representation")),
            };
            let coordinate = locate(program, |_, _, row| matches!(row,
                MirInstruction::Invoke { operation: InvokeOperation::ArrayElementWrite {
                    site_id, kind: ArrayElementWriteKind::LiteralAppend,
                    producer: ArrayWriteProducerKind::Literal, receiver, index: None, value: operand,
                }, .. } if *site_id == site && *receiver == array && operand.scalar() == Some(value)))?;
            writes.push(CompiledEntryArrayWriteV1 { function_index: coordinate.0, block: coordinate.1,
                instruction_index: coordinate.2, site, array, value: CompiledEntryArrayWriteValueV1::Scalar { value, kind } });
        }
    } else if let Some(source) = view.retained_root_source() {
        for (function_index, physical) in program.functions().iter().enumerate() {
            let function = view.module().functions.get(physical.name())
                .ok_or_else(|| fault("compiled-entry-field-array-function"))?;
            for binding in source.finished_field_array_appends_v1(view.module(), function, view.validated_named_arrays()?)? {
                let MirInstruction::Invoke { operation: InvokeOperation::ArrayElementWrite {
                    site_id, kind: ArrayElementWriteKind::Push, producer: ArrayWriteProducerKind::MethodCall,
                    receiver, index: None, value,
                }, .. } = &binding.1 else { return Err(fault("compiled-entry-field-array-shape")); };
                let coordinate = locate(program, |index, block, row|
                    index as usize == function_index && block == binding.0 && row == &binding.1)?;
                // The same finalized packet above independently checks its
                // original Integer expression or Text evaluation/descriptor.
                // Opcode shape alone never issues this source-bound payload.
                let value = match value {
                    crate::mir::instruction::ArrayElementWriteValueV1::Scalar(value) =>
                        CompiledEntryArrayWriteValueV1::Scalar { value: *value, kind: CompiledEntryArrayValueKindV1::I64 },
                    crate::mir::instruction::ArrayElementWriteValueV1::Text(value) if value.is_text() =>
                        CompiledEntryArrayWriteValueV1::Text(value.clone()),
                    _ => return Err(fault("compiled-entry-field-array-shape")),
                };
                writes.push(CompiledEntryArrayWriteV1 { function_index: coordinate.0, block: coordinate.1,
                    instruction_index: coordinate.2, site: *site_id, array: *receiver, value });
            }
        }
    }
    let mut coordinates = BTreeSet::new();
    for row in &writes {
        if !coordinates.insert((row.function_index, row.block, row.instruction_index)) {
            return Err(fault("compiled-entry-array-duplicate-coordinate"));
        }
    }
    Ok((claims, writes))
}

fn locate(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
    mut matches: impl FnMut(u32, BasicBlockId, &MirInstruction) -> bool,
) -> Result<Coordinate, String> {
    let mut found = None;
    for (index, function) in program.functions().iter().enumerate() {
        let index = u32::try_from(index).map_err(|_| fault("compiled-entry-array-function-index"))?;
        for block in function.blocks() {
            for row in block.instructions().iter().copied().chain(std::iter::once(block.terminator())) {
                if matches(index, block.id(), row.instruction()) {
                    if found.replace((index, block.id(), row.index())).is_some() {
                        return Err(fault("compiled-entry-array-coordinate-duplicate"));
                    }
                }
            }
        }
    }
    found.ok_or_else(|| fault("compiled-entry-array-coordinate-missing"))
}
