---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector trim support family migration
Related:
  - src/mir/join_ir/lowering/common/condition_only_emitter.rs
  - src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
  - src/mir/join_ir/lowering/carrier_info/types.rs
  - src/mir/builder/control_flow/plan/route_prep_pipeline.rs
  - src/mir/builder/control_flow/plan/trim_lowerer.rs
  - src/mir/builder/control_flow/plan/trim_loop_lowering.rs
  - docs/development/current/main/phases/phase-291x/291x-373-joinir-route-detector-small-compatibility-export-prune-card.md
---

# 291x-374: JoinIR Route Detector Trim Support Family Migration

## Goal

Migrate `TrimLoopHelper` callers to the semantic support owner path.

This is BoxShape-only. Do not remove the parent compatibility export in this
card.

## Change

Migrated source callers from:

```text
loop_route_detection::trim_loop_helper::TrimLoopHelper
```

to:

```text
loop_route_detection::support::trim::TrimLoopHelper
```

## Preserved Behavior

- No Trim helper logic changed.
- No compatibility export was removed.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Boundary Improvement

Trim route support now depends on the stable semantic support facade instead of
the parent legacy-named compatibility export.

## Next Cleanup

Verify that `loop_route_detection::trim_loop_helper` has no source callers and
prune the parent compatibility export in a separate card.

## Non-Goals

- No parent compatibility export deletion.
- No physical file move.
- No migration for function-scope or condition-scope families.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
