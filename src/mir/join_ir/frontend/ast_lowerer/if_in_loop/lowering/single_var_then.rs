//! Phase P1: then のみ単一変数更新 - ケース 2
//!
//! `if cond { x = expr }` → `x = cond ? expr : x`
//! then で変数を更新し、else では元の値を維持する。

use super::super::super::{AstToJoinIrLowerer, ExtractCtx, JoinInst, StatementEffect};
use crate::mir::ValueId;

/// ケース 2: then のみ単一変数更新
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `ctx` - 変数コンテキスト
/// * `insts` - 条件式の命令列
/// * `cond_id` - 条件変数 ID
/// * `var_name` - 更新する変数名
/// * `then_stmts` - then 分岐のステートメント配列
pub fn lower(
    lowerer: &mut AstToJoinIrLowerer,
    ctx: &mut ExtractCtx,
    mut insts: Vec<JoinInst>,
    cond_id: ValueId,
    var_name: &str,
    then_stmts: &[serde_json::Value],
) -> (Vec<JoinInst>, StatementEffect) {
    // then の式を評価
    let then_expr = &then_stmts[0]["expr"];
    let (then_val, then_insts) = lowerer.extract_value(then_expr, ctx);
    insts.extend(then_insts);

    // else は元の値
    let else_val = ctx
        .get_var(var_name)
        .unwrap_or_else(|| panic!("Variable '{}' must exist for If/else", var_name));

    // Select: cond ? then_val : else_val
    let result_id = ctx.alloc_var();
    insts.push(JoinInst::Select {
        dst: result_id,
        cond: cond_id,
        then_val,
        else_val,
        type_hint: None, // Phase 63-3
    });

    ctx.register_param(var_name.to_string(), result_id);

    (
        insts,
        StatementEffect::VarUpdate {
            name: var_name.to_string(),
            value_id: result_id,
        },
    )
}
