//! Physical write boundary for the root exit graph.
//! The raw implementation preserves the current block and binding order.
use super::*;

pub(in crate::mir::builder) trait RootGraphWriterPortV1 {
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String>;
    fn append_terminated(
        &mut self,
        builder: &mut MirBuilder,
        id: BasicBlockId,
        terminator: MirInstruction,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String>;
    fn emit_current(
        &mut self,
        builder: &mut MirBuilder,
        instruction: MirInstruction,
    ) -> Result<(BasicBlockId, MirInstruction), String>;
    fn insert_projection(
        &mut self,
        builder: &mut MirBuilder,
        landing: BasicBlockId,
        projection: MirInstruction,
    ) -> Result<(), String>;
}

pub(super) struct RawRootGraphWriterV1;

impl RootGraphWriterPortV1 for RawRootGraphWriterV1 {
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String> {
        Ok(builder.next_block_id())
    }

    fn append_terminated(
        &mut self,
        builder: &mut MirBuilder,
        id: BasicBlockId,
        terminator: MirInstruction,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        if !matches!(
            terminator,
            MirInstruction::Return { .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
                | MirInstruction::Invoke { .. }
        ) {
            return Err(freeze("root-graph-terminator-unsupported"));
        }
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| freeze("no-function"))?;
        if function.blocks.contains_key(&id) {
            return Err(freeze("duplicate-block"));
        }
        let mut block = BasicBlock::new(id);
        block.set_terminator(terminator.clone());
        function.add_block(block);
        bindings.push((id, terminator));
        Ok(())
    }

    fn emit_current(
        &mut self,
        builder: &mut MirBuilder,
        instruction: MirInstruction,
    ) -> Result<(BasicBlockId, MirInstruction), String> {
        if !matches!(
            instruction,
            MirInstruction::Jump {
                edge_args: None,
                ..
            } | MirInstruction::Invoke { .. }
        ) {
            return Err(freeze("root-graph-current-unsupported"));
        }
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| freeze("no-block"))?;
        builder.emit_instruction(instruction.clone())?;
        Ok((block, instruction))
    }

    fn insert_projection(
        &mut self,
        builder: &mut MirBuilder,
        landing: BasicBlockId,
        projection: MirInstruction,
    ) -> Result<(), String> {
        if !matches!(projection, MirInstruction::InvokeNormalResult { .. }) {
            return Err(freeze("root-graph-projection-unsupported"));
        }
        builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| freeze("no-function"))?
            .blocks
            .get_mut(&landing)
            .ok_or_else(|| freeze("no-normal-landing"))?
            .add_instruction(projection);
        Ok(())
    }
}

/// One physical cleanup algorithm for a caller-owned clean continuation. The
/// root caller supplies a Return block; a body expression supplies its next
/// block. This port neither infers Home obligations nor manufactures origins.
pub(super) struct CleanupSuffixGraphV1 {
    pub(super) clean: BasicBlockId,
    pub(super) fault_terminal: BasicBlockId,
    pub(super) acquisition_fault: BasicBlockId,
    pub(super) normal_origins: Vec<(BasicBlockId, MirInstruction)>,
}

pub(super) fn emit_cleanup_suffixes_with_writer(
    writer: &mut impl RootGraphWriterPortV1,
    builder: &mut MirBuilder,
    frame: ValueId,
    mut clean: BasicBlockId,
    normal_operations: &[InvokeOperation],
    fault_operations: &[InvokeOperation],
    after_normal: &[Vec<usize>],
    acquisition: &[usize],
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<CleanupSuffixGraphV1, String> {
    if after_normal.len() != normal_operations.len()
        || after_normal
            .iter()
            .flatten()
            .chain(acquisition)
            .any(|index| *index >= fault_operations.len())
    {
        return Err(freeze("root-fault-sequence-shape"));
    }
    let fault_terminal = writer.reserve_block(builder)?;
    writer.append_terminated(
        builder,
        fault_terminal,
        MirInstruction::ReturnFault { fault_frame: frame },
        bindings,
    )?;
    let mut fault_nodes = std::collections::BTreeMap::new();
    fault_nodes.insert(Vec::<usize>::new(), fault_terminal);
    let mut normal_origins = Vec::with_capacity(normal_operations.len());
    for (index, operation) in normal_operations.iter().enumerate().rev() {
        let fault = emit_pending_root_fault_sequence(
            writer,
            builder,
            frame,
            fault_operations,
            &after_normal[index],
            &mut fault_nodes,
            bindings,
        )?;
        clean = cleanup_step_with_writer(
            writer,
            builder,
            frame,
            operation.clone(),
            clean,
            fault,
            bindings,
        )?;
        normal_origins.push(
            bindings
                .last()
                .cloned()
                .ok_or_else(|| freeze("root-home-release-binding-missing"))?,
        );
    }
    let acquisition_fault = emit_pending_root_fault_sequence(
        writer,
        builder,
        frame,
        fault_operations,
        acquisition,
        &mut fault_nodes,
        bindings,
    )?;
    normal_origins.reverse();
    Ok(CleanupSuffixGraphV1 {
        clean,
        fault_terminal,
        acquisition_fault,
        normal_origins,
    })
}

/// Share only exact original-origin suffixes within this emission.
fn emit_pending_root_fault_sequence(
    writer: &mut impl RootGraphWriterPortV1,
    builder: &mut MirBuilder,
    frame: ValueId,
    full: &[InvokeOperation],
    sequence: &[usize],
    nodes: &mut std::collections::BTreeMap<Vec<usize>, crate::mir::BasicBlockId>,
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
) -> Result<crate::mir::BasicBlockId, String> {
    let mut next = *nodes
        .get(&Vec::new())
        .ok_or_else(|| freeze("root-fault-terminal-missing"))?;
    for offset in (0..sequence.len()).rev() {
        let suffix = &sequence[offset..];
        if let Some(existing) = nodes.get(suffix) {
            next = *existing;
            continue;
        }
        let origin = full
            .get(sequence[offset])
            .ok_or_else(|| freeze("root-fault-origin-missing"))?;
        next =
            cleanup_step_with_writer(writer, builder, frame, origin.clone(), next, next, bindings)?;
        nodes.insert(suffix.to_vec(), next);
    }
    Ok(next)
}

pub(super) fn cleanup_step_with_writer(
    writer: &mut impl RootGraphWriterPortV1,
    builder: &mut MirBuilder,
    frame: ValueId,
    operation: InvokeOperation,
    normal_next: BasicBlockId,
    fault_next: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    let origin = writer.reserve_block(builder)?;
    let normal = writer.reserve_block(builder)?;
    let fault = writer.reserve_block(builder)?;
    for (landing, target) in [(normal, normal_next), (fault, fault_next)] {
        writer.append_terminated(
            builder,
            landing,
            MirInstruction::Jump {
                target,
                edge_args: None,
            },
            bindings,
        )?;
    }
    writer.append_terminated(
        builder,
        origin,
        MirInstruction::Invoke {
            operation,
            fault_frame: frame,
            normal_landing: normal,
            fault_landing: fault,
        },
        bindings,
    )?;
    Ok(origin)
}
