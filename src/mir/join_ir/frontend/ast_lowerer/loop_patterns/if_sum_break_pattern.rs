//! Phase 61: If-Sum + Break pattern (dev-only)
//!
//! ## Responsibility
//! Break 付き if-sum ループを、sum/count の複数キャリアで k_exit に渡し、
//! k_exit で `sum + count` を返す。
//!
//! ## Fail-Fast Boundary
//! - Return が `Var + Var` 以外 → not matched
//! - ループ末尾の counter update が `i = i + 1` 形で検出できない → Err
//! - Ownership relay が single-hop 以外 → Err
//! - loop-carried carriers が Return の 2 変数と一致しない → Err

#![cfg(feature = "normalized_dev")]

use super::common::{
    build_join_module, create_k_exit_function, create_loop_context, parse_program_json,
    process_local_inits,
};
use super::{AstToJoinIrLowerer, JoinModule, LoweringError};
use crate::mir::join_ir::ownership::{plan_to_p3_inputs_with_relay, OwnershipAnalyzer};
use crate::mir::join_ir::{BinOpKind, JoinFunction, JoinInst, MirLikeInst};
use crate::mir::ValueId;

pub fn try_lower_if_sum_break(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<Option<JoinModule>, LoweringError> {
    let parsed = parse_program_json(program_json);

    let return_expr = parsed.stmts.last().and_then(|s| {
        if s["type"].as_str() == Some("Return") {
            s.get("expr")
        } else {
            None
        }
    });

    let Some((ret_lhs, ret_rhs)) = parse_return_var_plus_var(return_expr) else {
        return Ok(None);
    };

    let loop_node = &parsed.stmts[parsed.loop_node_idx];
    let loop_body = loop_node["body"]
        .as_array()
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "Loop must have 'body' array".to_string(),
        })?;

    let (break_if_idx, break_if_stmt) = match find_break_if(loop_body) {
        Some(v) => v,
        None => return Ok(None),
    };
    if break_if_idx != 0 {
        return Ok(None);
    }
    let break_cond_expr = &break_if_stmt["cond"];

    // Limit scope (Phase 61 dev-only): [break-if, update-if, counter-update] only.
    if loop_body.len() != 3 {
        return Ok(None);
    }

    let update_if_stmt = match find_single_update_if(loop_body, break_if_idx) {
        Some(v) => v,
        None => return Ok(None),
    };

    let counter_update_stmt = loop_body.last().expect("loop_body len checked").clone();

    let loop_var_name = detect_counter_update_loop_var(loop_body).ok_or_else(|| {
        LoweringError::InvalidLoopBody {
            message: "if-sum-break requires trailing counter update like i = i + 1".to_string(),
        }
    })?;

    if ret_lhs == loop_var_name || ret_rhs == loop_var_name || ret_lhs == ret_rhs {
        return Ok(None);
    }

    // === Ownership SSOT: param order = [loop_var] + carriers + captures ===
    let (ctx, mut entry_ctx) = create_loop_context(lowerer, &parsed);
    let init_insts = process_local_inits(lowerer, &parsed, &mut entry_ctx);

    let mut analyzer = OwnershipAnalyzer::new();
    let plans = analyzer
        .analyze_json(program_json)
        .map_err(|e| LoweringError::JsonParseError { message: e })?;

    let loop_plan = plans
        .iter()
        .find(|p| p.relay_writes.iter().any(|r| r.name == loop_var_name))
        .or_else(|| {
            plans
                .iter()
                .find(|p| p.owned_vars.iter().any(|v| v.name == loop_var_name))
        })
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "if-sum-break: failed to find loop ownership plan".to_string(),
        })?;

    let inputs = plan_to_p3_inputs_with_relay(loop_plan, &loop_var_name)
        .map_err(|e| LoweringError::JsonParseError { message: e })?;

    // Ensure carriers are exactly the return vars (fail-fast mixing protection).
    let carrier_names: std::collections::BTreeSet<String> =
        inputs.carriers.iter().map(|c| c.name.clone()).collect();
    let expected: std::collections::BTreeSet<String> =
        [ret_lhs.clone(), ret_rhs.clone()].into_iter().collect();

    if carrier_names != expected {
        return Err(LoweringError::InvalidLoopBody {
            message: format!(
                "if-sum-break: carriers {:?} must equal return vars {:?}",
                carrier_names, expected
            ),
        });
    }

    let mut param_order: Vec<(String, ValueId)> = Vec::new();
    let mut seen = std::collections::BTreeSet::<String>::new();

    let loop_var_id =
        entry_ctx
            .get_var(&loop_var_name)
            .ok_or_else(|| LoweringError::InvalidLoopBody {
                message: format!(
                    "loop var '{}' must be initialized before loop",
                    loop_var_name
                ),
            })?;
    param_order.push((loop_var_name.clone(), loop_var_id));
    seen.insert(loop_var_name.clone());

    for carrier in &inputs.carriers {
        let id =
            entry_ctx
                .get_var(&carrier.name)
                .ok_or_else(|| LoweringError::InvalidLoopBody {
                    message: format!("carrier '{}' must be initialized before loop", carrier.name),
                })?;
        param_order.push((carrier.name.clone(), id));
        seen.insert(carrier.name.clone());
    }

    for cap_name in &inputs.captures {
        if seen.contains(cap_name) {
            continue;
        }
        if let Some(id) = entry_ctx.get_var(cap_name) {
            param_order.push((cap_name.clone(), id));
            seen.insert(cap_name.clone());
        }
    }

    // Include remaining params/vars deterministically
    for (name, var_id) in &entry_ctx.var_map {
        if !seen.contains(name) {
            param_order.push((name.clone(), *var_id));
            seen.insert(name.clone());
        }
    }

    let entry_func =
        create_entry_function_if_sum_break(&ctx, &parsed, init_insts, &mut entry_ctx, &param_order);

    let loop_step_func = create_loop_step_function_if_sum_break(
        lowerer,
        &ctx,
        &parsed.func_name,
        &loop_node["cond"],
        break_cond_expr,
        update_if_stmt,
        &counter_update_stmt,
        &param_order,
        &loop_var_name,
        &ret_lhs,
        &ret_rhs,
    )?;

    let k_exit_func = create_k_exit_function(&ctx, &parsed.func_name);

    Ok(Some(build_join_module(
        entry_func,
        loop_step_func,
        k_exit_func,
    )))
}

