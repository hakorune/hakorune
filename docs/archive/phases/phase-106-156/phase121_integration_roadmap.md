# Phase 121: hako_check JoinIR 統合ロードマップ

## Phase 122: 環境変数で JoinIR 選択可能に

### 目標

hako_check で `NYASH_HAKO_CHECK_JOINIR=1` を指定すると、JoinIR 経路を使用するようにする。
デフォルトは旧経路を維持（互換性重視）。

### 実装ステップ

#### Step 1: 環境変数読み込み機能追加

**ファイル**: `src/config/env.rs`

**実装内容**:
```rust
/// hako_check で JoinIR 経路を有効化するフラグ
pub fn hako_check_joinir_enabled() -> bool {
    env_flag("NYASH_HAKO_CHECK_JOINIR").unwrap_or(false)
}
```

**チェックリスト**:
- [ ] `src/config/env.rs` に `hako_check_joinir_enabled()` 関数追加
- [ ] 既存の `joinir_core_enabled()` との関係を整理
- [ ] ドキュメント更新（環境変数リスト）

#### Step 2: If 文の JoinIR 統合

**ファイル**: `src/mir/builder/if_form.rs`

**実装内容**:
```rust
// Phase 122: 環境変数で JoinIR 経路を選択可能に
pub fn cf_if(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // Phase 122: 環境変数チェック
    if crate::config::env::hako_check_joinir_enabled() {
        // JoinIR If Lowering を使用
        return self.cf_if_joinir(condition, then_block, else_block);
    }

    // 旧 PHI 生成器（互換性維持）
    self.cf_if_legacy(condition, then_block, else_block)
}

/// Phase 122: JoinIR If Lowering 経由
fn cf_if_joinir(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    use crate::mir::join_ir::lowering::if_select::lower_if_to_mir;
    lower_if_to_mir(self, condition, then_block, else_block)
}

/// 既存ロジックを cf_if_legacy に移動
fn cf_if_legacy(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // 既存の cf_if() ロジックをここに移動
    // ...
}
```

**チェックリスト**:
- [ ] `cf_if_joinir()` 関数を追加（JoinIR Lowering 呼び出し）
- [ ] `cf_if()` に環境変数チェックを追加
- [ ] 既存ロジックを `cf_if_legacy()` に移動
- [ ] JoinIR If Lowering（`if_select.rs`）との連携確認

#### Step 3: Loop の JoinIR 統合拡張

**ファイル**: `src/mir/builder/control_flow.rs`

**実装内容**:
```rust
// Phase 122: 環境変数で全 Loop を JoinIR 経由に
pub fn cf_loop(
    &mut self,
    condition: &ASTNode,
    body: &ASTNode,
) -> Result<ValueId, String> {
    // Phase 122: hako_check 環境変数チェック
    if crate::config::env::hako_check_joinir_enabled() {
        // すべての Loop を JoinIR 経由に
        if let Some(result) = self.try_cf_loop_joinir_all(&condition, &body)? {
            return Ok(result);
        }
        // JoinIR 失敗時はエラー（フォールバック禁止）
        return Err("JoinIR Loop Lowering failed (NYASH_HAKO_CHECK_JOINIR=1)".to_string());
    }

    // Phase 49: Mainline Targets のみ JoinIR 経由（既存ロジック）
    if let Some(result) = self.try_cf_loop_joinir(&condition, &body)? {
        return Ok(result);
    }

    // Fallback: 旧 LoopBuilder
    self.cf_loop_legacy(condition, body)
}

/// Phase 122: すべての Loop を JoinIR 経由に
fn try_cf_loop_joinir_all(
    &mut self,
    condition: &ASTNode,
    body: &ASTNode,
) -> Result<Option<ValueId>, String> {
    // Mainline Targets 制限を削除
    let debug = std::env::var("NYASH_JOINIR_MAINLINE_DEBUG").is_ok();
    let func_name = self.current_function_name();

    if debug {
        eprintln!(
            "[cf_loop/joinir/all] Routing {} through JoinIR Frontend (all loops)",
            func_name
        );
    }

    self.cf_loop_joinir_impl(condition, body, &func_name, debug)
}
```

**チェックリスト**:
- [ ] `try_cf_loop_joinir_all()` 関数を追加（Mainline 制限なし）
- [ ] `cf_loop()` に環境変数チェックを追加
- [ ] フォールバック禁止ロジックを実装
- [ ] JoinIR Loop Lowering との連携確認

