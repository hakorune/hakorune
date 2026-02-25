# Phase 120: selfhost Stage-3 代表パスの安定化

## 0. ゴール

- **JoinIR Strict ON 環境**で selfhost 経路（Stage-3 .hako コンパイラ）が安定動作することを確認
- 代表的な .hako プログラム（2-3本）を選定し、**NYASH_JOINIR_STRICT=1** での実行を記録
- フォールバック・警告・エラーを洗い出し、**Phase 106-115 完了時点のベースライン**を確立

---

## 1. スコープと非スコープ

### スコープ（今回やること）

1. **代表パス選定**: selfhost で使う .hako ファイルから代表的なものを2-3本選定
2. **docs 整理**: 代表パスの期待フロー・JoinIR Strict モードの意味を1ドキュメントにまとめる
3. **実行調査**: `NYASH_JOINIR_STRICT=1` で各代表パスを実行し、フォールバック・警告・エラーを記録
4. **スモークスクリプト作成**: 代表パスの実行を再現できる smoke スクリプト化（`tools/smokes/v2/` 形式）
5. **ベースライン確立**: Phase 106-115 完了時点での動作状況を記録（Phase 120 実装前の基準点）

### 非スコープ（今回はやらない）

- **実装修正**: JoinIR 経路の実装バグ修正（Phase 122+ に回す）
- **hako_check 統合**: Phase 121 で設計を行う（今回は selfhost パスのみ）
- **全プログラム網羅**: 代表的なもの2-3本のみ（全 .hako の検証は別 Phase）

---

## 2. Task 1: 代表パス選定と期待フロー整理

### 2.1 代表パスの選定基準

**Phase 120 で扱う「代表パス」の条件**:

| 基準 | 説明 |
|------|------|
| **selfhost 経路で実行** | `NYASH_USE_NY_COMPILER=1` 等の selfhost 環境変数で動作するもの |
| **Stage-3 対象** | .hako コンパイラ自体、または selfhost で動作する実用プログラム |
| **複雑さのバランス** | 単純すぎず（hello world）、複雑すぎず（全 selfhost コンパイラ）のもの |
| **既存テスト可能** | apps/ または local_tests/ に既に存在し、動作確認済みのもの |

**推奨候補**（2-3本選定）:

1. **簡易パーサーテスト**: `apps/tests/peek_expr_block.hako` 等（簡単な制御構造）
2. **ループ・PHI 含む**: `apps/tests/loop_min_while.hako` 等（JoinIR If/Loop Lowering 対象）
3. **実用スクリプト**: `apps/examples/` から1本（FileBox/StringBox 使用等）

### 2.2 期待フローのドキュメント化

**ファイル**: `docs/development/current/main/selfhost_stage3_expected_flow.md`（新規）

**記載内容**:

```markdown
# selfhost Stage-3 期待フロー（Phase 120 時点）

## 概要

Phase 106-115 完了時点での selfhost 経路（Stage-3 .hako コンパイラ）の動作フローを記録。

## 実行環境

- **VM バックエンド**: `./target/release/nyash program.hako`（デフォルト）
- **LLVM バックエンド**: `./target/release/nyash --backend llvm program.hako`
- **selfhost 有効**: `NYASH_USE_NY_COMPILER=1` 等の環境変数

## JoinIR Strict モードとは

**環境変数**: `NYASH_JOINIR_STRICT=1`

**目的**: JoinIR 経路で旧 MIR/PHI 経路へのフォールバックを禁止し、厳格に JoinIR Lowering のみを使用

**期待される動作**:
- ✅ If/Loop Lowering が完全に JoinIR 経由で動作
- ❌ 旧 PHI 生成器へのフォールバックは禁止（エラーで停止）
- ⚠️ 警告: フォールバック候補があれば警告出力

## 代表パスの期待フロー

### 1. peek_expr_block.hako（簡易パーサーテスト）

**期待**:
- ✅ If 文が JoinIR If Lowering で処理
- ✅ ブロック式が正常に評価
- ✅ NYASH_JOINIR_STRICT=1 でもエラーなし

### 2. loop_min_while.hako（ループ・PHI 含む）

**期待**:
- ✅ Loop が JoinIR Loop Lowering で処理
- ✅ PHI 命令が正しく生成（ループ変数の合流）
- ⚠️ 警告: 旧 PHI 経路へのフォールバック候補があるかもしれない（Phase 120 調査対象）

### 3. [実用スクリプト名]（実用例）

**期待**:
- ✅ FileBox/StringBox 等の Box 操作が正常動作
- ✅ 複雑な制御構造が JoinIR 経由で処理
- ⚠️ 警告: 複雑さによってはフォールバックや警告が出る可能性

## Phase 120 の目標

上記の「期待」と「実際の動作」を比較し、ギャップを記録する。
実装修正は Phase 122+ で行う（Phase 120 はベースライン確立のみ）。
```

