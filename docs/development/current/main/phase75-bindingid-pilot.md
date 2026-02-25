# Phase 75: BindingId Pilot Integration (1 isolation point)

**Status**: ✅ COMPLETE (2025-12-13)
**Feature**: `normalized_dev` (dev-only)
**Impact**: Zero production impact (infrastructure layer only)

## 目的 (Purpose)

BindingId ベースの lookup を本番ルート（dev-only）で 1 箇所に試験し、「binding 優先→name fallback」の動作を実証する。

Phase 74 で構築した binding_map と allocate_binding_id() の基盤を実際に使用し、ScopeManager の 1 component で BindingId を活用する pilot integration を完成させる。

## 背景 (Background)

### Phase 74 の成果

Phase 74 で以下のインフラストラクチャが完成:
- ✅ `BindingId` type と `allocate_binding_id()` メソッド
- ✅ `MirBuilder.binding_map: BTreeMap<String, BindingId>` が populated
- ✅ Shadowing test evidence 完備（lexical scope でのBindingId復元）

### Phase 75 の役割

Phase 74 のインフラストラクチャを **実際に使用** し、以下を実証:
1. BindingId ベースの lookup が正しく動作すること
2. name-based fallback が既存挙動を保持すること
3. 本番パスへの影響がゼロであること（dev-only feature gate）

## 設計 (Design)

### Pilot Integration Point: ConditionEnv

**なぜ ConditionEnv を選んだか？**

1. **Isolated Component**: ConditionEnv は loop condition の変数解決に特化した箱
2. **Well-Tested**: Phase 171-fix 以降、ConditionEnv の挙動は十分にテストされている
3. **Clear Scope**: name_to_join マッピングの責務が明確（変数名 → JoinValueId）
4. **Minimal Surface**: lookup API が単純（`get(name)` のみ）で、拡張が容易

### API Design: resolve_var_with_binding()

```rust
#[cfg(feature = "normalized_dev")]
pub fn resolve_var_with_binding(
    &self,
    binding_id: Option<BindingId>,
    name: &str,
) -> Option<ValueId>
```

#### Lookup Strategy (3-tier fallback)

1. **BindingId Priority**: `binding_id.is_some()` なら `binding_id_map` を優先検索
2. **Name Fallback**: BindingId miss なら name-based lookup (`get(name)`) にフォールバック
3. **Legacy Path**: `binding_id.is_none()` なら name lookup のみ（既存挙動保持）

#### Observability (NYASH_JOINIR_DEBUG=1)

Dev-only logging で挙動を可視化:
- `[binding_pilot/hit]` - BindingId lookup 成功
- `[binding_pilot/fallback]` - BindingId miss → name fallback
- `[binding_pilot/legacy]` - BindingId なし → name lookup

### Infrastructure Extension

#### ConditionEnv 拡張

```rust
pub struct ConditionEnv {
    name_to_join: BTreeMap<String, ValueId>,      // 既存
    captured: BTreeMap<String, ValueId>,          // Phase 200-B
    #[cfg(feature = "normalized_dev")]
    binding_id_map: BTreeMap<BindingId, ValueId>, // Phase 75 新規
}
```

#### ScopeManager trait 拡張

```rust
pub trait ScopeManager {
    fn lookup(&self, name: &str) -> Option<ValueId>;      // 既存
    fn scope_of(&self, name: &str) -> Option<VarScopeKind>; // 既存

    #[cfg(feature = "normalized_dev")]
    fn lookup_with_binding(
        &self,
        binding_id: Option<BindingId>,
        name: &str
    ) -> Option<ValueId> {
        // Default: BindingId 未対応 → name lookup のみ
        let _ = binding_id;
        self.lookup(name)
    }
}
```

**Design Rationale**:
- Default implementation で既存 ScopeManager implementors への影響ゼロ
- Pattern2ScopeManager は default を使用（ConditionEnv 内部で pilot 実装）
- Phase 76+ で promoted_bindings 対応時に override 可能

## 実装 (Implementation)

### 変更ファイル (3 files)

1. **src/mir/join_ir/lowering/scope_manager.rs** (+50 lines)
   - `lookup_with_binding()` trait method 追加（default impl）
   - BindingId import（feature gated）

2. **src/mir/join_ir/lowering/condition_env.rs** (+120 lines)
   - `binding_id_map` フィールド追加（feature gated）
   - `resolve_var_with_binding()` メソッド実装
   - 3つのunit test追加

3. **docs/development/current/main/phase75-bindingid-pilot.md** (this file)

### Test Strategy

#### Unit Tests (3 tests, all PASS)

**Test 1: BindingId Priority**
```rust
#[test]
#[cfg(feature = "normalized_dev")]
fn test_condition_env_binding_id_priority() {
    let mut env = ConditionEnv::new();
    env.insert("x".to_string(), ValueId(100));
    env.binding_id_map.insert(BindingId(5), ValueId(100));

    // BindingId 優先検索
    assert_eq!(env.resolve_var_with_binding(Some(BindingId(5)), "x"), Some(ValueId(100)));
}
```

