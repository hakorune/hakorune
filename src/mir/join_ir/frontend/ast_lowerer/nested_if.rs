//! Phase P5/41-4: Nested If パターン lowering
//!
//! ## 責務（1行で表現）
//! **深いネスト if（3-4レベル）を NestedIfMerge 命令に落とす**
//!
//! ## パターン例
//! ```nyash
//! if cond0 {
//!     if cond1 {
//!         if cond2 {
//!             x = new_value  // deepest level
//!         }
//!     }
//! }
//! // At merge point: x has PHI semantics
//! ```
//!
//! ## 生成する JoinIR 構造
//! - NestedIfMerge 命令でマルチレベル条件を表現
//! - 各レベルの条件を accumulate して最終値を決定

use super::BTreeMap;
use super::{AstToJoinIrLowerer, ExtractCtx, JoinFunction, JoinInst, JoinModule};
use crate::mir::join_ir::JoinIrPhase;

impl AstToJoinIrLowerer {
    /// Phase 41-4.2: ネスト if パターンの lowering
    ///
    /// # Purpose
    ///
    /// 深いネスト if（3-4レベル）を `NestedIfMerge` 命令に変換する。
    /// 対象: `ParserControlBox.parse_loop()` 関数
    ///
    /// # Pattern
    ///
    /// ```nyash,ignore
    /// // Level 0 (outer)
    /// if cond0 {
    ///   // Level 1
    ///   if cond1 {
    ///     // Level 2
    ///     if cond2 {
    ///       // Level 3 (deepest)
    ///       x = new_value
    ///     }
    ///   }
    /// }
    /// // At merge point: x has PHI semantics
    /// ```
    ///
    /// # Output JoinIR
    ///
    /// - `NestedIfMerge { conds: [cond0, cond1, cond2], merges: [(x, new_value, old_x)], k_next }`
    ///
    /// # Dev Flag
    ///
    /// 環境変数 `HAKO_JOINIR_NESTED_IF=1` が必須。
    pub fn lower_nested_if_pattern(&mut self, program_json: &serde_json::Value) -> JoinModule {
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

        // 2. ExtractCtx 作成とパラメータ登録
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
        let stmts = body.as_array().expect("Function body must be array");

        // 4. nested if パターンを検出
        let nested_pattern = self.try_match_nested_if_pattern(stmts, &mut ctx);

        // 5. JoinIR 生成
        let func_id = self.next_func_id();
        let mut insts = Vec::new();

        // 5.1. Local 初期化を処理（ネスト if の前にある Local 文）
        for stmt in stmts {
            let stmt_type = stmt["type"].as_str().unwrap_or("");
            if stmt_type == "Local" {
                let var_name = stmt["name"]
                    .as_str()
                    .expect("Local must have 'name'")
                    .to_string();
                let expr = &stmt["expr"];
                let (var_id, local_insts) = self.extract_value(expr, &mut ctx);
                insts.extend(local_insts);
                ctx.register_param(var_name, var_id);
            } else if stmt_type == "If" {
                // If 文でパターン開始
                break;
            }
        }

        // 5.2. NestedIfMerge パターンがマッチした場合
        if let Some(pattern) = nested_pattern {
            // 条件を評価
            let mut cond_vars = Vec::new();
            for cond_expr in &pattern.conds {
                let (cond_var, cond_insts) = self.extract_value(cond_expr, &mut ctx);
                insts.extend(cond_insts);
                cond_vars.push(cond_var);
            }

            // merges を構築
            let mut merges = Vec::new();
            for (var_name, then_expr, else_expr) in &pattern.merges {
                let (then_var, then_insts) = self.extract_value(then_expr, &mut ctx);
                insts.extend(then_insts);

                let else_var = if let Some(else_e) = else_expr {
                    let (e_var, e_insts) = self.extract_value(else_e, &mut ctx);
                    insts.extend(e_insts);
                    e_var
                } else {
                    // else がない場合は既存の変数値を使用
                    ctx.get_var(var_name)
                        .unwrap_or_else(|| panic!("Undefined variable in merge: {}", var_name))
                };

                // 新しい dst を割り当て、merge 後の値として ctx に登録
                let dst = ctx.alloc_var();
                ctx.register_param(var_name.clone(), dst);

                merges.push(crate::mir::join_ir::MergePair {
                    dst,
                    then_val: then_var,
                    else_val: else_var,
                    type_hint: None, // Phase 63-3
                });
            }

            // NestedIfMerge 命令を追加
            insts.push(JoinInst::NestedIfMerge {
                conds: cond_vars,
                merges,
                k_next: None, // 関数末尾なので継続なし
            });
        } else {
            // パターンがマッチしない場合は panic（dev フラグ ON の時のみ到達）
            panic!(
                "lower_nested_if_pattern: No nested if pattern found in parse_loop. \
                 Expected 2-4 level nested if structure."
            );
        }

        // 5.3. Return 文を処理
        let return_stmt = stmts.iter().find(|s| s["type"].as_str() == Some("Return"));
        if let Some(ret) = return_stmt {
            let (ret_var, ret_insts) = self.extract_value(&ret["expr"], &mut ctx);
            insts.extend(ret_insts);
            insts.push(JoinInst::Ret {
                value: Some(ret_var),
            });
        } else {
            // Return がない場合は void return
            insts.push(JoinInst::Ret { value: None });
        }

        // 6. JoinFunction 構築
        let func = JoinFunction {
            id: func_id,
            name: func_name.to_string(),
            params: (0..params.len())
                .map(|i| crate::mir::ValueId(i as u32))
                .collect(),
            body: insts,
            exit_cont: None,
        };

        let mut functions = BTreeMap::new();
        functions.insert(func_id, func);

        JoinModule {
            functions,
            entry: Some(func_id),
            phase: JoinIrPhase::Structured,
        }
    }

