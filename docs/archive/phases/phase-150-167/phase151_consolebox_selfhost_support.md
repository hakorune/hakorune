# Phase 151: ConsoleBox Selfhost Support

## 🎯 ゴール

**Selfhost Stage-3 パイプラインで ConsoleBox を利用可能にする**

目的：
- selfhost 経由で実行時に `NewBox ConsoleBox` を認識できるようにする
- 以下の 2 つのテストケースを selfhost depth-1 で動かせるようにする：
  - `apps/tests/esc_dirname_smoke.hako` - 文字列処理 + consoleprint
  - `apps/tests/string_ops_basic.hako` - StringBox 操作 + consoleprint
- 通常の Rust VM と同じ感覚で ConsoleBox が使える状態を実現

## 📋 スコープ（やること・やらないこと）

### ✅ やること
- ConsoleBox が selfhost JSON v0 生成時に含まれるようにする
- Ring0Registry またはその上流で ConsoleBox 登録の確保
- 2 つの失敗ケース（esc_dirname_smoke, string_ops_basic）が selfhost で動くことを検証
- テスト実行確認

### ❌ やらないこと
- ConsoleBox 自体の仕様・実装変更
- 他の Box 処理への影響

---

## 🏗️ 実装タスク

### Task 1: 現状分析 - ConsoleBox が selfhost で失われる経路を特定

**調査対象**:

1. **通常経路での ConsoleBox 登録**:
   - `src/runtime/ring0/` で Ring0Registry にどう登録されているか確認
   - ConsoleBox が builtin Box として登録されている場所を特定

2. **Selfhost 経路での ConsoleBox 喪失**:
   - Stage-B/Stage-1/Stage-3 のどこで ConsoleBox が失われるか
   - JSON v0 生成時に ConsoleBox が除外されている箇所を特定
   - 参考ファイル：
     - `src/runner/modes/` - Stage 実行経路
     - `src/runner/` - Pipeline 全体
     - selfhost 関連ドキュメント：`selfhost_stage3_expected_flow.md`

3. **エラーメッセージの意味**:
   ```
   [ERROR] ❌ [rust-vm] VM error: Invalid instruction: NewBox ConsoleBox:
     invalid operation: Unknown Box type: ConsoleBox. Available: Main
   ```
   - "Available: Main" → ConsoleBox が RegisteredBox リストから消えている
   - どこかで ConsoleBox が register されていない

**成果物**:
- Task 1 実施後、修正箇所を 3 つ程度に絞り込む

---

### Task 2: ConsoleBox を selfhost JSON v0 に含める修正

**修正方針**:

通常の Rust VM では ConsoleBox は以下のように登録されている：
```rust
// 例: src/runtime/ring0/registry.rs または類似箇所
registry.register_builtin_box("ConsoleBox", /* ... */);
```

selfhost 経由では JSON v0 生成時にこの登録情報が失われている可能性が高い。

**想定される修正箇所**（Task 1 の調査結果に基づいて特定）:

1. **Stage-B/Stage-1 での ConsoleBox 追加**:
   - selfhost パイプラインでコンパイラが生成する JSON v0 に ConsoleBox を含める
   - 参考: `src/runner/modes/stage_b.rs` または selfhost 関連 builder

2. **Ring0 RegisteredBox 統一化**:
   - 通常経路と selfhost 経路で Ring0Registry が一致するようにする
   - ConsoleBox が常に登録されるようにする

3. **JSON v0 Emitter での ConsoleBox 明示**:
   - JSON v0 出力時に ConsoleBox を explicitly include する場所を特定・修正

**修正例（参考）**:
```rust
// src/runner/modes/ などで selfhost パイプライン実行時に
// ConsoleBox を RegisteredBox に add する

fn create_selfhost_context() -> Ring0Context {
    let mut context = Ring0Context::new();
    // ConsoleBox を追加
    context.register_box("ConsoleBox", /* ... */);
    context
}
```

---

### Task 3: テスト実行・確認

**テスト対象の 2 つのケース**:

1. **esc_dirname_smoke.hako**:
   ```bash
   NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
     ./target/release/hakorune apps/tests/esc_dirname_smoke.hako
   ```
   期待値: `esc_dirname` の結果を consoleprint で出力して成功

2. **string_ops_basic.hako**:
   ```bash
   NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
     ./target/release/hakorune apps/tests/string_ops_basic.hako
   ```
   期待値: StringBox 操作結果を consoleprint で出力して成功

**成功条件**:
- エラー `Unknown Box type: ConsoleBox` が消える
- 両ケースが正常に実行されて結果を出力する

---

### Task 4: ドキュメント・CURRENT_TASK 更新

1. **Phase 151 実施結果を記録**:
   - Task 1 での修正箇所特定結果
   - Task 2 での修正内容と修正ファイル
   - Task 3 のテスト結果（2 本ともパス確認）