fn parse_return_var_plus_var(expr: Option<&serde_json::Value>) -> Option<(String, String)> {
    let expr = expr?;
    if expr["type"].as_str()? != "Binary" {
        return None;
    }
    if expr["op"].as_str()? != "+" {
        return None;
    }
    let lhs = expr["lhs"].as_object()?;
    let rhs = expr["rhs"].as_object()?;
    if lhs.get("type")?.as_str()? != "Var" || rhs.get("type")?.as_str()? != "Var" {
        return None;
    }
    Some((
        lhs.get("name")?.as_str()?.to_string(),
        rhs.get("name")?.as_str()?.to_string(),
    ))
}

fn find_break_if(loop_body: &[serde_json::Value]) -> Option<(usize, &serde_json::Value)> {
    loop_body.iter().enumerate().find(|(_, stmt)| {
        stmt["type"].as_str() == Some("If")
            && stmt["then"].as_array().map_or(false, |then| {
                then.iter().any(|s| s["type"].as_str() == Some("Break"))
            })
    })
}

fn detect_counter_update_loop_var(loop_body: &[serde_json::Value]) -> Option<String> {
    let last = loop_body.last()?;
    if last["type"].as_str()? != "Local" {
        return None;
    }
    let name = last["name"].as_str()?.to_string();
    let expr = last.get("expr")?;
    if expr["type"].as_str()? != "Binary" || expr["op"].as_str()? != "+" {
        return None;
    }
    let lhs = &expr["lhs"];
    let rhs = &expr["rhs"];
    if lhs["type"].as_str()? != "Var" || lhs["name"].as_str()? != name {
        return None;
    }
    if rhs["type"].as_str()? != "Int" || rhs["value"].as_i64()? != 1 {
        return None;
    }
    Some(name)
}

fn create_entry_function_if_sum_break(
    ctx: &super::common::LoopContext,
    parsed: &super::common::ParsedProgram,
    init_insts: Vec<JoinInst>,
    entry_ctx: &mut super::super::context::ExtractCtx,
    param_order: &[(String, ValueId)],
) -> JoinFunction {
    let loop_args: Vec<ValueId> = param_order.iter().map(|(_, id)| *id).collect();
    let loop_result = entry_ctx.alloc_var();
    let mut body = init_insts;
    body.push(JoinInst::Call {
        func: ctx.loop_step_id,
        args: loop_args,
        k_next: None,
        dst: Some(loop_result),
    });
    body.push(JoinInst::Ret {
        value: Some(loop_result),
    });
    JoinFunction {
        id: ctx.entry_id,
        name: parsed.func_name.clone(),
        params: (0..parsed.param_names.len())
            .map(|i| ValueId(i as u32))
            .collect(),
        body,
        exit_cont: None,
    }
}

