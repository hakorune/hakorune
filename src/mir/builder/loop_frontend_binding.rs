//! Phase 50: Loop Frontend Binding
//!
//! このモジュールは cf_loop から JoinIR Frontend への変数マッピングを提供する。
//!
//! ## 問題背景
//!
//! JoinIR Frontend (`loop_patterns.rs`) はハードコードされた変数名を期待:
//! - `i`: カウンタ変数
//! - `acc`: アキュムレータ変数
//! - `n`: ループ上限
//!
//! しかし実際のループは異なる名前や構造を持つ:
//! - print_tokens: acc が存在しない（副作用のみ）
//! - filter: acc は `out` という名前
//!
//! ## 解決策
//!
//! LoopFrontendBinding が「実際の変数名」と「JoinIR 期待変数名」をマッピングし、
//! JSON v0 生成時に適切な Local 宣言を注入する。

use crate::ast::ASTNode;

/// ループ変数と JoinIR Frontend 期待変数のマッピング
#[derive(Debug, Clone)]
pub struct LoopFrontendBinding {
    /// カウンタ変数名 (e.g., "i")
    pub counter_var: String,

    /// カウンタの初期値
    pub counter_init: i64,

    /// アキュムレータ変数名 (None for side-effect loops like print_tokens)
    pub accumulator_var: Option<String>,

    /// ループ上限式 (e.g., "n" or "arr.size()")
    pub bound_expr: BoundExpr,

    /// 外部参照 (e.g., ["me.tokens", "arr", "pred"])
    pub external_refs: Vec<String>,

    /// ループパターン種別
    pub pattern: LoopPattern,
}

/// ループ上限の表現
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BoundExpr {
    /// 変数名 (e.g., "n")
    Variable(String),
    /// メソッド呼び出し (e.g., "me.tokens", "length")
    MethodCall { receiver: String, method: String },
    /// 定数値 (e.g., 3 for `i < 3`) - Phase 188-Impl-1
    Constant(i64),
}

/// ループパターン種別
#[derive(Debug, Clone, PartialEq)]
pub enum LoopPattern {
    /// 単純カウンタループ (print_tokens 相当)
    /// has_accumulator: アキュムレータが存在するか
    Simple { has_accumulator: bool },

    /// フィルターパターン (if + push)
    Filter,
}

impl LoopFrontendBinding {
    /// Phase 52: Check if `me` receiver is needed for this loop
    ///
    /// Returns true if any external_ref starts with "me" (e.g., "me.tokens")
    /// This indicates an instance method that needs access to `me`.
    pub fn needs_me_receiver(&self) -> bool {
        self.external_refs
            .iter()
            .any(|r| r == "me" || r.starts_with("me."))
    }

    /// print_tokens 専用のバインディングを生成
    ///
    /// print_tokens の構造:
    /// ```nyash
    /// local i = 0
    /// loop(i < me.tokens.length()) {
    ///     local token = me.tokens.get(i)
    ///     print(token)
    ///     i = i + 1
    /// }
    /// ```
    pub fn for_print_tokens() -> Self {
        Self {
            counter_var: "i".to_string(),
            counter_init: 0,
            accumulator_var: None, // print_tokens has no accumulator
            bound_expr: BoundExpr::MethodCall {
                receiver: "me.tokens".to_string(),
                method: "length".to_string(),
            },
            external_refs: vec!["me.tokens".to_string()],
            pattern: LoopPattern::Simple {
                has_accumulator: false,
            },
        }
    }

