use super::super::super::ast::ExprV0;
use super::super::BridgeEnv;
use super::VarScope;
use crate::ast::Span;
use crate::mir::{BasicBlockId, ConstValue, MirFunction, MirInstruction, ValueId};

pub(super) fn lower_binary_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    op: &str,
    lhs: &ExprV0,
    rhs: &ExprV0,
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    let (left, cur_after_left) = super::lower_expr_with_scope(env, f, cur_bb, lhs, vars)?;
    let (right, cur_after_right) = super::lower_expr_with_scope(env, f, cur_after_left, rhs, vars)?;
    let binop = match crate::mir::ssot::binop_lower::parse_binop_str(op) {
        Some(value) => value,
        None => return Err("unsupported op".into()),
    };
    let dst =
        crate::mir::ssot::binop_lower::emit_binop_func(f, cur_after_right, binop, left, right);
    Ok((dst, cur_after_right))
}

pub(super) fn lower_compare_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    op: &str,
    lhs: &ExprV0,
    rhs: &ExprV0,
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    let (left, cur_after_left) = super::lower_expr_with_scope(env, f, cur_bb, lhs, vars)?;
    let (right, cur_after_right) = super::lower_expr_with_scope(env, f, cur_after_left, rhs, vars)?;
    let compare_op = match op {
        "==" => crate::mir::CompareOp::Eq,
        "!=" => crate::mir::CompareOp::Ne,
        "<" => crate::mir::CompareOp::Lt,
        "<=" => crate::mir::CompareOp::Le,
        ">" => crate::mir::CompareOp::Gt,
        ">=" => crate::mir::CompareOp::Ge,
        _ => return Err("unsupported compare op".into()),
    };
    let dst = f.next_value_id();
    crate::mir::ssot::cf_common::emit_compare_func(
        f,
        cur_after_right,
        dst,
        compare_op,
        left,
        right,
    );
    Ok((dst, cur_after_right))
}

pub(super) fn lower_logical_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    op: &str,
    lhs: &ExprV0,
    rhs: &ExprV0,
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    let (left, cur_after_left) = super::lower_expr_with_scope(env, f, cur_bb, lhs, vars)?;
    let rhs_bb = super::super::merge::new_block(f);
    let fall_bb = super::super::merge::new_block(f);
    let merge_bb = super::super::merge::new_block(f);
    let is_and = matches!(op, "&&" | "and");
    if is_and {
        crate::mir::ssot::cf_common::set_branch(f, cur_after_left, left, rhs_bb, fall_bb);
    } else {
        crate::mir::ssot::cf_common::set_branch(f, cur_after_left, left, fall_bb, rhs_bb);
    }

    let const_dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(fall_bb) {
        let const_value = if is_and {
            ConstValue::Bool(false)
        } else {
            ConstValue::Bool(true)
        };
        bb.add_instruction(MirInstruction::Const {
            dst: const_dst,
            value: const_value,
        });
    }
    crate::mir::ssot::cf_common::set_jump(f, fall_bb, merge_bb);

    let (right_value, rhs_end) = super::lower_expr_with_scope(env, f, rhs_bb, rhs, vars)?;
    if let Some(bb) = f.get_block_mut(rhs_end) {
        if !bb.is_terminated() {
            crate::mir::ssot::cf_common::set_jump(f, rhs_end, merge_bb);
        }
    }

    let out = f.next_value_id();
    let mut inputs: Vec<(BasicBlockId, ValueId)> = vec![(fall_bb, const_dst)];
    if rhs_end != fall_bb {
        inputs.push((rhs_end, right_value));
    } else {
        inputs.push((fall_bb, right_value));
    }
    crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
        f,
        merge_bb,
        out,
        inputs,
        Span::unknown(),
    )?;
    Ok((out, merge_bb))
}
