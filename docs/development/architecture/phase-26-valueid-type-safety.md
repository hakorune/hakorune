# Phase 26-A: ValueId型安全化設計

## 🎯 目的

**GUARD checkバグのような「ValueIdの意味的曖昧性」から生じるバグを根絶する**

### 問題の本質

```rust
// ❌ 現状: ValueId(0) が何を意味するか不明
let value = ValueId(0);
// Parameter? Local? Constant? Temporary?
// → 実行時エラーまで気づかない！
```

**GUARD checkバグの例**:
```rust
// ValueId(0) を「常に未初期化」と誤判定
for (name, value) in &current_vars {
    if value.0 == 0 {  // ← Parameter s=ValueId(0) も弾いてしまう！
        return Ok(ValueId(0));
    }
}
```

---

## 📐 設計

### 1. MirValueKind列挙型

```rust
/// ValueIdの意味的分類（型安全性強化）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MirValueKind {
    /// 関数パラメータ
    /// - パラメータインデックス（0-based）を保持
    /// - 例: fn skip_whitespace(s, idx) → s=Parameter(0), idx=Parameter(1)
    Parameter(u32),

    /// ローカル変数
    /// - スコープ内のローカル番号（関数ごとに独立）
    /// - 例: local i = 0 → Local(0)
    Local(u32),

    /// 定数値
    /// - コンパイル時に値が確定
    /// - 例: 42, "hello", true
    Constant,

    /// 一時値
    /// - 式評価・演算の中間結果
    /// - 例: a + b の結果、copy, phi の結果
    Temporary,

    /// Pinned変数
    /// - ブロック跨ぎの一時変数（SSA構築用）
    /// - 例: __pin$N$@binop_lhs
    Pinned,

    /// LoopCarrier
    /// - ループ内で再定義される変数
    /// - PHI nodeで複数の値をマージ
    /// - 例: loop内で更新される i
    LoopCarrier,
}
```

### 2. TypedValueId構造体

```rust
/// 型付きValueId - ValueIdに意味情報を付与
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
    pub fn is_parameter(&self) -> bool {
        matches!(self.kind, MirValueKind::Parameter(_))
    }

    /// ローカル変数か判定
    pub fn is_local(&self) -> bool {
        matches!(self.kind, MirValueKind::Local(_))
    }

    /// 定数か判定
    pub fn is_constant(&self) -> bool {
        matches!(self.kind, MirValueKind::Constant)
    }

    /// LoopCarrierか判定
    pub fn is_loop_carrier(&self) -> bool {
        matches!(self.kind, MirValueKind::LoopCarrier)
    }

    /// ValueIdを取得（後方互換性）
    pub fn value_id(&self) -> ValueId {
        self.id
    }
}

impl From<TypedValueId> for ValueId {
    fn from(typed: TypedValueId) -> ValueId {
        typed.id
    }
}
```

### 3. MirBuilder統合

```rust
/// MirBuilderに追加するフィールド
pub struct MirBuilder {
    // ... 既存フィールド ...

    /// ValueIdの型情報マップ
    /// Phase 26-A: ValueId型安全化
    value_kinds: HashMap<ValueId, MirValueKind>,
}

impl MirBuilder {
    /// 型付きValueId発行（新API）
    pub fn new_typed_value(&mut self, kind: MirValueKind) -> TypedValueId {
        let id = self.next_value_id();
        self.value_kinds.insert(id, kind);
        TypedValueId::new(id, kind)
    }

    /// 既存ValueIdの型取得
    pub fn get_value_kind(&self, id: ValueId) -> Option<MirValueKind> {
        self.value_kinds.get(&id).copied()
    }

    /// 型情報を後付け（レガシー互換用）
    pub fn register_value_kind(&mut self, id: ValueId, kind: MirValueKind) {
        self.value_kinds.insert(id, kind);
    }

    /// 既存next_value_id()との互換性
    pub fn next_value_id(&mut self) -> ValueId {
        // 既存実装維持
        // デフォルトでTemporary扱い
        let id = ValueId(self.value_counter);
        self.value_counter += 1;
        self.value_kinds.insert(id, MirValueKind::Temporary);
        id
    }
}
```

---

## 🔧 実装戦略

### Phase 26-A-1: 基礎型実装（1日）

**ファイル**: `src/mir/value_kind.rs` (新規)

- [ ] `MirValueKind` 列挙型実装
- [ ] `TypedValueId` 構造体実装
- [ ] ユニットテスト作成

### Phase 26-A-2: MirBuilder統合（1日）

**ファイル**: `src/mir/builder.rs`

- [ ] `value_kinds` フィールド追加
- [ ] `new_typed_value()` 実装
- [ ] `get_value_kind()` 実装
- [ ] `register_value_kind()` 実装
- [ ] 既存`next_value_id()`の互換性維持

### Phase 26-A-3: パラメータ登録（0.5日）

**ファイル**: `src/mir/builder.rs`

関数パラメータ登録時に型情報を付与：

```rust
// setup_function_params() 修正箇所
for (idx, param_name) in params.iter().enumerate() {
    let param_id = ValueId(offset + idx);

    // Phase 26-A: パラメータ型登録
    self.register_value_kind(param_id, MirValueKind::Parameter(idx as u32));

    self.variable_map.insert(param_name.clone(), param_id);
}
```