#### Step 4: テスト追加

**スモークテスト**: `tools/smokes/v2/profiles/quick/analyze/hako_check_joinir_smoke.sh`

**実装内容**:
```bash
#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../.." && pwd)"

# Phase 122: hako_check JoinIR 経路スモークテスト
echo "=== hako_check JoinIR Smoke Test ==="

# Test 1: If 文（JoinIR 経由）
echo "Test 1: If statement with JoinIR"
cat > /tmp/hako_check_if_test.hako <<'EOF'
static box Main {
  method main(args) {
    local x = 1
    if x == 1 {
      print("then")
    } else {
      print("else")
    }
    return 0
  }
}
EOF

NYASH_HAKO_CHECK_JOINIR=1 "$ROOT/tools/hako_check.sh" /tmp/hako_check_if_test.hako
echo "✅ If statement test passed"

# Test 2: Loop（JoinIR 経由）
echo "Test 2: Loop with JoinIR"
cat > /tmp/hako_check_loop_test.hako <<'EOF'
static box Main {
  method main(args) {
    local i = 0
    loop (i < 3) {
      print(i)
      i = i + 1
    }
    return 0
  }
}
EOF

NYASH_HAKO_CHECK_JOINIR=1 "$ROOT/tools/hako_check.sh" /tmp/hako_check_loop_test.hako
echo "✅ Loop test passed"

# Test 3: 互換性（旧経路）
echo "Test 3: Legacy path compatibility"
"$ROOT/tools/hako_check.sh" /tmp/hako_check_if_test.hako
echo "✅ Legacy path test passed"

echo "=== All tests passed ==="
```

**チェックリスト**:
- [ ] スモークテストスクリプト作成
- [ ] If 文テスト実装
- [ ] Loop テスト実装
- [ ] 互換性テスト実装（旧経路）
- [ ] CI に統合

#### Step 5: ドキュメント更新

**更新ファイル**:
- `docs/development/current/main/hako_check_design.md`: Phase 122 実装完了を記録
- `docs/reference/env-vars.md`: `NYASH_HAKO_CHECK_JOINIR=1` を追加
- `CLAUDE.md`: Phase 122 完了を記録

**チェックリスト**:
- [ ] 設計ドキュメント更新
- [ ] 環境変数リファレンス更新
- [ ] CLAUDE.md 更新

### 完了条件

- ✅ 環境変数なし: 旧経路で全テスト PASS
- ✅ `NYASH_HAKO_CHECK_JOINIR=1`: JoinIR 経路で代表テスト PASS
- ✅ ドキュメント更新完了

### タイムライン（目安）

| タスク | 所要時間 | 担当 |
|-------|---------|-----|
| Step 1: 環境変数読み込み | 30分 | 実装者 |
| Step 2: If 文 JoinIR 統合 | 2-3時間 | 実装者 |
| Step 3: Loop JoinIR 統合拡張 | 1-2時間 | 実装者 |
| Step 4: テスト追加 | 1-2時間 | 実装者 |
| Step 5: ドキュメント更新 | 30分 | 実装者 |
| **合計** | **1日** | - |

---

## Phase 123: JoinIR 経路をデフォルトに

### 目標

hako_check のデフォルトを JoinIR 経路に変更。
旧経路は `NYASH_LEGACY_PHI=1` でのみ使用可能に。

### 実装ステップ

#### Step 1: デフォルト値変更

**ファイル**: `src/config/env.rs`

**実装内容**:
```rust
/// hako_check で JoinIR 経路を有効化するフラグ
/// Phase 123: デフォルトを true に変更
pub fn hako_check_joinir_enabled() -> bool {
    // Phase 123: デフォルトを true に変更
    // NYASH_LEGACY_PHI=1 で旧経路に戻せる
    if env_flag("NYASH_LEGACY_PHI").unwrap_or(false) {
        return false;
    }

    // NYASH_HAKO_CHECK_JOINIR=0 で明示的に無効化
    env_flag("NYASH_HAKO_CHECK_JOINIR").unwrap_or(true)
}
```

**チェックリスト**:
- [ ] `hako_check_joinir_enabled()` のデフォルトを `true` に変更
- [ ] `NYASH_LEGACY_PHI=1` サポート追加
- [ ] 環境変数優先順位を整理

#### Step 2: 警告メッセージ追加