fn create_loop_step_function_if_sum_break(
    lowerer: &mut AstToJoinIrLowerer,
    ctx: &super::common::LoopContext,
    func_name: &str,
    loop_cond_expr: &serde_json::Value,
    break_cond_expr: &serde_json::Value,
    update_if_stmt: &serde_json::Value,
    counter_update_stmt: &serde_json::Value,
    param_order: &[(String, ValueId)],
    loop_var_name: &str,
    sum_name: &str,
    count_name: &str,
) -> Result<JoinFunction, LoweringError> {
    use super::super::context::ExtractCtx;

    let param_names: Vec<String> = param_order.iter().map(|(name, _)| name.clone()).collect();
    let mut step_ctx = ExtractCtx::new(param_names.len() as u32);
    for (idx, name) in param_names.iter().enumerate() {
        step_ctx.register_param(name.clone(), ValueId(idx as u32));
    }

    let mut body = Vec::new();

    // Header condition: if !loop_cond -> exit with (sum, count)
    let (loop_cond_var, loop_cond_insts) = lowerer.extract_value(loop_cond_expr, &mut step_ctx);
    body.extend(loop_cond_insts);
    let header_exit_flag = step_ctx.alloc_var();
    body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
        dst: header_exit_flag,
        op: crate::mir::join_ir::UnaryOp::Not,
        operand: loop_cond_var,
    }));
    let sum_before = step_ctx
        .get_var(sum_name)
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: format!("{} must exist", sum_name),
        })?;
    let count_before =
        step_ctx
            .get_var(count_name)
            .ok_or_else(|| LoweringError::InvalidLoopBody {
                message: format!("{} must exist", count_name),
            })?;
    let acc_before = step_ctx.alloc_var();
    body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: acc_before,
        op: BinOpKind::Add,
        lhs: sum_before,
        rhs: count_before,
    }));
    body.push(JoinInst::Jump {
        cont: ctx.k_exit_id.as_cont(),
        args: vec![acc_before],
        cond: Some(header_exit_flag),
    });

    // Break condition: if break_cond -> exit with (sum, count)
    let (break_cond_var, break_cond_insts) = lowerer.extract_value(break_cond_expr, &mut step_ctx);
    body.extend(break_cond_insts);
    body.push(JoinInst::Jump {
        cont: ctx.k_exit_id.as_cont(),
        args: vec![acc_before],
        cond: Some(break_cond_var),
    });

    // Update-if: cond ? then_update : else_update (Select-based, no if-in-loop lowering)
    let update_cond_expr = &update_if_stmt["cond"];
    let (update_cond_var, update_cond_insts) =
        lowerer.extract_value(update_cond_expr, &mut step_ctx);
    body.extend(update_cond_insts);

    let (sum_then_expr, sum_else_expr) = extract_if_branch_assignment(update_if_stmt, sum_name)?;
    let (count_then_expr, count_else_expr) =
        extract_if_branch_assignment(update_if_stmt, count_name)?;

    let (sum_then_val, sum_then_insts) = lowerer.extract_value(&sum_then_expr, &mut step_ctx);
    let (sum_else_val, sum_else_insts) = lowerer.extract_value(&sum_else_expr, &mut step_ctx);
    let (count_then_val, count_then_insts) = lowerer.extract_value(&count_then_expr, &mut step_ctx);
    let (count_else_val, count_else_insts) = lowerer.extract_value(&count_else_expr, &mut step_ctx);
    body.extend(sum_then_insts);
    body.extend(sum_else_insts);
    body.extend(count_then_insts);
    body.extend(count_else_insts);

    let sum_next = step_ctx.alloc_var();
    body.push(JoinInst::Compute(MirLikeInst::Select {
        dst: sum_next,
        cond: update_cond_var,
        then_val: sum_then_val,
        else_val: sum_else_val,
    }));
    step_ctx.register_param(sum_name.to_string(), sum_next);

    let count_next = step_ctx.alloc_var();
    body.push(JoinInst::Compute(MirLikeInst::Select {
        dst: count_next,
        cond: update_cond_var,
        then_val: count_then_val,
        else_val: count_else_val,
    }));
    step_ctx.register_param(count_name.to_string(), count_next);

    // Counter update (must update loop var)
    let counter_expr =
        counter_update_stmt
            .get("expr")
            .ok_or_else(|| LoweringError::InvalidLoopBody {
                message: "counter update must have 'expr'".to_string(),
            })?;
    let (i_next, i_insts) = lowerer.extract_value(counter_expr, &mut step_ctx);
    body.extend(i_insts);
    step_ctx.register_param(loop_var_name.to_string(), i_next);

    // Recurse with updated params.
    let recurse_result = step_ctx.alloc_var();
    let mut recurse_args = Vec::new();
    for name in &param_names {
        let arg = step_ctx
            .get_var(name)
            .unwrap_or_else(|| panic!("param {} must exist", name));
        recurse_args.push(arg);
    }
    body.push(JoinInst::Call {
        func: ctx.loop_step_id,
        args: recurse_args,
        k_next: None,
        dst: Some(recurse_result),
    });
    body.push(JoinInst::Ret {
        value: Some(recurse_result),
    });

    Ok(JoinFunction {
        id: ctx.loop_step_id,
        name: format!("{}_loop_step", func_name),
        params: (0..param_names.len()).map(|i| ValueId(i as u32)).collect(),
        body,
        exit_cont: None,
    })
}

