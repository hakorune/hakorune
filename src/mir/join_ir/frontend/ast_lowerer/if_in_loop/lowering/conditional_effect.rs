//! Phase P1: 条件付き側効果 - ケース 4
//!
//! `if pred(v) { acc.push(v) }` → ConditionalMethodCall
//! filter パターン用の特殊 JoinIR 命令を生成する。

use super::super::super::{AstToJoinIrLowerer, ExtractCtx, JoinInst, StatementEffect};
use crate::mir::ValueId;

/// ケース 4: 条件付き側効果パターン
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `ctx` - 変数コンテキスト
/// * `insts` - 条件式の命令列
/// * `cond_id` - 条件変数 ID
/// * `then_stmts` - then 分岐のステートメント配列
/// * `receiver_name` - receiver 変数名（acc 等）
/// * `method_name` - メソッド名（push 等）
pub fn lower(
    lowerer: &mut AstToJoinIrLowerer,
    ctx: &mut ExtractCtx,
    mut insts: Vec<JoinInst>,
    cond_id: ValueId,
    then_stmts: &[serde_json::Value],
    receiver_name: &str,
    method_name: &str,
) -> (Vec<JoinInst>, StatementEffect) {
    let stmt = &then_stmts[0];
    let receiver_expr = stmt.get("receiver").or_else(|| stmt.get("object"));
    let args_array = stmt.get("args").or_else(|| stmt.get("arguments"));

    if let (Some(receiver_expr), Some(args_array)) = (receiver_expr, args_array) {
        // receiver を評価
        let (receiver_var, receiver_insts) = lowerer.extract_value(receiver_expr, ctx);
        insts.extend(receiver_insts);

        // args を評価
        let mut arg_vars = Vec::new();
        if let Some(args) = args_array.as_array() {
            for arg_expr in args {
                let (arg_var, arg_insts) = lowerer.extract_value(arg_expr, ctx);
                arg_vars.push(arg_var);
                insts.extend(arg_insts);
            }
        }

        // 結果変数を割り当て
        let dst = ctx.alloc_var();

        // ConditionalMethodCall 命令を生成
        insts.push(JoinInst::ConditionalMethodCall {
            cond: cond_id,
            dst,
            receiver: receiver_var,
            method: method_name.to_string(),
            args: arg_vars,
        });

        // receiver 変数を更新された値で登録
        ctx.register_param(receiver_name.to_string(), dst);

        return (
            insts,
            StatementEffect::VarUpdate {
                name: receiver_name.to_string(),
                value_id: dst,
            },
        );
    }

    panic!("ConditionalEffect pattern: invalid receiver or args");
}
