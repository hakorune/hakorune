use super::super::super::ast::{ExprV0, StmtV0};
use super::super::{BridgeEnv, LoopContext};
use crate::mir::{BasicBlockId, ConstValue, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

pub(super) fn lower_blockexpr_with_vars(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    prelude: &[StmtV0],
    tail: &serde_json::Value,
    vars: &mut BTreeMap<String, ValueId>,
) -> Result<(ValueId, BasicBlockId), String> {
    if prelude
        .iter()
        .any(|stmt| matches!(stmt, StmtV0::FiniReg { .. }))
    {
        return lower_blockexpr_with_scope_exit(env, f, cur_bb, prelude, tail, vars);
    }

    let normalized_prelude = super::super::normalize_scope_exit_registrations(prelude)?;
    let mut cur = cur_bb;
    let mut idx = 0usize;
    let mut legacy_local_chain = false;
    while idx < normalized_prelude.len() {
        let stmt = &normalized_prelude[idx];
        match stmt {
            StmtV0::Local { name, expr } => {
                if matches!(expr, ExprV0::Var { name: kw } if kw == "local") {
                    let dst = emit_null_local(f, cur, name)?;
                    vars.insert(name.clone(), dst);
                    legacy_local_chain = true;
                    idx += 1;
                    continue;
                }
                let (v, next) = super::lower_expr_with_vars(env, f, cur, expr, vars)?;
                vars.insert(name.clone(), v);
                cur = next;
                legacy_local_chain = false;
            }
            StmtV0::Expr { expr } => {
                if legacy_local_chain {
                    if let ExprV0::Var { name } = expr {
                        if !vars.contains_key(name) {
                            let dst = emit_null_local(f, cur, name)?;
                            vars.insert(name.clone(), dst);
                            idx += 1;
                            continue;
                        }
                    }
                }
                let (_v, next) = super::lower_expr_with_vars(env, f, cur, expr, vars)?;
                cur = next;
                legacy_local_chain = false;
            }
            StmtV0::Extern {
                iface,
                method,
                args,
            } => {
                let (arg_ids, next) = super::lower_args_with_vars(env, f, cur, args, vars)?;
                cur = next;
                if let Some(bb) = f.get_block_mut(cur) {
                    bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
                        None,
                        iface.clone(),
                        method.clone(),
                        arg_ids,
                        EffectMask::IO,
                    ));
                }
                legacy_local_chain = false;
            }
            StmtV0::If { .. } => {
                validate_blockexpr_prelude_stmt(stmt)?;
                let mut loop_stack: Vec<LoopContext> = Vec::new();
                cur = super::super::lower_stmt_with_vars(f, cur, stmt, vars, &mut loop_stack, env)?;
                legacy_local_chain = false;
            }
            StmtV0::Loop { .. } => {
                let mut loop_stack: Vec<LoopContext> = Vec::new();
                cur = super::super::lower_stmt_with_vars(f, cur, stmt, vars, &mut loop_stack, env)?;
                legacy_local_chain = false;
            }
            StmtV0::Try { .. } => {
                let mut loop_stack: Vec<LoopContext> = Vec::new();
                cur = super::super::lower_stmt_with_vars(f, cur, stmt, vars, &mut loop_stack, env)?;
                legacy_local_chain = false;
            }
            _ => {
                return Err(format!(
                    "[freeze:contract][json_v0][blockexpr] unsupported prelude stmt: {:?}",
                    stmt
                ));
            }
        }
        idx += 1;
    }

    // Stage-B currently emits tail as a statement wrapper: {"type":"Expr","expr":{...}}
    let tail_type = tail.get("type").and_then(|v| v.as_str());
    let tail_expr = if tail_type == Some("Expr") {
        let stmt: StmtV0 = serde_json::from_value(tail.clone()).map_err(|e| {
            format!("[freeze:contract][json_v0][blockexpr] invalid tail stmt JSON: {e}")
        })?;
        match stmt {
            StmtV0::Expr { expr } => expr,
            other => {
                return Err(format!(
                    "[freeze:contract][json_v0][blockexpr] tail must be Expr stmt, got: {:?}",
                    other
                ));
            }
        }
    } else if let Ok(stmt) = serde_json::from_value::<StmtV0>(tail.clone()) {
        validate_blockexpr_tail_stmt(&stmt)?;
        let mut loop_stack: Vec<LoopContext> = Vec::new();
        let end_bb = super::super::lower_stmt_with_vars(f, cur, &stmt, vars, &mut loop_stack, env)?;
        let dst = f.next_value_id();
        if let Some(bb) = f.get_block_mut(end_bb) {
            bb.add_instruction(MirInstruction::Const {
                dst,
                value: ConstValue::Void,
            });
        }
        return Ok((dst, end_bb));
    } else {
        serde_json::from_value::<ExprV0>(tail.clone()).map_err(|e| {
            format!("[freeze:contract][json_v0][blockexpr] invalid tail expr JSON: {e}")
        })?
    };

    super::lower_expr_with_vars(env, f, cur, &tail_expr, vars)
}

