---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv selected-set first slice for same-module function emission in ny-llvmc
Related:
  - docs/development/current/main/phases/phase-29cv/P381AT-UNIFORM-MULTI-FUNCTION-EMITTER-GAP-PLAN.md
  - docs/development/current/main/phases/phase-29cv/P381AU-STRINGLEN-FAST-HELPER-SPLIT.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_leaf_function_emit.inc
---

# P381AV: Selected-set First Slice

## Problem

The Stage0 side already had the beginnings of a selected-set plan:

- `plan_module_generic_string_definitions()` walked lowering-plan edges from the
  active entry root and followed same-module direct-call edges transitively
- `emit_generic_pure_string_function_definition()` already checked
  `module_generic_symbol_will_be_defined(name)`

But two whole-module behaviors were still left in place:

- numeric leaf definitions were emitted by scanning every function
- generic string/direct-function bodies were also emitted by scanning every
  function and letting late guards skip most of them

That left Stage0 more whole-module-shaped than the selected-set contract
intended.

## Decision

Land the smallest owner-correct slice:

1. record `typed_global_call_leaf_numeric_i64` targets in the planner
2. only emit numeric leaf definitions when the symbol was planned
3. iterate generic body emission from the planned symbol set, not from
   `program.functions`

This keeps `.inc` as plan-reader/emitter glue. It does not add new body
semantics, source-name switches, or broader shape inference.

## Implemented

- `planned_module_leaf_symbols[]` registry on the Stage0 side
- `module_leaf_symbol_will_be_defined(name)` guard for leaf definitions
- lowering-plan scan now records direct `numeric_i64_leaf` targets
- generic body emission now iterates `planned_module_generic_string_symbols[]`
  instead of walking every module function

## Boundary

Allowed:

- selected-set tightening for same-module definition emission
- planner-owned symbol recording based on existing lowering-plan metadata

Not allowed:

- widening Stage0 body understanding
- reintroducing whole-module semantic scanning
- changing runtime attrs or StringLen ownership

## Acceptance

```bash
cargo fmt --all
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Result

Done:

- selected-set planning now covers direct numeric leaf targets
- leaf definitions and generic same-module bodies no longer rely on a whole-module
  scan to discover what to emit

Next:

1. narrow module-function declarations so unselected same-module functions do not
   stay implicitly externalizable
2. then retire temporary `GlobalCallTargetShape::*Body` capsules
3. then finish the delete-last compat sweep
