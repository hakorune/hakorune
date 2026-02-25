//! Phase P4: Continue パターン lowering
//!
//! ## 責務（1行で表現）
//! **if continue 条件で処理をスキップするループを Select に落とす**
//!
//! ## パターン例
//! ```nyash
//! loop(i < n) {
//!     i = i + 1
//!     if i == 3 { continue }
//!     acc = acc + i
//! }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - entry 関数: Call(loop_step)
//! - loop_step 関数:
//!   - exit 条件チェック
//!   - i++ 処理
//!   - continue 条件評価
//!   - Select: continue なら acc そのまま、そうでなければ更新
//!   - 再帰
//! - k_exit 関数: Return(acc)

use super::common::{
    build_join_module, create_k_exit_function, create_loop_context, parse_program_json,
    process_local_inits,
};
use super::step_calculator::StepCalculator;
use super::{AstToJoinIrLowerer, JoinModule, LoweringError};
use crate::mir::join_ir::{CompareOp, ConstValue, JoinFunction, JoinInst, MirLikeInst};
use crate::mir::ValueId;

/// Continue パターンを JoinModule に変換
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
pub fn lower(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    // 1. Program(JSON) をパース
    let parsed = parse_program_json(program_json);

    // 2. LoopContext と entry_ctx を作成
    let (ctx, mut entry_ctx) = create_loop_context(lowerer, &parsed);

    // 3. Local 初期化を処理
    let init_insts = process_local_inits(lowerer, &parsed, &mut entry_ctx);

    // 4. Loop の cond を取得
    let loop_node = &parsed.stmts[parsed.loop_node_idx];
    let loop_cond_expr = &loop_node["cond"];

    // 5. Loop body から Continue If を探す
    let loop_body = loop_node["body"]
        .as_array()
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "Loop must have 'body' array".to_string(),
        })?;

    let continue_if_stmt = loop_body
        .iter()
        .find(|stmt| {
            stmt["type"].as_str() == Some("If")
                && stmt["then"].as_array().map_or(false, |then| {
                    then.iter().any(|s| s["type"].as_str() == Some("Continue"))
                })
        })
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "Continue pattern must have If + Continue".to_string(),
        })?;

    let continue_cond_expr = &continue_if_stmt["cond"];

    // 6. entry 関数を生成
    let entry_func = create_entry_function_continue(&ctx, &parsed, init_insts, &mut entry_ctx);

    // 7. loop_step 関数を生成
    let loop_step_func = create_loop_step_function_continue(
        lowerer,
        &ctx,
        &parsed.func_name,
        loop_cond_expr,
        continue_cond_expr,
        continue_if_stmt,
        loop_body,
    )?;

    // 8. k_exit 関数を生成
    let k_exit_func = create_k_exit_function(&ctx, &parsed.func_name);

    // 9. JoinModule を構築
    Ok(build_join_module(entry_func, loop_step_func, k_exit_func))
}

