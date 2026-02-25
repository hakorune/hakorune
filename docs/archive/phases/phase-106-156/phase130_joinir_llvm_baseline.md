# Phase 130: JoinIR → LLVM ベースライン確立

## 🎯 ゴール

「JoinIR で selfhost/hako_check まで安定した」現在の状態から、**JoinIR → LLVM 経路の現状を観測・記録する** フェーズ。

目的：
- 代表的な .hako を LLVM ラインで実行し、「どこまで通っているか、どこが赤いか」を一覧化
- JoinIR → MIR → LLVM（ny-llvmc / ハーネス）経路での問題点（命令未対応 / ABI ズレ / print系など）を洗い出す
- **実装修正は Phase 131+ に回す**（Phase 130 は「観測専用」と割り切る）

```
Phase 124: JoinIR/selfhost 第2章 完了 ✅
        ↓
Phase 130: 「JoinIR → LLVM どこが赤いか」を観測・記録 ← ← ここ！
        ↓
Phase 131+: 個別の LLVM 側問題を潰す
```

---

## 📋 スコープ（やること・やらないこと）

### ✅ やること
1. 代表ケース選定（JoinIR/selfhost/hako_check から 5〜8 本）
2. LLVM 実行コマンドと環境変数の整理（Rust VM と LLVM ハーネス両方）
3. 実行結果（成功 / 失敗 / 既知問題）を 1 つの docs にまとめる
4. CURRENT_TASK / Backlog に「JoinIR→LLVM 第3章の入り口」を追記

### ❌ やらないこと
- LLVM 側の実装修正（Phase 131 以降の役割）
- 新しい最適化パスやコード生成ルールの追加
- JoinIR / Ring0 の設計変更

---

## 🏗️ Task 1: 代表ケース選定（完了）✅

Phase 130 では以下の **7 本** を代表ケースとして選定しました：

| # | カテゴリ | ファイル | 用途 | 選定理由 |
|---|---------|---------|-----|---------|
| 1 | selfhost Stage-3 | `apps/tests/peek_expr_block.hako` | 式ブロック・peek構文 | Phase 120で検証済み、基本的な式評価 |
| 2 | selfhost Stage-3 | `apps/tests/loop_min_while.hako` | ループ・条件分岐 | Phase 120で検証済み、ループとPHI |
| 3 | selfhost Stage-3 | `apps/tests/esc_dirname_smoke.hako` | ConsoleBox.println・複合処理 | Phase 120/122で検証済み、複雑な制御フロー |
| 4 | hako_check / Phase 123 | `local_tests/phase123_simple_if.hako` | シンプルなif文 | Phase 124でJoinIR専用化テスト済み |
| 5 | hako_check / Phase 123 | `local_tests/phase123_while_loop.hako` | while loop | Phase 124でJoinIR専用化テスト済み |
| 6 | JoinIR/PHI | `apps/tests/joinir_if_select_simple.hako` | IfSelect（単純） | JoinIR If Lowering基本ケース |
| 7 | JoinIR/PHI | `apps/tests/joinir_min_loop.hako` | 最小ループ | JoinIR Loop基本ケース |

### 選定基準
- **多様性**: selfhost/hako_check/JoinIR の3カテゴリから選定
- **段階性**: 基本→複雑の順でカバー
- **実績**: Phase 120-124 で検証済みのケースを優先
- **LLVM適合性**: Console/StringBox/基本制御フローを含む

---

## 🔧 Task 2: LLVM 実行コマンドの整理

### 環境変数一覧

| 環境変数 | 用途 | 必須 |
|---------|-----|------|
| `NYASH_LLVM_USE_HARNESS=1` | Python/llvmlite ハーネス使用 | ✅ LLVM実行時必須 |
| `LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix)` | LLVM 18 設定 | ✅ LLVM実行時必須 |

### 実行コマンド例

#### Rust VM 実行（比較用）
```bash
./target/release/nyash --backend vm apps/tests/peek_expr_block.hako
```

#### LLVM ハーネス実行
```bash
LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix) \
NYASH_LLVM_USE_HARNESS=1 \
  ./target/release/nyash --backend llvm apps/tests/peek_expr_block.hako
```

