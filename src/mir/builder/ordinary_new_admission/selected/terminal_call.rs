//! Physical terminal Call lowering uses the ledger's original literal relation.
//! The affine row stays owned by root exit progress after emission.
use super::*;
use crate::mir::definitions::MirCall;
use crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

pub(super) struct Emission {
    pub(super) row: AppMainDirectCallDispositionRowV1,
    pub(super) arguments: Vec<(BasicBlockId, MirInstruction)>,
    pub(super) call: MirCall,
}

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    row: AppMainDirectCallDispositionRowV1,
) -> Result<ValueId, String> {
    let emission = row
        .lifecycle_emission()
        .map_err(|_| freeze("call-source-mismatch"))?;
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let mut arguments = Vec::new();
    let mut values = Vec::new();
    for literal in ledger
        .terminal_call_arguments_for_owner(owner)
        .ok_or_else(|| freeze("call-source-missing"))?
    {
        let value = crate::mir::builder::emission::constant::emit_integer(builder, *literal)?;
        arguments.push((
            block,
            MirInstruction::Const {
                dst: value,
                value: crate::mir::ConstValue::Integer(*literal),
            },
        ));
        values.push(value);
    }
    let call = emission
        .materialize_call(None, values)
        .map_err(|_| freeze("call-projection-failed"))?;
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        Some(value),
        value,
        Some(Emission {
            row,
            arguments,
            call,
        }),
    )
}

pub(super) fn emit_ingress(
    builder: &mut MirBuilder,
    frame: ValueId,
    value: ValueId,
    clean: BasicBlockId,
    fault: BasicBlockId,
    call: MirCall,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<
    (
        (BasicBlockId, MirInstruction),
        (BasicBlockId, MirInstruction),
    ),
    String,
> {
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal_landing = builder.next_block_id();
    let fault_landing = builder.next_block_id();
    for (id, target) in [(normal_landing, clean), (fault_landing, fault)] {
        append_block(
            builder,
            id,
            MirInstruction::Jump {
                target,
                edge_args: None,
            },
            bindings,
        )?;
    }
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: value,
    };
    builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| freeze("no-function"))?
        .blocks
        .get_mut(&normal_landing)
        .ok_or_else(|| freeze("no-normal-landing"))?
        .add_instruction(projection.clone());
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::I64,
        },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    Ok(((origin, invoke), (normal_landing, projection)))
}

/// Emit one source-issued local Call result and leave its normal continuation
/// open for the enclosing local statement. This bounded slice accepts only a
/// local relation with no prior Homes; terminal cleanup remains root-owned.
pub(in crate::mir::builder) fn emit_local(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: AppMainDirectCallDispositionRowV1,
    arguments: Vec<ValueId>,
) -> Result<ValueId, String> {
    let relation = ledger
        .local_i64_call_for_owner(owner, site)
        .ok_or_else(|| freeze("local-call-source-missing"))?;
    if !relation.prior_homes().is_empty() {
        return Err(freeze("local-call-prior-homes-unsupported"));
    }
    let call = row
        .lifecycle_emission()
        .map_err(|_| freeze("local-call-source-mismatch"))?
        .materialize_call(None, arguments)
        .map_err(|_| freeze("local-call-projection-failed"))?;
    let frame = state.borrow_fault_frame(builder)?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal_landing = builder.next_block_id();
    let fault_landing = builder.next_block_id();
    let result = builder.next_value_id();
    let mut bindings = Vec::new();
    append_block(
        builder,
        fault_landing,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::I64,
        },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    builder.start_new_block(normal_landing)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Integer);
    bindings.push((origin, invoke));
    bindings.push((normal_landing, projection));
    ledger.record_root_local_call_bindings(
        owner,
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone()),
        bindings,
    )?;
    Ok(result)
}
