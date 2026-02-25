//! Phase 192: Generic Case A - Entry Function Builder
//!
//! 責務: 4つのループパターン共通のEntry関数構築処理
//! - ValueId/BTreeMap の初期化ボイラープレート統一化
//! - Pinned/Carrier 変数の一元管理

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Entry関数構築用の統一ビルダー
///
/// 4つのループパターン（skip_ws, trim, append_defs, stage1）で
/// 共通するボイラープレート処理を集約
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct EntryFunctionBuilder {
    /// 変数名 → ValueId のマッピング（決定性重視で BTreeMap使用）
    name_to_id: BTreeMap<String, ValueId>,
    /// Pinned変数のリスト
    pinned_vars: Vec<String>,
    /// Carrier変数のリスト
    carrier_vars: Vec<String>,
}

impl EntryFunctionBuilder {
    /// 新しいビルダーを作成
    pub fn new() -> Self {
        Self {
            name_to_id: BTreeMap::new(),
            pinned_vars: Vec::new(),
            carrier_vars: Vec::new(),
        }
    }

    /// Pinned変数を追加
    #[allow(dead_code)]
    pub fn add_pinned(&mut self, name: String, id: ValueId) {
        self.name_to_id.insert(name.clone(), id);
        self.pinned_vars.push(name);
    }

    /// Carrier変数を追加
    #[allow(dead_code)]
    pub fn add_carrier(&mut self, name: String, id: ValueId) {
        self.name_to_id.insert(name.clone(), id);
        self.carrier_vars.push(name);
    }

    /// 一般的な変数を追加（pinned/carrier以外）
    pub fn add_var(&mut self, name: String, id: ValueId) {
        self.name_to_id.insert(name, id);
    }

    /// ループ開始時の引数リストを構築
    #[allow(dead_code)]
    pub fn build_loop_args(&self) -> Option<Vec<ValueId>> {
        // Pinned変数をループ開始時の引数として返す
        if self.pinned_vars.is_empty() {
            return None;
        }

        let args = self
            .pinned_vars
            .iter()
            .filter_map(|name| self.name_to_id.get(name).copied())
            .collect::<Vec<_>>();

        if args.is_empty() {
            None
        } else {
            Some(args)
        }
    }

    /// 指定された変数のValueIdを取得
    #[allow(dead_code)]
    pub fn get_id(&self, name: &str) -> Option<ValueId> {
        self.name_to_id.get(name).copied()
    }

    /// すべての変数IDを取得
    #[allow(dead_code)]
    pub fn get_all_ids(&self) -> Vec<ValueId> {
        self.name_to_id.values().copied().collect()
    }

    /// マッピング全体を取得
    pub fn get_map(&self) -> &BTreeMap<String, ValueId> {
        &self.name_to_id
    }

    /// Pinned変数の数
    #[allow(dead_code)]
    pub fn pinned_count(&self) -> usize {
        self.pinned_vars.len()
    }

    /// Carrier変数の数
    #[allow(dead_code)]
    pub fn carrier_count(&self) -> usize {
        self.carrier_vars.len()
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
    fn test_entry_builder_new() {
        let builder = EntryFunctionBuilder::new();
        assert_eq!(builder.pinned_count(), 0);
        assert_eq!(builder.carrier_count(), 0);
    }

    #[test]
    fn test_entry_builder_add_pinned() {
        let mut builder = EntryFunctionBuilder::new();
        builder.add_pinned("i".to_string(), ValueId(0));
        assert_eq!(builder.pinned_count(), 1);
        assert_eq!(builder.get_id("i"), Some(ValueId(0)));
    }

    #[test]
    fn test_entry_builder_add_carrier() {
        let mut builder = EntryFunctionBuilder::new();
        builder.add_carrier("j".to_string(), ValueId(1));
        assert_eq!(builder.carrier_count(), 1);
        assert_eq!(builder.get_id("j"), Some(ValueId(1)));
    }

    #[test]
    fn test_entry_builder_loop_args() {
        let mut builder = EntryFunctionBuilder::new();
        builder.add_pinned("i".to_string(), ValueId(0));
        builder.add_pinned("j".to_string(), ValueId(1));

        let args = builder.build_loop_args();
        assert!(args.is_some());
        assert_eq!(args.unwrap().len(), 2);
    }

    #[test]
    fn test_entry_builder_no_pinned() {
        let builder = EntryFunctionBuilder::new();
        assert_eq!(builder.build_loop_args(), None);
    }

    #[test]
    fn test_entry_builder_add_var() {
        let mut builder = EntryFunctionBuilder::new();
        builder.add_var("temp".to_string(), ValueId(10));
        assert_eq!(builder.get_id("temp"), Some(ValueId(10)));
    }

    #[test]
    fn test_entry_builder_get_map() {
        let mut builder = EntryFunctionBuilder::new();
        builder.add_pinned("x".to_string(), ValueId(5));
        builder.add_carrier("y".to_string(), ValueId(6));

        let map = builder.get_map();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("x"), Some(&ValueId(5)));
        assert_eq!(map.get("y"), Some(&ValueId(6)));
    }
}
