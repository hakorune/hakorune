//! BoxCompilationContext - 箱理論による静的Box コンパイル時のコンテキスト分離
//!
//! 設計原則：
//! - 各static boxコンパイルごとに独立したコンテキストを作成
//! - グローバル状態への依存を排除し、汚染を構造的に不可能にする
//! - コンテキストのライフタイムでリソース管理を自動化

use hakorune_mir_core::{MirType, ValueId};
use std::collections::BTreeMap; // Phase 25.1: 決定性確保

/// 静的Boxコンパイル時のコンテキスト
///
/// 箱理論の原則に従い、各static boxのコンパイルは独立したコンテキストで実行されます。
/// これにより、using文や前のboxからのメタデータ汚染を構造的に防止します。
///
/// # 使用例
/// ```rust,ignore
/// let mut ctx = BoxCompilationContext::new();
/// // ctx を使ってメソッドをコンパイル
/// // スコープを抜けると自動的にクリーンアップ
/// ```
#[derive(Debug, Clone, Default)]
pub struct BoxCompilationContext {
    /// 変数名 → ValueId マッピング
    /// 例: "args" → ValueId(0), "result" → ValueId(42)
    /// Phase 25.1: HashMap → BTreeMap（PHI生成の決定性確保）
    #[allow(dead_code)]
    pub variable_map: BTreeMap<String, ValueId>,

    /// ValueId → 起源Box名 マッピング
    /// NewBox命令で生成されたValueIdがどのBox型から来たかを追跡
    /// 例: ValueId(10) → "ParserBox"
    /// Phase 25.1: HashMap → BTreeMap（決定性確保）
    #[allow(dead_code)]
    pub value_origin_newbox: BTreeMap<ValueId, String>,

    /// ValueId → MIR型 マッピング
    /// 各ValueIdの型情報を保持
    /// 例: ValueId(5) → MirType::Integer
    /// Phase 25.1: HashMap → BTreeMap（決定性確保）
    #[allow(dead_code)]
    pub value_types: BTreeMap<ValueId, MirType>,
}

impl BoxCompilationContext {
    /// 新しい空のコンテキストを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// コンテキストが空（未使用）かどうかを判定
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.variable_map.is_empty()
            && self.value_origin_newbox.is_empty()
            && self.value_types.is_empty()
    }

    /// デバッグ用：コンテキストのサイズ情報を取得
    #[allow(dead_code)]
    pub fn size_info(&self) -> (usize, usize, usize) {
        (
            self.variable_map.len(),
            self.value_origin_newbox.len(),
            self.value_types.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = BoxCompilationContext::new();
        assert!(ctx.is_empty());
    }

    #[test]
    fn test_context_isolation() {
        let mut ctx1 = BoxCompilationContext::new();
        ctx1.variable_map.insert("x".to_string(), ValueId::new(1));

        let ctx2 = BoxCompilationContext::new();
        assert!(ctx2.is_empty(), "新しいコンテキストは空であるべき");
        assert!(!ctx1.is_empty(), "ctx1は変更されたまま");
    }

    #[test]
    fn test_size_info() {
        let mut ctx = BoxCompilationContext::new();
        ctx.variable_map.insert("a".to_string(), ValueId::new(1));
        ctx.value_origin_newbox
            .insert(ValueId::new(2), "StringBox".to_string());
        ctx.value_types.insert(ValueId::new(3), MirType::Integer);

        let (vars, origins, types) = ctx.size_info();
        assert_eq!(vars, 1);
        assert_eq!(origins, 1);
        assert_eq!(types, 1);
    }
}
