# Phase 72-73 ENV 使用状況棚卸し（ENV/JoinIR整理）

**調査日**: 2025-12-02
**目的**: Stage-3 / JoinIR / selfhost 関連の ENV 使用状況を洗い出し、Phase 72-73 整理の優先順位を決定する。

---

## 📊 Executive Summary（総括）

### 全体傾向
- **Stage-3 関連**: 既に `NYASH_FEATURES=stage3` への移行が進行中（96箇所のスクリプトで使用）
- **JoinIR 関連**: `NYASH_JOINIR_EXPERIMENT` 系が実験モードとして広く使用（直接読み込み多数）
- **直接読み込み問題**: JoinIR系で20ファイル以上が `std::env::var()` 直接読み込み → SSOT整理が急務

### 優先順位（推奨整理順）
1. **CRITICAL**: JoinIR 直接読み込みの SSOT 化（20+ファイル）
2. **HIGH**: `HAKO_JOINIR_*` の統合整理（12種類のENV、一部は実験用）
3. **MEDIUM**: Stage-3 legacy ENV の完全削除（`NYASH_PARSER_STAGE3`, `HAKO_PARSER_STAGE3`）
4. **LOW**: `NYASH_FEATURES=stage3` のデフォルト化完了確認

---

## 1. Stage-3 関連 ENV

### 1.1 `NYASH_PARSER_STAGE3`

**使用箇所総数**: 62箇所

#### カテゴリ別内訳
- **Rust SSOT経由**: 2箇所（config/env.rs での定義のみ）
  - `src/config/env.rs:715` - `env_flag("NYASH_PARSER_STAGE3")` で読み込み
  - `src/mir/builder.rs:538` - エラーメッセージでの言及のみ

- **Rust 直接読み込み**: 27箇所（⚠️ **問題箇所**）
  - `tests/parser_stage3.rs`: 5箇所（テスト用 set/remove_var）
  - `tests/mir_*.rs`: 18箇所（テスト用 set_var）
  - `src/tests/*.rs`: 4箇所（テスト用 set_var）

- **Scripts**: 35箇所
  - `tools/selfhost/*.sh`: 12箇所
  - `tools/dev/*.sh`: 8箇所
  - `tools/smokes/*.sh`: 2箇所
  - `tools/hako_check/*.sh`: 1箇所
  - その他: 12箇所

- **Child env 伝播**: 2箇所
  - `src/runner/child_env.rs:57-58`
  - `src/runner/stage1_bridge/env.rs:131-132`

#### 現状
- ✅ **SSOT関数**: `config::env::parser_stage3_enabled()` が定義済み
- ✅ **優先順位**: `NYASH_FEATURES=stage3` > `NYASH_PARSER_STAGE3` > `HAKO_PARSER_STAGE3` > デフォルトtrue
- ⚠️ **問題**: テストでの直接読み込みが27箇所残存
- 📌 **推奨**: legacy ENV として deprecation warning を既に出力中（config/env.rs:716）

---

### 1.2 `HAKO_PARSER_STAGE3`

**使用箇所総数**: 46箇所

#### カテゴリ別内訳
- **Rust SSOT経由**: 2箇所（config/env.rs での定義のみ）
  - `src/config/env.rs:719` - `env_flag("HAKO_PARSER_STAGE3")` で読み込み
  - `src/mir/builder.rs:538` - エラーメッセージでの言及のみ

- **Rust 直接読み込み**: 19箇所（⚠️ **問題箇所**）
  - `tests/parser_stage3.rs`: 5箇所（テスト用 set/remove_var）
  - `tests/mir_*.rs`: 12箇所（テスト用 set_var）
  - `src/runner/child_env.rs`: 2箇所（子プロセスへの伝播）

- **Scripts**: 25箇所
  - `tools/selfhost/*.sh`: 10箇所
  - `tools/dev/*.sh`: 7箇所
  - `tools/smokes/v2/*.sh`: 2箇所
  - その他: 6箇所

- **Child env 伝播**: 2箇所
  - `src/runner/child_env.rs:57-58`
  - `src/runner/stage1_bridge/env.rs:131-132`

