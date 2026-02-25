# Phase 123 proper: hako_check JoinIR 実装（環境変数選択可能化）

## 🎯 ゴール

hako_check を **JoinIR 経路でも動かせる** ように拡張し、環境変数で以下の2つのルートを切り替え可能にする：

```
NYASH_HAKO_CHECK_JOINIR=0 (or unset)  →  既存レガシー経路
NYASH_HAKO_CHECK_JOINIR=1             →  JoinIR 経路（If/Loop を JoinIR lowering 経由に）
```

これにより、Phase 121 で設計した JoinIR 統合を hako_check でも実現し、Phase 124 での JoinIR デフォルト化への準備を整える。

## 📋 スコープ（やること・やらないこと）

### ✅ やること
1. **環境変数フラグ導入**: `NYASH_HAKO_CHECK_JOINIR` のヘルパー関数作成
2. **hako_check ランナー修正**: JoinIR 経路スイッチの実装
3. **代表ケース検証**: Phase 121 で洗ったケースで両経路を確認
4. **ドキュメント更新**: 設計・実装結果・Known Limitations を記録

### ❌ やらないこと
- JoinIR をいきなりデフォルト化（Phase 124 の役目）
- JoinIR Lowerer 自体のロジック変更
- hako_check .hako スクリプトの API 変更（呼び出し仕様は現状維持）

## 🏗️ 実装の 4 つのタスク

### Task 1: 環境変数フラグの追加（Config 層）

**ファイル**:
- `src/config/env/hako_check.rs`（新規 or 既存拡張）
- `docs/reference/environment-variables.md`（更新）

**実装内容**:

```rust
// src/config/env/hako_check.rs

/// hako_check で JoinIR 経路を使用するかどうか判定
///
/// 環境変数 NYASH_HAKO_CHECK_JOINIR で制御：
/// - "1" or "true": JoinIR 経路を使用
/// - その他: レガシー経路を使用（デフォルト）
pub fn hako_check_joinir_enabled() -> bool {
    matches!(
        std::env::var("NYASH_HAKO_CHECK_JOINIR").as_deref(),
        Ok("1") | Ok("true")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hako_check_joinir_flag_parsing() {
        // 環境変数パースのテスト
    }
}
```

**ドキュメント更新** (`docs/reference/environment-variables.md`):

```markdown
### NYASH_HAKO_CHECK_JOINIR

**用途**: hako_check で JoinIR lowering 経路を使用

**値**:
- `1` or `true`: JoinIR 経路を使用（Phase 123+）
- その他（未設定含む）: レガシー経路を使用

**デフォルト**: `0`（レガシー経路）

**例**:
```bash
# JoinIR 経路で hako_check 実行
NYASH_HAKO_CHECK_JOINIR=1 ./target/release/nyash tools/hako_check/cli.hako
```

**参照**: [Phase 123 hako_check JoinIR 実装](../main/phase123_hako_check_joinir_implementation.md)
```

### Task 2: hako_check 実装に JoinIR スイッチを差し込む

**ファイル候補**:
- `src/runner/hako_check_runner.rs`（or 類似のメインランナー）
- `src/mir/builder.rs`（MIR 構築時の分岐）
- Phase 121 docs の設計図を参照して確認

**実装方針**:

#### Step 2A: ランナー側でフラグを読み込む

```rust
// src/runner/hako_check_runner.rs（例）

use crate::config::env;

pub fn run_hako_check(
    input_path: &str,
    // ... その他の引数
) -> Result<HakoCheckResult, Error> {
    // フラグを読み込む
    let use_joinir = env::hako_check::hako_check_joinir_enabled();

    eprintln!("[hako_check] Using JoinIR path: {}", use_joinir);

    // フラグに応じて経路を分ける
    if use_joinir {
        run_hako_check_with_joinir(input_path, /* ... */)
    } else {
        run_hako_check_legacy(input_path, /* ... */)
    }
}
```

