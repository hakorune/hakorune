//! Phase P4: Break パターン lowering
//!
//! ## 責務（1行で表現）
//! **if break 条件で早期 return するループを Jump(k_exit, cond) に落とす**
//!
//! ## パターン例
//! ```nyash
//! loop {
//!     if i >= n { break }
//!     acc = acc + i
//!     i = i + 1
//! }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - entry 関数: Call(loop_step)
//! - loop_step 関数:
//!   - break 条件評価
//!   - true: Jump(k_exit, acc)
//!   - false: body 処理 + 再帰
//! - k_exit 関数: Return(acc)

use super::common::{
    build_join_module, create_k_exit_function, create_loop_context, parse_program_json,
    process_local_inits,
};
use super::param_guess::{build_param_order, compute_param_guess};
use super::{AstToJoinIrLowerer, JoinModule, LoweringError};
use crate::mir::join_ir::{JoinFunction, JoinInst};
use crate::mir::ValueId;

/// Break パターンを JoinModule に変換
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
pub fn lower(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    lower_legacy_param_guess(lowerer, program_json)
}

/// Legacy Break lowering (Phase P4) using param_guess heuristics.
///
/// This remains as a fallback and is also used for Phase 60 comparison tests.
fn lower_legacy_param_guess(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    // 1. Program(JSON) をパース
    let parsed = parse_program_json(program_json);

    // 2. LoopContext と entry_ctx を作成
    let (ctx, mut entry_ctx) = create_loop_context(lowerer, &parsed);

    // 3. Local 初期化を処理
    let init_insts = process_local_inits(lowerer, &parsed, &mut entry_ctx);

    // 4. Loop body から Break If を探す
    let loop_node = &parsed.stmts[parsed.loop_node_idx];
    let loop_body = loop_node["body"]
        .as_array()
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "Loop must have 'body' array".to_string(),
        })?;

    let (break_if_idx, break_if_stmt) = loop_body
        .iter()
        .enumerate()
        .find(|(_, stmt)| {
            stmt["type"].as_str() == Some("If")
                && stmt["then"].as_array().map_or(false, |then| {
                    then.iter().any(|s| s["type"].as_str() == Some("Break"))
                })
        })
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "Break pattern must have If + Break".to_string(),
        })?;

    let break_cond_expr = &break_if_stmt["cond"];

    let param_guess = compute_param_guess(&entry_ctx);
    let param_order = build_param_order(&param_guess, &entry_ctx);
    let loop_var_name = param_guess.loop_var.0.clone();
    let acc_name = param_guess.acc.0.clone();
    let loop_cond_expr = &loop_node["cond"];

    // 5. entry 関数を生成
    let entry_func =
        create_entry_function_break(&ctx, &parsed, init_insts, &mut entry_ctx, &param_order);

    // 6. loop_step 関数を生成
    let loop_step_func = create_loop_step_function_break(
        lowerer,
        &ctx,
        &parsed.func_name,
        loop_cond_expr,
        break_cond_expr,
        loop_body,
        &param_order,
        &loop_var_name,
        &acc_name,
        break_if_idx,
    )?;

    // 7. k_exit 関数を生成
    let k_exit_func = create_k_exit_function(&ctx, &parsed.func_name);

    // 8. JoinModule を構築
    Ok(build_join_module(entry_func, loop_step_func, k_exit_func))
}

/// Break パターン用 entry 関数を生成
fn create_entry_function_break(
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

/// Break パターン用 loop_step 関数を生成
fn create_loop_step_function_break(
    lowerer: &mut AstToJoinIrLowerer,
    ctx: &super::common::LoopContext,
    func_name: &str,
    loop_cond_expr: &serde_json::Value,
    break_cond_expr: &serde_json::Value,
    loop_body: &[serde_json::Value],
    param_order: &[(String, ValueId)],
    loop_var_name: &str,
    acc_name: &str,
    break_if_idx: usize,
) -> Result<JoinFunction, LoweringError> {
    use super::super::context::ExtractCtx;

    let param_names: Vec<String> = param_order.iter().map(|(name, _)| name.clone()).collect();

    let mut step_ctx = ExtractCtx::new(param_names.len() as u32);
    for (idx, name) in param_names.iter().enumerate() {
        step_ctx.register_param(name.clone(), ValueId(idx as u32));
    }

    let mut body = Vec::new();

    let (loop_cond_var, loop_cond_insts) = lowerer.extract_value(loop_cond_expr, &mut step_ctx);
    body.extend(loop_cond_insts);
    let acc_current = step_ctx
        .get_var(acc_name)
        .unwrap_or_else(|| panic!("{} must be initialized", acc_name));
    let header_exit_flag = step_ctx.alloc_var();
    body.push(JoinInst::Compute(
        crate::mir::join_ir::MirLikeInst::UnaryOp {
            dst: header_exit_flag,
            op: crate::mir::join_ir::UnaryOp::Not,
            operand: loop_cond_var,
        },
    ));
    body.push(JoinInst::Jump {
        cont: ctx.k_exit_id.as_cont(),
        args: vec![acc_current],
        cond: Some(header_exit_flag),
    });

    for stmt in loop_body.iter().take(break_if_idx) {
        if stmt["type"].as_str() == Some("If") {
            continue;
        }
        let (insts, _effect) = lowerer.lower_statement(stmt, &mut step_ctx);
        body.extend(insts);
    }

    // Break 条件を評価（break_if までの body-local を評価したあと）
    let (break_cond_var, break_cond_insts) = lowerer.extract_value(break_cond_expr, &mut step_ctx);
    body.extend(break_cond_insts);

    // 早期 return: break_cond が true なら k_exit へ Jump
    body.push(JoinInst::Jump {
        cont: ctx.k_exit_id.as_cont(),
        args: vec![acc_current],
        cond: Some(break_cond_var),
    });

    // Loop body を処理（If + Break はスキップ）
    for body_stmt in loop_body.iter().skip(break_if_idx + 1) {
        if body_stmt["type"].as_str() == Some("If") {
            continue;
        }

        let (insts, _effect) = lowerer.lower_statement(body_stmt, &mut step_ctx);
        body.extend(insts);
    }

    // 再帰呼び出し
    let i_next = step_ctx
        .get_var(loop_var_name)
        .unwrap_or_else(|| panic!("{} must be updated", loop_var_name));
    let acc_next = step_ctx
        .get_var(acc_name)
        .unwrap_or_else(|| panic!("{} must be updated", acc_name));

    let recurse_result = step_ctx.alloc_var();
    let mut recurse_args = Vec::new();
    for name in &param_names {
        let arg = if name == loop_var_name {
            i_next
        } else if name == acc_name {
            acc_next
        } else {
            step_ctx
                .get_var(name)
                .unwrap_or_else(|| panic!("param {} must exist", name))
        };
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