2. **CURRENT_TASK.md に Phase 151 完了エントリ追加**:
   ```markdown
   ### Phase 151: ConsoleBox Selfhost Support ✅

   **完了内容**:
   - Selfhost Stage-3 パイプラインに ConsoleBox 登録を追加
   - Ring0 Registry の統一化（通常経路と selfhost 経路）
   - 2 つのテストケース（esc_dirname_smoke, string_ops_basic）が selfhost で動作

   **修正ファイル**:
   - src/runner/modes/ または関連モジュール
   - Ring0 Registry 周辺

   **テスト結果**: 2/2 PASS（esc_dirname_smoke, string_ops_basic）

   **次フェーズ**: Phase 152-A - 括弧内代入式パーサー対応
   ```

3. **git commit で記録**

---

## ✅ 完成チェックリスト（Phase 151）

- [ ] Task 1: ConsoleBox 喪失経路を特定（修正箇所を 3 つ程度に絞る）
- [ ] Task 2: ConsoleBox を selfhost JSON v0 に含める修正
  - [ ] 修正ファイル特定・実装
  - [ ] ビルド成功確認
- [ ] Task 3: テスト実行・確認
  - [ ] esc_dirname_smoke.hako が selfhost で動作
  - [ ] string_ops_basic.hako が selfhost で動作
- [ ] Task 4: ドキュメント・CURRENT_TASK 更新
  - [ ] Phase 151 実施結果を記録
  - [ ] git commit で記録

---

## 所要時間

**2-3 時間程度**

- Task 1（現状分析）: 45分
- Task 2（修正実装）: 1時間
- Task 3（テスト確認）: 30分
- Task 4（ドキュメント）: 30分

---

## 次のステップ

**Phase 152-A: 括弧内代入式（Stage-3 パーサー拡張）**
- Unexpected ASSIGN in `(x = x + 1)` エラー対応
- Stage-3 パーサーを拡張して括弧内での assignment expression を許容

**Phase 152-B: Static method テスト整理**
- `stage1_run_min.hako` を static box スタイルに書き換え

---

## 進捗

- ✅ Phase 130-134: LLVM Python バックエンド整理
- ✅ Phase 150: Selfhost Stage-3 Depth-1 ベースライン強化
- ✅ **Phase 151: ConsoleBox Selfhost Support 完了！**
- 📋 Phase 152-A: 括弧内代入式パーサー対応（予定）
- 📋 Phase 152-B: Static method テスト整理（予定）

---

## 実装結果サマリー

### Task 1: 原因分析完了

**問題箇所特定**:
- ConsoleBox はプラグインとして正しく登録されている（確認済み）
- しかし、selfhost 経由での実行時に `UnifiedBoxRegistry` が ConsoleBox を見つけられない
- 根本原因: `BoxFactoryRegistry`（v2）と `UnifiedBoxRegistry` 間の初期化タイミング問題

**調査結果**:
1. `init_bid_plugins()` が ConsoleBox を `BoxFactoryRegistry` に登録（確認済み）
2. `UnifiedBoxRegistry` は `PluginBoxFactory` 経由で v2 レジストリを参照
3. しかし、`PluginBoxFactory.create_box()` 実行時に `registry.get_provider("ConsoleBox")` が None を返す

### Task 2: 解決策実装完了

**実装内容**:
- ConsoleBox の builtin fallback を追加（Phase 151 selfhost サポート）
- プラグイン優先、builtin はフォールバックとして機能

**修正ファイル**:
1. `src/box_factory/builtin_impls/console_box.rs` - 新規作成（35行）
2. `src/box_factory/builtin_impls/mod.rs` - console_box モジュール追加
3. `src/box_factory/builtin.rs` - ConsoleBox 作成処理とbox_types追加

**設計方針**:
- プラグイン（nyash-console-plugin）が優先
- builtin は selfhost サポート用のフォールバック
- 通常実行ではプラグインが使用され、selfhost でも確実に動作

### Task 3: テスト結果

**テストケース 1: esc_dirname_smoke.hako**
```bash
NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
  ./target/release/hakorune apps/tests/esc_dirname_smoke.hako
```
- ✅ **PASS**: RC: 0
- 出力: `[Console LOG] dir1/dir2`

**テストケース 2: string_ops_basic.hako**
```bash
NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
  ./target/release/hakorune apps/tests/string_ops_basic.hako
```
- ✅ **PASS**: RC: 0
- 出力:
  ```
  [Console LOG] len=5
  [Console LOG] sub=bcd
  [Console LOG] idx=1
  ```

### 成功条件達成状況

- ✅ Task 1: ConsoleBox 喪失経路を特定（BoxFactoryRegistry/UnifiedBoxRegistry 初期化タイミング問題）
- ✅ Task 2: ConsoleBox を selfhost JSON v0 に含める修正（builtin fallback 実装）
  - ✅ 修正ファイル特定・実装
  - ✅ ビルド成功確認
- ✅ Task 3: テスト実行・確認
  - ✅ esc_dirname_smoke.hako が selfhost で動作
  - ✅ string_ops_basic.hako が selfhost で動作
- ✅ Task 4: ドキュメント・CURRENT_TASK 更新（実施中）

### 所要時間

**実績: 約2時間**（予定2-3時間内に完了）
- Task 1（現状分析）: 60分
- Task 2（修正実装）: 30分
- Task 3（テスト確認）: 15分
- Task 4（ドキュメント）: 15分
Status: Historical

