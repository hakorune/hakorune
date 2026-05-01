---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: retire `HAKO_CAPI_PURE` as a route selector.
Related:
  - docs/development/current/main/phases/phase-29cv/P96-HAKO-CAPI-PURE-ALIAS-RETIREMENT-INVENTORY.md
  - docs/development/current/main/phases/phase-29cv/P99-HAKO-CAPI-PURE-ALIAS-WARNING.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/reference/environment-variables.md
  - crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs
  - lang/c-abi/shims/hako_llvmc_ffi.c
  - lang/c-abi/shims/hako_llvmc_ffi_common.inc
  - lang/c-abi/shims/hako_llvmc_ffi_route.inc
  - src/config/env/llvm_provider_flags.rs
  - tools/dev/phase29ck_boundary_historical_alias_probe.sh
---

# P100 HAKO_CAPI_PURE Alias Fail-Fast

## Goal

Remove `HAKO_CAPI_PURE=1` as an active route selector after P97-P99 moved
active callers to the canonical backend recipe spelling and locked warning
observability.

The canonical spelling is:

```text
HAKO_BACKEND_COMPILE_RECIPE=pure-first
```

## Decision

- C generic FFI export fails fast when `HAKO_CAPI_PURE=1` is present.
- `ny-llvmc` boundary driver fails fast when `HAKO_CAPI_PURE=1` is present.
- Rust env helper still emits the P99 deprecation warning, but no longer treats
  the alias as a pure-first request.
- The historical alias probe becomes a retired-alias fail-fast probe.

## Stable Failure

```text
[freeze:contract][env/hako_capi_pure_retired] HAKO_CAPI_PURE is retired; use HAKO_BACKEND_COMPILE_RECIPE=pure-first
```

## Non-goals

- no compat replay behavior change
- no default backend recipe change
- no pure-first export removal
- no C shim route widening

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q -p nyash-rust --lib config::env::llvm_provider_flags
cargo test -q -p nyash-llvm-compiler boundary_driver_ffi
bash tools/dev/phase29ck_boundary_historical_alias_probe.sh
bash tools/dev/phase29ck_boundary_explicit_compat_probe.sh
set +e; HAKO_CAPI_PURE=1 target/release/ny-llvmc --driver boundary \
  --in apps/tests/mir_shape_guard/ret_const_min_v1.mir.json \
  --out /tmp/p100_alias_failfast.o >/tmp/p100_alias_failfast.log 2>&1; test "$?" -ne 0; \
  grep -F "[freeze:contract][env/hako_capi_pure_retired]" /tmp/p100_alias_failfast.log; \
  rm -f /tmp/p100_alias_failfast.o /tmp/p100_alias_failfast.log
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
