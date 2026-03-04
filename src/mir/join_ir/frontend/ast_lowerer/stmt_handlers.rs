//! Phase 53: Statement Handler モジュール
//!
//! ループ本体の各ステートメントタイプを JoinIR に変換する。
//!
//! ## 対応ステートメント
//!
//! - `Local`: 変数宣言 + 初期化
//! - `Assignment`: 変数への代入
//! - `Print`: 出力（副作用）
//! - `If`: 条件分岐
//! - `Method`: メソッド呼び出し（式文として）

use super::{AstToJoinIrLowerer, ExtractCtx, JoinInst};
use crate::mir::join_ir::MirLikeInst;
use crate::mir::ValueId;

/// ステートメントの効果（変数更新 or 副作用のみ）
#[derive(Debug, Clone)]
pub(crate) enum StatementEffect {
    /// 変数を更新（Assignment/Local）
    VarUpdate { name: String, value_id: ValueId },
    /// 副作用のみ（Print）
    SideEffect,
    /// 効果なし（空の If など）
    None,
}

impl AstToJoinIrLowerer {
    /// Phase 53: ステートメントを JoinIR に変換
    ///
    /// 対応タイプ: Local, Assignment, Print, If, Method
    pub(crate) fn lower_statement(
        &mut self,
        stmt: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (Vec<JoinInst>, StatementEffect) {
        let stmt_type = stmt["type"]
            .as_str()
            .expect("Statement must have 'type' field");

        match stmt_type {
            "Local" => self.lower_local_stmt(stmt, ctx),
            "Assignment" => self.lower_assignment_stmt(stmt, ctx),
            "Print" => self.lower_print_stmt(stmt, ctx),
            "Method" => self.lower_method_stmt(stmt, ctx),
            "If" => self.lower_if_stmt_in_loop_boxified(stmt, ctx),
            other => panic!(
                "Unsupported statement type in loop body: {}. \
                 Expected: Local, Assignment, Print, Method, If",
                other
            ),
        }
    }

    /// Local ステートメント: `local x = expr`
    fn lower_local_stmt(
        &mut self,
        stmt: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (Vec<JoinInst>, StatementEffect) {
        let var_name = stmt["name"]
            .as_str()
            .expect("Local must have 'name'")
            .to_string();
        let expr = &stmt["expr"];

        let (value_id, insts) = self.extract_value(expr, ctx);
        ctx.register_param(var_name.clone(), value_id);

        (
            insts,
            StatementEffect::VarUpdate {
                name: var_name,
                value_id,
            },
        )
    }

    /// Assignment ステートメント: `x = expr`
    ///
    /// Phase 53-2: `i = i + 1` などの代入文を処理
    fn lower_assignment_stmt(
        &mut self,
        stmt: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (Vec<JoinInst>, StatementEffect) {
        let target = stmt["target"]
            .as_str()
            .expect("Assignment must have 'target'")
            .to_string();
        let expr = &stmt["expr"];

        let (value_id, insts) = self.extract_value(expr, ctx);
        ctx.register_param(target.clone(), value_id);

        (
            insts,
            StatementEffect::VarUpdate {
                name: target,
                value_id,
            },
        )
    }

    /// Print ステートメント: `print(expr)`
    ///
    /// Phase 53-3: ConsoleBox.print 呼び出しに変換
    fn lower_print_stmt(
        &mut self,
        stmt: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (Vec<JoinInst>, StatementEffect) {
        let expr = &stmt["expr"];
        let (arg_id, mut insts) = self.extract_value(expr, ctx);

        // print は BoxCall として実装（ConsoleBox.print）
        let result_id = ctx.alloc_var();
        insts.push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(result_id),
            box_name: "ConsoleBox".to_string(),
            method: "print".to_string(),
            args: vec![arg_id],
        }));

        (insts, StatementEffect::SideEffect)
    }

    /// Method ステートメント（式文として）: `obj.method(args)`
    ///
    /// 戻り値を捨てるメソッド呼び出し（print など）
    fn lower_method_stmt(
        &mut self,
        stmt: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (Vec<JoinInst>, StatementEffect) {
        // extract_value で Method を評価（戻り値は捨てる）
        let (_, insts) = self.extract_value(stmt, ctx);
        (insts, StatementEffect::SideEffect)
    }

    // Phase P1: lower_if_stmt_in_loop() は if_in_loop/ モジュールに箱化移行済み
    // lower_if_stmt_in_loop_boxified() を参照
}
