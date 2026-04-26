---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector function-scope support family migration
Related:
  - src/mir/control_tree/normalized_shadow/available_inputs_collector.rs
  - src/mir/join_ir/lowering/scope_manager.rs
  - src/mir/join_ir/lowering/loop_with_break_minimal/header_break_lowering.rs
  - src/mir/builder/control_flow/plan/condition_env_builder.rs
  - src/mir/builder/control_flow/plan/loop_break_prep_box.rs
  - src/mir/builder/control_flow/plan/loop_break_steps/gather_facts_step_box.rs
  - docs/development/current/main/phases/phase-291x/291x-375-joinir-route-detector-trim-compatibility-export-prune-card.md
---

# 291x-376: JoinIR Route Detector Function-Scope Support Family Migration

## Goal

Migrate `function_scope_capture` callers to the semantic support owner path.

This is BoxShape-only. Do not remove the parent compatibility export in this
card.

## Change

Migrated source callers from:

```text
loop_route_detection::function_scope_capture
```

to:

```text
loop_route_detection::support::function_scope
```

## Preserved Behavior

- No capture analysis logic changed.
- No compatibility export was removed.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Boundary Improvement

Function-scope capture now depends on the stable semantic support facade instead
of the parent legacy-named compatibility export.

## Next Cleanup

Verify that `loop_route_detection::function_scope_capture` has no source
callers and prune the parent compatibility export in a separate card.

## Non-Goals

- No parent compatibility export deletion.
- No physical file move.
- No migration for condition-scope or body-local families.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
