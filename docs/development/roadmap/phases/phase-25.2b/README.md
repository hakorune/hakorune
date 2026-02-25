# Phase 25.2b — Lambda / FunctionBox Semantics (Planning)

Status: planning-only（将来フェーズ用のメモ／実装はまだ行わない）

## ゴール

- Phase 25.1b で selfhost builder の Program→MIR パリティが整ったあとに、Hakoru­ne 言語としてのラムダ式（`fn(x, y) { ... }` / `fn(x) expr`）の意味論を Rust VM / MIR / FunctionBox まで一貫させる。
- MirBuilder / selfhost コードから暫定的に排除している `fn` ベースの helper（`norm_if` など）を、正式なクロージャ意味論の上に戻せるようにする。
- Stage1/Stage0 の責務分離を崩さずに、「lambda を使う Hako コード」が hv1 VM / AOT でも安全に実行できる状態を作る。

## 現状（Phase 25.1b 前提）

- 構文／AST:
  - `docs/reference/language/LANGUAGE_REFERENCE_2025.md` に `fn(x, y) { ... }` / `fn(x) expr` のラムダ構文が記載済み。
  - Stage‑3 パーサは `fn` キーワードを認識し、AST 上では Lambda ノードを持っている（`exprs_lambda.rs` 前提）。
- MIR:
  - `src/mir/instruction.rs` に `MirInstruction::NewClosure { dst, params, body, captures, me }` が定義済み。
  - `src/mir/builder/exprs_lambda.rs` は AST ラムダを `NewClosure` に降ろし、`captures` / `me` 情報を構築する実装がある。
- 実行（VM）:
  - hv1 VM（`src/backend/mir_interpreter`）は `NewClosure` / `Callee::Closure` をまだ実装していない（catch-all で InvalidInstruction）。
  - `FunctionBox` / `ClosureEnv` は `src/boxes/function_box.rs` に存在し、手動で FunctionBox を作るテスト経路では動いている。
- selfhost 側:
  - Phase 25.1a/b では selfhost builder から `fn` を排除し、lambda なしでも Stage1 CLI を扱える Program→MIR ルートに集中する。

## 25.2b のスコープ（案）

- Rust VM:
  - `MirInstruction::NewClosure` を実装し、`FunctionBox` + `ClosureEnv` を構築する。
  - `execute_callee_call` に `Callee::Closure` / `Callee::Value` の処理を追加し、第一級関数呼び出しをサポートする。
- Hakorune selfhost:
  - selfhost builder (`MirBuilderBox` / `MirBuilderMinBox`) の helper の一部を lambda 版に戻し、`NewClosure` 経路が実際に踏まれるようにする（最初は dev トグル付きでもよい）。
  - Stage1 コードでの lambda 利用ポリシー（どの層で許可するか）を docs に明記する。
- テスト:
  - AST→MIR→VM で λ を含むケース（単純関数、captures、`me` 利用など）をカバーする canary を追加する。

このフェーズは「selfhost builder パリティ（25.1b）」完了後に着手する前提であり、本ドキュメントは計画メモのみとする。***