#### 現状
- ✅ **SSOT関数**: `config::env::parser_stage3_enabled()` が定義済み（共通関数）
- ✅ **優先順位**: `NYASH_FEATURES=stage3` > `NYASH_PARSER_STAGE3` > `HAKO_PARSER_STAGE3` > デフォルトtrue
- ⚠️ **問題**: テストでの直接読み込みが19箇所残存
- 📌 **推奨**: legacy ENV として deprecation warning を既に出力中（config/env.rs:720）

---

### 1.3 `NYASH_FEATURES=stage3`

**使用箇所総数**: 144箇所（推定）

#### カテゴリ別内訳
- **Rust SSOT経由**: 21箇所
  - `src/config/env.rs` の `feature_stage3_enabled()` → `parser_stage3_enabled()` 経由
  - Parser、Builder、Runner など主要モジュールで使用

- **Rust 直接設定**: 15箇所（テストでの `std::env::set_var("NYASH_FEATURES", "stage3")`）
  - `src/tests/mir_stage1_using_resolver_verify.rs`
  - `src/tests/stage1_cli_entry_ssa_smoke.rs`
  - `src/tests/mir_joinir_*.rs` など

- **Scripts**: 96箇所（`tools/smokes/v2/` が主体）
  - `tools/smokes/v2/profiles/quick/`: 多数
  - `tools/smokes/v2/lib/stageb_helpers.sh`: 4箇所
  - `tools/smokes/v2/lib/test_runner.sh`: 7箇所

- **Child env 伝播**: 3箇所
  - `src/runner/stage1_bridge/env.rs:118-124` - Stage-3フラグの自動伝播
  - `src/runner/child_env.rs:52` - Stage-3フラグの明示的設定

#### 現状
- ✅ **SSOT関数**: `config::env::feature_stage3_enabled()` が定義済み
- ✅ **デフォルト**: `parser_stage3_enabled()` が `true` を返す（Stage-3は標準構文化済み）
- ✅ **移行完了度**: 高（96箇所のスクリプトで採用）
- 📌 **推奨**: legacy ENV の完全削除に向けた最終段階

---

## 2. JoinIR 関連 ENV

### 2.1 `NYASH_JOINIR_CORE`

> 2025-12 現在: JoinIR は常時 ON。`NYASH_JOINIR_CORE` は警告のみで無視される（LoopBuilder 削除済み、config/env で no-op）。

**使用箇所総数**: 9箇所

#### カテゴリ別内訳
- **Rust SSOT経由**: 1箇所
  - `src/config/env.rs:242` - `env_flag("NYASH_JOINIR_CORE")` で読み込み

- **Rust 直接読み込み**: 6箇所（⚠️ **問題箇所**）
  - `src/tests/mir_stage1_staticcompiler_receiver.rs:39` - `std::env::set_var("NYASH_JOINIR_CORE", "1")`
  - `src/tests/mir_joinir_if_select.rs:14,19` - set/remove_var
  - `src/tests/mir_joinir_stage1_using_resolver_min.rs:31` - set_var
  - `src/tests/helpers/joinir_env.rs:7,12,17` - set/remove_var helper

- **Scripts**: 0箇所

- **Tests**: 6箇所（全てテストヘルパー経由）

#### 現状
- ✅ **SSOT関数**: `config::env::joinir_core_enabled()` が定義済み
- ⚠️ **問題**: テストでの直接読み込みが6箇所残存
- 📌 **推奨**: `tests/helpers/joinir_env.rs` のヘルパー関数を SSOT 化

---

### 2.2 `NYASH_JOINIR_DEV`

**使用箇所総数**: 2箇所

#### カテゴリ別内訳
- **Rust SSOT経由**: 2箇所
  - `src/config/env.rs:325` - `env_bool("NYASH_JOINIR_DEV")` 定義
  - `src/config/env.rs:328` - `joinir_dev_enabled()` での参照

- **Rust 直接読み込み**: 0箇所（✅ **SSOT完璧**）

- **Scripts**: 0箇所

#### 現状
- ✅ **SSOT関数**: `config::env::joinir_dev_enabled()` が定義済み
- ✅ **完璧な状態**: 全てSSAT経由、直接読み込みなし
- 📌 **推奨**: 現状維持（モデルケース）

---

### 2.3 `NYASH_JOINIR_STRICT`

**使用箇所総数**: 6箇所

