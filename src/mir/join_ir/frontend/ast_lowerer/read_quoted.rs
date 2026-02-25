//! Phase P5/45: Read Quoted パターン lowering
//!
//! ## 責務（1行で表現）
//! **Guard if + Loop with break + accumulator パターンを JoinIR に落とす**
//!
//! ## パターン例
//! ```nyash
//! read_quoted_from(s, pos) {
//!     local i = pos
//!     if s.substring(i, i+1) != "\"" { return "" }  // Guard if
//!     i = i + 1
//!     local out = ""
//!     loop (i < n) {
//!         local ch = s.substring(i, i+1)
//!         if ch == "\"" { break }  // Found closing quote
//!         out = out + ch
//!         i = i + 1
//!     }
//!     return out
//! }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - Guard if: 早期 return で不正入力を弾く
//! - Loop: accumulator パターンで文字列を構築
//! - Break: 終端条件で抜ける

use super::BTreeMap;
use super::{
    AstToJoinIrLowerer, ConstValue, ExtractCtx, JoinFunction, JoinInst, JoinModule, MergePair,
};
use crate::mir::join_ir::JoinIrPhase;

impl AstToJoinIrLowerer {
    /// Phase 45: read_quoted_from パターンの lowering
    ///
    /// # Pattern
    ///
    /// ```nyash,ignore
    /// read_quoted_from(s, pos) {
    ///   local i = pos
    ///   if s.substring(i, i+1) != "\"" { return "" }  // Guard if
    ///   i = i + 1
    ///   local out = ""
    ///   local n = s.length()
    ///   loop (i < n) {
    ///     local ch = s.substring(i, i+1)
    ///     if ch == "\"" { break }  // Found closing quote
    ///     // NOTE: Escape handling (if ch == "\\") has known PHI issue
    ///     //       Variable reassignment inside if-block doesn't generate PHI
    ///     //       This will be addressed by JoinIR IfMerge improvements
    ///     out = out + ch
    ///     i = i + 1
    ///   }
    ///   return out
    /// }
    /// ```
    ///
    /// # JoinIR Output
    ///
    /// - entry: Guard if check → Select(guard_passed ? Call(loop_step) : Return(""))
    /// - loop_step: (i, out, n, s) → exit check → break check → body → Call(loop_step)
    /// - k_exit: (out) → Return(out)
    ///
    /// # Dev Flag
    ///
    /// 環境変数 `HAKO_JOINIR_READ_QUOTED=1` が必須。
    pub(crate) fn lower_read_quoted_pattern(
        &mut self,
        program_json: &serde_json::Value,
    ) -> JoinModule {
        // 1. Program(JSON) から基本情報を取得
        let defs = program_json["defs"]
            .as_array()
            .expect("Program(JSON v0) must have 'defs' array");

        let func_def = defs
            .get(0)
            .expect("At least one function definition required");

        let func_name = func_def["name"]
            .as_str()
            .expect("Function must have 'name'");

        let params = func_def["params"]
            .as_array()
            .expect("Function must have 'params' array");

        // 2. ExtractCtx 作成とパラメータ登録 (s, pos)
        let mut ctx = ExtractCtx::new(params.len() as u32);
        for (i, param) in params.iter().enumerate() {
            let param_name = param
                .as_str()
                .expect("Parameter must be string")
                .to_string();
            ctx.register_param(param_name, crate::mir::ValueId(i as u32));
        }

        // 3. body を解析
        let body = &func_def["body"]["body"];
        let _stmts = body.as_array().expect("Function body must be array");

        // 4. AST 構造を解析:
        //    - Local i = pos
        //    - If guard { return "" }
        //    - i = i + 1
        //    - local out = ""
        //    - local n = s.length()
        //    - Loop { ... }
        //    - Return out

        // 5. JoinIR 生成: entry / loop_step / k_exit（3関数構造）
        let entry_id = self.next_func_id();
        let loop_step_id = self.next_func_id();
        let k_exit_id = self.next_func_id();

        // ========================================
        // Entry 関数の構築
        // ========================================
        let mut entry_body = Vec::new();

        // パラメータ取得 (s=0, pos=1)
        let s_param = ctx.get_var("s").expect("s must be parameter");
        let pos_param = ctx.get_var("pos").expect("pos must be parameter");

        // local i = pos
        ctx.register_param("i".to_string(), pos_param);
        let i_var = pos_param;

        // Guard: s.substring(i, i+1) を計算
        // i+1 を計算
        let one_const = ctx.alloc_var();
        entry_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: one_const,
            value: ConstValue::Integer(1),
        }));

        let i_plus_1 = ctx.alloc_var();
        entry_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::BinOp {
            dst: i_plus_1,
            op: crate::mir::join_ir::BinOpKind::Add,
            lhs: i_var,
            rhs: one_const,
        }));

        // s.substring(i, i+1) を呼び出し
        let first_char = ctx.alloc_var();
        entry_body.push(JoinInst::MethodCall {
            dst: first_char,
            receiver: s_param,
            method: "substring".to_string(),
            args: vec![i_var, i_plus_1],
            type_hint: Some(crate::mir::MirType::String), // Phase 65-2-A: substring → String
        });

        // Guard 条件: first_char != '"'
        let quote_const = ctx.alloc_var();
        entry_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: quote_const,
            value: ConstValue::String("\"".to_string()),
        }));

        let guard_cond = ctx.alloc_var();
        entry_body.push(JoinInst::Compute(
            crate::mir::join_ir::MirLikeInst::Compare {
                dst: guard_cond,
                op: crate::mir::join_ir::CompareOp::Ne,
                lhs: first_char,
                rhs: quote_const,
            },
        ));

        // Guard 失敗時の戻り値: ""
        let empty_string = ctx.alloc_var();
        entry_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: empty_string,
            value: ConstValue::String("".to_string()),
        }));

        // Guard 成功時: i = i + 1
        let i_after_guard = i_plus_1; // 既に計算済み
        ctx.register_param("i".to_string(), i_after_guard);

        // local out = ""
        let out_init = ctx.alloc_var();
        entry_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: out_init,
            value: ConstValue::String("".to_string()),
        }));
        ctx.register_param("out".to_string(), out_init);

        // local n = s.length()
        let n_var = ctx.alloc_var();
        entry_body.push(JoinInst::MethodCall {
            dst: n_var,
            receiver: s_param,
            method: "length".to_string(),
            args: vec![],
            type_hint: Some(crate::mir::MirType::Integer), // Phase 65-2-A: length → Integer
        });
        ctx.register_param("n".to_string(), n_var);

        // Guard check → Jump to early return if guard fails
        // 逆条件で Jump（guard_cond == true なら early return）
        let k_guard_fail_id = self.next_func_id();

        entry_body.push(JoinInst::Jump {
            cont: k_guard_fail_id.as_cont(),
            args: vec![empty_string],
            cond: Some(guard_cond),
        });

        // Guard 成功: loop_step を呼び出し
        let loop_result = ctx.alloc_var();
        entry_body.push(JoinInst::Call {
            func: loop_step_id,
            args: vec![i_after_guard, out_init, n_var, s_param],
            k_next: None,
            dst: Some(loop_result),
        });

        entry_body.push(JoinInst::Ret {
            value: Some(loop_result),
        });

        let entry_func = JoinFunction {
            id: entry_id,
            name: func_name.to_string(),
            params: (0..params.len())
                .map(|i| crate::mir::ValueId(i as u32))
                .collect(),
            body: entry_body,
            exit_cont: None,
        };

        // ========================================
        // k_guard_fail 関数（Guard 失敗時 early return）
        // ========================================
        let k_guard_fail_result = crate::mir::ValueId(0);
        let k_guard_fail_func = JoinFunction {
            id: k_guard_fail_id,
            name: format!("{}_k_guard_fail", func_name),
            params: vec![k_guard_fail_result],
            body: vec![JoinInst::Ret {
                value: Some(k_guard_fail_result),
            }],
            exit_cont: None,
        };

        // ========================================
        // loop_step 関数の構築
        // ========================================
        // params: (i, out, n, s)
        let step_i = crate::mir::ValueId(0);
        let step_out = crate::mir::ValueId(1);
        let step_n = crate::mir::ValueId(2);
        let step_s = crate::mir::ValueId(3);

        let mut step_ctx = ExtractCtx::new(4);
        step_ctx.register_param("i".to_string(), step_i);
        step_ctx.register_param("out".to_string(), step_out);
        step_ctx.register_param("n".to_string(), step_n);
        step_ctx.register_param("s".to_string(), step_s);

        let mut loop_step_body = Vec::new();

        // 1. Exit 条件チェック: !(i < n) = i >= n で抜ける
        let i_lt_n = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(
            crate::mir::join_ir::MirLikeInst::Compare {
                dst: i_lt_n,
                op: crate::mir::join_ir::CompareOp::Lt,
                lhs: step_i,
                rhs: step_n,
            },
        ));

        let false_const = step_ctx.alloc_var();
        let exit_cond = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: false_const,
            value: ConstValue::Bool(false),
        }));
        loop_step_body.push(JoinInst::Compute(
            crate::mir::join_ir::MirLikeInst::Compare {
                dst: exit_cond,
                op: crate::mir::join_ir::CompareOp::Eq,
                lhs: i_lt_n,
                rhs: false_const,
            },
        ));

        // i >= n なら k_exit へ Jump
        loop_step_body.push(JoinInst::Jump {
            cont: k_exit_id.as_cont(),
            args: vec![step_out],
            cond: Some(exit_cond),
        });

        // 2. ch = s.substring(i, i+1)
        let step_one = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: step_one,
            value: ConstValue::Integer(1),
        }));

        let step_i_plus_1 = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::BinOp {
            dst: step_i_plus_1,
            op: crate::mir::join_ir::BinOpKind::Add,
            lhs: step_i,
            rhs: step_one,
        }));

        let step_ch = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::MethodCall {
            dst: step_ch,
            receiver: step_s,
            method: "substring".to_string(),
            args: vec![step_i, step_i_plus_1],
            type_hint: Some(crate::mir::MirType::String), // Phase 65-2-A: substring → String
        });

        // 3. Break 条件: ch == '"'
        let step_quote = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: step_quote,
            value: ConstValue::String("\"".to_string()),
        }));

        let break_cond = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(
            crate::mir::join_ir::MirLikeInst::Compare {
                dst: break_cond,
                op: crate::mir::join_ir::CompareOp::Eq,
                lhs: step_ch,
                rhs: step_quote,
            },
        ));

        // ch == '"' なら k_exit へ Jump (break)
        loop_step_body.push(JoinInst::Jump {
            cont: k_exit_id.as_cont(),
            args: vec![step_out],
            cond: Some(break_cond),
        });

        // ========================================
        // 4. Escape 処理: if ch == "\\" { i = i + 1; ch = s.substring(i, i+1) }
        // ========================================
        // Phase 46: IfMerge で if-body 後の値をマージ
        let enable_escape = crate::mir::join_ir::env_flag_is_1("HAKO_JOINIR_READ_QUOTED_IFMERGE");

        // 条件と then 側の値を事前計算（投機的実行）
        let step_backslash = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
            dst: step_backslash,
            value: ConstValue::String("\\".to_string()),
        }));

        let esc_cond = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(
            crate::mir::join_ir::MirLikeInst::Compare {
                dst: esc_cond,
                op: crate::mir::join_ir::CompareOp::Eq,
                lhs: step_ch,
                rhs: step_backslash,
            },
        ));

        // i_esc = i + 1（then 側の i 値）
        let i_esc = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::BinOp {
            dst: i_esc,
            op: crate::mir::join_ir::BinOpKind::Add,
            lhs: step_i,
            rhs: step_one,
        }));

        // i_esc_plus_1 = i_esc + 1（substring の end 引数用）
        let i_esc_plus_1 = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::BinOp {
            dst: i_esc_plus_1,
            op: crate::mir::join_ir::BinOpKind::Add,
            lhs: i_esc,
            rhs: step_one,
        }));

        // ch_esc = s.substring(i_esc, i_esc+1)（then 側の ch 値）
        let ch_esc = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::MethodCall {
            dst: ch_esc,
            receiver: step_s,
            method: "substring".to_string(),
            args: vec![i_esc, i_esc_plus_1],
            type_hint: Some(crate::mir::MirType::String), // Phase 65-2-A: substring → String
        });

        // IfMerge: if-body 後の i と ch をマージ
        let (i_after_esc, ch_merged) = if enable_escape {
            let i_after_esc = step_ctx.alloc_var();
            let ch_merged = step_ctx.alloc_var();

            loop_step_body.push(JoinInst::IfMerge {
                cond: esc_cond,
                merges: vec![
                    MergePair {
                        dst: i_after_esc,
                        then_val: i_esc,  // i + 1 (BinOp::Add, Integer)
                        else_val: step_i, // i (Integer param)
                        type_hint: Some(crate::mir::MirType::Integer), // Phase 64-2: ループカウンタ型確定
                    },
                    MergePair {
                        dst: ch_merged,
                        then_val: ch_esc,  // substring 結果 (String)
                        else_val: step_ch, // substring 結果 (String)
                        type_hint: Some(crate::mir::MirType::String), // Phase 64-2: 文字列型確定
                    },
                ],
                k_next: None,
            });

            (i_after_esc, ch_merged)
        } else {
            // 旧パス: escape 未対応（step_i と step_ch をそのまま使う）
            (step_i, step_ch)
        };

        // ========================================
        // 5. Accumulator: out = out + ch_merged
        // ========================================
        let out_next = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::BinOp {
            dst: out_next,
            op: crate::mir::join_ir::BinOpKind::Add, // String concatenation
            lhs: step_out,
            rhs: ch_merged, // ← ch_merged を使う！
        }));

        // ========================================
        // 6. i_next = i_after_esc + 1
        // ========================================
        let i_next = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Compute(crate::mir::join_ir::MirLikeInst::BinOp {
            dst: i_next,
            op: crate::mir::join_ir::BinOpKind::Add,
            lhs: i_after_esc, // ← i_after_esc を使う！
            rhs: step_one,
        }));

        // 7. 末尾再帰: Call(loop_step)
        let recurse_result = step_ctx.alloc_var();
        loop_step_body.push(JoinInst::Call {
            func: loop_step_id,
            args: vec![i_next, out_next, step_n, step_s],
            k_next: None,
            dst: Some(recurse_result),
        });

        loop_step_body.push(JoinInst::Ret {
            value: Some(recurse_result),
        });

        let loop_step_func = JoinFunction {
            id: loop_step_id,
            name: format!("{}_loop_step", func_name),
            params: vec![step_i, step_out, step_n, step_s],
            body: loop_step_body,
            exit_cont: None,
        };

        // ========================================
        // k_exit 関数の構築
        // ========================================
        let k_exit_out = crate::mir::ValueId(0);
        let k_exit_func = JoinFunction {
            id: k_exit_id,
            name: format!("{}_k_exit", func_name),
            params: vec![k_exit_out],
            body: vec![JoinInst::Ret {
                value: Some(k_exit_out),
            }],
            exit_cont: None,
        };

        // ========================================
        // JoinModule の構築
        // ========================================
        let mut functions = BTreeMap::new();
        functions.insert(entry_id, entry_func);
        functions.insert(k_guard_fail_id, k_guard_fail_func);
        functions.insert(loop_step_id, loop_step_func);
        functions.insert(k_exit_id, k_exit_func);

        JoinModule {
            functions,
            entry: Some(entry_id),
            phase: JoinIrPhase::Structured,
        }
    }
}
