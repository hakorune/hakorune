//! Physical write boundary for the root exit graph.
//! The raw implementation preserves the current block and binding order.
use super::*;
use crate::mir::normal_callable_semantic_package::RootCleanupActionV1;

pub(in crate::mir::builder) trait RootGraphWriterPortV1 {
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String>;
    /// Write this one nonfallible cleanup and its sole continuation together.
    fn append_array_release(
        &mut self, builder: &mut MirBuilder, id: BasicBlockId, value: ValueId,
        next: BasicBlockId, bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String>;
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

pub(in crate::mir::builder) struct RawRootGraphWriterV1;

impl RootGraphWriterPortV1 for RawRootGraphWriterV1 {
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String> {
        Ok(builder.next_block_id())
    }

    fn append_array_release(
        &mut self, builder: &mut MirBuilder, id: BasicBlockId, value: ValueId,
        next: BasicBlockId, bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let function = builder.function_state.current_function.as_mut().ok_or_else(|| freeze("no-function"))?;
        if function.blocks.contains_key(&id) { return Err(freeze("duplicate-block")); }
        if !function.blocks.contains_key(&next) { return Err(freeze("no-normal-landing")); }
        let release = MirInstruction::ArrayResidenceRelease { value };
        let jump = MirInstruction::Jump { target: next, edge_args: None };
        let mut block = BasicBlock::new(id);
        block.add_instruction(release.clone());
        block.set_terminator(jump.clone());
        function.add_block(block);
        bindings.extend([(id, release), (id, jump)]);
        Ok(())
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
    clean: BasicBlockId,
    normal_operations: &[InvokeOperation],
    fault_operations: &[InvokeOperation],
    after_normal: &[Vec<usize>],
    acquisition: &[usize],
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<CleanupSuffixGraphV1, String> {
    emit_cleanup_actions_with_writer(writer, builder, frame, clean,
        &normal_operations.iter().cloned().map(RootCleanupActionV1::Invoke).collect::<Vec<_>>(),
        &fault_operations.iter().cloned().map(RootCleanupActionV1::Invoke).collect::<Vec<_>>(),
        after_normal, acquisition, bindings)
}

pub(super) fn emit_cleanup_actions_with_writer(
    writer: &mut impl RootGraphWriterPortV1, builder: &mut MirBuilder,
    frame: ValueId, clean: BasicBlockId,
    normal_operations: &[RootCleanupActionV1], fault_operations: &[RootCleanupActionV1],
    after_normal: &[Vec<usize>], acquisition: &[usize],
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
    emit_cleanup_actions_to_fault_with_writer(writer, builder, frame, clean,
        normal_operations, fault_operations, after_normal, acquisition, fault_terminal, bindings)
}

/// Compose a younger cleanup prefix with an already emitted older Fault suffix.
/// The suffix runs once; it must not also appear in fault_operations.
pub(super) fn emit_cleanup_suffixes_to_fault_with_writer(
    writer: &mut impl RootGraphWriterPortV1,
    builder: &mut MirBuilder,
    frame: ValueId,
    clean: BasicBlockId,
    normal_operations: &[InvokeOperation],
    fault_operations: &[InvokeOperation],
    after_normal: &[Vec<usize>],
    acquisition: &[usize],
    fault_terminal: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<CleanupSuffixGraphV1, String> {
    emit_cleanup_actions_to_fault_with_writer(writer, builder, frame, clean,
        &normal_operations.iter().cloned().map(RootCleanupActionV1::Invoke).collect::<Vec<_>>(),
        &fault_operations.iter().cloned().map(RootCleanupActionV1::Invoke).collect::<Vec<_>>(),
        after_normal, acquisition, fault_terminal, bindings)
}

pub(super) fn emit_cleanup_actions_to_fault_with_writer(
    writer: &mut impl RootGraphWriterPortV1, builder: &mut MirBuilder,
    frame: ValueId, mut clean: BasicBlockId,
    normal_operations: &[RootCleanupActionV1], fault_operations: &[RootCleanupActionV1],
    after_normal: &[Vec<usize>], acquisition: &[usize], fault_terminal: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<CleanupSuffixGraphV1, String> {
    if after_normal.len() != normal_operations.len()
        || after_normal.iter().flatten().chain(acquisition)
            .any(|index| *index >= fault_operations.len()) {
        return Err(freeze("root-fault-sequence-shape"));
    }
    let mut fault_nodes = std::collections::BTreeMap::new();
    fault_nodes.insert(Vec::<usize>::new(), fault_terminal);
    let mut normal_origins = Vec::with_capacity(normal_operations.len());
    for (index, operation) in normal_operations.iter().enumerate().rev() {
        let fault = if matches!(operation, RootCleanupActionV1::Invoke(_)) { emit_pending_root_fault_sequence(
            writer,
            builder,
            frame,
            fault_operations,
            &after_normal[index],
            &mut fault_nodes,
            bindings,
        )? } else { clean };
        let (entry, binding) = cleanup_action_step_with_writer(
            writer,
            builder,
            frame,
            operation.clone(),
            clean,
            fault,
            bindings,
        )?;
        clean = entry;
        normal_origins.push(binding);
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
    full: &[RootCleanupActionV1],
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
            cleanup_action_step_with_writer(writer, builder, frame, origin.clone(), next, next, bindings)?.0;
        nodes.insert(suffix.to_vec(), next);
    }
    Ok(next)
}

fn cleanup_action_step_with_writer(
    writer: &mut impl RootGraphWriterPortV1, builder: &mut MirBuilder,
    frame: ValueId, action: RootCleanupActionV1, normal_next: BasicBlockId, fault_next: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(BasicBlockId, (BasicBlockId, MirInstruction)), String> {
    match action {
        RootCleanupActionV1::Invoke(operation) => {
            let entry = cleanup_step_with_writer(writer, builder, frame, operation, normal_next, fault_next, bindings)?;
            let binding = bindings.last().cloned().ok_or_else(|| freeze("root-home-release-binding-missing"))?;
            Ok((entry, binding))
        }
        RootCleanupActionV1::ArrayRelease { value } => {
            let entry = writer.reserve_block(builder)?;
            writer.append_array_release(builder, entry, value, normal_next, bindings)?;
            Ok((entry, (entry, MirInstruction::ArrayResidenceRelease { value })))
        }
    }
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

/// Observations from the same argument prefix and older root suffix writer.
pub(in crate::mir::builder) type RootArgumentCleanupV1 = (
    (BasicBlockId, MirInstruction), (BasicBlockId, MirInstruction),
    Vec<(BasicBlockId, MirInstruction)>, BasicBlockId,
);

pub(in crate::mir::builder) fn emit_root_graph_with_arguments_v1(
    builder: &mut MirBuilder,
    frame_port: &mut impl RootExitFramePortV1,
    writer: &mut impl RootGraphWriterPortV1,
    operations: Vec<RootHomeReleaseOriginV1>,
    fault_sequences: RootHomeFaultSequencesV1,
    return_value: Option<ValueId>,
    statement_result: ValueId,
    ingress_operation: Option<InvokeOperation>,
    argument_ends: &[InvokeOperation],
) -> Result<RootGraphEmissionV1, String> {
    let fault_operations: Vec<_> = fault_sequences
        .full
        .iter()
        .map(|origin| origin.operation().clone())
        .collect();
    let mut bindings = Vec::new();
    let mut clean = writer.reserve_block(builder)?;
    writer.append_terminated(
        builder,
        clean,
        MirInstruction::Return {
            value: return_value,
        },
        &mut bindings,
    )?;
    let mut origins = Vec::with_capacity(operations.len());
    let mut frame_value = None;
    let mut ingress = None;
    let mut argument_cleanup = None;
    // Empty-Home Plain has no Fault edge and therefore no frame definition.
    if !operations.is_empty() || ingress_operation.is_some() {
        let frame = frame_port.materialize(builder)?;
        frame_value = Some(frame);
        let cleanup = root_graph::emit_cleanup_suffixes_with_writer(
            writer, builder, frame, clean,
            &operations.iter().map(|origin| origin.operation().clone()).collect::<Vec<_>>(),
            &fault_operations, &fault_sequences.after_normal,
            if ingress_operation.is_some() { &fault_sequences.acquisition } else { &[] },
            &mut bindings,
        )?;
        clean = cleanup.clean;
        origins.extend(operations.into_iter().zip(cleanup.normal_origins)
            .map(|(origin, (block, instruction))| (origin, block, instruction)));
        let fault = if ingress_operation.is_some() {
            cleanup.acquisition_fault
        } else {
            cleanup.fault_terminal
        };
        if let Some(operation) = ingress_operation {
            let mut normal_landing = clean;
            let mut fault_landing = fault;
            let mut prefix_receipt = None;
            if !argument_ends.is_empty() {
                let cancellation = fault_sequences.result_cancellation_v1()?;
                let mut pending = argument_ends.to_vec();
                pending.extend(cancellation);
                let after: Vec<_> = (0..argument_ends.len())
                    .map(|i| ((i + 1)..pending.len()).collect()).collect();
                let acquisition: Vec<_> = (0..argument_ends.len()).collect();
                let continuation_block = writer.reserve_block(builder)?;
                let continuation = (continuation_block, MirInstruction::Jump { target: clean, edge_args: None });
                writer.append_terminated(builder, continuation.0, continuation.1.clone(), &mut bindings)?;
                let prefix = emit_cleanup_suffixes_to_fault_with_writer(writer, builder, frame,
                    continuation_block, argument_ends, &pending, &after, &acquisition, fault, &mut bindings)?;
                normal_landing = prefix.clean;
                fault_landing = prefix.acquisition_fault;
                prefix_receipt = Some((continuation, prefix.normal_origins));
            }
            ingress = Some(terminal_call::emit_ingress(
                writer,
                builder,
                frame,
                statement_result,
                normal_landing,
                fault_landing,
                operation,
                &mut bindings,
            )?);
            if let Some((continuation, normal)) = prefix_receipt {
                let landing = ingress.as_ref().unwrap().1.0;
                let start = (landing, MirInstruction::Jump { target: normal_landing, edge_args: None });
                if !bindings.contains(&start) { return Err(freeze("call-argument/cleanup-delimiters")); }
                argument_cleanup = Some((start, continuation, normal, fault));
            }
        }
    }
    if ingress.is_none() {
        bindings.push(writer.emit_current(
            builder,
            MirInstruction::Jump {
                target: clean,
                edge_args: None,
            },
        )?);
    }
    Ok(RootGraphEmissionV1 {
        origins,
        bindings,
        ingress,
        frame: frame_value,
        clean,
        argument_cleanup,
    })
}

/// Physical cleanup delimiters shared by owned operation consumers.
pub(in crate::mir::builder) type OperationCleanupV1 = (
    BasicBlockId, BasicBlockId, (BasicBlockId, MirInstruction), Vec<(BasicBlockId, MirInstruction)>,
);

/// End the caller's deferred acquisitions on Normal and before prior Homes on
/// Fault. Source owners supply the exact operations; this writer adds no rights.
pub(in crate::mir::builder) fn prepare_operation_cleanup_v1(
    builder: &mut MirBuilder, frame: ValueId, outward: BasicBlockId,
    normal_ends: Vec<InvokeOperation>, unwind: Vec<InvokeOperation>,
    normal_fault_cancellation: &[InvokeOperation], bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(BasicBlockId, Option<OperationCleanupV1>), String> {
    let result = if normal_ends.is_empty() {
        append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, bindings)?;
        (cleanup_chain(builder, frame, unwind, outward, bindings)?, None)
    } else {
        let mut fault_operations = normal_ends.clone();
        fault_operations.extend_from_slice(normal_fault_cancellation);
        fault_operations.extend(unwind);
        let mut seen = std::collections::BTreeSet::new();
        for operation in &fault_operations {
            if let InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value })
                | InvokeOperation::DynamicCarrierEnd { carrier: value } = operation {
                if !seen.insert(*value) { return Err(freeze("call-argument/duplicate-acquisition")); }
            }
        }
        let clean = builder.next_block_id();
        let next = builder.next_block_id();
        let continuation = (clean, MirInstruction::Jump { target: next, edge_args: None });
        append_block(builder, clean, continuation.1.clone(), bindings)?;
        let after_normal: Vec<_> = (0..normal_ends.len())
            .map(|index| ((index + 1)..fault_operations.len()).collect()).collect();
        // A result exists only after operation Normal. Its cancellation belongs
        // to Normal cleanup Fault suffixes, never the original operation Fault.
        let call_fault: Vec<_> = (0..normal_ends.len())
            .chain((normal_ends.len() + normal_fault_cancellation.len())..fault_operations.len()).collect();
        let graph = emit_cleanup_suffixes_with_writer(
            &mut RawRootGraphWriterV1, builder, frame, clean, &normal_ends, &fault_operations,
            &after_normal, &call_fault, bindings)?;
        (graph.acquisition_fault, Some((graph.clean, next, continuation, graph.normal_origins)))
    };
    Ok(result)
}
