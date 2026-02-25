# BoxFactory Priority 問題と解決策

## 現状の問題

### FactoryPolicy の種類

| ポリシー | 優先順位 | 状態 |
|---------|---------|------|
| `StrictPluginFirst` | Plugin > User > Builtin | 理想（実装済・未使用） |
| `CompatPluginFirst` | Plugin > Builtin > User | 互換（実装済・未使用） |
| `BuiltinFirst` | Builtin > User > Plugin | **現デフォルト（問題）** |

### 問題の詳細

**場所**: `src/box_factory/mod.rs::UnifiedBoxRegistry::new()`

```rust
// 現在の実装（問題）- Line 120-122
impl UnifiedBoxRegistry {
    pub fn new() -> Self {
        Self::with_policy(FactoryPolicy::BuiltinFirst)  // ← ここが問題
    }
}
```

**影響**:
- プラグイン版 StringBox が無視される
- プラグイン版 ArrayBox が無視される
- プラグイン版 MapBox が無視される
- Phase 15.5 で特定された優先度問題の根本原因

**環境変数による回避**:
```bash
NYASH_BOX_FACTORY_POLICY=strict_plugin_first ./target/release/nyash program.hako
```

しかし、これは一時的な回避策であり、デフォルト動作が間違っている。

### 既存の実装状況

**良いニュース**: `with_env_policy()` は既に実装済み（Line 134-146）

```rust
pub fn with_env_policy() -> Self {
    let policy = match std::env::var("NYASH_BOX_FACTORY_POLICY").ok().as_deref() {
        Some("compat_plugin_first") => FactoryPolicy::CompatPluginFirst,
        Some("builtin_first") => FactoryPolicy::BuiltinFirst,
        Some("strict_plugin_first") | _ => FactoryPolicy::StrictPluginFirst, // Plugin First DEFAULT!
    };

    eprintln!(
        "[UnifiedBoxRegistry] 🎯 Factory Policy: {:?} (Phase 15.5: Everything is Plugin!)",
        policy
    );
    Self::with_policy(policy)
}
```

**問題**: `new()` が `with_env_policy()` を呼ばず、ハードコードされた `BuiltinFirst` を使用。

### is_reserved_type() の実装状況

**場所**: `src/box_factory/mod.rs` Line 176-194（rebuild_cache内のローカル関数）

**現在の core_required リスト**:
- StringBox
- IntegerBox
- BoolBox
- FloatBox
- NullBox
- ArrayBox
- MapBox
- ResultBox
- MethodBox

**環境変数サポート**:
- `NYASH_USE_PLUGIN_BUILTINS=1`: 予約型保護を解除（既に実装済み）
- `NYASH_PLUGIN_OVERRIDE_TYPES=Type1,Type2`: 個別指定（既に実装済み）

---

## Phase 86 の目標ポリシー

### 基本方針

| Box 種別 | ポリシー | 環境変数 override |
|---------|---------|------------------|
| **core_required** | Builtin 固定（予約名） | `NYASH_USE_PLUGIN_BUILTINS=1` で可 |
| **それ以外** | Plugin が Builtin override 可 | デフォルト動作 |
| **BuiltinFirst** | Legacy 専用（非推奨） | CI/古いプロファイルのみ |

### core_required Box リスト

Phase 85 調査結果および既存実装より：

**Core value types**:
- StringBox
- IntegerBox
- BoolBox
- FloatBox
- NullBox

**Core containers and result**:
- ArrayBox
- MapBox
- ResultBox

**Core method indirection**:
- MethodBox

**ConsoleBox の扱い**:
- Phase 85 調査では core_required とされたが、現在の `is_reserved_type()` には含まれていない
- Phase 86 では既存実装を尊重し、ConsoleBox を予約型に追加しない
- 将来的に必要であれば Phase 85 で再検討

これらは `is_reserved_type()` で保護される（既に実装済み）。

### デフォルトポリシー

**新デフォルト**: `StrictPluginFirst`

