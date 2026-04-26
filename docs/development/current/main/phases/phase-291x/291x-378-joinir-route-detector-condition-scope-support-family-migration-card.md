---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector condition-scope support family migration
Related:
  - src/mir/join_ir/lowering/loop_with_break_minimal.rs
  - src/mir/join_ir/lowering/loop_with_break_minimal/tests.rs
  - src/mir/builder/control_flow/plan/body_local_policy_inputs.rs
  - src/mir/builder/control_flow/plan/loop_break/api/promote_prepare_helpers.rs
  - src/mir/builder/control_flow/plan/body_local_policy_runner.rs
  - src/mir/builder/control_flow/plan/body_local_policy.rs
  - src/mir/builder/control_flow/cleanup/policies/trim_policy.rs
  - src/mir/loop_route_detection/legacy/loop_body_carrier_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_digitpos_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_cond_promoter.rs
  - docs/development/current/main/phases/phase-291x/291x-377-joinir-route-detector-function-scope-compatibility-export-prune-card.md
---

# 291x-378: JoinIR Route Detector Condition-Scope Support Family Migration

## Goal

Migrate condition-scope callers off the parent `loop_condition_scope`
compatibility export.

This is BoxShape-only. Do not remove the parent compatibility export in this
card.

## Change

External callers now use:

```text
loop_route_detection::support::condition_scope
```

Legacy-internal callers now use owner-local paths:

```text
super::loop_condition_scope
```

## Preserved Behavior

- No condition-scope logic changed.
- No compatibility export was removed.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Boundary Improvement

External code depends on the stable semantic support facade, while legacy
internal code no longer routes through the parent compatibility surface.

## Next Cleanup

Verify that `loop_route_detection::loop_condition_scope` has no source callers
and prune the parent compatibility export in a separate card.

## Non-Goals

- No parent compatibility export deletion.
- No physical file move.
- No body-local promoter compatibility export pruning.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
