// value_kind.rs
// Phase 26-A: ValueId型安全化
//
// Purpose:
// - ValueIdの意味的分類（Parameter, Local, Constant等）を導入
// - GUARDバグのような「ValueId(0)の曖昧性」から生じるバグを根絶

use crate::ValueId;

/// ValueIdの意味的分類（型安全性強化）
///
/// # 目的
///
/// ValueIdは単なる整数ラッパー `ValueId(u32)` であり、
/// その値が「パラメータ」「ローカル変数」「定数」のいずれを表すか区別できない。
/// これにより、以下のようなバグが発生していた：
///
/// ## GUARDバグの例（修正済み）
///
/// ```rust
/// // ❌ ValueId(0) を「常に未初期化」と誤判定
/// for (name, value) in &current_vars {
///     if value.0 == 0 {  // ← Parameter s=ValueId(0) も弾いてしまう！
///         return Ok(ValueId(0));
///     }
/// }
/// ```
///
/// ## 解決策
///
/// `MirValueKind` で値の種類を明示的に管理し、
/// `TypedValueId` で ValueId + 型情報をペアで保持することで、
/// コンパイル時・実行時の型安全性を確保する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MirValueKind {
    /// 関数パラメータ
    ///
    /// - パラメータインデックス（0-based）を保持
    /// - 例: `fn skip_whitespace(s, idx)` → s=Parameter(0), idx=Parameter(1)
    ///
    /// # 重要
    ///
    /// ValueId(0) であってもParameterなら正当な値！
    /// GUARD checkバグはこれを見落としていた。
    Parameter(u32),

    /// ローカル変数
    ///
    /// - スコープ内のローカル番号（関数ごとに独立）
    /// - 例: `local i = 0` → Local(0)
    ///
    /// # SSA形式との関係
    ///
    /// SSA形式では再代入時に新しいValueIdが割り当てられるため、
    /// 同じ名前でも複数のLocal(N)が存在しうる。
    Local(u32),

    /// 定数値
    ///
    /// - コンパイル時に値が確定している
    /// - 例: 42, "hello", true
    ///
    /// # Note
    ///
    /// ConstValueの実体はMirInstruction::Constに格納される。
    /// MirValueKindはあくまで「定数である」というマーカー。
    Constant,

    /// 一時値
    ///
    /// - 式評価・演算の中間結果
    /// - 例: `a + b` の結果、copy, phi の結果
    ///
    /// # デフォルト扱い
    ///
    /// 明示的に分類されないValueIdはTemporaryとして扱われる。
    Temporary,

    /// Pinned変数
    ///
    /// - ブロック跨ぎの一時変数（SSA構築用）
    /// - 命名規則: `__pin$N$@<suffix>`
    /// - 例: `__pin$42$@binop_lhs`
    ///
    /// # 実装詳細
    ///
    /// MIRビルダーがSSA形式を構築する際、
    /// ブロック境界でのdef-use関係を追跡するために使用される。
    Pinned,

    /// LoopCarrier
    ///
    /// - ループ内で再定義される変数
    /// - PHI nodeで複数の値をマージ
    /// - 例: loop内で更新される `i`
    ///
    /// # PHI nodeとの関係
    ///
    /// ```hako
    /// local i = 0
    /// loop(i < 10) {
    ///     i = i + 1  // ← i は LoopCarrier
    /// }
    /// ```
    ///
    /// MIR表現:
    /// ```
    /// header:
    ///   %i_phi = phi [(%i_entry, preheader), (%i_next, latch)]
    /// body:
    ///   %i_next = binop %i_phi + 1
    /// ```
    LoopCarrier,
}

impl MirValueKind {
    /// パラメータか判定
    pub fn is_parameter(&self) -> bool {
        matches!(self, MirValueKind::Parameter(_))
    }

    /// ローカル変数か判定
    pub fn is_local(&self) -> bool {
        matches!(self, MirValueKind::Local(_))
    }

    /// 定数か判定
    pub fn is_constant(&self) -> bool {
        matches!(self, MirValueKind::Constant)
    }

    /// 一時値か判定
    pub fn is_temporary(&self) -> bool {
        matches!(self, MirValueKind::Temporary)
    }

