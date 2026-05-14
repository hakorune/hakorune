/*!
 * JSON v0 LoopRange Lowering Pilot
 *
 * LOOP-003B owns the first executable route for `loop i in start..end`.
 *
 * Stop line:
 * - no Stage0 desugar
 * - no re-evaluated end bound
 * - no loop-carried variable writes in the pilot
 * - no silent fallback to legacy for-range lowering
 */

use super::super::ast::{ExprV0, StmtV0};
use super::{expr, lower_stmt_list_with_vars, new_block, BridgeEnv, LoopContext};
use crate::ast::Span;
use crate::mir::{
    BasicBlockId, BinaryOp, CompareOp, ConstValue, LoopRangeFact, MirFunction, MirInstruction,
    ValueId,
};
use std::collections::BTreeMap;

pub(super) fn lower_loop_range_stmt(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    var_name: &str,
    start: &ExprV0,
    end: &ExprV0,
    body: &[StmtV0],
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    reject_pilot_unsupported_writes(var_name, body)?;

    let (start_v, cur) = expr::lower_expr_with_vars(env, f, cur_bb, start, vars)?;
    let (end_v, preheader) = expr::lower_expr_with_vars(env, f, cur, end, vars)?;
    let base_vars = vars.clone();

    let header_bb = new_block(f);
    let body_bb = new_block(f);
    let step_bb = new_block(f);
    let exit_bb = new_block(f);

    crate::mir::ssot::cf_common::set_jump(f, preheader, header_bb);

    let index_phi = f.next_value_id();
    let cond_v = f.next_value_id();
    crate::mir::ssot::cf_common::emit_compare_func(
        f,
        header_bb,
        cond_v,
        CompareOp::Lt,
        index_phi,
        end_v,
    );
    crate::mir::ssot::cf_common::set_branch(f, header_bb, cond_v, body_bb, exit_bb);

    let mut body_vars = base_vars.clone();
    body_vars.insert(var_name.to_string(), index_phi);
    loop_stack.push(LoopContext {
        cond_bb: header_bb,
        exit_bb,
        continue_merge_bb: Some(step_bb),
    });
    let body_end = lower_stmt_list_with_vars(f, body_bb, body, &mut body_vars, loop_stack, env)?;
    loop_stack.pop();

    if let Some(bb) = f.get_block_mut(body_end) {
        if !bb.is_terminated() {
            crate::mir::ssot::cf_common::set_jump(f, body_end, step_bb);
        }
    }

    let one_v = f.next_value_id();
    if let Some(bb) = f.get_block_mut(step_bb) {
        bb.add_instruction(MirInstruction::Const {
            dst: one_v,
            value: ConstValue::Integer(1),
        });
    }
    let next_index_v = f.next_value_id();
    if let Some(bb) = f.get_block_mut(step_bb) {
        bb.add_instruction(MirInstruction::BinOp {
            dst: next_index_v,
            op: BinaryOp::Add,
            lhs: index_phi,
            rhs: one_v,
        });
    }
    crate::mir::ssot::cf_common::set_jump(f, step_bb, header_bb);

    crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
        f,
        header_bb,
        index_phi,
        vec![(preheader, start_v), (step_bb, next_index_v)],
        Span::unknown(),
    )?;

    f.metadata.loop_range_facts.push(LoopRangeFact {
        index_name: var_name.to_string(),
        start_value: start_v,
        end_value: end_v,
        index_phi,
        preheader_bb: preheader,
        header_bb,
        body_bb,
        step_bb,
        exit_bb,
        step: 1,
        end_exclusive: true,
        index_read_only: true,
        body_writes_supported: false,
    });

    *vars = base_vars;
    Ok(exit_bb)
}

fn reject_pilot_unsupported_writes(var_name: &str, body: &[StmtV0]) -> Result<(), String> {
    for stmt in body {
        reject_stmt_write(var_name, stmt)?;
    }
    Ok(())
}

fn reject_stmt_write(var_name: &str, stmt: &StmtV0) -> Result<(), String> {
    match stmt {
        StmtV0::Local { name, .. } if name == var_name => Err(format!(
            "[freeze:contract][json_v0_bridge/loop_range_index_write] LoopRange index `{}` is read-only in LOOP-003B",
            var_name
        )),
        StmtV0::Local { name, .. } => Err(format!(
            "[freeze:contract][json_v0_bridge/loop_range_carrier_unsupported] LoopRange pilot does not yet support body writes; first write is `{}`",
            name
        )),
        StmtV0::If { then, r#else, .. } => {
            reject_pilot_unsupported_writes(var_name, then)?;
            if let Some(else_body) = r#else {
                reject_pilot_unsupported_writes(var_name, else_body)?;
            }
            Ok(())
        }
        StmtV0::Loop { body, .. } | StmtV0::LoopRange { body, .. } => {
            reject_pilot_unsupported_writes(var_name, body)
        }
        StmtV0::Try {
            try_body,
            catches,
            finally,
        } => {
            reject_pilot_unsupported_writes(var_name, try_body)?;
            for catch in catches {
                reject_pilot_unsupported_writes(var_name, &catch.body)?;
            }
            reject_pilot_unsupported_writes(var_name, finally)
        }
        StmtV0::FiniReg { prelude, fini } => {
            reject_pilot_unsupported_writes(var_name, prelude)?;
            reject_pilot_unsupported_writes(var_name, fini)
        }
        _ => Ok(()),
    }
}
