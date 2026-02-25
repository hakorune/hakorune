---
Status: SSOT (design-only)
Scope: ValueFlow (SSA merge) representation SSOT for Stage-B / JoinIR lowering
Related:
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
- docs/development/current/main/design/compiler-task-map-ssot.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
---

# ValueFlow SSOT: BlockParams + edge_args (design-only)

## Purpose

Selfhost Stage-B で観測される dominance 違反（`mir/verify:dominator_violation`）を「原因側」に寄せるために、
**値の合流(ValueFlow)の表現SSOT**を固定する。

このドキュメントは設計SSOTであり、挙動変更・受理拡張は行わない（design-only）。

## Terminology

- **BlockParams**: block (basic block) が受け取る引数（BlockArgs 相当）。このリポジトリでは `BlockParams` を用語SSOTとする。
- **edge_args**: terminator（Branch/Jump など）の edge に乗る引数（BlockParams に渡す値の列）。
- **ValueFlow**: predecessor -> join の「値の流し」。merge を意味論として表す層。
- **Materialize**: ValueFlow（BlockParams/edge_args）を、実装詳細として PHI/Copy に落とす行為。

## Scope (design-only)

- ValueFlow（合流/merge）の意味論表現は **BlockParams + edge_args に一本化**する。
- PHI / predecessor-Copy は **materialize の実装詳細**として扱い、ValueFlow のSSOTではない。
- Verifier は唯一の受理ゲートであり、壊れたIRを「救済」しない（検出のみ）。

## Non-goals

- 受理拡張（BoxCount）は行わない。
- AST rewrite（見かけ等価の式変形）は行わない。
- release 挙動を変えない（strict/dev の fail-fast 追加は別タスク）。

## Invariants (SSOT)

1) **Join/merge で、predecessor 由来の値を直接参照しない**
   - join 側は BlockParams（受け取り値）だけを参照する。

2) **predecessor -> join の値渡しは edge_args で表す**
   - pred 側は `jmp join(args=...)` / `br ... then_edge_args=... else_edge_args=...` の形式で渡す。

3) **PHI/Copy は “ValueFlow を materialize した結果” に限定**
   - merge の意味論を PHI/Copy に持たせない（混線防止）。

4) **Verifier は修正しない**
   - dominance を破る IR は transform のバグとして検出し、strict/dev では fail-fast に寄せる。

## Stage-B contract note (json_v0_bridge)

Stage-B の JSON v0 bridge では、変数スナップショット漏れが dominance 違反/無効な PHI 入力に繋がりやすい。
そのため、`get_variable_at_block(name, pred_bb)` は **strict/dev + planner_required のときに fallback（現在値 `self.vars`）を禁止**
する（観測SSOTとして固定）。

- File: `src/runner/json_v0_bridge/lowering/loop_.rs`
- Rule: strict/dev + planner_required → `get_variable_at_block(...) == None`（fallback禁止）
- Non-goal: これ自体で受理拡張はしない（欠落は “原因側” で可視化されるべき）

## Drift checks (design-only)

- ValueFlow が terminator に集約されているか:
  - `rg -n "\\bedge_args\\b|\\bBlockParams\\b" src/mir/ src/mir/builder/ src/mir/control_flow/edgecfg/`
- merge の意味論を Copy で救済していないか（残骸検出の入口）:
  - `rg -n "non_dominating_copy|merge.*Copy|emit_merge_copies" src/`

## Planned migration (task decomposition; design-only)

移行は “1ファイル=1コミット” で進める（BoxShape）。

1) bridge 層の direct Copy を禁止し、ValueFlow（edge_args/BlockParams）へ寄せる
2) materialize（PHI/Copy）責務を Lower/Emitter 側へ集約する
3) schedule/local_ssa の “ブロック外救済” を廃止し、ValueFlow/Verifier に責務を戻す
