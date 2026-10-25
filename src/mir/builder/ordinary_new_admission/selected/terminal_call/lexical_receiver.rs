//! One source-qualified receiver preparation for local and terminal calls.
use super::*;
use crate::mir::builder::normal_callable_semantic_lowering_state::ExactLexicalReadV1;
type Binding = (BasicBlockId, MirInstruction);
use crate::mir::normal_callable_semantic_package::{StoredReceiverReadV1, StoredReceiverFaultV1};

pub(super) enum Fault<'a> {
    Call(&'a [InvokeOperation]),
    Return(&'a crate::mir::resolved_semantics::SourceStmtSiteV1),
    Constructor { frame: ValueId, landing: BasicBlockId },
}

pub(super) fn prepare(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    owner: FunctionOwnerIdV1,
    row: &LexicalInstanceCallDispositionRowV1,
    ledger: &OrdinaryNewClaimLedgerV1,
    fault_source: Fault<'_>,
    bindings: &mut Vec<Binding>,
) -> Result<(ExactLexicalReadV1, Option<StoredReceiverReadV1>), String> {
    let stored = row.source_target().stored_receiver();
    let (site, binding) = match stored {
        Some((binding, site, _, _)) => (site, binding),
        None => (row.receiver_site(), row.receiver_binding()?),
    };
    let receiver = state.take_exact_lexical_read(owner, site.node(), binding)
        .map_err(|error| error.to_string())?;
    let field = if let Some((_, site, field, child)) = stored {
        let base = receiver.value_for(owner, site.node(), binding)
            .map_err(|error| error.to_string())?;
        if ledger.stored_receiver_uses_dynamic_field_v1(row.source_target())? {
            let (frame, prior_fault, unwind, owner) = match fault_source {
                Fault::Call(unwind) => (state.borrow_fault_frame(builder)?, None, unwind.to_vec(), StoredReceiverFaultV1::Call),
                Fault::Return(exit) => (state.borrow_fault_frame(builder)?, None,
                    ledger.stored_receiver_return_unwind_v1(row.call_site(), exit)?, StoredReceiverFaultV1::Return(exit.clone())),
                Fault::Constructor { frame, landing } => (frame, Some(landing), Vec::new(), StoredReceiverFaultV1::Constructor),
            };
            let shared = fault_frame_binding(builder, state, frame)?;
            if !bindings.contains(&shared) { bindings.push(shared); }
            let mut fault = Vec::new();
            let landing = if let Some(landing) = prior_fault { landing } else {
                let outward = builder.next_block_id();
                append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, &mut fault)?;
                cleanup_chain(builder, frame, unwind, outward, &mut fault)?
            };
            bindings.extend_from_slice(&fault);
            let start = bindings.len();
            // Original stored-call admission already proves a nonnull child
            // (including the sole Birth writer and nullable guard). Preserve
            // that refinement; nullable field observations grant no receiver.
            let (value, normal) = super::super::invoke_operation::emit(builder, frame,
                InvokeOperation::ObjectFieldBorrow { base, field, child, nullable: false }, landing, bindings)?;
            let value = value.ok_or_else(|| freeze("lexical-terminal/stored-read-kind"))?;
            let normal = normal.ok_or_else(|| freeze("lexical-terminal/stored-read-kind"))?;
            builder.function_state.type_ctx.value_types.insert(value, MirType::Box(row.target().owner().into()));
            Some(StoredReceiverReadV1::Checked { invoke: bindings[start].clone(), normal, fault, owner })
        } else {
            let (_, read) = builder.append_exact_object_field_read_v1(
                base, field, MirType::Box(row.target().owner().into()))?;
            bindings.push(read.clone());
            Some(read.into())
        }
    } else { None };
    Ok((receiver, field))
}
