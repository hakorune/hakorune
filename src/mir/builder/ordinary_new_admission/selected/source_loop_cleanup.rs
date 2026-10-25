//! Loop ports share the existing suffix graph and installed End plans.
//! Each operation is localized at its actual Invoke block, never at entry.
use super::*;
use std::rc::Rc;
use crate::mir::normal_callable_semantic_package::SourceLoopCleanupPlanV2;
use crate::mir::builder::ssa::local::{self, LocalKind, LocalSsaPhysicalRelationV1};

#[derive(Debug)]
pub(in crate::mir) struct CleanupOperationLoanV2 {
    pub(in crate::mir) original: InvokeOperation,
    pub(in crate::mir) binding: (BasicBlockId, MirInstruction),
    pub(in crate::mir) operands: Vec<Rc<LocalSsaPhysicalRelationV1>>,
}
#[derive(Debug)]
pub(in crate::mir) struct EmittedSourceLoopCleanupV2 {
    pub(in crate::mir) plan: SourceLoopCleanupPlanV2,
    pub(in crate::mir) from: BasicBlockId,
    pub(in crate::mir) entry: BasicBlockId,
    pub(in crate::mir) continuation: BasicBlockId,
    pub(in crate::mir) bindings: Vec<(BasicBlockId, MirInstruction)>,
    pub(in crate::mir) operations: Vec<CleanupOperationLoanV2>,
    pub(in crate::mir) normal_origins: Vec<(BasicBlockId, MirInstruction)>,
}

/// Calls the sole cleanup algorithm after the source plan's ledger borrow has
/// ended. The caller owns the actual continuation and emits its final Jump.
pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState, plan: SourceLoopCleanupPlanV2,
) -> Result<EmittedSourceLoopCleanupV2, String> {
    let from = builder.function_state.current_block.ok_or_else(|| freeze("source-loop/cleanup-block"))?;
    if plan.normal_is_empty() {
        return Ok(EmittedSourceLoopCleanupV2 { plan, from, entry: from, continuation: from,
            bindings: Vec::new(), operations: Vec::new(), normal_origins: Vec::new() });
    }
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    let continuation = builder.next_block_id();
    builder.ensure_block_exists(continuation)?;
    let mut writer = LocalizedCleanupWriterV2 { operations: Vec::new() };
    let graph = root_graph::emit_cleanup_suffixes_with_writer(&mut writer, builder, frame, continuation,
        &plan.normal_operations(), &plan.fault_operations(), plan.residuals(), &[], &mut bindings)?;
    // graph.clean is an ENTRY; only continuation becomes the Loop predecessor.
    let jump = writer.emit_current(builder, MirInstruction::Jump { target: graph.clean, edge_args: None })?;
    bindings.push(jump);
    builder.start_new_block(continuation)?;
    Ok(EmittedSourceLoopCleanupV2 { plan, from, entry: graph.clean, continuation,
        bindings, operations: writer.operations, normal_origins: graph.normal_origins })
}

struct LocalizedCleanupWriterV2 { operations: Vec<CleanupOperationLoanV2> }
impl RootGraphWriterPortV1 for LocalizedCleanupWriterV2 {
    fn append_array_release(&mut self, builder: &mut MirBuilder, id: BasicBlockId,
        value: ValueId, next: BasicBlockId, bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        // An owned Array retains its original producer identity, like String
        // End; it must not be localized into a scalar argument Copy.
        RawRootGraphWriterV1.append_array_release(builder, id, value, next, bindings)
    }
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String> {
        RawRootGraphWriterV1.reserve_block(builder)
    }
    fn append_terminated(&mut self, builder: &mut MirBuilder, id: BasicBlockId,
        terminator: MirInstruction, bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let MirInstruction::Invoke { mut operation, fault_frame, normal_landing, fault_landing } = terminator else {
            return RawRootGraphWriterV1.append_terminated(builder, id, terminator, bindings);
        };
        let saved = builder.function_state.current_block.ok_or_else(|| freeze("source-loop/cleanup-saved-block"))?;
        if builder.function_state.current_function.as_ref().is_none_or(|function| function.blocks.contains_key(&id)) {
            return Err(freeze("source-loop/cleanup-origin-repeat"));
        }
        let original = operation.clone();
        // An owned String pair is not a scalar: its source End must borrow
        // the original producer, never a LocalSSA Copy of its representation.
        let original_string_end = matches!(operation,
            InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { .. }));
        let action = (|| {
            builder.start_new_block(id)?;
            let mut error = None;
            let mut pairs = Vec::new();
            operation.rewrite_values(|value| {
                if error.is_some() { return; }
                let before = *value;
                if original_string_end { pairs.push((before, before)); return; }
                match local::try_ensure(builder, before, LocalKind::Arg) {
                    Ok(after) => { *value = after; pairs.push((before, after)); }
                    Err(failure) => error = Some(failure),
                }
            });
            if let Some(error) = error { return Err(error); }
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
            let mut operands = Vec::new();
            for (before, after) in pairs {
                let relation = builder.function_state.local_ssa_correspondence.loan(function, before, after)
                    .ok_or_else(|| freeze("source-loop/cleanup-operand-relation"))?;
                relation.verify(function)?;
                operands.push(relation);
            }
            bindings.extend(function.blocks[&id].instructions.iter().cloned().map(|row| (id, row)));
            let actual = MirInstruction::Invoke { operation, fault_frame, normal_landing, fault_landing };
            builder.emit_instruction(actual.clone())?;
            bindings.push((id, actual.clone()));
            self.operations.push(CleanupOperationLoanV2 { original, binding: (id, actual), operands });
            Ok(())
        })();
        let restored = builder.start_new_block(saved);
        match (action, restored) {
            (Err(primary), Err(restore)) => Err(format!("{primary} [cleanup-restore: {restore}]")),
            (Err(error), _) | (_, Err(error)) => Err(error),
            _ => Ok(()),
        }
    }
    fn emit_current(&mut self, builder: &mut MirBuilder, instruction: MirInstruction)
        -> Result<(BasicBlockId, MirInstruction), String> {
        RawRootGraphWriterV1.emit_current(builder, instruction)
    }
    fn insert_projection(&mut self, builder: &mut MirBuilder, landing: BasicBlockId, projection: MirInstruction)
        -> Result<(), String> {
        RawRootGraphWriterV1.insert_projection(builder, landing, projection)
    }
}
