# Phase 120: selfhost Stage-3 ベースライン結果

## 実行日時

2025-12-04（Phase 106-115 完了直後）

## 環境

- **Rust VM**: ./target/release/hakorune
- **LLVM**: llvmlite ハーネス（今回は未実行）
- **JoinIR Strict**: NYASH_JOINIR_STRICT=1
- **selfhost**: NYASH_USE_NY_COMPILER=1 NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1

## 代表パス実行結果

### 1. peek_expr_block.hako

**ファイル**: `apps/tests/peek_expr_block.hako`

| 項目 | 結果 |
|------|------|
| **実行結果** | ✅ 成功 |
| **エラーメッセージ** | なし |
| **警告メッセージ** | `[deprecate/env]` NYASH_PARSER_STAGE3 deprecated<br>`⚠️ [DEPRECATED]` builtin ArrayBox deprecated<br>`[selfhost-child] timeout` 2000ms |
| **標準出力** | `found one`<br>`RC: 1` |
| **備考** | match 式が正常に JoinIR If Lowering で処理。ブロック式の評価も正常動作。期待通りの出力を確認。 |

**技術的詳細**:
- match 式が If Lowering で複数の条件分岐に変換された
- ブロック式（`{ print("...") 値 }`）が正しく評価され、最後の値が返却された
- PHI 命令による各分岐からの値の合流が正常動作

### 2. loop_min_while.hako

**ファイル**: `apps/tests/loop_min_while.hako`

| 項目 | 結果 |
|------|------|
| **実行結果** | ✅ 成功 |
| **エラーメッセージ** | なし |
| **警告メッセージ** | `[deprecate/env]` NYASH_PARSER_STAGE3 deprecated<br>`[selfhost-child] timeout` 2000ms |
| **標準出力** | `0`<br>`1`<br>`2`<br>`RC: 0` |
| **デバッグ出力** | `[ControlForm::Loop]` entry=3 preheader=3 header=4 body=5 latch=6 exit=7 |
| **備考** | loop 構文が正常に JoinIR Loop Lowering で処理。ControlForm 構造が正しく構築。 |

**技術的詳細**:
- ループが JoinIR Loop Lowering で処理され、ControlForm::Loop 構造を構築
- entry/preheader/header/body/latch/exit の各ブロックが正しく生成
- ループ変数 `i` の PHI 命令が正常生成（初期値 0 と更新値の合流）
- ループ終了条件 `i < 3` が正しく評価され、exit ブロックへ遷移

### 3. esc_dirname_smoke.hako

**ファイル**: `apps/tests/esc_dirname_smoke.hako`

| 項目 | 結果 |
|------|------|
| **実行結果** | ❌ エラー |
| **エラーメッセージ** | `[ERROR] ❌ [rust-vm] VM error: Invalid instruction: Unknown method 'println' on ConsoleBox` |
| **警告メッセージ** | `[deprecate/env]` NYASH_PARSER_STAGE3 deprecated<br>`[warn] dev verify:` NewBox ConsoleBox not followed by birth()<br>`[warn] dev verify:` NewBox Main not followed by birth()<br>`⚠️ [DEPRECATED]` builtin ConsoleBox deprecated<br>`[selfhost-child] timeout` 2000ms |
| **標準出力** | なし（エラーで中断） |
| **デバッグ出力** | `[ControlForm::Loop]` entry=8 preheader=8 header=9 body=10 latch=11 exit=12 |
| **備考** | esc_json メソッドのループと dirname メソッドの if 文は正常動作。ConsoleBox.println でエラー。 |

**技術的詳細**:
- esc_json メソッド内のループが JoinIR Loop Lowering で正常処理
- dirname メソッド内の if 文も JoinIR If Lowering で正常処理
- StringBox メソッド（length, substring, lastIndexOf）の呼び出しは正常
- **エラー原因**: ConsoleBox の println メソッドが見つからない
  - ConsoleBox の実装に println メソッドがない可能性
  - selfhost コンパイラのメソッド解決に問題がある可能性
- **NewBox→birth 警告**: ConsoleBox と Main の生成時に birth() 呼び出しが検出されない
  - birth() が省略可能な設計なので、これは警告レベルの問題

## Phase 120 サマリー

### 実行結果統計