    /// array_ext.filter 専用のバインディングを生成
    ///
    /// filter の構造:
    /// ```nyash
    /// local out = new ArrayBox()
    /// local i = 0
    /// local n = arr.size()   // ← JoinIR では arr.size() を直接評価
    /// loop(i < n) {
    ///     local v = arr.get(i)
    ///     if pred(v) { out.push(v) }
    ///     i = i + 1
    /// }
    /// return out
    /// ```
    ///
    /// Phase 56: `BoundExpr::Variable("n")` から `BoundExpr::MethodCall` に変更。
    /// `n` はループ外で宣言されているため、JoinIR コンテキストに存在しない。
    /// 代わりに `arr.size()` を直接呼び出して `n` を初期化する。
    pub fn for_array_filter() -> Self {
        Self {
            counter_var: "i".to_string(),
            counter_init: 0,
            accumulator_var: Some("out".to_string()), // filter has "out" as accumulator
            // Phase 56: Variable("n") → MethodCall { arr, size }
            // n はループ外で宣言されているため、arr.size() を直接呼び出す
            bound_expr: BoundExpr::MethodCall {
                receiver: "arr".to_string(),
                method: "size".to_string(),
            },
            external_refs: vec!["arr".to_string(), "pred".to_string()],
            pattern: LoopPattern::Filter,
        }
    }

    /// Phase 188-Impl-1: main function simple while loop binding
    ///
    /// Structure for loop_min_while.hako:
    /// ```nyash
    /// local i = 0
    /// loop(i < 3) {
    ///     print(i)
    ///     i = i + 1
    /// }
    /// ```
    ///
    /// This is a simple counter loop with:
    /// - Counter variable: i (starts at 0)
    /// - Bound: constant 3
    /// - No accumulator (side-effect only loop)
    /// - No external refs
    #[allow(dead_code)]
    pub fn for_main_simple_while() -> Self {
        Self {
            counter_var: "i".to_string(),
            counter_init: 0,
            accumulator_var: None,              // No accumulator
            bound_expr: BoundExpr::Constant(3), // Constant bound
            external_refs: vec![],              // No external refs
            pattern: LoopPattern::Simple {
                has_accumulator: false,
            },
        }
    }

    /// ループ条件と本体から変数パターンを分析して Binding を生成
    ///
    /// Phase 50-3 で実装予定の汎用分析。現在は関数名ベースのハードコード。
    #[allow(dead_code)]
    pub fn analyze(_condition: &ASTNode, _body: &[ASTNode]) -> Option<Self> {
        // TODO: Phase 50-3 で AST 分析を実装
        // 1. カウンタ変数の検出 (i = i + 1 パターン)
        // 2. アキュムレータの検出 (return で返される変数)
        // 3. ループ上限の検出 (i < X の X)
        None
    }

    /// Phase 52: ドット区切りの変数名を Field ノード構造に変換
    ///
    /// 例: "me.tokens" → {"type": "Field", "object": {"type": "Var", "name": "me"}, "field": "tokens"}
    /// 例: "arr" → {"type": "Var", "name": "arr"}
    fn receiver_to_json(receiver: &str) -> serde_json::Value {
        use serde_json::json;

        if let Some(dot_pos) = receiver.find('.') {
            // ドット区切りがある場合 → Field ノードに分解
            let object_name = &receiver[..dot_pos];
            let field_name = &receiver[dot_pos + 1..];

            // ネストしたフィールドアクセス (e.g., "a.b.c") は再帰的に処理
            if field_name.contains('.') {
                // "me.tokens.inner" → Field(Field(Var("me"), "tokens"), "inner")
                let inner_receiver = Self::receiver_to_json(field_name);
                json!({
                    "type": "Field",
                    "object": { "type": "Var", "name": object_name },
                    "field": inner_receiver
                })
            } else {
                // 単一レベルのフィールドアクセス (e.g., "me.tokens")
                json!({
                    "type": "Field",
                    "object": { "type": "Var", "name": object_name },
                    "field": field_name
                })
            }
        } else {
            // ドットなし → 単純な Var ノード
            json!({
                "type": "Var",
                "name": receiver
            })
        }
    }