#### カテゴリ別内訳
- **Rust SSOT経由**: 2箇所
  - `src/config/env.rs:263,265` - `env_flag("NYASH_JOINIR_STRICT")` 定義

- **Rust 直接読み込み**: 4箇所（⚠️ **問題箇所**）
  - `src/tests/mir_joinir_stage1_using_resolver_min.rs:32` - `std::env::set_var("NYASH_JOINIR_STRICT", "1")`
  - `src/tests/mir_joinir_if_select.rs:15,20` - set/remove_var
  - `src/tests/mir_stage1_staticcompiler_receiver.rs:40` - set_var

- **Scripts**: 0箇所

#### 現状
- ✅ **SSOT関数**: `config::env::joinir_strict_enabled()` が定義済み
- ⚠️ **問題**: テストでの直接読み込みが4箇所残存
- 📌 **推奨**: `tests/helpers/joinir_env.rs` にヘルパー追加

---

### 2.4 `NYASH_JOINIR_EXPERIMENT`

**使用箇所総数**: 49箇所

#### カテゴリ別内訳
- **Rust SSOT経由**: 7箇所
  - `src/config/env.rs:234` - `env_bool("NYASH_JOINIR_EXPERIMENT")` 定義
  - `src/runner/modes/vm.rs`, `llvm.rs` など主要実行器で参照

- **Rust 直接読み込み**: 39箇所（⚠️ **深刻な問題**）
  - `src/tests/mir_joinir_*.rs`: 15箇所（実験モードチェック）
  - `src/tests/joinir_*.rs`: 10箇所（実験モードチェック）
  - `src/mir/join_ir/`: 5箇所（条件付き有効化）
  - `src/mir/phi_core/loopform_builder.rs:81` - コメント言及
  - その他: 9箇所

- **Scripts**: 3箇所
  - `tools/joinir_ab_test.sh`: 4箇所

- **Tests**: 36箇所（テストスキップ判定）

#### 現状
- ✅ **SSOT関数**: `config::env::joinir_experiment_enabled()` が定義済み
- ❌ **深刻な問題**: 直接読み込みが39箇所残存（最多）
- 📌 **推奨**: **Phase 72 最優先タスク** - 全テストを SSOT 化

---

### 2.5 `HAKO_JOINIR_*` 系（12種類）

#### 2.5.1 `HAKO_JOINIR_IF_SELECT`

**使用箇所総数**: 19箇所

- **Rust SSOT経由**: 3箇所（config/env.rs）
- **Rust 直接読み込み**: 13箇所（テスト用 set/remove_var）
  - `src/tests/mir_joinir_if_select.rs`: 9箇所
  - `src/tests/helpers/joinir_env.rs`: 2箇所
- **Scripts**: 0箇所

**SSOT関数**: `config::env::joinir_if_select_enabled()` ✅

---

#### 2.5.2 `HAKO_JOINIR_DEBUG`

**使用箇所総数**: 20箇所

- **Rust SSOT経由**: 4箇所（config/env.rs）
- **Rust 直接読み込み**: 0箇所（✅ **完璧**）
- **使用関数**: `config::env::joinir_debug_level()` → 返り値 0-3

**SSOT関数**: `config::env::joinir_debug_level()` ✅（完璧な例）

---

#### 2.5.3 `HAKO_JOINIR_STAGE1`

**使用箇所総数**: 3箇所

- **Rust SSOT経由**: 2箇所（config/env.rs での定義のみ）
- **Rust 直接読み込み**: 0箇所（✅ **完璧**）
- **Scripts**: 0箇所

**SSOT関数**: `config::env::joinir_stage1_enabled()` ✅（未使用だが準備済み）

---

#### 2.5.4 `HAKO_JOINIR_IF_IN_LOOP_DRYRUN`

**使用箇所総数**: 4箇所

- **Rust SSOT経由**: 2箇所（config/env.rs + 関数使用2箇所）
- **Rust 直接読み込み**: 2箇所（テスト用 set/remove_var）
  - `src/tests/phase61_if_in_loop_dryrun.rs:13,16`

**SSOT関数**: `config::env::joinir_if_in_loop_dryrun_enabled()` ✅

---

#### 2.5.5 `HAKO_JOINIR_IF_IN_LOOP_ENABLE`

