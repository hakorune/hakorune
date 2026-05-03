---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P296a, no-dst void logging global call emission
Related:
  - docs/development/current/main/phases/phase-29cv/P295A-BUILDER-REGISTRY-DISPATCH-CONTRACT-READER.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P296a: Void Logging Global Call No-Dst Emit

## Problem

After P295a, the source-execution probe advances into
`BuilderRegistryAuthorityBox.try_lower/1` and fails during body emission:

```text
reason=module_generic_body_emit_failed
target_shape_blocker_symbol=BuilderRegistryAuthorityBox.try_lower/1
target_shape_blocker_reason=-
```

The first failing body site is a planned void logging call:

```text
site=b11220.i1
callee=BuilderRegistryAuthorityBox._debug_registry_hit/1
proof=typed_global_call_generic_string_void_logging
target_shape=generic_string_void_logging_body
return_shape=void_sentinel_i64_zero
result_value=null
dst=null
```

The route metadata is already explicit. The C global-call shell rejects it
because it currently requires every DirectAbi global call to have a result
register.

## Decision

Allow no-dst emission only for the existing
`typed_global_call_generic_string_void_logging` route.

Emit the same direct function call without assigning its `i64` void-sentinel
return value. Keep all value-returning DirectAbi global calls on the existing
`result_value == dst` contract.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new body-specific emitter
- no generic no-dst global-call acceptance
- no debug policy change
- no fallback or externalization

## Acceptance

- `BuilderRegistryAuthorityBox._debug_registry_hit/1` no-dst callsites emit
  through the existing void logging route.
- value-returning global calls still require `result_value == dst`.
- `BuilderRegistryAuthorityBox.try_lower/1` advances to the next blocker or
  emits successfully.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

The no-dst `_debug_registry_hit/1` calls now emit through the existing
`typed_global_call_generic_string_void_logging` route. The source-execution
probe advances past `BuilderRegistryAuthorityBox.try_lower/1` body emission and
returns to the next prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=-
```

No value-returning global-call contract was relaxed; those still require
`result_value == dst`.