    /// JoinIR Frontend 用の JSON v0 Local 宣言を生成
    ///
    /// Returns: (i_local, acc_local, n_local) の JSON Value タプル
    ///
    /// Note: JoinIR Frontend expects specific type names:
    /// - "Int" for integer literals (with "value" field)
    /// - "Var" for variable references (with "name" field)
    /// - "Field" for field access (with "object", "field" fields) - Phase 52
    /// - "Method" for method calls (with "receiver", "method", "args" fields)
    /// - "NewBox" for box instantiation
    pub fn generate_local_declarations(
        &self,
    ) -> (serde_json::Value, serde_json::Value, serde_json::Value) {
        use serde_json::json;

        // i の Local 宣言
        // JoinIR Frontend expects "Int" type with direct "value" field
        let i_local = json!({
            "type": "Local",
            "name": "i",
            "expr": {
                "type": "Int",
                "value": self.counter_init
            }
        });

        // acc の Local 宣言（合成または実際の変数から）
        let acc_local = if self.accumulator_var.is_some() {
            // 実際のアキュムレータがある場合（filter 等）
            // "out" → "acc" にリネームして宣言
            json!({
                "type": "Local",
                "name": "acc",
                "expr": {
                    "type": "NewBox",
                    "box_name": "ArrayBox",
                    "args": []
                }
            })
        } else {
            // アキュムレータがない場合（print_tokens 等）
            // 合成の unit 値を使う（Int 0 として表現）
            json!({
                "type": "Local",
                "name": "acc",
                "expr": {
                    "type": "Int",
                    "value": 0
                }
            })
        };

        // n の Local 宣言
        let n_local = match &self.bound_expr {
            BoundExpr::Variable(var) => {
                // 既存変数を参照（filter の n 等）
                // JoinIR Frontend expects "Var" type
                json!({
                    "type": "Local",
                    "name": "n",
                    "expr": {
                        "type": "Var",
                        "name": var
                    }
                })
            }
            BoundExpr::MethodCall { receiver, method } => {
                // Phase 52: メソッド呼び出しを評価（print_tokens の me.tokens.length() 等）
                // receiver が "me.tokens" のようにドット区切りの場合は Field ノードに分解
                let receiver_json = Self::receiver_to_json(receiver);
                json!({
                    "type": "Local",
                    "name": "n",
                    "expr": {
                        "type": "Method",
                        "receiver": receiver_json,
                        "method": method,
                        "args": []
                    }
                })
            }
            BoundExpr::Constant(value) => {
                // Phase 188-Impl-1: 定数値（loop_min_while.hako の 3 等）
                // JoinIR Frontend expects "Int" type with direct "value" field
                json!({
                    "type": "Local",
                    "name": "n",
                    "expr": {
                        "type": "Int",
                        "value": value
                    }
                })
            }
        };

        (i_local, acc_local, n_local)
    }

    /// ループ本体の変数名をリネーム
    ///
    /// 実際の変数名 (e.g., "out") を JoinIR 期待名 (e.g., "acc") に置換
    pub fn rename_body_variables(&self, body_json: &mut Vec<serde_json::Value>) {
        if let Some(ref acc_name) = self.accumulator_var {
            if acc_name != "acc" {
                // "out" → "acc" にリネーム
                Self::rename_variable_in_json(body_json, acc_name, "acc");
            }
        }
    }

    /// JSON 内の変数名を再帰的にリネーム
    fn rename_variable_in_json(json_array: &mut Vec<serde_json::Value>, from: &str, to: &str) {
        for item in json_array.iter_mut() {
            Self::rename_variable_in_value(item, from, to);
        }
    }