**使用箇所総数**: 3箇所

- **Rust SSOT経由**: 3箇所（config/env.rs + 関数使用）
- **Rust 直接読み込み**: 0箇所（✅ **完璧**）

**SSOT関数**: `config::env::joinir_if_in_loop_enable()` ✅

---

#### 2.5.6 `HAKO_JOINIR_IF_TOPLEVEL`

**使用箇所総数**: 3箇所

- **Rust SSOT経由**: 3箇所（config/env.rs + 関数使用）
- **Rust 直接読み込み**: 0箇所（✅ **完璧**）

**SSOT関数**: `config::env::joinir_if_toplevel_enabled()` ✅

---

#### 2.5.7 `HAKO_JOINIR_IF_TOPLEVEL_DRYRUN`

**使用箇所総数**: 2箇所

- **Rust SSOT経由**: 2箇所（config/env.rs での定義のみ）
- **Rust 直接読み込み**: 0箇所（✅ **完璧**）

**SSOT関数**: `config::env::joinir_if_toplevel_dryrun_enabled()` ✅（未使用だが準備済み）

---

#### 2.5.8 `HAKO_JOINIR_PRINT_TOKENS_MAIN` / `HAKO_JOINIR_ARRAY_FILTER_MAIN`

**使用箇所総数**: 10箇所

- **Rust SSOT経由**: 0箇所（❌ **SSOT未定義**）
- **Rust 直接読み込み**: 6箇所（⚠️ **問題箇所**）
  - `src/mir/builder/control_flow.rs:92,102` - 直接読み込み
  - `src/tests/joinir/mainline_phase49.rs`: 4箇所（set/remove_var）

- **Scripts**: 0箇所

**SSOT関数**: ❌ **未定義**（dev専用フラグ）

---

#### 2.5.9 `HAKO_JOINIR_IF_IN_LOOP_TRACE` / `HAKO_JOINIR_IF_TOPLEVEL_TRACE`

**使用箇所総数**: 2箇所

- **Rust SSOT経由**: 0箇所（❌ **SSOT未定義**）
- **Rust 直接読み込み**: 2箇所（⚠️ **問題箇所**）
  - `src/mir/loop_builder/if_in_loop_phi_emitter.rs:72,184` - 直接読み込み

**SSOT関数**: ❌ **未定義**（trace専用フラグ）

---

#### 2.5.10 `HAKO_JOINIR_READ_QUOTED` / `HAKO_JOINIR_READ_QUOTED_IFMERGE`

**使用箇所総数**: 8箇所

- **Rust SSOT経由**: 0箇所（❌ **SSOT未定義**）
- **Rust 直接読み込み**: 8箇所（⚠️ **問題箇所**）
  - `src/mir/join_ir_vm_bridge/tests.rs`: 4箇所（テストスキップ判定）
  - `src/mir/join_ir/frontend/ast_lowerer/tests.rs`: 3箇所（テストスキップ判定）
  - `src/mir/join_ir/frontend/ast_lowerer/read_quoted.rs:351` - 条件分岐

**SSOT関数**: ❌ **未定義**（実験フラグ）

---

#### 2.5.11 `HAKO_JOINIR_NESTED_IF`

**使用箇所総数**: 7箇所

- **Rust SSOT経由**: 0箇所（❌ **SSOT未定義**）
- **Rust 直接読み込み**: 7箇所（⚠️ **問題箇所**）
  - `src/mir/join_ir/frontend/ast_lowerer/mod.rs:107,122` - 条件分岐
  - `src/mir/join_ir/frontend/ast_lowerer/tests.rs`: 2箇所（テストスキップ判定）
  - `src/tests/phase41_nested_if_merge_test.rs`: 3箇所（テストスキップ判定）

**SSOT関数**: ❌ **未定義**（実験フラグ）

---

#### 2.5.12 `HAKO_JOINIR_IF_SELECT_DRYRUN`

**使用箇所総数**: 1箇所

- **Rust SSOT経由**: 0箇所（❌ **SSOT未定義**）
- **Rust 直接読み込み**: 1箇所
  - `src/tests/helpers/joinir_env.rs:19` - `std::env::remove_var("HAKO_JOINIR_IF_SELECT_DRYRUN")`

