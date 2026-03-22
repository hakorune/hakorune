use super::{MirBuilder, ValueId};
use hakorune_mir_core::{MirValueKind, TypedValueId};

impl MirBuilder {
    // ============================================================================
    // Phase 26-A: ValueId型安全化メソッド
    // ============================================================================

    /// 型付きValueIdを発行（新API）
    /// Phase 136 P0: Use SSOT allocator (next_value_id) to respect function context
    pub fn new_typed_value(&mut self, kind: MirValueKind) -> TypedValueId {
        let id = self.next_value_id();
        self.type_ctx.value_kinds.insert(id, kind);
        TypedValueId::new(id, kind)
    }

    /// 既存ValueIdの型情報を取得
    pub fn get_value_kind(&self, id: ValueId) -> Option<MirValueKind> {
        self.type_ctx.value_kinds.get(&id).copied()
    }

    /// 既存ValueIdに型情報を後付け（レガシー互換用）
    pub fn register_value_kind(&mut self, id: ValueId, kind: MirValueKind) {
        self.type_ctx.value_kinds.insert(id, kind);
    }

    /// 型安全なパラメータ判定（ValueIdベース） - GUARD Bug Prevention
    pub fn is_value_parameter(&self, id: ValueId) -> bool {
        self.get_value_kind(id)
            .map(|kind| kind.is_parameter())
            .unwrap_or(false)
    }

    /// 型安全なローカル変数判定（ValueIdベース）
    pub fn is_value_local(&self, id: ValueId) -> bool {
        self.get_value_kind(id)
            .map(|kind| kind.is_local())
            .unwrap_or(false)
    }

    /// 型安全なLoopCarrier判定（ValueIdベース）
    pub fn is_value_loop_carrier(&self, id: ValueId) -> bool {
        self.get_value_kind(id)
            .map(|kind| kind.is_loop_carrier())
            .unwrap_or(false)
    }
}