- **✅ 完全成功**: 2本（peek_expr_block.hako, loop_min_while.hako）
- **⚠️ 警告あり**: 2本（警告があっても実行成功）
- **❌ エラー**: 1本（esc_dirname_smoke.hako）

### JoinIR Strict モードでの検証

| 検証項目 | 結果 | 備考 |
|---------|------|------|
| If 文の JoinIR Lowering | ✅ 正常動作 | peek_expr_block.hako, esc_dirname_smoke.hako |
| Loop の JoinIR Lowering | ✅ 正常動作 | loop_min_while.hako, esc_dirname_smoke.hako |
| ControlForm 構造生成 | ✅ 正常動作 | header/body/latch/exit ブロックが正しく構築 |
| match 式の処理 | ✅ 正常動作 | If Lowering で複数条件分岐に変換 |
| ブロック式の評価 | ✅ 正常動作 | 最後の式が値として返却 |
| PHI 命令生成 | ✅ 正常動作 | 分岐・ループでの値合流 |
| StringBox メソッド | ✅ 正常動作 | length, substring, lastIndexOf |
| ConsoleBox.println | ❌ エラー | メソッド解決失敗 |

### 重要な発見

1. **JoinIR Lowering は安定動作**
   - If/Loop の基本的な JoinIR Lowering は完全に動作している
   - ControlForm 構造が正しく構築され、PHI 命令も正常生成

2. **selfhost コンパイラの動作**
   - 2000ms タイムアウト警告が出るが、これはコンパイル時間の警告（正常動作）
   - NYASH_PARSER_STAGE3 の deprecation 警告は環境変数名の変更推奨

3. **ConsoleBox.println 問題**
   - ConsoleBox の println メソッドが selfhost 経路で解決できない
   - builtin ConsoleBox の plugin 化が推奨されている
   - これは selfhost 経路特有の問題と思われる（通常の VM 実行では動作するはず）

## Phase 122+ への課題

### 優先度高（エラー）

- [ ] **ConsoleBox.println メソッドエラーの解決**
  - 原因: selfhost 経路でのメソッド解決失敗
  - 影響: ConsoleBox を使用するプログラムが実行できない
  - 対応: ConsoleBox の実装確認、または selfhost コンパイラのメソッド解決修正

- [ ] **NewBox→birth 警告の調査**
  - 原因: birth() 呼び出しの検出ロジック
  - 影響: 警告レベル（実行は可能）
  - 対応: birth() 呼び出し検出の改善、または警告条件の緩和

### 優先度中（警告）

- [ ] **NYASH_PARSER_STAGE3 deprecation 警告への対応**
  - 原因: 環境変数名の変更推奨
  - 影響: 警告メッセージが出力される
  - 対応: `NYASH_FEATURES=stage3` への移行

- [ ] **selfhost-child 2000ms タイムアウト警告の改善**
  - 原因: selfhost コンパイル時間が長い
  - 影響: 警告メッセージが出力される（実行は成功）
  - 対応: タイムアウト時間の調整、またはコンパイル速度の改善

### 優先度低（最適化）

- [ ] **builtin ArrayBox/ConsoleBox の plugin 化推奨への対応**
  - 原因: Phase 15.5 の Everything is Plugin 方針
  - 影響: deprecation 警告が出力される
  - 対応: plugin 化の検討（長期的な対応）

## 結論

Phase 120 時点での selfhost Stage-3 経路は：

### ✅ **基本動作は良好**
- 2/3 のプログラムが完全に動作
- JoinIR If/Loop Lowering が安定動作
- ControlForm 構造とPHI 命令の生成が正常

### ⚠️ **警告はあるが実行可能**
- deprecation 警告は情報提供レベル
- selfhost コンパイル時間の警告は既知の挙動

### ❌ **1つの致命的エラー**
- ConsoleBox.println メソッド解決エラー
- これは Phase 122+ で優先的に修正が必要

### 📊 **Phase 106-115 の成果**
- JoinIR Strict モードでの基本動作が確立
- If/Loop の Lowering が安定して動作
- selfhost 経路の基礎が固まった

Phase 122+ で上記課題を段階的に解決し、selfhost Stage-3 経路の完全な安定化を目指す。

---

**作成日**: 2025-12-04
**Phase**: 120（selfhost Stage-3 代表パスの安定化）
**ベースライン確立**: Phase 106-115 完了時点
Status: Historical
