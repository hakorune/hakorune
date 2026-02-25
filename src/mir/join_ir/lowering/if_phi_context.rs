//! Phase 61-1: If-in-loop 用 PHI コンテキスト
//!
//! loop_builder.rs から carrier_names 情報を JoinIR に渡すための構造体
//!
//! ## 背景
//!
//! ループ内の if では、ループキャリア変数に対して「片腕 PHI」が必要になる。
//! 従来は PhiBuilderBox::set_if_context() で carrier_names を渡していたが、
//! Phase 61-1 で JoinIR 経路に統一するため、この情報を渡す手段が必要。
//!
//! ## Phase 61-3 拡張
//!
//! incoming 値解決メソッドを追加し、CFG依存ロジックをIfPhiContextに集約。
//!
//! ## 設計
//!
//! ```
//! // loop_builder.rs
//! let carrier_names = ...;
//! let context = IfPhiContext::for_loop_body(carrier_names);
//! try_lower_if_to_joinir(func, block_id, debug, Some(&context));
//! ```
//!
//! ## 責務
//!
//! - ループ内 if のコンテキスト情報を保持
//! - carrier_names の判定ユーティリティ提供
//! - Phase 61-3: incoming値解決（CFG依存ロジック）

use std::collections::BTreeSet;

/// If-in-loop 用 PHI コンテキスト
#[derive(Debug, Clone)]
pub struct IfPhiContext {
    /// ループ内の if かどうか
    ///
    /// true の場合、carrier_names に含まれる変数は「片腕 PHI」が必要
    pub in_loop_body: bool,

    /// ループキャリア変数名リスト
    ///
    /// ループ内 if で片腕 PHI が必要な変数を指定する。
    /// 例: `loop(i < 3) { if cond { x = 1 } }` の場合、
    /// x は carrier 変数として扱われ、else 側で pre_if 値を使用する PHI を生成する。
    pub carrier_names: BTreeSet<String>,
}

impl IfPhiContext {
    /// ループ内 if 用コンテキスト作成
    ///
    /// # Arguments
    ///
    /// * `carrier_names` - ループキャリア変数名リスト（BTreeSet で決定的イテレーション保証）
    ///
    /// # Example
    ///
    /// ```ignore
    /// let carrier_names = pre_if_var_map
    ///     .keys()
    ///     .filter(|name| !name.starts_with("__pin$"))
    ///     .cloned()
    ///     .collect();
    ///
    /// let context = IfPhiContext::for_loop_body(carrier_names);
    /// ```
    pub fn for_loop_body(carrier_names: BTreeSet<String>) -> Self {
        Self {
            in_loop_body: true,
            carrier_names,
        }
    }

    /// ループ外 if 用コンテキスト作成（Phase 61-4）
    ///
    /// ループ外の純粋な if に対しては、carrier_names は空になる。
    /// これにより、JoinIR は純粋な Select/IfMerge パターンとして処理する。
    ///
    /// # Example
    ///
    /// ```ignore
    /// let context = IfPhiContext::pure_if();
    /// try_lower_if_to_joinir(func, block_id, debug, Some(&context));
    /// ```
    pub fn pure_if() -> Self {
        Self {
            in_loop_body: false,
            carrier_names: BTreeSet::new(),
        }
    }

    /// 指定された変数がループキャリアかどうか判定
    ///
    /// # Arguments
    ///
    /// * `var_name` - 判定対象の変数名
    ///
    /// # Returns
    ///
    /// - `true`: ループキャリア変数（片腕 PHI が必要）
    /// - `false`: 通常変数（純粋な if PHI）
    ///
    /// # Example
    ///
    /// ```ignore
    /// if context.is_carrier("x") {
    ///     // 片腕 PHI 生成: phi [then_val, pre_if_val]
    /// } else {
    ///     // 純粋 if PHI: phi [then_val, else_val]
    /// }
    /// ```
    pub fn is_carrier(&self, var_name: &str) -> bool {
        self.carrier_names.contains(var_name)
    }

    /// ループキャリア変数の数を取得
    pub fn carrier_count(&self) -> usize {
        self.carrier_names.len()
    }

    /// ループ内 if かどうか判定
    pub fn is_in_loop(&self) -> bool {
        self.in_loop_body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_loop_body() {
        let mut carrier_names = BTreeSet::new();
        carrier_names.insert("x".to_string());
        carrier_names.insert("y".to_string());

        let context = IfPhiContext::for_loop_body(carrier_names);

        assert!(context.is_in_loop());
        assert!(context.is_carrier("x"));
        assert!(context.is_carrier("y"));
        assert!(!context.is_carrier("z"));
        assert_eq!(context.carrier_count(), 2);
    }

    #[test]
    fn test_is_carrier() {
        let mut carrier_names = BTreeSet::new();
        carrier_names.insert("loop_var".to_string());

        let context = IfPhiContext::for_loop_body(carrier_names);

        assert!(context.is_carrier("loop_var"));
        assert!(!context.is_carrier("local_var"));
    }
}
