# 296x-1478 RUST-TO-HAKO-OWNERSHIP-CONVERTER-TASK-SEQUENCE-001

Status: closed
Date: 2026-06-20

## Purpose

Turn the ownership-aware converter answer into concrete follow-up tasks.

This row is docs/tasking only. It must not implement converter emission,
rustc integration, lifecycle resolver behavior, or backend behavior.

## Selected By

```text
296x-1477-POST-LIFECYCLE-FIXTURE-VERIFIER-SKELETON-OWNER-SELECTION-001
```

## Boundary

```text
accepted:
  rustc facts -> HakoLifecyclePlan -> verifier -> converter/emitter

rejected:
  converter reads Rust syntax and directly chooses ownership / borrow / Drop
```

`converter` is the final emission surface. It is not the lifecycle policy
owner.

## Task Sequence

```text
1. RUST-TO-HAKO-CONVERTER-TWO-INPUT-BOUNDARY-001
   define lifecycle-aware converter input as:
     RustSubsetModule-v0
     + verified HakoLifecyclePlan-v0
   missing verified plan for lifecycle parity => fail-fast

2. RUST-TO-HAKO-LIFECYCLE-EMITTER-SURFACE-001
   render one existing verified plan fixture into `.hako`
   no direct Rust syntax ownership decisions

3. RUST-TO-HAKO-LIFECYCLE-PARITY-GATE-001
   compare emitted `.hako` / canonical MIR against the Rust oracle for one
   selected family only

4. RUSTC-SEMIR-LIFECYCLE-FACTS-ADAPTER-PROBE-001
   later external adapter probe
   rustc facts only
   Hako representation policy remains resolver-owned
```

## Acceptance

```text
converter_ownership_boundary_documented=1
task_sequence_recorded=1
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

## Result

```text
converter_ownership_boundary_documented=1
task_sequence_recorded=1
converter_policy_owner=0
implementation_started=0
converter_emission_started=0
rustc_integration_started=0
backend_behavior_changed=0

selected_next_task=RUST-TO-HAKO-CONVERTER-TWO-INPUT-BOUNDARY-001
```

## Stop Line

```text
do_not_add_rust_lifetime_syntax=1
do_not_make_converter_choose_ownership_policy=1
do_not_emit_lifecycle_claim_without_verified_plan=1
do_not_start_rustc_adapter_probe_in_this_row=1
```