**理由**:
1. ✅ プラグインによる拡張を優先（Nyash の設計思想）
2. ✅ core_required は予約名保護で安全性を確保
3. ✅ 開発者の期待に合致（プラグインが優先されるべき）
4. ✅ "Everything is Plugin" 哲学の体現

### 環境変数

| 環境変数 | 用途 | デフォルト値 |
|---------|------|------------|
| `NYASH_BOX_FACTORY_POLICY` | ポリシー選択 | `strict_plugin_first` |
| `NYASH_USE_PLUGIN_BUILTINS` | core_required override 許可 | 未設定（無効） |
| `NYASH_PLUGIN_OVERRIDE_TYPES` | 個別 Box override 許可 | 未設定（空） |

**使用例**:
```bash
# core_required もプラグイン版を使用（開発用）
NYASH_USE_PLUGIN_BUILTINS=1 ./target/release/nyash program.hako

# 特定 Box のみプラグイン版を使用
NYASH_PLUGIN_OVERRIDE_TYPES=StringBox,ArrayBox ./target/release/nyash program.hako

# Legacy モード（古いテスト用）
NYASH_BOX_FACTORY_POLICY=builtin_first ./target/release/nyash program.hako
```

---

## Phase 86 実装内容

### 修正箇所

**1つの修正のみ**: `UnifiedBoxRegistry::new()` を変更

```rust
// Before (Line 120-122)
impl UnifiedBoxRegistry {
    pub fn new() -> Self {
        Self::with_policy(FactoryPolicy::BuiltinFirst)
    }
}

// After
impl UnifiedBoxRegistry {
    pub fn new() -> Self {
        Self::with_env_policy()  // with_env_policy() を使用
    }
}
```

**変更理由**:
- `with_env_policy()` は既に完全実装済み（Line 134-146）
- デフォルトで `StrictPluginFirst` を返す
- 環境変数 `NYASH_BOX_FACTORY_POLICY` による制御も完全対応
- `is_reserved_type()` による保護も実装済み
- 追加実装は不要！

### 追加実装は不要

以下の機能は既に実装済み：
- ✅ `with_env_policy()` の実装（Line 134-146）
- ✅ `is_reserved_type()` の実装（Line 176-194）
- ✅ 環境変数 `NYASH_USE_PLUGIN_BUILTINS` のサポート（Line 178-184）
- ✅ 環境変数 `NYASH_PLUGIN_OVERRIDE_TYPES` のサポート（Line 179-183）
- ✅ Policy-based priority ordering（Line 224-256）

**Phase 86 の本質**:
- デフォルト動作を正常化する（1行の変更）
- 既存の完全実装を活用する
- テストで検証する

---

## テスト戦略

### 追加するテスト（5件）

**ファイル**: `src/box_factory/mod.rs` の `#[cfg(test)] mod tests` セクション

1. **test_default_policy_is_strict_plugin_first**
   - `new()` のデフォルトポリシーを確認

2. **test_env_policy_override**
   - 環境変数 `NYASH_BOX_FACTORY_POLICY` の動作確認

3. **test_reserved_type_protection**
   - 予約型が非 builtin factory で登録されないことを確認

4. **test_plugin_override_with_env**
   - `NYASH_USE_PLUGIN_BUILTINS=1` での予約型 override を確認

5. **test_non_reserved_plugin_priority**
   - 非予約型（FileBox等）が plugin で override できることを確認

---

## 完了条件

- ✅ `docs/development/current/main/factory_priority.md` 作成完了
- ✅ `UnifiedBoxRegistry::new()` が `with_env_policy()` を使用
- ✅ デフォルトポリシーが `StrictPluginFirst`
- ✅ `is_reserved_type()` が Phase 85 の core_required リストと一致（既存実装確認）
- ✅ テスト 5件追加・全パス
- ✅ `CURRENT_TASK.md` 更新完了
- ✅ Phase 85 README 準備完了

---

## 次のステップ（Phase 85）

Phase 86 完了後、Phase 85 で以下を実施：
- Ring0/Ring1-Core 境界の文書化
- core_required Box の最終確定
- ConsoleBox の扱いの再検討

Phase 86 は Phase 85 の土台を安定させるための準備フェーズ。
