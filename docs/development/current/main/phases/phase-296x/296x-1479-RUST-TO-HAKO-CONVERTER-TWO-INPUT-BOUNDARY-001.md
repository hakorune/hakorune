# 296x-1479 RUST-TO-HAKO-CONVERTER-TWO-INPUT-BOUNDARY-001

Status: open
Date: 2026-06-20

## Purpose

Define the lifecycle-aware converter input boundary.

This row must not implement converter emission, rustc integration, lifecycle
resolver behavior, or backend behavior.

## Selected By

```text
296x-1478-RUST-TO-HAKO-OWNERSHIP-CONVERTER-TASK-SEQUENCE-001
```

## Target Contract

```text
lifecycle-aware converter input:
  RustSubsetModule-v0
  + verified HakoLifecyclePlan-v0

required verifier result:
  Allow

missing verified plan for lifecycle parity:
  fail-fast

lossy skeleton route:
  still allowed for TODO comments
  must not claim ownership / borrow / move / Drop parity
```

## Acceptance

```text
two_input_boundary_documented=1
missing_verified_plan_fail_fast_documented=1
skeleton_todo_route_separated=1
converter_policy_owner=0
implementation_started=0
converter_emission_started=0
rustc_integration_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_emit_lifecycle_surface=1
do_not_start_rustc_adapter_probe=1
do_not_add_converter_fallback_policy=1
do_not_merge_skeleton_and_lifecycle_routes=1
```
