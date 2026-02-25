# Phase 150: Selfhost Stage-3 Depth-1 ベースライン強化

## 🎯 ゴール

**Rust → Ny コンパイラ(Stage-3構文) → JSON v0 → Rust VM/LLVM** の selfhost 1周目（depth-1）を、代表 3 本だけでなく、より広いパターンで安定して動くラインとして確定する。

目的：
- Stage-3 構文の selfhost コンパイラで確実に 1 周目を回すベースラインを確立
- 将来の変更時に簡単に「足場チェック」できるスモーク体制構築
- selfhost 特有のバグを小フェーズに分離・可視化

## 📋 スコープ（やること・やらないこと）

### ✅ やること
- selfhost パイプラインの現状図を 1 ファイルに統合
- 代表ケース（3本）→ 5-7本に拡張
  - 文字列処理ケース
  - JSON 処理ケース
  - FileBox 軽量ケース
  - using 多めの CLI ケース
- selfhost depth-1 スモークスクリプト作成（`selfhost_phase150_depth1_smoke.sh`）
- 見つかったバグを Phase 151+ に切り出し
- CURRENT_TASK にロードマップを反映

### ❌ やらないこと
- JoinIR/MIR 生成の意味論・実装（Rust側）
- .hako JoinIR/MIR ビルダーの実装（Phase 160+ で実施）
- ny-llvmc への移行（Phase 140+ に先送り）

---

## 🏗️ 5 つのタスク

### Task 1: Selfhost パイプラインの現状図を 1 枚にまとめる

**ファイル**: `docs/development/current/main/selfhost_stage3_expected_flow.md`（更新）

**やること**:

1. **Rust バイナリ → JSON v0 → VM/LLVM のフロー図**:
   ```
   target/release/hakorune (Rust)
       ↓ [Stage-B: 関数スキャン + scaffold]
   stage1_cli.hako (JSON v0 scaffold)
       ↓ [Stage-1: CLI/using 解決]
   stage1_output.hako (Stage-3 構文)
       ↓ [Stage-3: 実際のコンパイル本体]
   Program(JSON v0)
       ↓ [dev verify]
   VM/LLVM 実行
   ```

2. **各ステージの責務を短く追記**:
   - Stage-B: 関数スキャン、Program scaffold、JSON v0 初期生成
   - Stage-1: CLI/using 解決、引数処理
   - Stage-3: 実際のコンパイル本体（IR 生成）
   - dev_verify: JSON v0 形式検証、実行準備

3. **更新対象の参照ファイル**:
   - phase120_baseline_results.md
   - phase124_json_v0_ir_bridge.md
   - selfhost_stage3_expected_flow.md

**成果物**:
- 更新版 `selfhost_stage3_expected_flow.md`（図 + ステージ責務）

---

### Task 2: Selfhost 代表ケースを 3 → 5〜7 本に拡張

**対象ファイル**: `apps/tests/` の候補ケース

**現在の 3 本**:
- `peek_expr_block.hako` - ブロック式・peek 基本
- `loop_min_while.hako` - ループ基本
- `esc_dirname_smoke.hako` - 文字列処理（dirname）

**追加候補（2-4本を選択）**:

1. **文字列長めケース**（候補: `string_*` ファイルから 1-2本）
   - 条件: 文字列操作が複数行、中程度の複雑度
   - 例: `string_ops_basic.hako` など

2. **JSON 処理ケース**（候補: `json_*` ファイルから 1本）
   - 条件: JSON 読み込み/操作、軽量
   - 例: `json_parse_simple.hako` など

3. **FileBox ケース**（候補: `file_*` から 1本、NoFs 外す）
   - 条件: FileBox/FileHandleBox 軽量使用、Default プロファイル
   - 例: `file_read_simple.hako` など

4. **using 多めケース**（候補: 新規作成または既存軽量ファイル）
   - 条件: using nyashstd の使用が目立つ、ミニ CLI
   - 例: `using_cli_simple.hako` など

**作業**:

1. 代表候補を `tools/selfhost_candidates.txt` にリスト化
2. 各ケースについて selfhost 実行テスト:
   ```bash
   NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
     ./target/release/hakorune <candidate>.hako
   ```
3. 成功/失敗・エラー傾向を記録

**成果物**:
- `docs/development/current/main/phase150_selfhost_stage3_depth1_baseline.md`（新規）
  - 表形式で候補一覧、実行結果（Rust VM / LLVM）
  - known issue あればメモ欄

---

### Task 3: Selfhost 用スモークスクリプトを整備

**ファイル**: `tools/smokes/v2/profiles/integration/selfhost_phase150_depth1_smoke.sh`（新規）

**目的**: Task 2 で選んだ 5-7本を 1 コマンドで実行、足場チェック

**実装**:

