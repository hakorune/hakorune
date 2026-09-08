//! Value-operand substitution only; CFG/PHI predecessor decisions stay in flow.
use crate::mir::{BasicBlock, MirFunction, MirInstruction, ValueId};

pub(super) fn rewrite_value_uses_in_function(function: &mut MirFunction, from: ValueId, to: ValueId) {
    for block in function.blocks.values_mut() {
        rewrite_value_uses_in_block(block, from, to);
    }
}

pub(super) fn rewrite_value_uses_in_block(block: &mut BasicBlock, from: ValueId, to: ValueId) {
    for instruction in &mut block.instructions {
        rewrite_value_uses_in_instruction(instruction, from, to);
    }
    if let Some(terminator) = &mut block.terminator {
        rewrite_value_uses_in_instruction(terminator, from, to);
    }
    if let Some(return_env) = &mut block.return_env {
        for value in return_env {
            rewrite_value_use(value, from, to);
        }
    }
}

fn rewrite_value_uses_in_instruction(instruction: &mut MirInstruction, from: ValueId, to: ValueId) {
    match instruction {
        MirInstruction::Invoke {
            operation,
            fault_frame,
            ..
        } => {
            operation.rewrite_values(|value| rewrite_value_use(value, from, to));
            rewrite_value_use(fault_frame, from, to);
        }
        MirInstruction::ReturnFault { fault_frame } => rewrite_value_use(fault_frame, from, to),
        MirInstruction::InvokeNormalResult { .. } | MirInstruction::FaultFrameEnter { .. } => {}
        MirInstruction::Const { .. }
        | MirInstruction::Catch { .. }
        | MirInstruction::Safepoint
        | MirInstruction::CheckedCallOutEnd { .. }
        | MirInstruction::CheckedCallOutFault { .. }
        | MirInstruction::PinnedTextResidenceFinish { .. }
        | MirInstruction::PinnedTextResidenceEnter { .. }
        | MirInstruction::PinnedTextResidenceTrap { .. } => {}
        MirInstruction::MemOp { operands, .. } => {
            for operand in operands {
                rewrite_value_use(operand, from, to);
            }
        }
        MirInstruction::PinnedTextOp { kind, .. } => {
            kind.rewrite_values(|value| rewrite_value_use(value, from, to));
        }
        MirInstruction::BinOp { lhs, rhs, .. } | MirInstruction::Compare { lhs, rhs, .. } => {
            rewrite_value_use(lhs, from, to);
            rewrite_value_use(rhs, from, to);
        }
        MirInstruction::UnaryOp { operand, .. }
        | MirInstruction::Load { ptr: operand, .. }
        | MirInstruction::StaticDataLoad { index: operand, .. }
        | MirInstruction::FieldGet { base: operand, .. }
        | MirInstruction::ObjectFieldGet { base: operand, .. }
        | MirInstruction::VariantTag { value: operand, .. }
        | MirInstruction::VariantProject { value: operand, .. }
        | MirInstruction::TypeOp { value: operand, .. }
        | MirInstruction::Copy { src: operand, .. }
        | MirInstruction::CopyOwned { src: operand, .. }
        | MirInstruction::LocalContractWrite { src: operand, .. }
        | MirInstruction::RecordFieldContractCheck { value: operand, .. }
        | MirInstruction::Debug { value: operand, .. }
        | MirInstruction::Throw {
            exception: operand, ..
        }
        | MirInstruction::RefNew {
            box_val: operand, ..
        }
        | MirInstruction::WeakRef { value: operand, .. }
        | MirInstruction::Barrier { ptr: operand, .. }
        | MirInstruction::FutureNew { value: operand, .. }
        | MirInstruction::Await {
            future: operand, ..
        } => rewrite_value_use(operand, from, to),
        MirInstruction::Store { value, ptr } => {
            rewrite_value_use(value, from, to);
            rewrite_value_use(ptr, from, to);
        }
        MirInstruction::MapLiteralEntryWrite { receiver, key, value } => {
            rewrite_value_use(receiver, from, to);
            rewrite_value_use(key, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::ArrayElementWrite {
            receiver,
            index,
            value,
            ..
        } => {
            rewrite_value_use(receiver, from, to);
            if let Some(index) = index {
                rewrite_value_use(index, from, to);
            }
            rewrite_value_use(value, from, to);
        }
        MirInstruction::ArrayStateContractClaim { array, .. } => {
            rewrite_value_use(array, from, to);
        }
        MirInstruction::FieldSet { base, value, .. } => {
            rewrite_value_use(base, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::WeakFieldWrite { base, value, .. } => {
            rewrite_value_use(base, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::VariantMake { payload, .. } => {
            if let Some(payload) = payload {
                rewrite_value_use(payload, from, to);
            }
        }
        MirInstruction::Call(call) => {
            call.callee
                .rewrite_value_operands(|value| rewrite_value_use(value, from, to));
            for arg in &mut call.args {
                rewrite_value_use(arg, from, to);
            }
        }
        MirInstruction::LegacyCallV0 {
            func, callee, args, ..
        } => {
            if callee.is_none() {
                rewrite_value_use(func, from, to);
            }
            if let Some(callee) = callee {
                callee.rewrite_value_operands(|value| rewrite_value_use(value, from, to));
            }
            for arg in args {
                rewrite_value_use(arg, from, to);
            }
        }
        MirInstruction::NewClosure { captures, me, .. } => {
            for (_, capture) in captures {
                rewrite_value_use(capture, from, to);
            }
            if let Some(me) = me {
                rewrite_value_use(me, from, to);
            }
        }
        MirInstruction::Branch {
            condition,
            then_edge_args,
            else_edge_args,
            ..
        } => {
            rewrite_value_use(condition, from, to);
            rewrite_edge_args_values(then_edge_args, from, to);
            rewrite_edge_args_values(else_edge_args, from, to);
        }
        MirInstruction::Jump { edge_args, .. } => {
            rewrite_edge_args_values(edge_args, from, to);
        }
        MirInstruction::Return { value } => {
            if let Some(value) = value {
                rewrite_value_use(value, from, to);
            }
        }
        MirInstruction::CheckedCallOut {
            receiver,
            arguments,
            ..
        } => {
            rewrite_value_use(receiver, from, to);
            for argument in arguments {
                rewrite_value_use(argument, from, to);
            }
        }
        MirInstruction::CheckedCallOutNormalResult { .. } => {}
        MirInstruction::Phi { inputs, .. } => {
            for (_, incoming_value) in inputs {
                rewrite_value_use(incoming_value, from, to);
            }
        }
        MirInstruction::NewBox { args, .. } => {
            for arg in args {
                rewrite_value_use(arg, from, to);
            }
        }
        MirInstruction::RecordValuePublish { base, fields, .. } => {
            if let Some(base) = base {
                rewrite_value_use(base, from, to);
            }
            for field in fields {
                rewrite_value_use(field, from, to);
            }
        }
        MirInstruction::KeepAlive { values } | MirInstruction::ReleaseStrong { values } => {
            for value in values {
                rewrite_value_use(value, from, to);
            }
        }
        MirInstruction::DestroyOwned { value }
        | MirInstruction::ArrayResidenceRelease { value } => rewrite_value_use(value, from, to),
        MirInstruction::FutureSet { future, value } => {
            rewrite_value_use(future, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::Select {
            cond,
            then_val,
            else_val,
            ..
        } => {
            rewrite_value_use(cond, from, to);
            rewrite_value_use(then_val, from, to);
            rewrite_value_use(else_val, from, to);
        }
    }
}

fn rewrite_edge_args_values(
    edge_args: &mut Option<crate::mir::EdgeArgs>,
    from: ValueId,
    to: ValueId,
) {
    if let Some(edge_args) = edge_args {
        for value in &mut edge_args.values {
            rewrite_value_use(value, from, to);
        }
    }
}

fn rewrite_value_use(value: &mut ValueId, from: ValueId, to: ValueId) {
    if *value == from {
        *value = to;
    }
}

#[cfg(test)]
#[path = "value_uses_tests.rs"]
mod tests;
