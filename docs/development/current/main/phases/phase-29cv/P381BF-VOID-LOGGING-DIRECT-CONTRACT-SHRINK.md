---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: shrink C-side void-logging direct-call handling toward return/value contract reading
Related:
  - docs/development/current/main/phases/phase-29cv/P381BE-UNIFORM-BODY-EMITTER-CONTRACT-INVENTORY.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BF: Void Logging Direct Contract Shrink

## Problem

P381BE picked `GenericStringVoidLoggingBody` as the first capsule probe because
it has the smallest result-origin surface. The call result has no handle origin
to propagate, but it still has an important contract:

- direct same-module call
- no-result callsite is allowed
- `return_shape = "void_sentinel_i64_zero"`
- `value_demand = "scalar_i64"`

The C side was still reading this as a shape-specific
`generic_string_void_logging_body` predicate in the direct-call shell, selected
planner, and generic module prepass.

## Decision

Shrink the C-side consumer to read the void-sentinel return/value contract
instead of the void-logging target shape.

Implemented:

- added `lowering_plan_global_call_view_is_direct_void_sentinel_i64_zero(...)`
- direct-call shell uses that predicate for no-result validation and selected
  symbol checks
- selected-set planning uses that predicate when recording generic module
  symbols
- generic module body prepass uses that predicate when assigning the scalar
  result type

## Boundary

Allowed:

- move C consumers from the shape-specific predicate to the return/value
  contract
- keep the existing trace consumer string for compatibility
- keep Rust route proof and `GlobalCallTargetShape::GenericStringVoidLoggingBody`
  intact in this card

Not allowed:

- deleting the Rust variant in the same slice
- changing `void_sentinel_i64_zero` semantics
- adding any new `GlobalCallTargetShape`
- changing origin propagation for other capsules

## Result

Done:

- C call/planner/prepass handling no longer needs to ask specifically for
  `generic_string_void_logging_body`
- the unused exact C direct predicate was removed
- the remaining shape-specific references are Rust classification, metadata
  proof allowlisting, and tests

Next:

1. decide whether the Rust-side void-logging classifier can fold into a
   MIR-owned void-sentinel contract without losing diagnostics
2. only then remove `GenericStringVoidLoggingBody` from the shape inventory

## Acceptance

```bash
bash tools/checks/stage0_shape_inventory_guard.sh
cargo test --release void_logging -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