fn find_single_update_if<'a>(
    loop_body: &'a [serde_json::Value],
    break_if_idx: usize,
) -> Option<&'a serde_json::Value> {
    let mut found: Option<&serde_json::Value> = None;
    for (idx, stmt) in loop_body.iter().enumerate() {
        if idx == break_if_idx {
            continue;
        }
        if stmt["type"].as_str() == Some("If") {
            if found.is_some() {
                return None;
            }
            found = Some(stmt);
        }
    }
    found
}

fn extract_if_branch_assignment(
    if_stmt: &serde_json::Value,
    target: &str,
) -> Result<(serde_json::Value, serde_json::Value), LoweringError> {
    fn find_assignment_expr(
        branch: &[serde_json::Value],
        target: &str,
    ) -> Result<Option<serde_json::Value>, LoweringError> {
        let mut found: Option<serde_json::Value> = None;
        for stmt in branch {
            match stmt["type"].as_str() {
                Some("Local") => {
                    let name =
                        stmt["name"]
                            .as_str()
                            .ok_or_else(|| LoweringError::InvalidLoopBody {
                                message: "Local must have 'name'".to_string(),
                            })?;
                    if name != target {
                        continue;
                    }
                    if found.is_some() {
                        return Err(LoweringError::InvalidLoopBody {
                            message: format!("if-sum-break: multiple assignments to '{}'", target),
                        });
                    }
                    let expr = stmt
                        .get("expr")
                        .ok_or_else(|| LoweringError::InvalidLoopBody {
                            message: "Local must have 'expr'".to_string(),
                        })?;
                    found = Some(expr.clone());
                }
                Some("Assignment") | Some("Assign") => {
                    let name =
                        stmt["target"]
                            .as_str()
                            .ok_or_else(|| LoweringError::InvalidLoopBody {
                                message: "Assignment must have 'target'".to_string(),
                            })?;
                    if name != target {
                        continue;
                    }
                    if found.is_some() {
                        return Err(LoweringError::InvalidLoopBody {
                            message: format!("if-sum-break: multiple assignments to '{}'", target),
                        });
                    }
                    let expr = stmt
                        .get("expr")
                        .or_else(|| stmt.get("value"))
                        .ok_or_else(|| LoweringError::InvalidLoopBody {
                            message: "Assignment must have 'expr' or 'value'".to_string(),
                        })?;
                    found = Some(expr.clone());
                }
                _ => {
                    return Err(LoweringError::InvalidLoopBody {
                        message: "if-sum-break: unsupported statement in update if".to_string(),
                    });
                }
            }
        }
        Ok(found)
    }

    let then_branch = if_stmt["then"]
        .as_array()
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "If must have 'then' array".to_string(),
        })?;
    let else_branch = if_stmt["else"]
        .as_array()
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "If must have 'else' array".to_string(),
        })?;

    let then_expr = find_assignment_expr(then_branch, target)?
        .unwrap_or_else(|| serde_json::json!({"type":"Var","name":target}));
    let else_expr = find_assignment_expr(else_branch, target)?
        .unwrap_or_else(|| serde_json::json!({"type":"Var","name":target}));
    Ok((then_expr, else_expr))
}
