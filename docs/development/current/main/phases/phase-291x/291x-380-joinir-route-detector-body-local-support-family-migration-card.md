---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector body-local support family migration
Related:
  - src/mir/builder/control_flow/plan/body_local_policy_runner.rs
  - src/mir/builder/control_flow/plan/body_local_policy_inputs.rs
  - src/mir/builder/control_flow/plan/trim_loop_lowering.rs
  - src/mir/loop_route_detection/legacy/loop_body_cond_promoter.rs
  - docs/development/current/main/phases/phase-291x/291x-379-joinir-route-detector-condition-scope-compatibility-export-prune-card.md
---

# 291x-380: JoinIR Route Detector Body-Local Support Family Migration

## Goal

Migrate body-local promoter callers off the remaining parent compatibility
exports.

This is BoxShape-only. Do not remove the parent compatibility exports in this
card.

## Change

External callers now use:

```text
loop_route_detection::support::body_local::carrier
loop_route_detection::support::body_local::condition
```

Legacy-internal carrier promoter access now uses:

```text
super::loop_body_carrier_promoter
```

## Preserved Behavior

- No body-local promoter logic changed.
- No compatibility export was removed.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Boundary Improvement

Body-local route support now depends on semantic support owner paths externally
and owner-local paths internally.

## Next Cleanup

Verify that these parent exports have no source callers and prune them:

```text
loop_body_carrier_promoter
loop_body_cond_promoter
```

## Non-Goals

- No parent compatibility export deletion.
- No physical file move.
- No support facade removal.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
