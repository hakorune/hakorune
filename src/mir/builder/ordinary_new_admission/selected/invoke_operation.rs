//! Shared physical Invoke/Normal writer; source selection stays with its caller.
use super::*;
/// One Invoke layout for both current-block writes and prepared root packets.
/// The source owner supplies the selected operation and both continuations.
pub(in crate::mir::builder) fn instruction(
    operation: InvokeOperation,
    frame: ValueId,
    normal: BasicBlockId,
    fault: BasicBlockId,
) -> MirInstruction {
    MirInstruction::Invoke {
        operation,
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: fault,
    }
}

pub(in crate::mir::builder) fn emit_control_to_landing_v1(
    builder: &mut MirBuilder,
    writer: &mut RawRootGraphWriterV1,
    operation: InvokeOperation,
    frame: ValueId,
    normal: BasicBlockId,
    fault: BasicBlockId,
) -> Result<(BasicBlockId, MirInstruction), String> {
    writer.emit_current(builder, instruction(operation, frame, normal, fault))
}

pub(in crate::mir::builder) fn emit(
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
    let invoke = emit_control_to_landing_v1(
        builder,
        &mut RawRootGraphWriterV1,
        operation,
        frame,
        normal,
        fault,
    )?;
    bindings.push(invoke);
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
