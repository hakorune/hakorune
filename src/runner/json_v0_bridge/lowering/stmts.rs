use super::super::ast::StmtV0;
use super::{
    expr, if_else, if_legacy, lambda_legacy, loop_, loop_runtime, normalize_scope_exit_registrations,
    throw_lower, try_catch, while_legacy, BridgeEnv, LoopContext,
};
use crate::mir::{BasicBlockId, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

pub(super) fn lower_stmt_with_vars(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    s: &StmtV0,
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    match s {
        StmtV0::Return { expr } => {
            let (v, cur) = expr::lower_expr_with_vars(env, f, cur_bb, expr, vars)?;
            if let Some(bb) = f.get_block_mut(cur) {
                bb.set_terminator(MirInstruction::Return { value: Some(v) });
            }
            Ok(cur)
        }
        StmtV0::Extern {
            iface,
            method,
            args,
        } => {
            let (arg_ids, cur) = expr::lower_args_with_vars(env, f, cur_bb, args, vars)?;
            if let Some(bb) = f.get_block_mut(cur) {
                bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
                    None,
                    iface.clone(),
                    method.clone(),
                    arg_ids,
                    EffectMask::IO,
                ));
            }
            Ok(cur)
        }
        StmtV0::Expr { expr } => {
            let (_v, cur) = expr::lower_expr_with_vars(env, f, cur_bb, expr, vars)?;
            Ok(cur)
        }
        StmtV0::Local { name, expr } => {
            let (v, cur) = expr::lower_expr_with_vars(env, f, cur_bb, expr, vars)?;
            vars.insert(name.clone(), v);
            Ok(cur)
        }
        StmtV0::Throw { expr } => {
            let (exc, cur) = expr::lower_expr_with_vars(env, f, cur_bb, expr, vars)?;
            let (_v, next_bb) = throw_lower::lower_throw(env, f, cur, exc, Some(vars));
            Ok(next_bb)
        }
        StmtV0::Break => {
            if let Some(ctx) = loop_stack.last().copied() {
                // snapshot variables at break
                loop_runtime::record_exit_snapshot(cur_bb, vars);
                loop_runtime::lower_break_stmt(f, cur_bb, ctx.exit_bb);
            }
            Ok(cur_bb)
        }
        StmtV0::Continue => {
            if let Some(ctx) = loop_stack.last().copied() {
                // Optional: apply increment hint before continue (so header sees updated var)
                if let Some((ref var_name, step)) = loop_runtime::peek_increment_hint() {
                    let _ = crate::mir::ssot::loop_common::apply_increment_before_continue(
                        f, cur_bb, vars, var_name, step,
                    );
                }
                // snapshot variables at continue (after increment)
                loop_runtime::record_continue_snapshot(cur_bb, vars);
                let target = ctx.continue_merge_bb.unwrap_or(ctx.cond_bb);
                loop_runtime::lower_continue_stmt(f, cur_bb, target);
            }
            Ok(cur_bb)
        }
        StmtV0::Try {
            try_body,
            catches,
            finally,
        } => {
            try_catch::lower_try_stmt(f, cur_bb, try_body, catches, finally, vars, loop_stack, env)
        }
        StmtV0::If { cond, then, r#else } => {
            if_else::lower_if_stmt(f, cur_bb, cond, then, r#else, vars, loop_stack, env)
        }
        StmtV0::Loop { cond, body } => {
            loop_::lower_loop_stmt(f, cur_bb, cond, body, vars, loop_stack, env)
        }
        StmtV0::FiniReg { .. } => Err(
            "[freeze:contract][json_v0_bridge/fini_marker_leak] unnormalized FiniReg marker"
                .to_string(),
        ),
    }
}

pub(super) fn lower_stmt_list_with_vars(
    f: &mut MirFunction,
    start_bb: BasicBlockId,
    stmts: &[StmtV0],
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    let normalized = normalize_scope_exit_registrations(stmts)?;
    let mut cur = start_bb;
    let mut i: usize = 0;
    while i < normalized.len() {
        if let Some((cur2, consumed)) =
            if_legacy::try_lower_stageb_legacy_if_not_stmt_quad(f, cur, &normalized, i, vars, loop_stack, env)?
        {
            cur = cur2;
            i += consumed;
            if let Some(bb) = f.blocks.get(&cur) {
                if bb.is_terminated() {
                    break;
                }
            }
            continue;
        }

        if let Some((cur2, consumed)) = while_legacy::try_lower_stageb_legacy_while_stmt_triplet(
            f, cur, &normalized, i, vars, loop_stack, env,
        )? {
            cur = cur2;
            i += consumed;
            if let Some(bb) = f.blocks.get(&cur) {
                if bb.is_terminated() {
                    break;
                }
            }
            continue;
        }

        if let Some((cur2, consumed)) = lambda_legacy::try_lower_stageb_legacy_fn_literal_stmt_pair(
            f, cur, &normalized, i, vars, loop_stack, env,
        )? {
            cur = cur2;
            i += consumed;
            // Terminator may have been set (e.g., return).
            if let Some(bb) = f.blocks.get(&cur) {
                if bb.is_terminated() {
                    break;
                }
            }
            continue;
        }

        let s = &normalized[i];
        cur = lower_stmt_with_vars(f, cur, s, vars, loop_stack, env)?;
        i += 1;
        if let Some(bb) = f.blocks.get(&cur) {
            if bb.is_terminated() {
                break;
            }
        }
    }
    Ok(cur)
}