---

## 3. Task 2: 実行調査とログ記録

### 3.1 実行コマンド

各代表パスについて、以下のコマンドで実行し、出力を記録する:

```bash
# VM バックエンド（デフォルト）
NYASH_JOINIR_STRICT=1 NYASH_USE_NY_COMPILER=1 \
  ./target/release/nyash [代表パス.hako] 2>&1 | tee /tmp/phase120_vm_[name].log

# LLVM バックエンド（オプション）
NYASH_JOINIR_STRICT=1 NYASH_USE_NY_COMPILER=1 \
  ./target/release/nyash --backend llvm [代表パス.hako] 2>&1 | tee /tmp/phase120_llvm_[name].log
```

### 3.2 記録内容

**ログファイル**: `/tmp/phase120_execution_results.txt`（新規）

**記録形式**:

```
=== Phase 120: selfhost Stage-3 代表パス実行記録 ===
実行日時: 2025-12-04
Phase 106-115 完了時点のベースライン

--- 代表パス 1: peek_expr_block.hako ---
コマンド: NYASH_JOINIR_STRICT=1 NYASH_USE_NY_COMPILER=1 ./target/release/nyash apps/tests/peek_expr_block.hako
結果: ✅ 成功 / ❌ エラー / ⚠️ 警告あり

エラー・警告メッセージ:
[ログ出力をここに貼り付け]

備考:
- [気づいた点や特記事項]

--- 代表パス 2: loop_min_while.hako ---
...
```

### 3.3 分類基準

**ログ分類**:

| 分類 | 判定基準 | 対応 |
|------|---------|------|
| ✅ **完全成功** | エラーなし、警告なし、期待通りの出力 | ベースライン記録 |
| ⚠️ **警告あり** | 実行成功、警告メッセージあり | Phase 122+ で調査 |
| ❌ **エラー** | 実行失敗、エラーで停止 | Phase 122+ で修正 |

---

## 4. Task 3: スモークスクリプト作成

### 4.1 実装内容

**ファイル**: `tools/smokes/v2/profiles/integration/selfhost/phase120_stable_paths.sh`（新規）

**スクリプト構造**:

```bash
#!/bin/bash
# Phase 120: selfhost Stage-3 代表パス smoke テスト

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../common.sh"

# Phase 120 環境変数
export NYASH_JOINIR_STRICT=1
export NYASH_USE_NY_COMPILER=1

# 代表パス 1: peek_expr_block.hako
run_test "selfhost_peek_expr" "apps/tests/peek_expr_block.hako" "vm"

# 代表パス 2: loop_min_while.hako
run_test "selfhost_loop_min" "apps/tests/loop_min_while.hako" "vm"

# 代表パス 3: [実用スクリプト]
run_test "selfhost_example" "apps/examples/[name].hako" "vm"

# LLVM バックエンド版（オプション）
# run_test "selfhost_peek_expr_llvm" "apps/tests/peek_expr_block.hako" "llvm"

echo "[Phase 120] selfhost stable paths smoke test completed"
```

### 4.2 統合

**スモークテストランナーに追加**:

`tools/smokes/v2/profiles/integration/integration_profile.txt` に以下を追加:

```
selfhost/phase120_stable_paths.sh
```

### 4.3 実行確認

```bash
# 単発実行
bash tools/smokes/v2/profiles/integration/selfhost/phase120_stable_paths.sh

# integration プロファイル全体実行
tools/smokes/v2/run.sh --profile integration --filter "selfhost_*"
```

---

## 5. Task 4: ベースライン確立とドキュメント更新