### Phase 26-A-4: loop_builder.rs修正（0.5日）

**ファイル**: `src/mir/loop_builder.rs`

`is_parameter()` の根本修正：

```rust
// ❌ 旧実装（名前ベース、脆弱）
fn is_parameter(&self, name: &str) -> bool {
    if name.starts_with("__pin$") { return false; }
    if name == "me" { return true; }

    let is_param = self.parent_builder.function_param_names.contains(name);
    // ...
}

// ✅ 新実装（型ベース、型安全）
fn is_parameter(&self, value_id: ValueId) -> bool {
    self.parent_builder
        .get_value_kind(value_id)
        .map(|kind| matches!(kind, MirValueKind::Parameter(_)))
        .unwrap_or(false)
}
```

**呼び出し箇所の修正**:

```rust
// Phase 26-A: 名前ベース → ValueIdベースに変更
for (var_name, value_id) in &current_vars {
    if self.is_parameter(*value_id) {  // ← 修正
        // パラメータ処理
    }
}
```

### Phase 26-A-5: テスト作成（1日）

**ファイル**: `src/mir/value_kind.rs`, `src/tests/mir_value_kind.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_detection() {
        let typed = TypedValueId::new(ValueId(0), MirValueKind::Parameter(0));
        assert!(typed.is_parameter());
        assert!(!typed.is_local());
    }

    #[test]
    fn test_local_variable() {
        let typed = TypedValueId::new(ValueId(10), MirValueKind::Local(0));
        assert!(typed.is_local());
        assert!(!typed.is_parameter());
    }

    #[test]
    fn test_loop_carrier() {
        let typed = TypedValueId::new(ValueId(5), MirValueKind::LoopCarrier);
        assert!(typed.is_loop_carrier());
    }
}

#[test]
fn test_guard_check_bug_prevention() {
    // GUARD checkバグの再現防止テスト
    let mut builder = MirBuilder::new();

    // パラメータ s=ValueId(0), idx=ValueId(1)
    let s = builder.new_typed_value(MirValueKind::Parameter(0));
    let idx = builder.new_typed_value(MirValueKind::Parameter(1));

    // ValueId(0) でもパラメータとして正しく判定される
    assert!(s.is_parameter());
    assert_eq!(s.value_id(), ValueId(0));

    // ローカル変数 i=ValueId(2)
    let i = builder.new_typed_value(MirValueKind::Local(0));
    assert!(!i.is_parameter());
    assert!(i.is_local());
}
```

---

## 📊 期待される効果

### 定量的効果

| 指標 | 現状 | Phase 26-A後 | 改善率 |
|-----|------|--------------|--------|
| **ValueId判定ミス** | 年1-2回 | **0回** | **100%削減** |
| **is_parameter実装** | 20行（脆弱） | **3行（堅牢）** | **85%削減** |
| **型安全性** | なし | **完全** | - |
| **デバッグ時間** | 2-3時間/bug | **10分** | **95%削減** |

### 質的効果

- ✅ **GUARDバグ完全根絶**: ValueId(0)がParameterか判定可能
- ✅ **コンパイル時エラー**: 型不一致を実行前検出
- ✅ **自己文書化**: `MirValueKind::Parameter(0)` で意味が明確
- ✅ **保守性向上**: 新メンバーが理解しやすい

---

## 🚀 実装スケジュール

### Day 1: 基礎実装
- **午前**: `MirValueKind` + `TypedValueId` 実装
- **午後**: ユニットテスト作成

### Day 2: 統合
- **午前**: `MirBuilder` 統合
- **午後**: パラメータ登録修正

### Day 3: 修正＆テスト
- **午前**: `loop_builder.rs` 修正
- **午後**: 統合テスト + 回帰テスト

### Day 4: レビュー＆ドキュメント
- **午前**: コードレビュー
- **午後**: ドキュメント更新

---

## ⚠️ リスクと対策

### リスク1: 既存コードとの互換性

**対策**:
- `From<TypedValueId> for ValueId` 実装で自動変換
- `next_value_id()` の既存挙動維持（デフォルトTemporary）
- 段階的移行（全置換不要）

### リスク2: パフォーマンス

**対策**:
- `HashMap<ValueId, MirValueKind>` のオーバーヘッド評価
- 必要に応じて `Vec<MirValueKind>` に最適化

### リスク3: テストカバレッジ

**対策**:
- 既存テスト全実行
- GUARD checkバグ再現テスト追加
- Smoke tests全実行

---

## 📋 チェックリスト

- [ ] `src/mir/value_kind.rs` 作成
- [ ] `MirValueKind` 実装
- [ ] `TypedValueId` 実装
- [ ] ユニットテスト作成
- [ ] `MirBuilder` 統合
- [ ] パラメータ登録修正
- [ ] `loop_builder.rs` 修正
- [ ] 統合テスト作成
- [ ] 既存テスト全PASS確認
- [ ] ドキュメント更新
- [ ] コミット＆プッシュ

---

**Phase 26-A完了後の次ステップ**: Phase 26-B（LoopVariableSnapshot統一）
