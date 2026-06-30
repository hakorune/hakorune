# 1886 - MIRBUILDER-LOOP-COND-UTILITY-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-UTILITY-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected LoopCondUtility projection-policy cluster.

The selected subcluster contains two diagnostic tag helpers:

```text
direct_exit_reject(error_prefix: &str, reason: DirectExitRejectReason) -> String
is_direct_exit_reject(err: &str) -> bool
```

These helpers encode and recognize loop-condition direct-exit rejection
diagnostics. They are owned by the LoopCond utility surface and do not open a
standalone Hako projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_loop_cond_utility_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-utility-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_utility_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::loop_cond_utility
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 2
symbols:
  direct_exit_reject
  is_direct_exit_reject

markers:
  DirectExitRejectReason
  DIRECT_EXIT_REJECT_TAG
  direct_exit_reason_text
  Backward-compatible fallback for legacy message shapes.
  BlockContainsDirectExit
  ExitMustBeInsideIf
  BreakMustBeLast
  ReturnMustBeLast
```

## Acceptance

```text
policy = KeepParentOwner
projection_surface_selected = 0
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no standalone Hako projection surface
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
