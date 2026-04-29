//! Phase 192: Generic Case A - Entry Function Builder
//!
//! 責務: 4つのループパターン共通のEntry関数 name → ValueId map 構築処理
//! - ValueId/BTreeMap の初期化ボイラープレート統一化

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Entry関数構築用の統一ビルダー
///
/// 4つのループパターン（skip_ws, trim, append_defs, stage1）で
/// 共通する name → ValueId map のボイラープレート処理を集約
#[derive(Clone, Debug)]
pub struct EntryFunctionBuilder {
    /// 変数名 → ValueId のマッピング（決定性重視で BTreeMap使用）
    name_to_id: BTreeMap<String, ValueId>,
}

impl EntryFunctionBuilder {
    /// 新しいビルダーを作成
    pub fn new() -> Self {
        Self {
            name_to_id: BTreeMap::new(),
        }
    }

    /// 変数を追加
    pub fn add_var(&mut self, name: String, id: ValueId) {
        self.name_to_id.insert(name, id);
    }

    /// マッピング全体を取得
    pub fn get_map(&self) -> &BTreeMap<String, ValueId> {
        &self.name_to_id
    }
}

impl Default for EntryFunctionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_builder_get_map() {
        let mut builder = EntryFunctionBuilder::new();
        builder.add_var("x".to_string(), ValueId(5));
        builder.add_var("y".to_string(), ValueId(6));

        let map = builder.get_map();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("x"), Some(&ValueId(5)));
        assert_eq!(map.get("y"), Some(&ValueId(6)));
    }
}