#### 統合スモークテスト（v2プロファイル）
```bash
# 全integration テスト
./tools/smokes/v2/run.sh --profile integration

# Phase 120 stable paths（Rust VM）
./tools/smokes/v2/run.sh --profile integration --filter "*phase120_stable_paths*"
```

### 各代表ケースの実行方法

代表ケースごとに、以下の2パターンで実行：

1. **Rust VM**（比較軸・greenベースライン）:
   ```bash
   ./target/release/nyash --backend vm <HAKO_FILE>
   ```

2. **LLVM harness**（観測対象）:
   ```bash
   LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix) \
   NYASH_LLVM_USE_HARNESS=1 \
     ./target/release/nyash --backend llvm <HAKO_FILE>
   ```

---

## 📊 Task 3: 実行結果とベースライン記録

### 凡例
- ✅ PASS: 正常終了、期待される出力
- ⚠️ PARTIAL: 部分的に動作、警告あり
- ❌ FAIL: エラーで失敗

---

### 1. apps/tests/peek_expr_block.hako

**期待される挙動**: peek式を使ったブロック式評価（match式、ブロック式で値を返す）

**実行結果**:

| 経路         | 結果  | メモ |
|--------------|-------|------|
| Rust VM      | ✅    | 正常動作。"found one"出力、RC=1（main関数の戻り値） |
| LLVM harness | ⚠️    | Mock backend使用中。MIRコンパイル成功、実LLVM実行は未対応 |

**VM出力**: `found one`, `RC: 1`

---

### 2. apps/tests/loop_min_while.hako

**期待される挙動**: 最小限のwhile loop、PHI命令生成

**実行結果**:

| 経路         | 結果  | メモ |
|--------------|-------|------|
| Rust VM      | ✅    | 正常動作。ループ（0,1,2出力）、ControlForm::Loop生成確認 |
| LLVM harness | ⚠️    | Mock backend使用中。MIRコンパイル成功、実LLVM実行は未対応 |

**VM出力**: `0`, `1`, `2`, `RC: 0`

---

### 3. apps/tests/esc_dirname_smoke.hako

**期待される挙動**: ConsoleBox.println、StringBox操作、複雑な制御フロー

**実行結果**:

| 経路         | 結果  | メモ |
|--------------|-------|------|
| Rust VM      | ❌    | ConsoleBox未対応エラー: "Unknown Box type: ConsoleBox" |
| LLVM harness | ⚠️    | Mock backend使用中。MIRコンパイル成功、実LLVM実行は未対応 |

**VMエラー**: `[ERROR] ❌ [rust-vm] VM error: Invalid instruction: NewBox ConsoleBox: invalid operation: Unknown Box type: ConsoleBox. Available: Main`

**根本原因**: ConsoleBoxがRust VM環境で登録されていない（PluginBox問題）

---

### 4. local_tests/phase123_simple_if.hako

**期待される挙動**: シンプルなif文のJoinIR lowering

**実行結果**:

| 経路         | 結果  | メモ |
|--------------|-------|------|
| Rust VM      | ✅    | 正常動作。JoinIR If lowering成功、RC=0 |
| LLVM harness | ⚠️    | Mock backend使用中。MIRコンパイル成功、実LLVM実行は未対応 |

**VM出力**: `RC: 0`

---

### 5. local_tests/phase123_while_loop.hako

**期待される挙動**: while loopのJoinIR lowering

**実行結果**:

| 経路         | 結果  | メモ |
|--------------|-------|------|
| Rust VM      | ✅    | 正常動作。ControlForm::Loop生成、RC=0 |
| LLVM harness | ⚠️    | Mock backend使用中。MIRコンパイル成功、実LLVM実行は未対応 |

**VM出力**: `RC: 0`

---

### 6. apps/tests/joinir_if_select_simple.hako

**期待される挙動**: IfSelectパターンの基本ケース

**実行結果**:

