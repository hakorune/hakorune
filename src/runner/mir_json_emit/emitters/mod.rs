mod basic;
mod calls;
mod control_flow;
mod fields;
mod phi;
mod weak;

use crate::mir::MirInstruction as I;

pub(crate) fn emit_phi_instructions(
    func: &crate::mir::MirFunction,
    block: &crate::mir::BasicBlock,
) -> Vec<serde_json::Value> {
    let mut insts = Vec::new();
    for inst in &block.instructions {
        if let I::Phi { .. } = inst {
            if let Some(value) = phi::emit_phi(inst, &func.metadata.value_types) {
                insts.push(value);
            }
        }
    }
    insts
}

pub(crate) fn emit_non_phi_instructions(
    func: &crate::mir::MirFunction,
    block: &crate::mir::BasicBlock,
    insts: &mut Vec<serde_json::Value>,
) -> Result<(), String> {
    for inst in &block.instructions {
        if let I::Phi { .. } = inst {
            continue;
        }
        let value = emit_instruction(func, inst)?;
        insts.push(value);
    }
    Ok(())
}

pub(crate) fn emit_terminator(
    terminator: &Option<crate::mir::MirInstruction>,
) -> Result<Option<serde_json::Value>, String> {
    match terminator.as_ref() {
        Some(term) => control_flow::emit_terminator(term).map(Some),
        None => Ok(None),
    }
}

fn emit_instruction(
    func: &crate::mir::MirFunction,
    inst: &crate::mir::MirInstruction,
) -> Result<serde_json::Value, String> {
    if let Some(code) = crate::mir::contracts::backend_core_ops::legacy_callsite_reject_code(inst) {
        return Err(format!(
            "[freeze:contract][mir-json/callsite:{}] inst={}",
            code,
            crate::mir::contracts::backend_core_ops::instruction_tag(inst)
        ));
    }

    if !crate::mir::contracts::backend_core_ops::is_supported_mir_json_instruction(inst) {
        return Err(format!(
            "MIR JSON emit contract violation: unsupported instruction {}",
            crate::mir::contracts::backend_core_ops::instruction_tag(inst)
        ));
    }

    match inst {
        I::Copy { dst, src } => Ok(basic::emit_copy(dst, src)),
        I::UnaryOp { dst, op, operand } => Ok(basic::emit_unary_op(dst, op, operand)),
        I::Const { dst, value } => Ok(basic::emit_const(dst, value)),
        I::TypeOp { dst, op, value, ty } => Ok(basic::emit_type_op(dst, op, value, ty)),
        I::BinOp { dst, op, lhs, rhs } => Ok(basic::emit_bin_op(
            dst,
            op,
            lhs,
            rhs,
            &func.metadata.value_types,
        )),
        I::Compare { dst, op, lhs, rhs } => Ok(basic::emit_compare(
            dst,
            op,
            lhs,
            rhs,
            &func.metadata.value_types,
        )),
        I::FieldGet {
            dst,
            base,
            field,
            declared_type,
        } => Ok(fields::emit_field_get(dst, base, field, declared_type)),
        I::FieldSet {
            base,
            field,
            value,
            declared_type,
        } => Ok(fields::emit_field_set(base, field, value, declared_type)),
        I::Select {
            dst,
            cond,
            then_val,
            else_val,
            ..
        } => Ok(basic::emit_select(dst, cond, then_val, else_val)),
        I::Call {
            dst,
            func,
            callee,
            args,
            effects,
            ..
        } => calls::emit_call(dst, func, callee.as_ref(), args, effects)
            .ok_or_else(|| "MIR JSON emit contract violation: failed to emit Call".to_string()),
        I::NewBox {
            dst,
            box_type,
            args,
        } => Ok(calls::emit_new_box(dst, box_type, args)),
        I::NewClosure {
            dst,
            params,
            captures,
            me,
            ..
        } => Ok(calls::emit_new_closure(dst, params, captures, me)),
        I::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        } => Ok(control_flow::emit_branch(condition, then_bb, else_bb)),
        I::Jump { target, .. } => Ok(control_flow::emit_jump(target)),
        I::Return { value } => Ok(control_flow::emit_return(value.as_ref())),
        I::WeakRef { dst, op, value } => Ok(weak::emit_weak_ref(dst, op, value)),
        I::KeepAlive { values } => Ok(weak::emit_keep_alive(values)),
        I::ReleaseStrong { values } => Ok(weak::emit_release_strong(values)),
        _ => unreachable!("pre-checked by backend_core_ops allowlist"),
    }
}
