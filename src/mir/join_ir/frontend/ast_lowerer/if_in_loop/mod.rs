//! Phase P1: If in Loop Handler - 箱化モジュール
//!
//! ループ内の If ステートメントを JoinIR に変換する。
//! 5 つのパターンに分類し、それぞれに適した lowering 戦略を適用する。

pub mod lowering;
pub mod pattern;

use super::{AstToJoinIrLowerer, ExtractCtx, JoinInst, StatementEffect};
use pattern::IfInLoopPattern;

impl AstToJoinIrLowerer {
    /// Phase P1: If ステートメント（ループ内）を JoinIR に変換
    ///
    /// 元の lower_if_stmt_in_loop() を箱化モジュール化したエントリーポイント。
    /// パターン検出 → 適切な lowering 関数に委譲する。
    pub(crate) fn lower_if_stmt_in_loop_boxified(
        &mut self,
        stmt: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (Vec<JoinInst>, StatementEffect) {
        let cond_expr = &stmt["cond"];
        let then_body = stmt["then"].as_array();
        let else_body = stmt["else"].as_array();

        // 条件を評価
        let (cond_id, insts) = self.extract_value(cond_expr, ctx);

        // then/else のステートメント配列を取得
        let then_stmts = then_body.map(|v| v.as_slice()).unwrap_or(&[]);
        let else_stmts = else_body.map(|v| v.as_slice()).unwrap_or(&[]);

        // パターンを検出
        let pattern = IfInLoopPattern::detect(then_stmts, else_stmts);

        // パターンごとに lowering
        match pattern {
            IfInLoopPattern::Empty => lowering::empty::lower(insts),
            IfInLoopPattern::SingleVarThen { var_name } => {
                lowering::single_var_then::lower(self, ctx, insts, cond_id, &var_name, then_stmts)
            }
            IfInLoopPattern::SingleVarBoth { var_name } => lowering::single_var_both::lower(
                self, ctx, insts, cond_id, &var_name, then_stmts, else_stmts,
            ),
            IfInLoopPattern::ConditionalEffect {
                receiver_name,
                method_name,
            } => lowering::conditional_effect::lower(
                self,
                ctx,
                insts,
                cond_id,
                then_stmts,
                &receiver_name,
                &method_name,
            ),
            IfInLoopPattern::Unsupported {
                then_count,
                else_count,
            } => lowering::unsupported::panic_unsupported(then_count, else_count),
        }
    }
}
