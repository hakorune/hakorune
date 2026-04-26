---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update caller surface inventory
Related:
  - src/mir/builder/control_flow/plan/route_prep_pipeline.rs
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - src/mir/loop_route_detection/features.rs
  - docs/development/current/main/phases/phase-291x/291x-334-joinir-loop-update-doc-comment-contract-card.md
---

# 291x-335: JoinIR Loop-update Caller Surface Inventory

## Goal

Inventory the remaining public/caller surface after the loop-update helper
split.

This card is audit-only. It does not change behavior.

## Findings

Real callers:

```text
RoutePrepContext::is_if_phi_join_pattern(...)
  -> analyze_loop_updates_from_ast(...)

CaseALoweringShape::detect_with_updates(...)
  -> LoopFeatures.update_summary
```

Test helpers build `LoopUpdateSummary` directly for Case-A shape tests.

Dead/reserved caller surface:

```text
RoutePrepContext.loop_update_summary: Option<LoopUpdateSummary>
```

The field is imported, constructed, and initialized to `None`, but no caller
reads it. `is_if_phi_join_pattern(...)` computes the summary locally from
`loop_body` instead of using the field.

## Decision

The next implementation target is:

```text
JoinIR loop-update reserved field prune
```

Implementation boundary:

```text
Remove RoutePrepContext.loop_update_summary and its import/initialization.
Keep analyze_loop_updates_from_ast(...) and LoopFeatures.update_summary.
```

## Non-Goals

- No loop-update classification behavior change.
- No IfPhiJoin route decision change.
- No Case-A shape change.
- No public analyzer API change.

## Acceptance

```bash
cargo test -q route_prep
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