**SSOT関数**: ❌ **未定義**（未使用？）

---

## 3. 直接読み込み問題ファイル一覧（重要！）

### 3.1 NYASH_JOINIR_EXPERIMENT 直接読み込み（39箇所）

| ファイル名 | 行数 | 用途 |
|----------|------|------|
| `src/tests/mir_joinir_funcscanner_append_defs.rs` | 36, 178 | テストスキップ判定 |
| `src/tests/mir_joinir_skip_ws.rs` | 34, 124 | テストスキップ判定 |
| `src/tests/mir_joinir_stage1_using_resolver_min.rs` | 41, 154 | テストスキップ判定 |
| `src/tests/joinir_runner_min.rs` | 15 | テストスキップ判定 |
| `src/tests/joinir_json_min.rs` | 12 | コメント言及 |
| `src/tests/mir_joinir_funcscanner_trim.rs` | 35, 133 | テストスキップ判定 |
| `src/tests/mir_joinir_min.rs` | 25, 144 | テストスキップ判定 |
| `src/tests/mir_joinir_stageb_funcscanner.rs` | 20 | テストスキップ判定 |
| `src/tests/joinir_runner_standalone.rs` | 18, 153 | テストスキップ判定 |
| `src/mir/join_ir_vm_bridge_dispatch/exec_routes.rs` | 11 | コメント言及 |
| `src/tests/helpers/joinir_env.rs` | 20 | ヘルパー関数 remove_var |
| `src/tests/mir_joinir_stageb_body.rs` | 20 | テストスキップ判定 |
| `src/mir/phi_core/loopform_builder.rs` | 81 | コメント言及 |
| `src/mir/join_ir/verify.rs` | 17 | コメント言及 |

### 3.2 HAKO_JOINIR_* 直接読み込み（28箇所）

| ENV名 | ファイル数 | 主な用途 |
|------|----------|---------|
| `HAKO_JOINIR_IF_SELECT` | 2ファイル（13箇所） | テスト用 set/remove_var |
| `HAKO_JOINIR_NESTED_IF` | 3ファイル（7箇所） | テストスキップ判定、条件分岐 |
| `HAKO_JOINIR_READ_QUOTED` | 3ファイル（8箇所） | テストスキップ判定、条件分岐 |
| `HAKO_JOINIR_PRINT_TOKENS_MAIN` | 2ファイル（6箇所） | dev専用フラグ（builder制御） |
| `HAKO_JOINIR_IF_IN_LOOP_TRACE` | 1ファイル（2箇所） | trace専用フラグ |
| `HAKO_JOINIR_IF_IN_LOOP_DRYRUN` | 1ファイル（2箇所） | テスト用 set/remove_var |

### 3.3 Stage-3 直接読み込み（46箇所）

| ENV名 | ファイル数 | 主な用途 |
|------|----------|---------|
| `NYASH_PARSER_STAGE3` | 10ファイル（27箇所） | テスト用 set/remove_var |
| `HAKO_PARSER_STAGE3` | 6ファイル（19箇所） | テスト用 set/remove_var |

---

## 4. 整理優先順位（推奨）

### 🔴 CRITICAL（即座に対応）

#### 4.1 NYASH_JOINIR_EXPERIMENT の SSOT 化
- **問題**: 39箇所の直接読み込み（最多）
- **影響範囲**: 14テストファイル + 3実装ファイル
- **推奨対応**:
  1. `tests/helpers/joinir_env.rs` に以下を追加:
     ```rust
     pub fn enable_joinir_experiment() {
         std::env::set_var("NYASH_JOINIR_EXPERIMENT", "1");
     }
     pub fn disable_joinir_experiment() {
         std::env::remove_var("NYASH_JOINIR_EXPERIMENT");
     }
     pub fn is_joinir_experiment_enabled() -> bool {
         crate::config::env::joinir_experiment_enabled()
     }
     ```
  2. 全テストファイルの `std::env::var("NYASH_JOINIR_EXPERIMENT")` を上記ヘルパーに置換
  3. テストスキップ判定を統一化

---

### 🟠 HIGH（Phase 72 で対応）