#### Step 2B: If/Loop lowering を JoinIR に乗せる

**現在の処理フロー（レガシー）**:
```
.hako → Tokenize → Parse → MIR Builder (legacy if/loop) → VM Interpret
```

**目標フロー（JoinIR）**:
```
.hako → Tokenize → Parse → MIR Builder → JoinIR Lowerer (if/loop) → MIR → VM Interpret
```

**実装例**:

```rust
// src/mir/builder.rs 内（MIR 構築後）

pub fn lower_if_and_loop_statements(
    mir: &mut MirFunction,
    use_joinir: bool,
) -> Result<(), Error> {
    if use_joinir {
        // JoinIR Lowerer を使用
        let mut lowerer = JoinIRIfLoopLowerer::new();
        lowerer.lower(mir)?;
    } else {
        // レガシー lowering（既存ロジック）
        legacy_lower_if_and_loop(mir)?;
    }
    Ok(())
}
```

#### Step 2C: 既存の JoinIR Lowerer を再利用

⚠️ **重要**: 新しいロジックは書かず、既存の以下を再利用：
- `src/mir/join_ir/lowering/if_select.rs`（If lowering）
- `src/mir/join_ir/lowering/loop_form.rs`（Loop lowering）
- これらを hako_check パイプラインに「差し込む」だけ

### Task 3: 代表ケースで JoinIR 実行を確認

**対象ケース** (Phase 121 docs から):

```
1. simple_if.hako        - 単純な if 文
2. nested_if.hako        - ネストされた if
3. while_loop.hako       - while ループ
4. nested_while.hako     - ネストされたループ
5. if_in_loop.hako       - if in loop
6. loop_in_if.hako       - loop in if
```

**テストスクリプト作成** (`tools/smokes/v2/profiles/integration/hako_check_joinir.sh`):

```bash
#!/bin/bash

# Phase 123: hako_check JoinIR 実装テスト

set -e

PROFILE_NAME="hako_check_joinir"

# test cases
test_cases=(
    "simple_if.hako"
    "nested_if.hako"
    "while_loop.hako"
    "nested_while.hako"
    "if_in_loop.hako"
    "loop_in_if.hako"
)

echo "=== Testing hako_check with JoinIR OFF (Legacy) ==="
for case in "${test_cases[@]}"; do
    NYASH_HAKO_CHECK_JOINIR=0 ./target/release/nyash "test_cases/$case"
    echo "✓ $case (legacy)"
done

echo ""
echo "=== Testing hako_check with JoinIR ON ==="
for case in "${test_cases[@]}"; do
    NYASH_HAKO_CHECK_JOINIR=1 ./target/release/nyash "test_cases/$case"
    echo "✓ $case (joinir)"
done
```

**確認内容**:

1. ✅ **JoinIR OFF**: すべてが現在どおり PASS
2. ✅ **JoinIR ON**: Phase 121 で想定した「JoinIR で食えるパターン」は green に
3. 📝 差分あれば Phase 123 docs の "Known Limitations" に記録

### Task 4: ドキュメント・CURRENT_TASK 更新

**ファイル**:
- `docs/development/current/main/phase121_hako_check_joinir_design.md`
- `docs/development/current/main/hako_check_design.md`
- `CURRENT_TASK.md`

#### 更新内容

**phase121_hako_check_joinir_design.md に追加**:

```markdown
## Phase 123実装完了セクション

### 環境変数フラグ導入
- NYASH_HAKO_CHECK_JOINIR で 2 つのルートを切り替え可能

### JoinIR スイッチ実装
- hako_check ランナーが use_joinir フラグを受け取り、処理を分岐
- 既存の JoinIR Lowerer を再利用

### 代表ケース検証結果
- legacy: N/N cases PASS
- joinir: N/N cases PASS
- Known Limitations: [あれば記載]
```

