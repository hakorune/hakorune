use super::VarId;
use std::collections::BTreeMap;

/// Phase 34-5: 式から値を抽出する際のコンテキスト
///
/// extract_value ヘルパ関数で使用する内部状態
pub(crate) struct ExtractCtx {
    /// 次に使える ValueId カウンタ
    pub(crate) next_var_id: u32,
    /// 変数名 → ValueId のマップ（パラメータなど）
    pub(crate) var_map: BTreeMap<String, VarId>,
}

impl ExtractCtx {
    /// 新しいコンテキストを作成
    pub(crate) fn new(start_var_id: u32) -> Self {
        Self {
            next_var_id: start_var_id,
            var_map: BTreeMap::new(),
        }
    }

    /// パラメータを登録
    pub(crate) fn register_param(&mut self, name: String, var_id: VarId) {
        self.var_map.insert(name, var_id);
    }

    /// 新しい ValueId を割り当て
    pub(crate) fn alloc_var(&mut self) -> VarId {
        let id = crate::mir::ValueId(self.next_var_id);
        self.next_var_id += 1;
        id
    }

    /// 変数名から ValueId を取得
    pub(crate) fn get_var(&self, name: &str) -> Option<VarId> {
        self.var_map.get(name).copied()
    }
}
