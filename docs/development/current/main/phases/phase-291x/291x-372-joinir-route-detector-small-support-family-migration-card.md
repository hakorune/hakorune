---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector small support family migration
Related:
  - src/mir/builder/control_flow/plan/loop_break_condition_policy_router.rs
  - src/mir/builder/control_flow/plan/loop_break_steps/gather_facts_step_box.rs
  - docs/development/current/main/phases/phase-291x/291x-371-joinir-route-detector-support-facade-add-card.md
---

# 291x-372: JoinIR Route Detector Small Support Family Migration

## Goal

Migrate the smallest remaining parent compatibility module callers to stable
`loop_route_detection::support` owner paths.

This is BoxShape-only. Do not remove compatibility exports in this card.

## Change

Migrated:

```text
break_condition_analyzer -> support::break_condition
pinned_local_analyzer -> support::locals::pinned
mutable_accumulator_analyzer -> support::locals::mutable_accumulator
```

Touched callers:

```text
src/mir/builder/control_flow/plan/loop_break_condition_policy_router.rs
src/mir/builder/control_flow/plan/loop_break_steps/gather_facts_step_box.rs
```

## Preserved Behavior

- No support helper logic changed.
- No compatibility export was removed.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Boundary Improvement

Small analysis support families now depend on semantic support owner paths
instead of parent legacy-named compatibility exports.

## Next Cleanup

Inventory whether these compatibility exports now have zero source callers:

```text
break_condition_analyzer
pinned_local_analyzer
mutable_accumulator_analyzer
```

Prune them only in a separate card.

## Non-Goals

- No compatibility export deletion.
- No physical file move.
- No caller migration for larger families.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