**hako_check_design.md に追加**:

```markdown
## 実行フロー図（2 パス）

### Legacy パス（NYASH_HAKO_CHECK_JOINIR=0 or unset）
```
.hako file
    ↓
Tokenize / Parse
    ↓
MIR Builder (legacy if/loop lowering)
    ↓
VM Interpreter
    ↓
Check Result
```

### JoinIR パス（NYASH_HAKO_CHECK_JOINIR=1）
```
.hako file
    ↓
Tokenize / Parse
    ↓
MIR Builder
    ↓
JoinIR Lowerer (if/loop → PHI)
    ↓
VM Interpreter
    ↓
Check Result
```
```

**CURRENT_TASK.md に追加**:

```markdown
### Phase 123 proper: hako_check JoinIR 実装 ✅

**完了内容**:
- NYASH_HAKO_CHECK_JOINIR 環境変数フラグ導入
- hako_check ランナーに JoinIR スイッチ実装
- 代表 6 ケースで両経路テスト実施
- ドキュメント更新完了

**テスト結果**:
- Legacy: 6/6 PASS
- JoinIR: 6/6 PASS

**次フェーズ**: Phase 124 - JoinIR デフォルト化＆レガシー経路削除
```

## 🔍 詳細実装ガイド

### Phase 121 との連携ポイント

Phase 121 で以下が既に設計・分析済み：

```
hako_check_design.md（セクション 10-12）
  ├─ If/Loop の処理フロー
  ├─ JoinIR Lowerer の差し込み位置
  └─ テスト ケース定義

phase121_integration_roadmap.md（セクション 3-5）
  ├─ Phase 123: 環境変数スイッチ + 代表テスト
  ├─ Phase 124: JoinIR デフォルト化
  └─ Phase 125: レガシー削除
```

**参照**：これらの設計を実装に落とすだけ

### ビルド・実行確認

```bash
# ビルド確認
cargo build --release 2>&1 | grep -E "error"

# レガシー経路確認
NYASH_HAKO_CHECK_JOINIR=0 ./target/release/nyash tools/hako_check/cli.hako

# JoinIR 経路確認
NYASH_HAKO_CHECK_JOINIR=1 ./target/release/nyash tools/hako_check/cli.hako
```

## ✅ 完成チェックリスト

- [ ] `src/config/env/hako_check.rs` で `hako_check_joinir_enabled()` 実装
- [ ] `docs/reference/environment-variables.md` に NYASH_HAKO_CHECK_JOINIR 記載
- [ ] hako_check ランナー修正で use_joinir フラグを受け取る
- [ ] MIR builder で legacy vs joinir lowering を切り替え
- [ ] 代表 6 ケースで JoinIR ON/OFF 両方テスト実施
- [ ] テスト結果（PASS/FAIL）を docs に記録
- [ ] phase121_hako_check_joinir_design.md に Phase 123 実装セクション追加
- [ ] hako_check_design.md にフロー図（2 パス）追加
- [ ] CURRENT_TASK.md に Phase 123 完了を反映
- [ ] ビルドエラーなし（Zero errors）

## 所要時間

**3時間程度**

- Task 1 (環境変数フラグ): 30分
- Task 2 (JoinIR スイッチ): 1.5時間
- Task 3 (テスト確認): 45分
- Task 4 (ドキュメント): 15分

## 次のステップ

**Phase 124**: JoinIR デフォルト化＆レガシー経路削除

```
現状: NYASH_HAKO_CHECK_JOINIR で選択可能
目標: JoinIR をデフォルト化＆レガシー削除
```

---

**進捗**:
- ✅ Phase 122-126: ConsoleBox 改善・統合完了
- 🎯 Phase 123 proper: hako_check JoinIR 実装 ← **現在のフェーズ**
- 📋 Phase 124: JoinIR デフォルト化
- 📋 Phase 125: レガシー経路削除
Status: Historical
