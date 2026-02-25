//! Phase 89 P1: ContinueReturn パターン lowering
//!
//! ## 責務（1行で表現）
//! **continue + early return 両方を持つループを複合処理で JoinIR に落とす**
//!
//! ## パターン例
//! ```nyash
//! loop(i < n) {
//!     if i == 5 { return acc }  // Early return
//!     if i == 2 {
//!         i = i + 1
//!         continue
//!     }
//!     acc = acc + 1
//!     i = i + 1
//! }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - entry 関数: Call(loop_step)
//! - loop_step 関数:
//!   - exit 条件チェック (i >= n で k_exit)
//!   - early return 条件チェック (i == 5 で Jump(k_exit, acc))
//!   - continue 条件チェック
//!   - Select: continue なら acc そのまま、そうでなければ更新
//!   - Select: continue なら i 追加更新、そうでなければ通常更新
//!   - 再帰
//! - k_exit 関数: Return(acc)
//!
//! ## 設計原則
//! - **StepCalculator 再利用**: Phase 89-2 の成果活用
//! - **Fail-Fast**: 複数 Return/Continue、Return が then 以外などは明示エラー
//! - **境界明確**: continue_pattern.rs の設計を継承

use super::common::{
    build_join_module, create_k_exit_function, create_loop_context, parse_program_json,
    process_local_inits,
};
use super::step_calculator::StepCalculator;
use super::{AstToJoinIrLowerer, JoinModule, LoweringError};
use crate::mir::join_ir::{CompareOp, ConstValue, JoinFunction, JoinInst, MirLikeInst};
use crate::mir::ValueId;

/// Refactor-B: JSON 値の等価性チェック（serde_json::Value の PartialEq を使用）
fn json_values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    a == b
}

/// ContinueReturn パターンを JoinModule に変換
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

    // 5. Loop body から Return If と Continue If を探す
    let loop_body = loop_node["body"]
        .as_array()
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "Loop must have 'body' array".to_string(),
        })?;

    // 5-1. Return If を探す（Refactor-B: 複数許可、値の等価性チェック）
    let return_if_stmts: Vec<_> = loop_body
        .iter()
        .filter(|stmt| {
            stmt["type"].as_str() == Some("If")
                && stmt["then"].as_array().map_or(false, |then| {
                    then.iter().any(|s| s["type"].as_str() == Some("Return"))
                })
        })
        .collect();

    if return_if_stmts.is_empty() {
        return Err(LoweringError::InvalidLoopBody {
            message: "ContinueReturn pattern must have If + Return".to_string(),
        });
    }

    // Refactor-B: 複数の return-if がある場合、返す値が全て同一かチェック
    if return_if_stmts.len() > 1 {
        // 各 return-if の return 値を抽出
        let return_values: Vec<&serde_json::Value> = return_if_stmts
            .iter()
            .map(|stmt| {
                let then = stmt["then"].as_array().expect("then must be array");
                let ret = then
                    .iter()
                    .find(|s| s["type"].as_str() == Some("Return"))
                    .expect("return must exist");
                &ret["expr"]
            })
            .collect();

        // 最初の値と他の値を比較
        let first_value = return_values[0];
        for (i, value) in return_values.iter().enumerate().skip(1) {
            if !json_values_equal(first_value, value) {
                return Err(LoweringError::InvalidLoopBody {
                    message: format!(
                        "ContinueReturn pattern validation failed: multiple return-if statements with different values.\n\
                         Expected: All early returns must have identical values.\n\
                         First return: {:?}\n\
                         Return #{}: {:?}\n\
                         Hint: Ensure all early returns have identical values, or combine conditions.",
                        first_value, i + 1, value
                    ),
                });
            }
        }
    }

    let return_if_stmt = return_if_stmts[0];

    // 5-2. Return が then にあることを確認（Fail-Fast）
    let return_then =
        return_if_stmt["then"]
            .as_array()
            .ok_or_else(|| LoweringError::InvalidLoopBody {
                message: "Return If must have 'then' array".to_string(),
            })?;

    let return_stmt = return_then
        .iter()
        .find(|s| s["type"].as_str() == Some("Return"))
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: "Return If 'then' must contain Return statement".to_string(),
        })?;

    // else に Return があったら Fail-Fast
    if let Some(else_branch) = return_if_stmt["else"].as_array() {
        if else_branch
            .iter()
            .any(|s| s["type"].as_str() == Some("Return"))
        {
            return Err(LoweringError::InvalidLoopBody {
                message: "ContinueReturn pattern validation failed: Return in 'else' branch.\n\
                     Expected: Return statement only in 'then' branch.\n\
                     Found: Return in 'else' branch.\n\
                     Hint: Move Return statement to 'then' branch or invert condition."
                    .to_string(),
            });
        }
    }

    let return_cond_expr = &return_if_stmt["cond"];
    let return_expr = &return_stmt["expr"];

    // 5-3. Continue If を探す（Fail-Fast: 複数あればエラー）
    let continue_if_stmts: Vec<_> = loop_body
        .iter()
        .filter(|stmt| {
            stmt["type"].as_str() == Some("If")
                && stmt["then"].as_array().map_or(false, |then| {
                    then.iter().any(|s| s["type"].as_str() == Some("Continue"))
                })
        })
        .collect();

    if continue_if_stmts.is_empty() {
        return Err(LoweringError::InvalidLoopBody {
            message: "ContinueReturn pattern must have If + Continue".to_string(),
        });
    }

    if continue_if_stmts.len() > 1 {
        return Err(LoweringError::InvalidLoopBody {
            message: format!(
                "ContinueReturn pattern validation failed: multiple Continue statements found.\n\
                 Expected: Exactly one 'if {{ continue }}' statement in loop body.\n\
                 Found: {} Continue statements.\n\
                 Hint: Combine multiple continue conditions into a single if statement.",
                continue_if_stmts.len()
            ),
        });
    }

    let continue_if_stmt = continue_if_stmts[0];
    let continue_cond_expr = &continue_if_stmt["cond"];

    // 6. entry 関数を生成
    let entry_func =
        create_entry_function_continue_return(&ctx, &parsed, init_insts, &mut entry_ctx);

    // 7. loop_step 関数を生成
    let loop_step_func = create_loop_step_function_continue_return(
        lowerer,
        &ctx,
        &parsed.func_name,
        loop_cond_expr,
        return_cond_expr,
        return_expr,
        continue_cond_expr,
        continue_if_stmt,
        loop_body,
    )?;

    // 8. k_exit 関数を生成
    let k_exit_func = create_k_exit_function(&ctx, &parsed.func_name);

    // 9. JoinModule を構築
    Ok(build_join_module(entry_func, loop_step_func, k_exit_func))
}

