//! Phase P2: Simple route lowering
//!
//! ## 責務（1行で表現）
//! **汎用的なループ処理を Jump/Call/Select に落とす**
//!
//! ## route 例
//! ```nyash
//! // Filter/Map/Reduce/PrintTokens 以外の汎用ループ
//! local sum = 0
//! local i = 0
//! loop(i < n) {
//!     sum = sum + i
//!     i = i + 1
//! }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - entry 関数: Call(loop_step)
//! - loop_step 関数:
//!   - 条件チェック
//!   - true: body 処理 + 再帰
//!   - false: Jump(k_exit)
//! - k_exit 関数: Return

use super::common::{
    build_join_module, build_recurse_args, build_step_params, create_entry_function,
    create_k_exit_function, create_loop_context, create_step_ctx, parse_program_json,
    process_local_inits,
};
use super::{AstToJoinIrLowerer, JoinModule, LoweringError};
use crate::mir::join_ir::CompareOp;
use crate::mir::join_ir::{ConstValue, JoinFunction, JoinInst, MirLikeInst};

/// Simple routeを JoinModule に変換
///
/// 汎用的なループ処理を JoinIR に変換する。
/// Filter/Map/Reduce/PrintTokens 以外のすべてのループがこれに該当する。
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

    // 4. entry 関数を生成
    let entry_func = create_entry_function(&ctx, &parsed, init_insts, &mut entry_ctx);

    // 5. Loop ノードを取得
    let loop_node = &parsed.stmts[parsed.loop_node_idx];
    let loop_cond_expr = &loop_node["cond"];
    let loop_body_stmts =
        loop_node["body"]
            .as_array()
            .ok_or_else(|| LoweringError::InvalidLoopBody {
                message: "Loop must have 'body' array".to_string(),
            })?;

    // 6. loop_step 関数を生成
    let loop_step_func = create_loop_step_function(
        lowerer,
        &ctx,
        &parsed.func_name,
        loop_cond_expr,
        loop_body_stmts,
    )?;

    // 7. k_exit 関数を生成
    let k_exit_func = create_k_exit_function(&ctx, &parsed.func_name);

    // 8. JoinModule を構築
    Ok(build_join_module(entry_func, loop_step_func, k_exit_func))
}

/// loop_step 関数を生成
fn create_loop_step_function(
    lowerer: &mut AstToJoinIrLowerer,
    ctx: &super::common::LoopContext,
    func_name: &str,
    loop_cond_expr: &serde_json::Value,
    loop_body_stmts: &[serde_json::Value],
) -> Result<JoinFunction, LoweringError> {
    // step_ctx を作成
    let mut step_ctx = create_step_ctx(ctx);

    // 条件式を評価（i < n）
    let (cond_var, cond_insts) = lowerer.extract_value(loop_cond_expr, &mut step_ctx);

    // !cond を計算（i >= n なら抜ける）
    let false_const = step_ctx.alloc_var();
    let exit_cond = step_ctx.alloc_var();

    let mut body = cond_insts;
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

    // 早期 return: exit_cond が true（i >= n）なら k_exit へ Jump
    let step_acc = step_ctx.get_var("acc").expect("acc must exist");
    body.push(JoinInst::Jump {
        cont: ctx.k_exit_id.as_cont(),
        args: vec![step_acc],
        cond: Some(exit_cond),
    });

    // Loop body を処理（汎用 statement handler を使用）
    for body_stmt in loop_body_stmts {
        let (insts, _effect) = lowerer.lower_statement(body_stmt, &mut step_ctx);
        body.extend(insts);
    }

    // 再帰呼び出し
    let recurse_args = build_recurse_args(ctx, &step_ctx);
    let recurse_result = step_ctx.alloc_var();

    body.push(JoinInst::Call {
        func: ctx.loop_step_id,
        args: recurse_args,
        k_next: None,
        dst: Some(recurse_result),
    });
    body.push(JoinInst::Ret {
        value: Some(recurse_result),
    });

    // パラメータリストを構築
    let params = build_step_params(ctx);

    Ok(JoinFunction {
        id: ctx.loop_step_id,
        name: format!("{}_loop_step", func_name),
        params,
        body,
        exit_cont: None,
    })
}