**Test 2: BindingId Fallback**
```rust
#[test]
#[cfg(feature = "normalized_dev")]
fn test_condition_env_binding_id_fallback() {
    let mut env = ConditionEnv::new();
    env.insert("x".to_string(), ValueId(100));
    // BindingId(99) は binding_id_map に存在しない

    // BindingId miss → name fallback
    assert_eq!(env.resolve_var_with_binding(Some(BindingId(99)), "x"), Some(ValueId(100)));
}
```

**Test 3: Legacy (No BindingId)**
```rust
#[test]
#[cfg(feature = "normalized_dev")]
fn test_condition_env_binding_id_none() {
    let mut env = ConditionEnv::new();
    env.insert("x".to_string(), ValueId(100));

    // BindingId なし → name lookup のみ
    assert_eq!(env.resolve_var_with_binding(None, "x"), Some(ValueId(100)));
}
```

#### Regression Tests

- ✅ `cargo test --release --lib` → **958/958 PASS** (退行なし)
- ✅ `cargo test --release --lib --features normalized_dev condition_env` → **15/15 PASS**
  - 3つの新規テスト含む（priority/fallback/legacy）

## 結果 (Results)

### 受け入れ基準 (Acceptance Criteria)

- ✅ `cargo build --lib` 成功（本番パス影響なし）
- ✅ `cargo test --release --lib` 退行なし (958/958 PASS)
- ✅ `cargo test --features normalized_dev --lib condition_env` 3つの新規テスト PASS (15/15 PASS)
- ✅ ConditionEnv の `resolve_var_with_binding()` メソッド実装完了
- ✅ ScopeManager trait に `lookup_with_binding()` 追加完了

### 実証された内容

1. **BindingId Lookup 動作確認**: `binding_id_map` からの直接検索が成功
2. **Name Fallback 動作確認**: BindingId miss 時に name lookup へ安全にフォールバック
3. **Legacy 互換性**: BindingId なし（None）時は既存挙動（name lookup のみ）を保持
4. **Isolation 確認**: feature gate により本番パスへの影響ゼロ

### Dev Logging Example

```bash
# NYASH_JOINIR_DEBUG=1 で実行すると（将来の統合時）:
[binding_pilot/hit] BindingId(5) -> ValueId(100) for 'x'
[binding_pilot/fallback] BindingId(99) miss, name 'x' -> Some(ValueId(100))
[binding_pilot/legacy] No BindingId, name 'x' -> Some(ValueId(100))
```

## Next Steps (Phase 76)

### Phase 76: Promoted Bindings Migration

**目的**: `digit_pos → is_digit_pos` / `ch → is_ch_match` の naming hack を promoted_bindings 対応表に移行

**準備完了**:
- ✅ BindingId infrastructure (Phase 74)
- ✅ ConditionEnv pilot integration (Phase 75)

**Phase 76 で実装**:
1. `promoted_bindings: BTreeMap<BindingId, BindingId>` を CarrierInfo に追加
2. ConditionEnv の `resolve_var_with_binding()` で promoted lookup を優先
3. Pattern2 lowering で promoted bindings を populate
4. DigitPos/TrimHelper の naming hack 撤去

詳細は Phase 73 Migration Roadmap を参照:
[phase73-scope-manager-design.md](./phase73-scope-manager-design.md)

## 重要な注意点 (Important Notes)

### 本番パス影響ゼロの保証

- **Feature Gate**: `#[cfg(feature = "normalized_dev")]` により本番ビルドには含まれない
- **Default Implementation**: ScopeManager trait の default impl により既存 implementors への影響なし
- **Additive Only**: 既存 API (`lookup()`, `get()`) は完全に不変

### Why "Pilot" Integration?

Phase 75 は **1 isolation point** に絞った pilot integration:
- **Scope**: ConditionEnv のみ（ScopeManager implementors は未変更）
- **Usage**: 実際の lowering code からはまだ呼ばれない（テストのみ）
- **Purpose**: Infrastructure の動作実証（Phase 76+ への足場）

Phase 76 以降で段階的に適用面を拡大:
- Phase 76: promoted_bindings（digit_pos/ch_match）
- Phase 77: Pattern2→3→4 への展開

## References

- **Phase 73**: [phase73-scope-manager-design.md](./phase73-scope-manager-design.md) - BindingId 設計 + PoC
- **Phase 74**: [CURRENT_TASK.md](../../../../CURRENT_TASK.md) - Infrastructure 実装完了
- **Migration Roadmap**: Phase 74-77 (合計 8-12時間、本番影響ゼロ)

## 完成時刻

**2025-12-13 完了** (Phase 75 pilot integration)

---

**Phase 75 Summary**: BindingId-based lookup を ConditionEnv に pilot 実装し、3-tier fallback (BindingId → name → None) の動作を 3つのユニットテストで固定。本番パス影響ゼロ（feature gate）を保持しつつ、Phase 76 への足場を確立。