**ファイル**: `src/mir/builder/if_form.rs`, `src/mir/builder/control_flow.rs`

**実装内容**:
```rust
// Phase 123: 旧経路使用時に警告
pub fn cf_if(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    if crate::config::env::hako_check_joinir_enabled() {
        return self.cf_if_joinir(condition, then_block, else_block);
    }

    // Phase 123: 旧経路使用時に警告
    eprintln!(
        "[WARN] Using legacy PHI path for If statement. Set NYASH_LEGACY_PHI=0 to use JoinIR path."
    );
    self.cf_if_legacy(condition, then_block, else_block)
}
```

**チェックリスト**:
- [ ] If 文の旧経路使用時に警告を追加
- [ ] Loop の旧経路使用時に警告を追加
- [ ] 警告メッセージに移行ガイドを含める

#### Step 3: 全テスト PASS 確認

**テスト内容**:
```bash
# JoinIR 経路で全テスト PASS（デフォルト）
./tools/hako_check.sh apps/
./tools/smokes/v2/run.sh --profile quick --filter "hc*"

# 旧経路で互換性維持確認
NYASH_LEGACY_PHI=1 ./tools/hako_check.sh apps/
NYASH_LEGACY_PHI=1 ./tools/smokes/v2/run.sh --profile quick --filter "hc*"
```

**チェックリスト**:
- [ ] JoinIR 経路で全テスト PASS
- [ ] `NYASH_LEGACY_PHI=1` で旧経路テスト PASS（互換性維持）
- [ ] 警告メッセージが正しく表示されることを確認

### 完了条件

- ✅ 環境変数なし: JoinIR 経路で全テスト PASS
- ✅ `NYASH_LEGACY_PHI=1`: 旧経路で全テスト PASS（警告あり）
- ✅ ドキュメント更新完了

### タイムライン（目安）

| タスク | 所要時間 | 担当 |
|-------|---------|-----|
| Step 1: デフォルト値変更 | 30分 | 実装者 |
| Step 2: 警告メッセージ追加 | 1時間 | 実装者 |
| Step 3: 全テスト PASS 確認 | 2-3時間 | 実装者 |
| ドキュメント更新 | 30分 | 実装者 |
| **合計** | **1日** | - |

---

## Phase 124: 旧経路完全削除

### 目標

旧 PHI 生成器を完全削除し、JoinIR 経路のみを使用する。

### 実装ステップ

#### Step 1: 旧経路コード削除

**ファイル**: `src/mir/builder/if_form.rs`

**実装内容**:
```rust
// Phase 124: 旧経路削除、JoinIR のみ使用
pub fn cf_if(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // JoinIR If Lowering のみ
    self.cf_if_joinir(condition, then_block, else_block)
}

// cf_if_legacy() 削除
```

**ファイル**: `src/mir/builder/control_flow.rs`

**実装内容**:
```rust
// Phase 124: 旧経路削除、JoinIR のみ使用
pub fn cf_loop(
    &mut self,
    condition: &ASTNode,
    body: &ASTNode,
) -> Result<ValueId, String> {
    // JoinIR Loop Lowering のみ
    if let Some(result) = self.try_cf_loop_joinir_all(&condition, &body)? {
        return Ok(result);
    }

    Err("JoinIR Loop Lowering failed".to_string())
}

// cf_loop_legacy() 削除
// try_cf_loop_joinir() 削除（Mainline Targets 制限版）
```

**チェックリスト**:
- [ ] `cf_if_legacy()` 削除（`if_form.rs`）
- [ ] `cf_loop_legacy()` 削除（`control_flow.rs`）
- [ ] 関連する旧 PHI 生成ヘルパー関数削除
- [ ] フォールバックロジック削除

#### Step 2: 環境変数削除

**ファイル**: `src/config/env.rs`

**実装内容**:
```rust
/// hako_check で JoinIR 経路を有効化するフラグ
/// Phase 124: 常に true（環境変数不要）
pub fn hako_check_joinir_enabled() -> bool {
    true  // 常に JoinIR 経路
}
```

**チェックリスト**:
- [ ] `NYASH_LEGACY_PHI=1` サポート削除
- [ ] `NYASH_HAKO_CHECK_JOINIR=1` サポート削除（常に有効）
- [ ] 関連する環境変数チェックコード削除

#### Step 3: ドキュメント更新