    fn rename_variable_in_value(value: &mut serde_json::Value, from: &str, to: &str) {
        match value {
            serde_json::Value::Object(map) => {
                // Variable/Var ノードの名前をチェック
                // Note: ast_to_json uses "kind", JoinIR Frontend expects "type"
                let is_var = map.get("kind").and_then(|v| v.as_str()) == Some("Variable")
                    || map.get("type").and_then(|v| v.as_str()) == Some("Var");
                if is_var {
                    if let Some(name) = map.get_mut("name") {
                        if name.as_str() == Some(from) {
                            *name = serde_json::Value::String(to.to_string());
                        }
                    }
                }
                // Local ノードの名前をチェック
                if map.get("type").and_then(|v| v.as_str()) == Some("Local")
                    || map.get("kind").and_then(|v| v.as_str()) == Some("Local")
                {
                    if let Some(name) = map.get_mut("name") {
                        if name.as_str() == Some(from) {
                            *name = serde_json::Value::String(to.to_string());
                        }
                    }
                }
                // 再帰的に処理
                for (_, v) in map.iter_mut() {
                    Self::rename_variable_in_value(v, from, to);
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr.iter_mut() {
                    Self::rename_variable_in_value(item, from, to);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_tokens_binding() {
        let binding = LoopFrontendBinding::for_print_tokens();
        assert_eq!(binding.counter_var, "i");
        assert_eq!(binding.counter_init, 0);
        assert!(binding.accumulator_var.is_none());
        assert_eq!(
            binding.pattern,
            LoopPattern::Simple {
                has_accumulator: false
            }
        );
    }

    #[test]
    fn test_array_filter_binding() {
        let binding = LoopFrontendBinding::for_array_filter();
        assert_eq!(binding.counter_var, "i");
        assert_eq!(binding.accumulator_var, Some("out".to_string()));
        assert_eq!(binding.pattern, LoopPattern::Filter);
    }

    #[test]
    fn test_local_declarations_no_acc() {
        let binding = LoopFrontendBinding::for_print_tokens();
        let (i_local, acc_local, _n_local) = binding.generate_local_declarations();

        // i should be initialized to 0
        assert_eq!(i_local["name"], "i");
        assert_eq!(i_local["expr"]["type"], "Int");
        assert_eq!(i_local["expr"]["value"], 0);

        // acc should be synthetic (Int 0 for side-effect loops)
        assert_eq!(acc_local["name"], "acc");
        assert_eq!(acc_local["expr"]["type"], "Int");
        assert_eq!(acc_local["expr"]["value"], 0);
    }

    #[test]
    fn test_local_declarations_with_acc() {
        let binding = LoopFrontendBinding::for_array_filter();
        let (i_local, acc_local, n_local) = binding.generate_local_declarations();

        // i should be initialized to 0
        assert_eq!(i_local["name"], "i");
        assert_eq!(i_local["expr"]["type"], "Int");

        // acc should be new ArrayBox()
        assert_eq!(acc_local["name"], "acc");
        assert_eq!(acc_local["expr"]["type"], "NewBox");
        assert_eq!(acc_local["expr"]["box_name"], "ArrayBox");

        // Phase 56: n should be arr.size() method call (not a Var anymore)
        assert_eq!(n_local["name"], "n");
        assert_eq!(n_local["expr"]["type"], "Method");
        assert_eq!(n_local["expr"]["method"], "size");

        // receiver should be a simple Var node pointing to "arr"
        let receiver = &n_local["expr"]["receiver"];
        assert_eq!(receiver["type"], "Var");
        assert_eq!(receiver["name"], "arr");
    }

    // Phase 52: receiver_to_json のテスト
    #[test]
    fn test_receiver_to_json_simple_var() {
        let json = LoopFrontendBinding::receiver_to_json("arr");
        assert_eq!(json["type"], "Var");
        assert_eq!(json["name"], "arr");
    }

    #[test]
    fn test_receiver_to_json_field_access() {
        let json = LoopFrontendBinding::receiver_to_json("me.tokens");
        assert_eq!(json["type"], "Field");
        assert_eq!(json["object"]["type"], "Var");
        assert_eq!(json["object"]["name"], "me");
        assert_eq!(json["field"], "tokens");
    }

    #[test]
    fn test_print_tokens_n_local_has_field() {
        // Phase 52: print_tokens の n (me.tokens.length()) が Field ノードを使っているか確認
        let binding = LoopFrontendBinding::for_print_tokens();
        let (_i_local, _acc_local, n_local) = binding.generate_local_declarations();

        // n = me.tokens.length() の構造を確認
        assert_eq!(n_local["name"], "n");
        assert_eq!(n_local["expr"]["type"], "Method");
        assert_eq!(n_local["expr"]["method"], "length");

        // receiver が Field ノードであることを確認
        let receiver = &n_local["expr"]["receiver"];
        assert_eq!(receiver["type"], "Field");
        assert_eq!(receiver["object"]["type"], "Var");
        assert_eq!(receiver["object"]["name"], "me");
        assert_eq!(receiver["field"], "tokens");
    }
}
