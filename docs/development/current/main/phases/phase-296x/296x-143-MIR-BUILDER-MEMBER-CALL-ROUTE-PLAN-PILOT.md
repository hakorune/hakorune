---
Status: Landed
Date: 2026-05-28
Scope: pilot member-call route selection separated from emission.
Blocker: MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-142-MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION.md
---

# 296x-143 MIR Builder Member Call Route Plan Pilot

## Purpose

Move member-call lowering from repeated route probes in `build.rs` to a
single route-plan owner. This is a BoxShape cleanup: no new accepted source
shape, no generic CSE, and no .hako source workaround.

## Required Output

```text
output_contract=mir-builder-member-call-route-plan-pilot-v0
input_contract=mir-builder-member-call-route-classification-v0
route_plan_owner
function_preflight_owner
single_eval_surface_ok
small_alloc_helper_copy_probe_ok
summary=ok
```

## Evidence

```text
output_contract=mir-builder-member-call-route-plan-pilot-v0
input_contract=mir-builder-member-call-route-classification-v0
route_plan_owner=src/mir/builder/calls/member_route.rs
function_preflight_owner=src/mir/builder/calls/function_preflight.rs
route_selection_once=1
generic_cse_opened=0
single_eval_surface_ok=1
small_alloc_helper_copy_probe_ok=1
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_builder_member_call_route_plan_pilot_guard.sh
```
