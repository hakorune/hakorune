//! Realize an existing source-sealed newest-first residence inventory.
//! This child neither discovers ownership nor owns source acceptance.
use super::*;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{OwnedFieldChildKindV1, OwnedFieldChildV1};
use crate::mir::{BasicBlock, MirBuilder, MirInstruction};

pub(super) fn emit_discharge(
    builder: &mut MirBuilder,
    fields: &[OwnedFieldChildV1],
    base: ValueId,
    fault_frame: ValueId,
    tail: BasicBlockId,
) -> Result<BasicBlockId, String> {
    let mut head = tail;
    // Prepending reverses construction order, so walk the sealed execution
    // inventory backwards to execute its newest-first order unchanged.
    for field in fields.iter().rev() {
        let id = builder.next_block_id();
        let normal = jump_landing(builder, head)?;
        let fault_edge = jump_landing(builder, head)?;
        let operation = match field.kind {
            OwnedFieldChildKindV1::Array => InvokeOperation::OwnedFieldResidenceRelease {
                field: field.field,
                base,
            },
            OwnedFieldChildKindV1::Object(child) => InvokeOperation::OwnedObjectFieldRelease {
                field: field.field,
                base,
                child,
            },
        };
        let mut block = BasicBlock::new(id);
        block.set_terminator(MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing: normal,
            fault_landing: fault_edge,
        });
        builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| fault("no-function"))?
            .add_block(block);
        head = id;
    }
    Ok(head)
}

/// Invoke outcomes converge through distinct single-predecessor landings.
pub(super) fn jump_landing(
    builder: &mut MirBuilder,
    target: BasicBlockId,
) -> Result<BasicBlockId, String> {
    let id = builder.next_block_id();
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| fault("no-function"))?;
    if function.blocks.contains_key(&id) {
        return Err(fault("duplicate-block"));
    }
    let mut block = BasicBlock::new(id);
    block.set_terminator(MirInstruction::Jump {
        target,
        edge_args: None,
    });
    function.add_block(block);
    Ok(id)
}