#### 4.2 HAKO_JOINIR_* dev/実験フラグの整理
- **対象**: 8種類のdev/実験フラグ（SSOT未定義）
  - `HAKO_JOINIR_NESTED_IF`
  - `HAKO_JOINIR_READ_QUOTED`
  - `HAKO_JOINIR_READ_QUOTED_IFMERGE`
  - `HAKO_JOINIR_PRINT_TOKENS_MAIN`
  - `HAKO_JOINIR_ARRAY_FILTER_MAIN`
  - `HAKO_JOINIR_IF_IN_LOOP_TRACE`
  - `HAKO_JOINIR_IF_TOPLEVEL_TRACE`
  - `HAKO_JOINIR_IF_SELECT_DRYRUN`

- **推奨対応**:
  1. 使用頻度が高いものは SSOT 関数化（例: `HAKO_JOINIR_NESTED_IF`）
  2. trace専用フラグは `HAKO_JOINIR_DEBUG` レベルに統合検討
  3. 未使用フラグは削除検討

---

#### 4.3 HAKO_JOINIR_IF_SELECT の SSOT 完全化
- **問題**: 13箇所の直接読み込み（テスト用）
- **推奨対応**:
  1. `tests/helpers/joinir_env.rs` に以下を追加:
     ```rust
     pub fn enable_joinir_if_select() {
         std::env::set_var("HAKO_JOINIR_IF_SELECT", "1");
     }
     pub fn disable_joinir_if_select() {
         std::env::remove_var("HAKO_JOINIR_IF_SELECT");
     }
     ```
  2. `src/tests/mir_joinir_if_select.rs` の9箇所を置換

---

### 🟡 MEDIUM（Phase 73 で対応）

#### 4.4 Stage-3 legacy ENV の完全削除
- **対象**: `NYASH_PARSER_STAGE3`, `HAKO_PARSER_STAGE3`
- **現状**: deprecation warning 出力中
- **推奨対応**:
  1. Phase 72 完了後、全テストを `NYASH_FEATURES=stage3` に統一
  2. child_env/stage1_bridge の伝播ロジック削減
  3. `config/env.rs` の優先順位ロジック簡素化（`NYASH_FEATURES` のみに）

---

### 🟢 LOW（Phase 74+ で対応）

#### 4.5 NYASH_FEATURES のデフォルト化完了確認
- **現状**: `parser_stage3_enabled()` が既に `true` をデフォルト返却
- **推奨対応**:
  1. スクリプトから `NYASH_FEATURES=stage3` の明示的指定を削減
  2. ドキュメント更新（Stage-3は標準構文であることを明記）

---

## 5. SSOT 化推奨パターン

### 5.1 テストヘルパー統一パターン（`tests/helpers/joinir_env.rs`）

```rust
/// JoinIR 実験モード有効化
pub fn enable_joinir_experiment() {
    std::env::set_var("NYASH_JOINIR_EXPERIMENT", "1");
}

/// JoinIR 実験モード無効化
pub fn disable_joinir_experiment() {
    std::env::remove_var("NYASH_JOINIR_EXPERIMENT");
}

/// JoinIR 実験モード確認（SSOT経由）
pub fn is_joinir_experiment_enabled() -> bool {
    crate::config::env::joinir_experiment_enabled()
}

/// テストスキップ判定の統一化
pub fn skip_unless_joinir_experiment(test_name: &str) {
    if !is_joinir_experiment_enabled() {
        eprintln!(
            "[{}] NYASH_JOINIR_EXPERIMENT=1 not set, skipping test",
            test_name
        );
        return;
    }
}
```

### 5.2 テスト内での使用例

```rust
// Before（❌ 直接読み込み）
#[test]
fn test_joinir_feature() {
    if std::env::var("NYASH_JOINIR_EXPERIMENT").ok().as_deref() != Some("1") {
        eprintln!("[test] NYASH_JOINIR_EXPERIMENT=1 not set, skipping");
        return;
    }
    // テスト本体
}

// After（✅ SSOT経由）
#[test]
fn test_joinir_feature() {
    use crate::tests::helpers::joinir_env::skip_unless_joinir_experiment;
    skip_unless_joinir_experiment("test_joinir_feature");
    // テスト本体
}
```

---

## 6. Phase 72-73 タスクブレークダウン

