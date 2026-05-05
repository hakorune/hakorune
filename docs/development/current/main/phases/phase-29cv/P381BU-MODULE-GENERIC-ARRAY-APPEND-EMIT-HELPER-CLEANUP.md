---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: remove duplicated array append LLVM call emission in the module generic string body emitter
Related:
  - docs/development/current/main/phases/phase-29cv/P381BT-MODULE-GENERIC-ARRAY-PUSH-HELPER-CLEANUP.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BU: Module Generic Array Append Emit Helper Cleanup

## Problem

After P381BT, the direct `ArrayBox.push` route and the remaining static string
array body path shared argument decoding and origin promotion, but still carried
two copies of the same LLVM call emission:

```text
nyash.array.slot_append_hh(recv, value)
```

Both copies formatted the same argument buffer, emitted the same `dst` and
non-`dst` call shapes, and set the destination type to `T_I64` when a result
register existed.

## Decision

Move only the shared emission mechanics into:

```text
module_generic_string_emit_array_slot_append_call(...)
```

Route acceptance stays outside the helper:

- direct `ArrayBox.push` still requires the LoweringPlan generic-method row
- static string array push still requires the active static-array owner, an array
  receiver, and a string value

This is behavior-preserving cleanup. It does not widen accepted bodies and does
not delete the static-array body-specific path.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test --release static_string_array -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
cargo test --release generic_method_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