**更新ファイル**:
- `docs/development/current/main/hako_check_design.md`: 旧経路に関する記述削除
- `docs/reference/env-vars.md`: 削除された環境変数を記録
- `CLAUDE.md`: Phase 124 完了を記録
- `CURRENT_TASK.md`: Phase 121-124 完了を記録

**チェックリスト**:
- [ ] 設計ドキュメントから旧経路の記述削除
- [ ] 環境変数リファレンス更新
- [ ] CLAUDE.md 更新
- [ ] CURRENT_TASK.md 更新

### 完了条件

- ✅ JoinIR 経路のみで全テスト PASS
- ✅ 旧経路コード完全削除
- ✅ ドキュメント更新完了

### タイムライン（目安）

| タスク | 所要時間 | 担当 |
|-------|---------|-----|
| Step 1: 旧経路コード削除 | 2-3時間 | 実装者 |
| Step 2: 環境変数削除 | 1時間 | 実装者 |
| Step 3: ドキュメント更新 | 1時間 | 実装者 |
| 全テスト PASS 確認 | 1時間 | 実装者 |
| **合計** | **1日** | - |

---

## タイムライン（目安）

| Phase | 実装期間 | 完了条件 |
|-------|---------|---------|
| Phase 122 | 1日 | 環境変数で切り替え可能 |
| Phase 123 | 1日 | JoinIR デフォルト化 |
| Phase 124 | 1日 | 旧経路完全削除 |

**合計**: 3日で hako_check JoinIR 統合完了見込み

---

## リスク管理

### リスク 1: 互換性問題

**内容**: 旧経路に依存しているテストがある

**対策**:
- Phase 122 で環境変数による切り替えを実装
- Phase 123 で段階的にデフォルト変更
- 各 Phase で全テスト PASS を確認

**検出方法**:
```bash
# 旧経路でのテスト
NYASH_LEGACY_PHI=1 cargo test --release

# JoinIR 経路でのテスト
NYASH_HAKO_CHECK_JOINIR=1 cargo test --release
```

### リスク 2: パフォーマンス劣化

**内容**: JoinIR 経路が旧経路より遅い

**対策**:
- Phase 123 でパフォーマンス計測
- 問題があれば Phase 123.5 で最適化
- ベンチマークスクリプト作成

**計測方法**:
```bash
# 旧経路でのベンチマーク
time NYASH_LEGACY_PHI=1 ./tools/hako_check.sh apps/

# JoinIR 経路でのベンチマーク
time NYASH_HAKO_CHECK_JOINIR=1 ./tools/hako_check.sh apps/
```

### リスク 3: 未発見バグ

**内容**: JoinIR 経路に未発見のバグが残っている

**対策**:
- Phase 122 で代表テストを追加
- Phase 123 で全テスト PASS を確認
- Phase 124 で最終確認テスト実施

**検出方法**:
```bash
# 全スモークテスト実行
./tools/smokes/v2/run.sh --profile quick

# 全 hako_check テスト実行
./tools/hako_check.sh apps/
./tools/hako_check.sh tools/
./tools/hako_check.sh local_tests/
```

### リスク 4: JoinIR If Lowering の未成熟

**内容**: JoinIR If Lowering（`if_select.rs`）が一部のパターンに対応していない

**対策**:
- Phase 122 でエラーハンドリングを強化
- 未対応パターンを明確に検出してエラー報告
- 必要に応じて JoinIR If Lowering を拡張

**検出方法**:
```bash
# JoinIR Strict モードでテスト
NYASH_JOINIR_STRICT=1 NYASH_HAKO_CHECK_JOINIR=1 ./tools/hako_check.sh apps/
```

---

## まとめ

Phase 121 で設計を確定し、Phase 122-124 で段階的に実装する。
各 Phase で互換性を維持しながら、最終的に JoinIR 統合を完了する。

### 実装優先順位

1. **Phase 122**: 環境変数で JoinIR 選択可能に（最優先）
2. **Phase 123**: JoinIR 経路をデフォルトに（優先度高）
3. **Phase 124**: 旧経路完全削除（クリーンアップ）

### 期待される効果

- **コード削減**: 約 500-600 行削減（MirBuilder の約 5-10%）
- **保守性向上**: PHI 生成ロジックが JoinIR に統一
- **安定性向上**: 旧 PHI 生成器のバグが根絶

### 次のステップ

Phase 122 の実装を開始する。特に **If 文の JoinIR 統合**が最優先課題。
Status: Historical