| 経路         | 結果  | メモ |
|--------------|-------|------|
| Rust VM      | ✅    | 正常動作。JoinIR If Lowering実装済み、RC=0 |
| LLVM harness | ⚠️    | Mock backend使用中。MIRコンパイル成功、実LLVM実行は未対応 |

**VM出力**: `RC: 0`

---

### 7. apps/tests/joinir_min_loop.hako

**期待される挙動**: JoinIR最小ループケース

**実行結果**:

| 経路         | 結果  | メモ |
|--------------|-------|------|
| Rust VM      | ✅    | 正常動作。ControlForm::Loop（breakブロック含む）生成、RC=0 |
| LLVM harness | ⚠️    | Mock backend使用中。MIRコンパイル成功、実LLVM実行は未対応 |

**VM出力**: `RC: 0`

---

## 📈 実行結果サマリー

### 統計

| 経路         | PASS | PARTIAL | FAIL | 合計 | 成功率 |
|--------------|------|---------|------|------|--------|
| Rust VM      | 6    | 0       | 1    | 7    | 85.7%  |
| LLVM harness | 0    | 7 (Mock)| 0    | 7    | 0% (Mock実行) |

**Rust VM結果詳細**:
- ✅ PASS: 6/7 (peek_expr_block, loop_min_while, phase123_simple_if, phase123_while_loop, joinir_if_select_simple, joinir_min_loop)
- ❌ FAIL: 1/7 (esc_dirname_smoke - ConsoleBox未対応)

**LLVM harness結果詳細**:
- ⚠️ 全テストがMock backend実行（実LLVM実行は未対応）
- ✅ MIRコンパイルは全7テストで成功
- ❌ 実際のLLVM IR生成・実行は未実装

### 検出された問題点

#### 1. LLVM Backend未対応（最重要）

**現象**:
```
🔧 Mock LLVM Backend Execution:
   Build with --features llvm-inkwell-legacy for Rust/inkwell backend,
   or set NYASH_LLVM_OBJ_OUT and NYASH_LLVM_USE_HARNESS=1 for harness.
✅ Mock exit code: 0
```

**原因**:
- `--backend llvm` 指定時、Mock backendにフォールバック
- 実際のLLVM IR生成・実行機構が無効化されている
- `--features llvm` ビルドが必要（未実施）

**影響範囲**: 全7テストケース

**Phase 131での対応**:
1. `cargo build --release --features llvm` でLLVM機能有効化ビルド
2. Python/llvmlite ハーネス（`src/llvm_py/`）の動作確認
3. 実LLVM実行での再テスト

---

#### 2. ConsoleBox未登録問題

**現象**: `apps/tests/esc_dirname_smoke.hako`
```
[ERROR] ❌ [rust-vm] VM error: Invalid instruction: NewBox ConsoleBox:
invalid operation: Unknown Box type: ConsoleBox. Available: Main
```

**原因**:
- Rust VM環境でConsoleBoxが登録されていない
- PluginBox（Console）とビルトインBoxの登録問題
- Phase 15.5の "Everything is Plugin" 方針と衝突

**影響範囲**:
- esc_dirname_smoke.hako（Console出力を使用）
- 他の複雑な制御フローテスト（潜在的）

**Phase 131での対応**:
1. ConsoleBoxのVM登録確認
2. PluginBox vs ビルトインBoxの登録優先順位整理
3. Phase 120/122で解決済みのはずだが、環境依存の可能性

---

#### 3. JoinIR → LLVM経路の不明確性

**観測事実**:
- JoinIR → MIR変換: ✅ 全テストで成功
- MIR → LLVM IR: ⚠️ Mock実行（未検証）
- LLVM実行: ❌ 未対応

**Phase 131での確認事項**:
1. `ny-llvmc` コンパイラの状態確認
2. Python/llvmlite ハーネスの動作確認
3. MIR14命令 → LLVM IR lowering実装状況
4. BoxCall/NewBox/PHI命令のLLVM対応

---

### Phase 131への引き継ぎ事項

#### 優先度1: LLVM Backend有効化
- [ ] `cargo build --release --features llvm` 実行
- [ ] Python/llvmlite 環境確認（`src/llvm_py/venv`）
- [ ] 実LLVM実行での7テスト再実行

