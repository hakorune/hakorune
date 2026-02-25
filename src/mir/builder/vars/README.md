# `mir::builder::vars`（変数/スコープ系の小箱）

このディレクトリは「MIR ビルダ内の変数・スコープ」に関する小さな責務を分離するための層だよ。

## 責務（この層がやる）

- **レキシカルスコープ**: `{...}` / `ScopeBox` の境界で `local` のシャドウイングを復元する。
- **AST 走査ユーティリティ**: free vars 収集など、純粋な走査処理。
- **代入の宣言ポリシー**: 未宣言名への代入を Fail-Fast にする（`AssignmentResolverBox`）。

## 非責務（この層がやらない）

- JoinIR lowering 側の名前解決（`join_ir/lowering/*` の `ScopeManager` が担当）。
- ループパターン/PHI/境界生成（`control_flow/joinir/*` が担当）。
- 言語仕様の追加（この層は既存仕様の実装に限定）。

## スコープ/名前解決の境界（SSOT）

同じ「名前」を扱っていても、層ごとに “解いている問題” が違うので混ぜない。

- **MIR（この層）**: `variable_map` + `LexicalScopeGuard` で「束縛の寿命・シャドウイング」を管理する（SSA 変換のため）。
- **JoinIR lowering**: `src/mir/join_ir/lowering/scope_manager.rs` は JoinIR 内の `name → ValueId` を解決する。
  - `ExprLowerer` は **ScopeManager 経由のみ** で名前解決する（env を直参照しない）。
- **解析箱**: `LoopConditionScopeBox` / `LoopBodyLocalEnv` は「禁止/許可」「スケジュール」などの補助情報で、束縛そのものではない。

この境界を跨ぐ “便利メソッド” を作るのは原則禁止（責務混線の温床）。