```bash
#!/bin/bash
# Selfhost Phase 150 Depth-1 Smoke Test
# Purpose: Verify 5-7 representative cases for selfhost Stage-3 pipeline

PASS=0
FAIL=0
RESULTS=()

# Test candidates (from Task 2)
CANDIDATES=(
    "apps/tests/peek_expr_block.hako"
    "apps/tests/loop_min_while.hako"
    "apps/tests/esc_dirname_smoke.hako"
    "apps/tests/<string_case>.hako"
    "apps/tests/<json_case>.hako"
    "apps/tests/<file_case>.hako"
    # ... more candidates
)

for candidate in "${CANDIDATES[@]}"; do
    echo "Testing: $candidate"

    if NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
       ./target/release/hakorune "$candidate" > /tmp/selfhost_test.log 2>&1; then
        echo "  ✅ PASS"
        ((PASS++))
        RESULTS+=("✅ $candidate")
    else
        echo "  ❌ FAIL"
        ((FAIL++))
        RESULTS+=("❌ $candidate")
    fi
done

echo ""
echo "=== Selfhost Phase 150 Depth-1 Results ==="
for result in "${RESULTS[@]}"; do
    echo "$result"
done
echo ""
echo "Summary: $PASS passed, $FAIL failed"
```

**成果物**:
- スクリプト + logging_policy への一言（「selfhost depth-1 スモークはここ」）

---

### Task 4: 見つかった Selfhost 専用バグを小フェーズに切り出す

**目的**: Task 2/3 で失敗したケースを Phase 151+ に分割

**作業**:

1. 失敗ケース分析:
   - どのステージ（Stage-B / JSON emit / dev verify / JoinIR）で落ちたか特定
   - エラーメッセージ/ログを記録

2. CURRENT_TASK に新エントリ:
   ```
   ### Phase 151: Selfhost Stage-3 X バグ修正

   **原因**: Stage-B JSON emit（例）
   **失敗ケース**: <case1>, <case2>
   **見込み工数**: 2-3 時間
   ```

**成果物**:
- `phase150_selfhost_stage3_depth1_baseline.md` に「Failure Summary」セクション追記
- CURRENT_TASK.md に Phase 151+ ToDo 行

---

### Task 5: CURRENT_TASK とロードマップの整合性更新

**ファイル**: `CURRENT_TASK.md`

**やること**:

1. CURRENT_TASK 冒頭ロードマップに Phase 150 追記:
   ```markdown
   ### Phase 150: Selfhost Stage-3 Depth-1 ベースライン強化 ← **進行中**

   **目的**: 3本 → 5-7本の代表ケースで depth-1 安定化
   **成果物**: パイプライン図、代表ケース拡張、スモークスクリプト
   **次フェーズ**: Phase 151+ selfhost 特有バグ修正

   **注**: .hako JoinIR/MIR 移植章（Phase 160+）は Phase 150 完了後に着手予定
   ```

2. Phase 150 実装結果サマリーを追記:
   - パイプライン図更新完了
   - 代表ケース数（3 → X本）
   - スモークスクリプト配置
   - 見つかったバグ数・分類

**成果物**:
- 更新版 `CURRENT_TASK.md`

---

## ✅ 完成チェックリスト（Phase 150）

- [ ] Task 1: Selfhost パイプライン図を 1 ファイルに統合
- [ ] Task 2: 代表ケース 3 → 5-7本に拡張
  - [ ] 候補リスト化（tools/selfhost_candidates.txt）
  - [ ] 各ケース実行テスト
  - [ ] 結果表を作成（phase150_selfhost_stage3_depth1_baseline.md）
- [ ] Task 3: Selfhost スモークスクリプト作成
  - [ ] selfhost_phase150_depth1_smoke.sh 実装
  - [ ] logging_policy への一言追記
- [ ] Task 4: 失敗ケースを Phase 151+ に切り出し
  - [ ] failure summary セクション追記
  - [ ] Phase 151+ ToDo を CURRENT_TASK に追加
- [ ] Task 5: CURRENT_TASK 更新
  - [ ] ロードマップに Phase 150 追記
  - [ ] Phase 150 実装結果サマリー
- [ ] git commit で記録

---

## 所要時間

**4-5 時間程度**

- Task 1（パイプライン図整理）: 45分
- Task 2（代表ケース拡張・テスト）: 2時間
- Task 3（スモークスクリプト）: 45分
- Task 4（バグ分類）: 1時間
- Task 5（CURRENT_TASK 更新）: 30分

---

## 次のステップ

**Phase 151+: Selfhost 特有バグ修正**
- Task 4 で見つかったバグを段階的に修正
- 各バグごとに小フェーズを切り出し

**Phase 160+: .hako JoinIR/MIR 移植章**
- Rust MIR ビルダーを .hako に移植
- depth-2 以上の selfhost サポート

---

## 進捗

- ✅ Phase 130-134: LLVM Python バックエンド整理（mir_call 統一、StringBox bridge）
- 🎯 Phase 150: Selfhost Stage-3 Depth-1 ベースライン強化（← **現在のフェーズ**）
- 📋 Phase 151+: Selfhost 特有バグ修正（予定）
- 📋 Phase 160+: .hako JoinIR/MIR 移植章（予定）
Status: Historical