    /// Pinned変数か判定
    pub fn is_pinned(&self) -> bool {
        matches!(self, MirValueKind::Pinned)
    }

    /// LoopCarrierか判定
    pub fn is_loop_carrier(&self) -> bool {
        matches!(self, MirValueKind::LoopCarrier)
    }

    /// パラメータインデックス取得（Parameterのみ）
    pub fn parameter_index(&self) -> Option<u32> {
        match self {
            MirValueKind::Parameter(idx) => Some(*idx),
            _ => None,
        }
    }

    /// ローカル変数番号取得（Localのみ）
    pub fn local_index(&self) -> Option<u32> {
        match self {
            MirValueKind::Local(idx) => Some(*idx),
            _ => None,
        }
    }
}

/// 型付きValueId - ValueIdに意味情報を付与
///
/// # 設計思想
///
/// - ValueId: 実際の識別子（既存システムとの互換性）
/// - MirValueKind: 値の種類（新規導入の型情報）
///
/// # 使用例
///
/// ```rust
/// // パラメータ s=ValueId(0)
/// let s = TypedValueId::new(ValueId(0), MirValueKind::Parameter(0));
/// assert!(s.is_parameter());
/// assert_eq!(s.value_id(), ValueId(0));
///
/// // ローカル変数 i=ValueId(10)
/// let i = TypedValueId::new(ValueId(10), MirValueKind::Local(0));
/// assert!(i.is_local());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TypedValueId {
    /// 実際のValueId（既存システムとの互換性）
    pub id: ValueId,

    /// 値の種類
    pub kind: MirValueKind,
}

impl TypedValueId {
    /// 新規作成
    pub fn new(id: ValueId, kind: MirValueKind) -> Self {
        Self { id, kind }
    }

    /// パラメータか判定（型安全）
    ///
    /// # GUARDバグ予防
    ///
    /// ```rust
    /// let s = TypedValueId::new(ValueId(0), MirValueKind::Parameter(0));
    /// assert!(s.is_parameter());  // ✅ ValueId(0) でも正しく判定！
    /// ```
    pub fn is_parameter(&self) -> bool {
        self.kind.is_parameter()
    }

    /// ローカル変数か判定
    pub fn is_local(&self) -> bool {
        self.kind.is_local()
    }

    /// 定数か判定
    pub fn is_constant(&self) -> bool {
        self.kind.is_constant()
    }

    /// 一時値か判定
    pub fn is_temporary(&self) -> bool {
        self.kind.is_temporary()
    }

    /// Pinned変数か判定
    pub fn is_pinned(&self) -> bool {
        self.kind.is_pinned()
    }

    /// LoopCarrierか判定
    pub fn is_loop_carrier(&self) -> bool {
        self.kind.is_loop_carrier()
    }

    /// ValueIdを取得（後方互換性）
    pub fn value_id(&self) -> ValueId {
        self.id
    }

    /// MirValueKindを取得
    pub fn kind(&self) -> MirValueKind {
        self.kind
    }

    /// パラメータインデックス取得（Parameterのみ）
    pub fn parameter_index(&self) -> Option<u32> {
        self.kind.parameter_index()
    }

    /// ローカル変数番号取得（Localのみ）
    pub fn local_index(&self) -> Option<u32> {
        self.kind.local_index()
    }
}

/// ValueIdへの自動変換（後方互換性）
///
/// # 使用例
///
/// ```rust
/// let typed = TypedValueId::new(ValueId(5), MirValueKind::Local(0));
/// let id: ValueId = typed.into();  // 自動変換
/// assert_eq!(id, ValueId(5));
/// ```
impl From<TypedValueId> for ValueId {
    fn from(typed: TypedValueId) -> ValueId {
        typed.id
    }
}

/// PartialEq実装（ValueIdベースで比較）
impl PartialEq for TypedValueId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TypedValueId {}

