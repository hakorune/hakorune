---
Status: Selected
Date: 2026-06-24
Scope: Select the next descriptor-generation owner after extern route need rows.
---

# SAME-MODULE-GLOBAL-CALL-DESCRIPTOR-GENERATION-001

## Decision

Select same-module global-call descriptor generation as the next owner. Do not
merge it with user-box method descriptor generation.

The descriptor lane now has generated generic-method route rows and generated
extern declaration-need rows. The remaining same-module work is broader than one
slice, so split it by route family:

```text
1. same-module global-call descriptor generation
2. same-module user-box method descriptor generation
```

## Target

The Rust global-call planner owns shape and route decisions under:

```text
src/mir/global_call_route_plan.rs
src/mir/global_call_route_plan/**
```

The first same-module descriptor slice should move one C-side global-call
consumer from handwritten route/proof tuples toward generated descriptor data.
It must not infer route meaning from callee spelling, box names, or neighboring
instructions.

## Minimal Slice

```text
source authority:
  global_call_routes / GlobalCallRoute metadata emitted by MIR finalization

first C consumer:
  choose one same-module global-call declaration/need/view consumer

non-target:
  user_box_method_route_plan
  exact seed userbox routes
  generic-method method-view registry
```

The implementation card should name the exact C consumer before code changes.
If more than one consumer must change to stay coherent, stop and split again.

## Acceptance

```text
generated descriptor data owns the selected global-call route fields
handwritten tuple copy for that selected consumer = 0
C shim consumes descriptor rows, not callee-name fallback
unknown/missing descriptor fails closed
same-module userbox method behavior changed = 0
extern descriptor behavior changed = 0
new canonical MIR instruction = 0
runtime fallback = 0
```

## Non-Claims

```text
full same-module descriptor generation = 0
user-box method descriptor generation = 0
exact seed route retirement = 0
global-call planner redesign = 0
same-module fusion plan redesign = 0
```

## Verification Plan

```text
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
focused same-module/global-call guard selected by the implementation card
```
