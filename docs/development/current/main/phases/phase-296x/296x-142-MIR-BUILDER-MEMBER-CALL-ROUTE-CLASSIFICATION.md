---
Status: Landed
Date: 2026-05-28
Scope: classify the member-call route ownership before same-module helper call lowering.
Blocker: MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-141-SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE.md
---

# 296x-142 MIR Builder Member Call Route Classification

## Purpose

Freeze the MIR builder owner boundary for member-call route selection before
touching same-module helper call lowering. Row 141 showed the active small-alloc
surface is not a broad acceptance gap; it is concentrated around facade
result/state helpers and their receiver/local-SSA copy chain.

## Required Output

```text
output_contract=mir-builder-member-call-route-classification-v0
input_contract=hako-mimalloc-small-alloc-helper-copy-family-probe-v0
route_owner
emit_owner
same_module_helper_call_lowering_allowed
generic_cse_opened=0
summary=ok
```

## Boundary

```text
Allowed:
  - separate member-call route selection from emission
  - keep static receiver / env method / this-me normalization as explicit routes
  - preserve single-evaluation boundaries

Not allowed:
  - generic MIR CSE
  - source-level facade wrapper inline as a keeper
  - widening behavior without a row guard
```

## Evidence

```text
output_contract=mir-builder-member-call-route-classification-v0
input_contract=hako-mimalloc-small-alloc-helper-copy-family-probe-v0
route_owner=src/mir/builder/calls/member_route.rs
emit_owner=src/mir/builder/calls/member_route.rs
source_preflight_owner=src/mir/builder/calls/function_preflight.rs
static_receiver_classifier=src/mir/builder/calls/static_resolution.rs
env_method_classifier=src/mir/builder/calls/extern_calls.rs
this_me_classifier=src/mir/builder/calls/receiver_binding.rs
same_module_helper_call_lowering_allowed=route_plan_pilot_only
generic_cse_opened=0
selected_next=mir_builder_member_call_route_plan_pilot
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_builder_member_call_route_classification_guard.sh
```