### Phase 72-A: JoinIR EXPERIMENT SSOT 化
- [ ] `tests/helpers/joinir_env.rs` にヘルパー追加
- [ ] 14テストファイルの直接読み込み置換（39箇所）
- [ ] 実装ファイルのコメント更新（3箇所）

### Phase 72-B: HAKO_JOINIR_IF_SELECT SSOT 化
- [ ] `tests/helpers/joinir_env.rs` にヘルパー追加
- [ ] `src/tests/mir_joinir_if_select.rs` 置換（9箇所）
- [ ] `tests/helpers/joinir_env.rs` 置換（2箇所）

### Phase 72-C: dev/実験フラグ整理
- [ ] 使用頻度調査（各フラグの利用実態確認）
- [ ] SSOT 関数化優先順位決定
- [ ] trace系フラグの統合検討

### Phase 73-A: Stage-3 legacy ENV 削除
- [ ] 全テストを `NYASH_FEATURES=stage3` に統一（46箇所）
- [ ] child_env/stage1_bridge の伝播ロジック簡素化
- [ ] `config/env.rs` の優先順位ロジック削減

### Phase 73-B: スクリプト整理
- [ ] `tools/selfhost/*.sh` の legacy ENV 削除（22箇所）
- [ ] `tools/dev/*.sh` の legacy ENV 削除（15箇所）
- [ ] smokes/v2 での `NYASH_FEATURES=stage3` 簡素化（96箇所）

---

## 7. 検証手順

### 7.1 Phase 72 完了条件
```bash
# 直接読み込みゼロ確認
rg 'std::env::var\("NYASH_JOINIR_EXPERIMENT"\)' --type rust
rg 'std::env::var\("HAKO_JOINIR_IF_SELECT"\)' --type rust

# 期待結果: config/env.rs とテストヘルパーのみ
```

### 7.2 Phase 73 完了条件
```bash
# legacy ENV ゼロ確認
rg 'NYASH_PARSER_STAGE3|HAKO_PARSER_STAGE3' --type rust --type sh
rg 'std::env::var\("NYASH_PARSER_STAGE3"\)' --type rust

# 期待結果: deprecation warning 削除済み
```

---

## 8. 既知の良い例（モデルケース）

### 8.1 `NYASH_JOINIR_DEV`（完璧な SSOT 実装）
- ✅ 全てSSAT経由（`config::env::joinir_dev_enabled()`）
- ✅ 直接読み込みゼロ
- ✅ 優先順位明確（NYASH_JOINIR_DEV > joinir_debug_level()>0）

### 8.2 `HAKO_JOINIR_DEBUG`（完璧な SSOT 実装）
- ✅ レベル制御（0-3）で段階的ログ制御
- ✅ 全てSSAT経由（`config::env::joinir_debug_level()`）
- ✅ 直接読み込みゼロ

### 8.3 `HAKO_JOINIR_IF_IN_LOOP_ENABLE`（完璧な SSOT 実装）
- ✅ 全てSSAT経由（`config::env::joinir_if_in_loop_enable()`）
- ✅ 直接読み込みゼロ
- ✅ dry-run と本番経路の独立制御

---

## 9. まとめと次のアクション

### 現状の問題点
1. **NYASH_JOINIR_EXPERIMENT**: 39箇所の直接読み込み（最多）
2. **HAKO_JOINIR_* dev/実験フラグ**: 8種類がSSAT未定義
3. **Stage-3 legacy ENV**: 46箇所残存（移行完了は96箇所のスクリプトで確認済み）

### 推奨優先順位
1. **CRITICAL**: `NYASH_JOINIR_EXPERIMENT` の SSOT 化（Phase 72-A）
2. **HIGH**: `HAKO_JOINIR_IF_SELECT` の SSOT 化（Phase 72-B）
3. **HIGH**: dev/実験フラグの整理（Phase 72-C）
4. **MEDIUM**: Stage-3 legacy ENV 削除（Phase 73-A/B）

### 期待効果
- ✅ ENV 直接読み込みの根絶（113箇所 → 0箇所）
- ✅ テストヘルパーの統一化（メンテナンス性向上）
- ✅ JoinIR実験→本線移行の加速（トグル管理の簡素化）
- ✅ Stage-3標準化の完了（legacy ENV 完全削除）

---

**次のアクション**: Phase 72-A の実装計画策定にゃ！
Status: Historical
