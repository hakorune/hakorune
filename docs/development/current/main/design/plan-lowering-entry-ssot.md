---
Status: SSOT
Scope: CorePlan → MIR の emission 入口（PlanLowerer entry）
Related:
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
- docs/development/current/main/design/recipe-tree-and-parts-ssot.md
- src/mir/builder/control_flow/plan/lowerer/core.rs
- tools/checks/no_unapproved_plan_lowerer_entry.sh
---

# PlanLowerer entry SSOT

目的: `CorePlan` の MIR emission 入口を固定し、`CoreEffectPlan/CorePlan/AST 直lower` の混線を増やさない。

## Rule (single entry contract)

- `PlanLowerer::lower(...)` の production callsite は **allowlist のみ**。
- 追加の callsite を作る場合は、**同コミットで**この SSOT と drift check allowlist を更新する。
- `lower_with_stack` / `emit_effect*` は lowerer 内部実装として扱い、外部入口として使わない。

## Allowlist (2026-02-06)

- `src/mir/builder/control_flow/joinir/patterns/router.rs`
- `src/mir/builder/control_flow/joinir/patterns/registry/handlers.rs`
- `src/mir/builder/stmts/return_stmt.rs`

例外理由:
- `return_stmt.rs` は match-return 最適化で、verified CorePlan を直接 lower する既存入口。

## Drift check

- Script: `tools/checks/no_unapproved_plan_lowerer_entry.sh`
- Fast gate 組み込み: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- 受け入れ基準: allowlist 外の `PlanLowerer::lower(...)` callsite が 0 件。

## Non-goals

- `CorePlan`/`CoreEffectPlan` 語彙そのものの変更。
- release 挙動の変更。
- AST rewrite による回避。