#### 優先度2: ConsoleBox問題解決
- [ ] Rust VMでのConsoleBox登録状況調査
- [ ] Phase 122で解決済みの内容との差分確認
- [ ] PluginBox登録機構の修正（必要に応じて）

#### 優先度3: LLVM IR生成確認
- [ ] MIR → LLVM IR lowering実装状況調査
- [ ] 未対応命令の洗い出し（BoxCall/NewBox/PHI等）
- [ ] 最小ケース（joinir_if_select_simple.hako）での詳細検証

---

## 🚀 Task 4: CURRENT_TASK / Backlog 更新（完了）✅

**実施内容**:

1. **CURRENT_TASK.md更新**:
   - Phase 130 セクション追加（実施日: 2025-12-04）
   - 実行結果統計（Rust VM: 6/7 PASS、LLVM: 0/7 Mock実行）
   - 検出された3つの問題点の記録
   - Phase 131への引き継ぎ事項を明記

2. **30-Backlog.md更新**:
   - 短期タスクを「第3章 - LLVM統合」に更新
   - Phase 131の3つの優先度タスクを追加
   - Phase 120-124を「完了済み第2章」に移動

---

## ✅ 完成チェックリスト（Phase 130）

- [x] `phase130_joinir_llvm_baseline.md` が存在し、代表パス/コマンド/結果が整理されている
- [x] 7 本の .hako が選定され、ドキュメントに記載されている
- [x] 各ケースが「Rust VM / LLVM backend」両方で実行されている
- [x] 実行結果の表が表形式で docs に記載されている
- [x] **実装修正は一切入れていない**（赤は赤のまま、一覧化だけしている）
- [x] CURRENT_TASK.md に Phase 130 完了行が追加されている
- [x] Backlog に「Phase 131: JoinIR→LLVM 個別修正ライン」が追加されている
- [x] git commit で記録（コミット: 43d59110 `docs(phase130): JoinIR→LLVM ベースライン確立`）

---

## 📋 次のステップ

**Phase 131: JoinIR→LLVM 個別修正ライン** - Phase 130 で検出された問題を優先度順に潰す

---

## 📝 進捗

- ✅ Phase 124: hako_check レガシー削除 & JoinIR 専用化（完了）
- ✅ Phase 56: array_ext.filter JoinIR 対応（テスト修正完了）
- ✅ Phase 130: JoinIR → LLVM ベースライン確立（← **完了！** 2025-12-04）
  - ✅ Task 1: 代表ケース選定（7本選定完了）
  - ✅ Task 2: LLVM実行コマンド整理（完了）
  - ✅ Task 3: 実行とベースライン記録（完了）
    - Rust VM: 6/7 PASS (85.7%)
    - LLVM: 0/7実行（Mock backend、要`--features llvm`ビルド）
  - 🔄 Task 4: ドキュメント更新（実行中）
- 📋 Phase 131+: JoinIR→LLVM 個別修正ライン（予定）

### Phase 130実行結果サマリー

**Rust VM（--backend vm）**:
- ✅ 6/7テストPASS（85.7%成功率）
- ❌ 1/7失敗: esc_dirname_smoke.hako（ConsoleBox未登録問題）

**LLVM harness（--backend llvm）**:
- ⚠️ 7/7テストがMock backend実行（実LLVM未対応）
- ✅ MIRコンパイルは全て成功
- ❌ `--features llvm` ビルドが必要と判明

**重要な発見**:
1. LLVM backend機能が現在のビルドで無効化されている
2. ConsoleBoxのRust VM登録問題が再発
3. JoinIR → MIR変換は全て正常動作
4. Phase 131での優先課題が明確化

---

## Phase 131 修正内容（2025-12-04実施）

### 修正1: LLVM Backend Re-enable ✅

**実施内容**:
1. `cargo build --release --features llvm` でLLVM機能有効化ビルド実行
2. Python/llvmlite環境確認（llvmlite 0.45.1インストール済み）
3. llvmlite非推奨API対応: `llvm.initialize()` 削除（自動初期化に移行）