/// Hash実装（ValueIdベースでハッシュ）
impl std::hash::Hash for TypedValueId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// ============================================================================
// ユニットテスト
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mir_value_kind_parameter() {
        let kind = MirValueKind::Parameter(0);
        assert!(kind.is_parameter());
        assert!(!kind.is_local());
        assert!(!kind.is_constant());
        assert_eq!(kind.parameter_index(), Some(0));
    }

    #[test]
    fn test_mir_value_kind_local() {
        let kind = MirValueKind::Local(5);
        assert!(kind.is_local());
        assert!(!kind.is_parameter());
        assert_eq!(kind.local_index(), Some(5));
    }

    #[test]
    fn test_mir_value_kind_constant() {
        let kind = MirValueKind::Constant;
        assert!(kind.is_constant());
        assert!(!kind.is_temporary());
    }

    #[test]
    fn test_mir_value_kind_temporary() {
        let kind = MirValueKind::Temporary;
        assert!(kind.is_temporary());
        assert!(!kind.is_constant());
    }

    #[test]
    fn test_mir_value_kind_pinned() {
        let kind = MirValueKind::Pinned;
        assert!(kind.is_pinned());
        assert!(!kind.is_temporary());
    }

    #[test]
    fn test_mir_value_kind_loop_carrier() {
        let kind = MirValueKind::LoopCarrier;
        assert!(kind.is_loop_carrier());
        assert!(!kind.is_local());
    }

    #[test]
    fn test_typed_value_id_parameter() {
        let typed = TypedValueId::new(ValueId(0), MirValueKind::Parameter(0));
        assert!(typed.is_parameter());
        assert!(!typed.is_local());
        assert_eq!(typed.value_id(), ValueId(0));
        assert_eq!(typed.parameter_index(), Some(0));
    }

    #[test]
    fn test_typed_value_id_local() {
        let typed = TypedValueId::new(ValueId(10), MirValueKind::Local(0));
        assert!(typed.is_local());
        assert!(!typed.is_parameter());
        assert_eq!(typed.value_id(), ValueId(10));
        assert_eq!(typed.local_index(), Some(0));
    }

    #[test]
    fn test_typed_value_id_conversion_to_value_id() {
        let typed = TypedValueId::new(ValueId(42), MirValueKind::Temporary);
        let id: ValueId = typed.into();
        assert_eq!(id, ValueId(42));
    }

    #[test]
    fn test_typed_value_id_equality() {
        let a = TypedValueId::new(ValueId(5), MirValueKind::Local(0));
        let b = TypedValueId::new(ValueId(5), MirValueKind::Parameter(0));
        // 同じValueIdなら種類が違っても等しい（ValueIdベース比較）
        assert_eq!(a, b);
    }

    /// GUARDバグ再現防止テスト
    ///
    /// # 背景
    ///
    /// loop_builder.rs で以下のバグがあった：
    ///
    /// ```rust
    /// // ❌ ValueId(0) を「常に未初期化」と誤判定
    /// for (name, value) in &current_vars {
    ///     if value.0 == 0 {  // ← Parameter s=ValueId(0) も弾く！
    ///         return Ok(ValueId(0));
    ///     }
    /// }
    /// ```
    ///
    /// # 期待動作
    ///
    /// TypedValueIdを使えば、ValueId(0)でも
    /// Parameterとして正しく判定できる。
    #[test]
    fn test_guard_check_bug_prevention() {
        // パラメータ s=ValueId(0), idx=ValueId(1)
        let s = TypedValueId::new(ValueId(0), MirValueKind::Parameter(0));
        let idx = TypedValueId::new(ValueId(1), MirValueKind::Parameter(1));

        // ✅ ValueId(0) でもパラメータとして正しく判定される
        assert!(s.is_parameter());
        assert_eq!(s.value_id(), ValueId(0));
        assert_eq!(s.parameter_index(), Some(0));

        assert!(idx.is_parameter());
        assert_eq!(idx.value_id(), ValueId(1));
        assert_eq!(idx.parameter_index(), Some(1));

        // ローカル変数 i=ValueId(2)
        let i = TypedValueId::new(ValueId(2), MirValueKind::Local(0));
        assert!(!i.is_parameter());
        assert!(i.is_local());

        // ❌ 旧実装（名前ベース判定）では不可能だった区別が可能に！
    }

    #[test]
    fn test_loop_carrier_detection() {
        let carrier = TypedValueId::new(ValueId(100), MirValueKind::LoopCarrier);
        assert!(carrier.is_loop_carrier());
        assert!(!carrier.is_parameter());
        assert!(!carrier.is_local());
    }
}
