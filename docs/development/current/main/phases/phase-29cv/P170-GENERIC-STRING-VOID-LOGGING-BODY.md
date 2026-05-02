---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P170, generic string void logging helper
Related:
  - docs/development/current/main/phases/phase-29cv/P169-GENERIC-STRING-INDEXOF-METHOD.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P170: Generic String Void Logging Body

## Problem

After P169, the source-execution probe advances to:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox.log_fail/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

`BuilderFinalizeChainBox.log_fail/1` is not a string-return helper. It is a
void-return logging helper that builds string messages, calls `print`, may read
`env.get/1`, and returns the void sentinel.

## Decision

Add a dedicated lowerable target shape:

```text
target_shape=generic_string_void_logging_body
proof=typed_global_call_generic_string_void_logging
return_shape=void_sentinel_i64_zero
value_demand=scalar_i64
```

The accepted body must:

- have a `void` return signature
- use handle-compatible parameters
- contain a string surface and at least one supported `print` call
- return only `void` / `null` sentinel values
- stay within the existing generic string body subset for string concat,
  `env.get/1`, control flow, and supported backend global calls

This card does not widen string-or-void return semantics and does not accept
general void helpers without logging evidence.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_void_logging_string_body --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p170_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`BuilderFinalizeChainBox.log_fail/1` is now direct:

```text
target_shape=generic_string_void_logging_body
proof=typed_global_call_generic_string_void_logging
return_shape=void_sentinel_i64_zero
value_demand=scalar_i64
tier=DirectAbi
```

The probe advances to:

```text
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

Treat `_decode_escapes/1` as the next card. Do not fold its method surface into
the void logging helper shape.