**修正ファイル**:
- `src/llvm_py/llvm_builder.py`: `llvm.initialize()` 呼び出しをコメントアウト

**結果**:
- ✅ `peek_expr_block.hako`: LLVM実行成功（Result: 1、Rust VM: RC: 1）
- ✅ Mock backendから実LLVM実行への移行成功
- ✅ LLVM harness経路が正常動作

### 修正2: PHI命令順序バグ発見 ⚠️

**検出された問題**:
LLVM IR生成時、PHI命令がreturn命令の**後**に配置されるバグを発見。

**問題例**（生成されたLLVM IR）:
```llvm
bb5:
  ret i64 %"ret_phi_16"
  %"ret_phi_16" = phi i64 [0, %"bb3"], [0, %"bb4"]  ; ← エラー！PHIはretの前に必要
}
```

**LLVM IRの制約**:
- PHI命令はBasic Blockの**先頭**に配置必須
- terminator命令（ret/br/switch等）の後に命令を配置不可

**影響範囲**:
- ❌ phase123_simple_if.hako: LLVM IR parsing error
- ❌ loop_min_while.hako: LLVM IR parsing error
- ❌ 制御フロー合流を含む全テストが影響

**根本原因**:
- `src/llvm_py/llvm_builder.py`の`finalize_phis()`関数
- PHI nodesがblock終端処理後に追加されている
- LLVM IRbuilderのblock構築順序の設計問題

**Phase 132への引き継ぎ**:
この問題は`finalize_phis()`の大規模リファクタリングが必要（100行以上の関数）。
Phase 131の最小スコープを超えるため、Phase 132で対応。

### 修正3: ConsoleBox LLVM統合 ⚠️

**現状確認**:
- ❌ Rust VM環境でもConsoleBox未登録（`apps/tests/esc_dirname_smoke.hako`実行不可）
- ❌ LLVM環境でもConsoleBox未対応

**Phase 132への引き継ぎ**:
ConsoleBoxの登録・実装はRust VM側の問題。LLVM統合の前提条件未達のため、Phase 132で対応。

### Phase 131 実行結果サマリー

**修正前（Phase 130）**:
| 経路         | PASS | PARTIAL | FAIL | 成功パス |
|--------------|------|---------|------|---------|
| Rust VM      | 6    | 0       | 1    | 6/7     |
| LLVM harness | 0    | 7 (Mock)| 0    | 0/7     |

**修正後（Phase 131）**:
| 経路         | PASS | PARTIAL | FAIL | 成功パス | メモ |
|--------------|------|---------|------|---------|------|
| Rust VM      | 6    | 0       | 1    | 6/7     | 変更なし |
| LLVM harness | 1    | 0       | 6    | 1/7     | peek_expr_block.hako成功 |

**成功ケース詳細**:
- ✅ `peek_expr_block.hako`: Rust VM ✅ → LLVM ✅（Result: 1）

**失敗ケース詳細**:
- ❌ `loop_min_while.hako`: PHI ordering bug
- ❌ `phase123_simple_if.hako`: PHI ordering bug
- ❌ `phase123_while_loop.hako`: PHI ordering bug
- ❌ `joinir_if_select_simple.hako`: PHI ordering bug
- ❌ `joinir_min_loop.hako`: PHI ordering bug
- ❌ `esc_dirname_smoke.hako`: ConsoleBox未登録

### Phase 131 完了判定

**達成内容**:
1. ✅ LLVM backend最小re-enable成功（peek_expr_block.hako ✅）
2. ⚠️ PHI ordering bug発見・記録（Phase 132対応）
3. ⚠️ ConsoleBox問題確認（Phase 132対応）

**最小成功パス確立**: ✅ 1/7テストで成功（目標：2-3本だが、根本的なPHI問題により1本に制限）

**Phase 132への優先課題**:
1. **最優先**: PHI命令順序バグ修正（`finalize_phis()`リファクタリング）
2. **優先**: ConsoleBox登録問題解決
3. **通常**: 残りテストケースのLLVM対応
Status: Historical