/// ContinueReturn パターン用 entry 関数を生成
fn create_entry_function_continue_return(
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

/// ContinueReturn パターン用 loop_step 関数を生成
#[allow(clippy::too_many_arguments)]
fn create_loop_step_function_continue_return(
    lowerer: &mut AstToJoinIrLowerer,
    ctx: &super::common::LoopContext,
    func_name: &str,
    loop_cond_expr: &serde_json::Value,
    return_cond_expr: &serde_json::Value,
    return_expr: &serde_json::Value,
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

    // 2. Early return 条件チェック（i == 5 で Jump(k_exit, return_val)）
    let (return_cond_var, return_cond_insts) =
        lowerer.extract_value(return_cond_expr, &mut step_ctx);
    body.extend(return_cond_insts);

    let (return_val, return_val_insts) = lowerer.extract_value(return_expr, &mut step_ctx);
    body.extend(return_val_insts);

    body.push(JoinInst::Jump {
        cont: ctx.k_exit_id.as_cont(),
        args: vec![return_val],
        cond: Some(return_cond_var),
    });

    // 3. Continue pattern: i のインクリメント処理
    // Continue If の then 内に i の更新がある場合、それを使う（例: i = i + 1）
    let continue_then =
        continue_if_stmt["then"]
            .as_array()
            .ok_or_else(|| LoweringError::InvalidLoopBody {
                message: "Continue If must have 'then' array".to_string(),
            })?;

    let i_update_in_continue = continue_then
        .iter()
        .find(|stmt| stmt["type"].as_str() == Some("Local") && stmt["name"].as_str() == Some("i"))
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: format!(
                "ContinueReturn pattern validation failed: missing 'i' increment in continue block.\n\
                 Expected: Continue block must contain 'local i = i + K' where K is a constant.\n\
                 Found: Continue block does not contain 'i' update.\n\
                 Hint: Add 'i = i + 1' inside the 'if {{ continue }}' block."
            ),
        })?;

    let i_expr = &i_update_in_continue["expr"];
    let (i_next, i_insts) = lowerer.extract_value(i_expr, &mut step_ctx);
    body.extend(i_insts);
    step_ctx.register_param("i".to_string(), i_next);

    // 通常の i 更新を探す（ループ末尾）
    let i_update_normal = loop_body
        .iter()
        .filter(|stmt| stmt["type"].as_str() == Some("Local") && stmt["name"].as_str() == Some("i"))
        .last() // Continue の後にある最後の i 更新を使う
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: format!(
                "ContinueReturn pattern validation failed: missing normal 'i' increment.\n\
                 Expected: Loop body must contain 'local i = i + K' outside continue block.\n\
                 Found: No 'i' update found in loop body.\n\
                 Hint: Add 'i = i + 1' at the end of the loop body."
            ),
        })?;

    // Continue パスと通常パスの i 更新の差分計算（StepCalculator 活用）
    let mut i_next_continue = i_next;
    let base_k = StepCalculator::extract_linear_increment(&i_update_normal["expr"], "i")
        .ok_or_else(|| {
            let expr_debug = serde_json::to_string(&i_update_normal["expr"])
                .unwrap_or_else(|_| "<invalid JSON>".to_string());
            LoweringError::InvalidLoopBody {
                message: format!(
                    "ContinueReturn pattern validation failed: invalid step increment form.\n\
                     Expected: i = i + const (e.g., 'i = i + 1', 'i = i + 2').\n\
                     Found: {}\n\
                     Hint: Change the 'i' update to addition form 'i = i + K' where K is a constant integer.",
                    expr_debug
                ),
            }
        })?;

    let then_k = StepCalculator::extract_linear_increment(i_expr, "i").ok_or_else(|| {
        let expr_debug =
            serde_json::to_string(i_expr).unwrap_or_else(|_| "<invalid JSON>".to_string());
        LoweringError::InvalidLoopBody {
            message: format!(
                "ContinueReturn pattern validation failed: invalid continue branch step increment.\n\
                 Expected: In continue block, 'i = i + const' (e.g., 'i = i + 2').\n\
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

    // 4. Continue 条件を評価
    let (continue_cond_var, continue_cond_insts) =
        lowerer.extract_value(continue_cond_expr, &mut step_ctx);
    body.extend(continue_cond_insts);

    // 5. acc の更新値を計算（通常パス）
    let acc_update_local = loop_body
        .iter()
        .find(|stmt| stmt["type"].as_str() == Some("Local") && stmt["name"].as_str() == Some("acc"))
        .ok_or_else(|| LoweringError::InvalidLoopBody {
            message: format!(
                "ContinueReturn pattern validation failed: missing accumulator update.\n\
                 Expected: Loop body must contain 'local acc = ...' statement.\n\
                 Found: No 'acc' update found in loop body.\n\
                 Hint: Add 'acc = acc + <value>' or similar accumulator update."
            ),
        })?;

    let acc_expr = &acc_update_local["expr"];
    let (acc_increment, acc_insts) = lowerer.extract_value(acc_expr, &mut step_ctx);
    body.extend(acc_insts);

    // Continue 分岐側でも acc を更新できる場合（例: acc += 1）
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

    // 6. Select: Continue/通常 で acc を切り替える
    let acc_next = step_ctx.alloc_var();
    body.push(JoinInst::Select {
        dst: acc_next,
        cond: continue_cond_var,
        then_val: acc_then_val,
        else_val: acc_increment,
        type_hint: None,
    });

    // 7. Select: Continue/通常 で次 i を切り替える
    // Note: 通常パスは i_next (continue の i 更新ベース)、continue パスは i_next_continue (差分加算済み)
    // しかし、この fixture では continue 時に i = i + 1、通常時も i = i + 1 なので差分なし
    // → 本実装では差分があっても動作するよう設計
    let i_next_selected = step_ctx.alloc_var();
    body.push(JoinInst::Select {
        dst: i_next_selected,
        cond: continue_cond_var,
        then_val: i_next_continue,
        else_val: i_next,
        type_hint: None,
    });

    // 8. 末尾再帰
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
