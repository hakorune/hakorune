# BalancedDepthScan: Analysis View SSOT

目的: Phase 107 の `balanced_depth_scan`（array/object の `find_balanced_*`）が selfhost / Stage‑B 由来の AST 形でも揺れないように、**観測（analysis-only view）** を SSOT 化する。

## Contract（SSOT）

- AST rewrite 禁止（見かけ等価の式変形をしない）。必要な観測は analysis-only view に閉じる。
- `+` は可換マッチのみ許可する: `depth + 1` / `1 + depth`。
- `-` は非可換のみ許可する: `depth - 1` のみ受理し、`1 - depth` は拒否する。
- `BlockExpr` は `prelude_stmts + tail_expr` として保守的に観測する。
  - `prelude_stmts` 内に `return/break/continue/if/loop` が含まれる場合は受理しない（fail-fast の対象）。
  - update（`depth = depth ± 1`）検出は **ちょうど1回** のみ許可する（0回/複数回は「形ではない」扱い）。
- `policy` / `join_ir` / `lowering` は、上記の view で得た判定結果（Recipe/view output）だけを参照する（AST 直読の分散禁止）。

## Placement（SSOT）

- analysis-only view の置き場所は中立配置とする: `src/mir/analysis/expr_view.rs`
- `balanced_depth_scan.rs` は `expr_view` を入口として使い、ローカルの ad-hoc matcher を増やさない。

## Gate（SSOT）

- Phase 107 の real-app derived fixtures は、strict/dev で安定に通ることを優先する。
  - `NYASH_JOINIR_DEV=1` または `HAKO_JOINIR_STRICT=1` のとき `LoopCondReturnInBodyFacts` が `balanced_depth_scan` 形を受理する。
  - `HAKO_JOINIR_PLANNER_REQUIRED=1` には依存しない（planner_required は診断用の fail-fast 強化であり、常時必須にはしない）。
  - release default（strict/dev が無効な状態）の挙動は変更しない。
