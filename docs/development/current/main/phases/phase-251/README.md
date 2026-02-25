Status: Active
Scope: Phase 251 (JoinIR 条件変数抽出の回帰修正 + 出力のクリーンアップ)
Related:
- docs/development/current/main/10-Now.md
- docs/reference/environment-variables.md

# Phase 251 Fix: json_lint_vm 回帰（ConditionEnv 変数抽出 / 出力ノイズ）

## 目的

- 回帰の修正（`json_lint_vm` で `arr.length()` 等の基底変数が ConditionEnv に入らず落ちる）
- smoke 出力を壊す無条件ログ（`eprintln!`）の除去

## 実装（完了）

### 1) 条件変数抽出の拡張（核心）

ファイル:
- `src/mir/join_ir/lowering/condition_var_extractor.rs`

問題:
- 例: `loop (j < valid.length())` の `valid` が抽出されず、ConditionEnv で `Variable 'valid' not found` になる

原因:
- `collect_variables_recursive()` が `MethodCall` 等の複合式を辿れていなかった

対応:
- `collect_variables_recursive()` に以下の AST ノード処理を追加し、基底と引数側を再帰収集する
  - `MethodCall`: `arr.length()` から `arr` を抽出（引数も再帰）
  - `FieldAccess`: `obj.count` から `obj` を抽出
  - `Index`: `arr[i]` から `arr` と `i` を抽出
  - `Call`: callee と arguments を再帰（関数参照・引数の変数を拾う）

### 2) デバッグ出力のクリーンアップ

ファイル:
- `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`

問題:
- 無条件 `eprintln!` が quick smoke の期待出力を壊す

対応:
- `crate::config::env::is_joinir_debug()` ガードで条件付き出力に変更
- 推奨 env: `HAKO_JOINIR_DEBUG=1`（`NYASH_JOINIR_DEBUG` は legacy）

### 3) ユニットテスト追加

ファイル:
- `src/mir/join_ir/lowering/condition_var_extractor.rs`

追加:
- `MethodCall/FieldAccess/Index` などの変数抽出が期待通りであることを固定（回帰防止）

## 検証（Phase 251 の範囲）

- 元の回帰（`arr.length()` の `arr` が ConditionEnv に入らない）: 解決
- `--profile quick` の `json_lint_vm`: 別件の失敗が露出（Phase 251 対象外）

## 残タスク（次の指示書 / Phase 252 案）

### 現象

- JoinIR Pattern2 の break 条件 lowering が `MethodCall` を扱えず失敗する
  - 例: `Me.is_whitespace(s.substring(i, i + 1))` のような `MethodCall` を含む条件

### 指示（Claude 実装用）

1. 再現コマンド
   - `./tools/smokes/v2/run.sh --profile quick`
   - 追加ログが必要なら `HAKO_JOINIR_DEBUG=1`（smoke 期待出力に混ざるため、比較用の runs では OFF にする）

2. 構造的な修正方針
   - 「条件 lowering 専用の箱」側で `ASTNode::MethodCall`（必要なら `Me` / `Call` / `FieldAccess` / `Index`）を fail-fast で受理できるようにする
   - 実装は 2 ルートのどちらかに統一する
     - A) 条件式を事前に式 lowering（ANF を含む）で `ValueId` に落としてから branch 用の bool 値として扱う
     - B) 受理範囲を明確化し、MethodCall を KnownIntrinsic に限定して lowering（逸脱はエラー）

3. ドキュメントとテスト（コードの前）
   - `condition_lowering_box` の責務（受理する AST の境界）を README / doc comment で明文化
   - `MethodCall` を含む break 条件を最小 fixture で固定し、v2 smoke に追加（quick 既定に入れるかは要検討）

4. 受け入れ基準
   - `./tools/smokes/v2/run.sh --profile quick` が緑
   - デフォルトで出力が汚れない（`HAKO_JOINIR_DEBUG=1` の時だけ追加ログ）
   - by-name や特定関数名での分岐など、対処療法的ハードコードを追加しない

