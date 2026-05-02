---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P164, generic i64 string ordered compare
Related:
  - docs/development/current/main/phases/phase-29cv/P163-MODULE-GENERIC-FAILURE-DIAGNOSTIC.md
  - docs/development/current/main/phases/phase-29cv/P144-GLOBAL-CALL-STRING-SCAN-I64-BODY.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P164: Generic I64 String Ordered Compare

## Problem

After P162/P163 and a rebuilt C FFI boundary, the source-execution probe reaches
the inactive emit-mir branch in `main`:

```text
first_block=14533
first_inst=2
reason=missing_multi_function_emitter
target_shape_blocker_symbol=StringHelpers.to_i64/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

`StringHelpers.to_i64/1` is a narrow i64 scanner. Its unsupported operation is
not `to_i64` by name; the body uses one-character string range checks:

```text
ch < "0"
ch > "9"
```

## Decision

Extend `generic_i64_body` by one accepted expression shape:

- string `Lt` / `Gt` compare between string-class values
- result class is bool
- emission uses the existing runtime helper `nyash.string.lt_hh`

This keeps the acceptance in the generic i64 scanner capsule. It does not add a
special route for `StringHelpers.to_i64/1`, does not accept arbitrary helper
names, and does not change the emit-mir authority boundary.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_string_ordered_compare_generic_i64_body --lib
cargo test -q generic_i64 --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p164_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe, not a full
green gate.

## Result

The `--emit-exe` probe advances past `StringHelpers.to_i64/1` after the C FFI
boundary is rebuilt. The next blocker is a separate generic string body method
call shape:

```text
target_shape_blocker_symbol=StringScanBox.read_char/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

Treat that as the next card; do not fold it into P164.
