---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: remove duplicated array-push plumbing in the module generic string body emitter
Related:
  - docs/development/current/main/phases/phase-29cv/P381BR-MODULE-GENERIC-SELECTED-KIND-REGISTRY.md
  - docs/development/current/main/phases/phase-29cv/P381BS-PARSER-PROGRAM-JSON-BODY-EMITTER-BLOCKER.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BT: Module Generic Array Push Helper Cleanup

## Problem

The module generic string body emitter had two copies of the same array-push
plumbing:

- one for generic `ArrayBox.push` when a `generic_method_routes` LoweringPlan row
  exists
- one for static string array bodies while their body-specific push path still
  exists

Both copies parsed the one-argument and two-argument push forms and then promoted
`ArrayBox` origin facts to `ORG_ARRAY_STRING_BIRTH` when the pushed value was a
string handle.

## Decision

Keep the static-array body path for now, but move shared mechanics into helpers:

```text
module_generic_string_read_array_push_value(...)
module_generic_string_promote_array_string_push(...)
```

This is behavior-preserving cleanup. It does not delete the static-array
body-specific emitter path and does not widen push acceptance.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test --release static_string_array -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
