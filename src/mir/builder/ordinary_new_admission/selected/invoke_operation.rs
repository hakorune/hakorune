//! Shared physical Invoke/Normal writer; source selection stays with its caller.
use super::*;
pub(super) fn emit(
    builder: &mut MirBuilder,
    frame: ValueId,
    operation: InvokeOperation,
    fault: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(Option<ValueId>, Option<(BasicBlockId, MirInstruction)>), String> {
    let value = operation
        .normal_result_kind()
        .map(|_| builder.next_value_id());
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal = builder.next_block_id();
    let instruction = MirInstruction::Invoke {
        operation,
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: fault,
    };
    builder.emit_instruction(instruction.clone())?;
    bindings.push((origin, instruction));
    builder.start_new_block(normal)?;
    let projection = value
        .map(|dst| {
            let projection = MirInstruction::InvokeNormalResult {
                invoke_block: origin,
                dst,
            };
            builder.emit_instruction(projection.clone())?;
            bindings.push((normal, projection.clone()));
            Ok::<(BasicBlockId, MirInstruction), String>((normal, projection))
        })
        .transpose()?;
    Ok((value, projection))
}