/// Continue パターン用 entry 関数を生成
fn create_entry_function_continue(
    ctx: &super::common::LoopContext,
    parsed: &super::common::ParsedProgram,
    init_insts: Vec<JoinInst>,
    entry_ctx: &mut super::super::context::ExtractCtx,
) -> JoinFunction {
    // i, acc, n を取得
    let i_init = entry_ctx.get_var("i").expect("i must be initialized");
    let acc_init = entry_ctx.get_var("acc").expect("acc must be initialized");
    let n_param = entry_ctx.get_var("n").expect("n must be parameter");

    let loop_result = entry_ctx.alloc_var();

    let mut body = init_insts;
    body.push(JoinInst::Call {
        func: ctx.loop_step_id,
        args: vec![i_init, acc_init, n_param],
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

/// Continue パターン用 loop_step 関数を生成
fn create_loop_step_function_continue(
    lowerer: &mut AstToJoinIrLowerer,
    ctx: &super::common::LoopContext,
    func_name: &str,
    loop_cond_expr: &serde_json::Value,
    continue_cond_expr: &serde_json::Value,
    continue_if_stmt: &serde_json::Value,
    loop_body: &[serde_json::Value],
) -> Result<JoinFunction, LoweringError> {
    use super::super::context::ExtractCtx;

    let step_i = ValueId(0);
    let step_acc = ValueId(1);
    let step_n = ValueId(2);

    let mut step_ctx = ExtractCtx::new(3);
    step_ctx.register_param("i".to_string(), step_i);
    step_ctx.register_param("acc".to_string(), step_acc);
    step_ctx.register_param("n".to_string(), step_n);

    let mut body = Vec::new();

    // 1. exit 条件チェック（!(i < n) = i >= n で抜ける）
    let (cond_var, cond_insts) = lowerer.extract_value(loop_cond_expr, &mut step_ctx);
    body.extend(cond_insts);

    let false_const = step_ctx.alloc_var();
    let exit_cond = step_ctx.alloc_var();

    body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: false_const,
        value: ConstValue::Bool(false),
    }));
    body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: exit_cond,
        op: CompareOp::Eq,
        lhs: cond_var,
        rhs: false_const,
    }));

    body.push(JoinInst::Jump {
        cont: ctx.k_exit_id.as_cont(),
        args: vec![step_acc],
        cond: Some(exit_cond),
    });

    // 2. Continue pattern 特有: i のインクリメントが先
    let first_local = loop_body
        .iter()
        .find(|stmt| stmt["type"].as_str() == Some("Local") && stmt["name"].as_str() == Some("i"))
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: format!(
                "Continue pattern validation failed: missing 'i' increment.\n\
                 Expected: First statement in loop body must be 'local i = i + K' where K is a constant.\n\
                 Found: Loop body does not contain 'local i = ...' statement.\n\
                 Hint: Add 'i = i + 1' as the first statement inside the loop body."
            ),
        })?;

    let i_expr = &first_local["expr"];
    let (i_next, i_insts) = lowerer.extract_value(i_expr, &mut step_ctx);
    body.extend(i_insts);
    step_ctx.register_param("i".to_string(), i_next);

    // Phase 88: Continue 分岐で i を追加更新できるようにする（例: i += 2）
    //
    // `continue_if` の then 内に `Local i = i + K` がある場合、
    // loop_body の通常更新 `Local i = i + K0` との差分 (K-K0) を i_next に加算して
    // continue パスの次 i を構成する（Linear increment のみ対応）。
    //
    // Phase 89-2: StepCalculator Box に抽出済み（再利用性向上）

    let mut i_next_continue = i_next;
    let continue_then =
        continue_if_stmt["then"]
            .as_array()
            .ok_or_else(|| LoweringError::InvalidLoopBody {
                message: "Continue pattern If must have 'then' array".to_string(),
            })?;
    if let Some(then_i_local) = continue_then
        .iter()
        .find(|stmt| stmt["type"].as_str() == Some("Local") && stmt["name"].as_str() == Some("i"))
    {
        let base_k = StepCalculator::extract_linear_increment(i_expr, "i").ok_or_else(|| {
            let expr_debug = serde_json::to_string(i_expr)
                .unwrap_or_else(|_| "<invalid JSON>".to_string());
            LoweringError::InvalidLoopBody {
                message: format!(
                    "Continue pattern validation failed: invalid step increment form.\n\
                     Expected: i = i + const (e.g., 'i = i + 1', 'i = i + 2').\n\
                     Found: {}\n\
                     Hint: Change the 'i' update to addition form 'i = i + K' where K is a constant integer.",
                    expr_debug
                ),
            }
        })?;
        let then_k = StepCalculator::extract_linear_increment(&then_i_local["expr"], "i")
            .ok_or_else(|| {
                let expr_debug = serde_json::to_string(&then_i_local["expr"])
                    .unwrap_or_else(|_| "<invalid JSON>".to_string());
                LoweringError::InvalidLoopBody {
                    message: format!(
                        "Continue pattern validation failed: invalid 'then' branch step increment.\n\
                         Expected: In 'if ... {{ continue }}' block, 'i = i + const' (e.g., 'i = i + 2').\n\
                         Found: {}\n\
                         Hint: Ensure the continue block updates 'i' using addition form 'i = i + K'.",
                        expr_debug
                    ),
                }
            })?;
        let delta = StepCalculator::calculate_step_difference(then_k, base_k);
        if delta != 0 {
            let delta_const = step_ctx.alloc_var();
            body.push(JoinInst::Compute(MirLikeInst::Const {
                dst: delta_const,
                value: ConstValue::Integer(delta),
            }));
            let bumped = step_ctx.alloc_var();
            body.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: bumped,
                op: crate::mir::join_ir::BinOpKind::Add,
                lhs: i_next,
                rhs: delta_const,
            }));
            i_next_continue = bumped;
        }
    }

    // 3. Continue 条件を評価
    let (continue_cond_var, continue_cond_insts) =
        lowerer.extract_value(continue_cond_expr, &mut step_ctx);
    body.extend(continue_cond_insts);

    // 4. acc の更新値を計算
    let acc_update_local = loop_body
        .iter()
        .find(|stmt| stmt["type"].as_str() == Some("Local") && stmt["name"].as_str() == Some("acc"))
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: format!(
                "Continue pattern validation failed: missing accumulator update.\n\
                 Expected: Loop body must contain 'local acc = ...' statement.\n\
                 Found: No 'acc' update found in loop body.\n\
                 Hint: Add 'acc = acc + <value>' or similar accumulator update."
            ),
        })?;

    let acc_expr = &acc_update_local["expr"];
    let (acc_increment, acc_insts) = lowerer.extract_value(acc_expr, &mut step_ctx);
    body.extend(acc_insts);

    // Phase 88: Continue 分岐側でも acc を更新できるようにする（例: acc += 1）
    let mut acc_then_val = step_acc;
    if let Some(then_acc_local) = continue_then
        .iter()
        .find(|stmt| stmt["type"].as_str() == Some("Local") && stmt["name"].as_str() == Some("acc"))
    {
        let (acc_then, acc_then_insts) =
            lowerer.extract_value(&then_acc_local["expr"], &mut step_ctx);
        body.extend(acc_then_insts);
        acc_then_val = acc_then;
    }

    // 5. Select: Continue/通常 で acc を切り替える
    let acc_next = step_ctx.alloc_var();
    body.push(JoinInst::Select {
        dst: acc_next,
        cond: continue_cond_var,
        then_val: acc_then_val,
        else_val: acc_increment,
        type_hint: None, // Phase 63-3
    });

    // Phase 88: Continue/通常 で次 i を切り替える
    let i_next_selected = step_ctx.alloc_var();
    body.push(JoinInst::Select {
        dst: i_next_selected,
        cond: continue_cond_var,
        then_val: i_next_continue,
        else_val: i_next,
        type_hint: None,
    });

    // 6. 末尾再帰
    let recurse_result = step_ctx.alloc_var();
    body.push(JoinInst::Call {
        func: ctx.loop_step_id,
        args: vec![i_next_selected, acc_next, step_n],
        k_next: None,
        dst: Some(recurse_result),
    });
    body.push(JoinInst::Ret {
        value: Some(recurse_result),
    });

    Ok(JoinFunction {
        id: ctx.loop_step_id,
        name: format!("{}_loop_step", func_name),
        params: vec![step_i, step_acc, step_n],
        body,
        exit_cont: None,
    })
}