    /// Phase 41-4.2: ネスト if パターンの検出
    ///
    /// # Purpose
    ///
    /// AST から 2-4 レベルのネスト if を検出し、`NestedIfPattern` として返す。
    ///
    /// # Algorithm
    ///
    /// 1. 最初の If 文を探す
    /// 2. then 分岐を再帰的に解析し、ネストレベルをカウント
    /// 3. 各レベルの条件式と変数代入を収集
    /// 4. 最大 4 レベルまで対応（それ以上は未サポート）
    ///
    /// # Returns
    ///
    /// - `Some(NestedIfPattern)`: パターンがマッチした場合
    /// - `None`: パターンがマッチしない場合
    pub(crate) fn try_match_nested_if_pattern(
        &self,
        stmts: &[serde_json::Value],
        _ctx: &mut ExtractCtx,
    ) -> Option<NestedIfPattern> {
        // 1. 最初の If 文を探す
        let first_if = stmts.iter().find(|s| s["type"].as_str() == Some("If"))?;

        // 2. ネスト構造を再帰的に解析
        let mut conds = Vec::new();
        let mut merges = Vec::new();

        self.collect_nested_if_structure(first_if, &mut conds, &mut merges, 0);

        // 3. 少なくとも 2 レベル以上のネストが必要
        if conds.len() < 2 {
            return None;
        }

        Some(NestedIfPattern { conds, merges })
    }

    /// Phase 41-4.2: ネスト if 構造の再帰収集
    ///
    /// # Arguments
    ///
    /// - `if_stmt`: 現在の If ノード
    /// - `conds`: 条件式リスト（外側から内側へ）
    /// - `merges`: 変数代入リスト
    /// - `depth`: 現在のネストレベル（0から開始）
    pub(crate) fn collect_nested_if_structure(
        &self,
        if_stmt: &serde_json::Value,
        conds: &mut Vec<serde_json::Value>,
        merges: &mut Vec<(String, serde_json::Value, Option<serde_json::Value>)>,
        depth: usize,
    ) {
        // 最大 4 レベルまで
        if depth >= 4 {
            return;
        }

        // 条件式を追加
        if let Some(cond) = if_stmt.get("cond") {
            conds.push(cond.clone());
        }

        // then 分岐を解析
        if let Some(then_body) = if_stmt.get("then").and_then(|t| t.as_array()) {
            for stmt in then_body {
                let stmt_type = stmt["type"].as_str().unwrap_or("");

                match stmt_type {
                    "If" => {
                        // ネスト if: 再帰処理
                        self.collect_nested_if_structure(stmt, conds, merges, depth + 1);
                    }
                    "Local" => {
                        // 変数代入を記録
                        if let Some(var_name) = stmt["name"].as_str() {
                            let expr = stmt.get("expr").cloned().unwrap_or(serde_json::Value::Null);
                            // then 値のみ記録（else はパターン解析時に決定）
                            merges.push((var_name.to_string(), expr, None));
                        }
                    }
                    "Return" => {
                        // 早期 return は無視（NestedIfMerge では扱わない）
                    }
                    _ => {}
                }
            }
        }

        // else 分岐の代入も収集（存在する場合）
        if let Some(else_body) = if_stmt.get("else").and_then(|e| e.as_array()) {
            for stmt in else_body {
                if stmt["type"].as_str() == Some("Local") {
                    if let Some(var_name) = stmt["name"].as_str() {
                        // 既存の merge エントリを探して else 値を更新
                        for (name, _, else_val) in merges.iter_mut() {
                            if name == var_name {
                                let expr =
                                    stmt.get("expr").cloned().unwrap_or(serde_json::Value::Null);
                                *else_val = Some(expr);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Phase 41-4.2: ネスト if パターン構造
///
/// AST から検出されたネスト if パターンを表現する。
#[derive(Debug)]
pub(crate) struct NestedIfPattern {
    /// 条件式リスト（外側から内側へ）
    pub(crate) conds: Vec<serde_json::Value>,
    /// 変数代入リスト: (変数名, then値, else値)
    pub(crate) merges: Vec<(String, serde_json::Value, Option<serde_json::Value>)>,
}