### 5.1 実装内容

**ファイル**: `docs/development/current/main/phase120_baseline_results.md`（新規）

**記載内容**:

```markdown
# Phase 120: selfhost Stage-3 ベースライン結果

## 実行日時

2025-12-04（Phase 106-115 完了直後）

## 環境

- **Rust VM**: ./target/release/nyash
- **LLVM**: llvmlite ハーネス（オプション）
- **JoinIR Strict**: NYASH_JOINIR_STRICT=1
- **selfhost**: NYASH_USE_NY_COMPILER=1

## 代表パス実行結果

### 1. peek_expr_block.hako

| 項目 | 結果 |
|------|------|
| **実行結果** | ✅ 成功 / ⚠️ 警告 / ❌ エラー |
| **エラーメッセージ** | [ログから抽出] |
| **警告メッセージ** | [ログから抽出] |
| **備考** | [特記事項] |

### 2. loop_min_while.hako

[同様の表]

### 3. [実用スクリプト名]

[同様の表]

## Phase 122+ への課題

**優先度高**:
- [ ] [エラー1の説明]
- [ ] [エラー2の説明]

**優先度中**:
- [ ] [警告1の説明]
- [ ] [警告2の説明]

**優先度低（最適化）**:
- [ ] [改善案1]
- [ ] [改善案2]

## 結論

Phase 120 時点での selfhost Stage-3 経路は：
- ✅ **基本動作**: [成功した代表パスの数]本
- ⚠️ **警告あり**: [警告があった数]本
- ❌ **エラー**: [エラーが出た数]本

Phase 122+ で上記課題を段階的に解決する。
```

### 5.2 CURRENT_TASK.md 更新

**ファイル**: `CURRENT_TASK.md`（修正）

**Phase 120 セクション追加**:

```markdown
### 🎯 Phase 120: selfhost Stage-3 代表パスの安定化（完了）

- ✅ 代表パス選定: [選定した .hako ファイル名]
- ✅ 期待フロー整理: selfhost_stage3_expected_flow.md 作成
- ✅ 実行調査完了: NYASH_JOINIR_STRICT=1 での動作確認
- ✅ ベースライン確立: phase120_baseline_results.md 作成
- ✅ スモークスクリプト: phase120_stable_paths.sh 作成

**次のステップ**: Phase 121（hako_check JoinIR 統合設計）
```

---

## 6. 完成チェックリスト（Phase 120）

- [ ] 代表パス 2-3本の選定完了（peek_expr_block.hako 等）
- [ ] selfhost_stage3_expected_flow.md 作成（期待フロー整理）
- [ ] NYASH_JOINIR_STRICT=1 での実行ログ記録（/tmp/phase120_execution_results.txt）
- [ ] phase120_baseline_results.md 作成（ベースライン確立）
- [ ] スモークスクリプト作成（phase120_stable_paths.sh）
- [ ] integration プロファイルへの統合確認
- [ ] CURRENT_TASK.md 更新（Phase 120 完了記録）
- [ ] ビルド・テスト全 PASS（cargo build --release && bash phase120_stable_paths.sh）

---

## 7. 設計原則（Phase 120 で確立）

### ベースライン First

```
【Phase 120 の哲学】
実装修正の前に、「現状を正確に記録する」

Flow:
    Phase 106-115 完了
        ↓
    Phase 120: 現状記録（ベースライン確立）
        ↓
    Phase 121: 設計（hako_check 統合計画）
        ↓
    Phase 122+: 実装修正（段階的改善）
```

### 代表パスの活用

**少数精鋭の代表パスで効率的に検証**:

- **2-3本**で selfhost 経路の主要パターンをカバー
- **簡単・中間・複雑**の3段階でバランス
- **既存テスト**を活用（新規作成は最小限）

### JoinIR Strict モードの意義

**Phase 120 での使い方**:

- **厳格モード**: フォールバックを禁止し、JoinIR 経路の完全性を確認
- **警告収集**: 現状の JoinIR 経路の課題を可視化
- **Phase 122+ の修正指針**: 警告・エラーが修正の優先順位を決める

---

**Phase 120 指示書完成日**: 2025-12-04（Phase 106-115 完了直後）
Status: Historical