fn lower_blockexpr_with_scope_exit(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    prelude: &[StmtV0],
    tail: &serde_json::Value,
    vars: &mut BTreeMap<String, ValueId>,
) -> Result<(ValueId, BasicBlockId), String> {
    let mut stmts = prelude.to_vec();
    let mut tail_value_var: Option<String> = None;

    let tail_type = tail.get("type").and_then(|v| v.as_str());
    if tail_type == Some("Expr") {
        let stmt: StmtV0 = serde_json::from_value(tail.clone()).map_err(|e| {
            format!("[freeze:contract][json_v0][blockexpr] invalid tail stmt JSON: {e}")
        })?;
        let expr = match stmt {
            StmtV0::Expr { expr } => expr,
            other => {
                return Err(format!(
                    "[freeze:contract][json_v0][blockexpr] tail must be Expr stmt, got: {:?}",
                    other
                ));
            }
        };
        let tmp_name = fresh_blockexpr_tmp_name(vars);
        stmts.push(StmtV0::Local {
            name: tmp_name.clone(),
            expr,
        });
        tail_value_var = Some(tmp_name);
    } else if let Ok(stmt) = serde_json::from_value::<StmtV0>(tail.clone()) {
        validate_blockexpr_tail_stmt(&stmt)?;
        stmts.push(stmt);
    } else {
        let expr = serde_json::from_value::<ExprV0>(tail.clone()).map_err(|e| {
            format!("[freeze:contract][json_v0][blockexpr] invalid tail expr JSON: {e}")
        })?;
        let tmp_name = fresh_blockexpr_tmp_name(vars);
        stmts.push(StmtV0::Local {
            name: tmp_name.clone(),
            expr,
        });
        tail_value_var = Some(tmp_name);
    }

    let mut loop_stack: Vec<LoopContext> = Vec::new();
    let end_bb =
        super::super::lower_stmt_list_with_vars(f, cur_bb, &stmts, vars, &mut loop_stack, env)?;

    if let Some(name) = tail_value_var {
        if let Some(&vid) = vars.get(&name) {
            return Ok((vid, end_bb));
        }
        return Err(format!(
            "[freeze:contract][json_v0][blockexpr] missing synthetic tail local '{}'",
            name
        ));
    }

    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(end_bb) {
        if !bb.is_terminated() {
            bb.add_instruction(MirInstruction::Const {
                dst,
                value: ConstValue::Void,
            });
            return Ok((dst, end_bb));
        }
    }
    Ok((ValueId::new(0), end_bb))
}

fn fresh_blockexpr_tmp_name(vars: &BTreeMap<String, ValueId>) -> String {
    let mut idx: usize = 0;
    loop {
        let name = format!("__blockexpr_tail_tmp_{}", idx);
        if !vars.contains_key(&name) {
            return name;
        }
        idx += 1;
    }
}

fn emit_null_local(f: &mut MirFunction, cur: BasicBlockId, name: &str) -> Result<ValueId, String> {
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur) {
        bb.add_instruction(MirInstruction::Const {
            dst,
            value: ConstValue::Null,
        });
        Ok(dst)
    } else {
        Err(format!(
            "[freeze:contract][json_v0][blockexpr] missing block for legacy local '{}'",
            name
        ))
    }
}

fn validate_blockexpr_tail_stmt(stmt: &StmtV0) -> Result<(), String> {
    match stmt {
        StmtV0::Return { .. } => Ok(()),
        StmtV0::Expr { .. } | StmtV0::Local { .. } | StmtV0::Extern { .. } => Ok(()),
        StmtV0::Loop { .. } => Ok(()),
        StmtV0::If { then, r#else, .. } => {
            for s in then {
                validate_blockexpr_tail_stmt(s)?;
            }
            if let Some(elses) = r#else.as_ref() {
                for s in elses {
                    validate_blockexpr_tail_stmt(s)?;
                }
            }
            Ok(())
        }
        StmtV0::Try {
            try_body,
            catches,
            finally,
        } => {
            for s in try_body {
                validate_blockexpr_tail_stmt(s)?;
            }
            for catch in catches {
                for s in &catch.body {
                    validate_blockexpr_tail_stmt(s)?;
                }
            }
            for s in finally {
                validate_blockexpr_tail_stmt(s)?;
            }
            Ok(())
        }
        other => Err(format!(
            "[freeze:contract][json_v0][blockexpr] unsupported tail stmt: {:?}",
            other
        )),
    }
}

fn validate_blockexpr_prelude_stmt(stmt: &StmtV0) -> Result<(), String> {
    match stmt {
        StmtV0::Return { .. } => Ok(()),
        StmtV0::Expr { .. } | StmtV0::Local { .. } | StmtV0::Extern { .. } => Ok(()),
        StmtV0::If { then, r#else, .. } => {
            for s in then {
                validate_blockexpr_prelude_stmt(s)?;
            }
            if let Some(elses) = r#else.as_ref() {
                for s in elses {
                    validate_blockexpr_prelude_stmt(s)?;
                }
            }
            Ok(())
        }
        StmtV0::Try {
            try_body,
            catches,
            finally,
        } => {
            for s in try_body {
                validate_blockexpr_prelude_stmt(s)?;
            }
            for catch in catches {
                for s in &catch.body {
                    validate_blockexpr_prelude_stmt(s)?;
                }
            }
            for s in finally {
                validate_blockexpr_prelude_stmt(s)?;
            }
            Ok(())
        }
        other => Err(format!(
            "[freeze:contract][json_v0][blockexpr] unsupported prelude stmt: {:?}",
            other
        )),
    }
}
