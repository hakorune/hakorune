---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: add warn-once signal for retired `HAKO_CAPI_PURE` alias use.
Related:
  - docs/development/current/main/phases/phase-29cv/P96-HAKO-CAPI-PURE-ALIAS-RETIREMENT-INVENTORY.md
  - docs/development/current/main/phases/phase-29cv/P98-HAKO-CAPI-PURE-ALIAS-PROBE-SPLIT.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/reference/environment-variables.md
  - crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs
  - lang/c-abi/shims/hako_llvmc_ffi_common.inc
  - src/config/env/llvm_provider_flags.rs
---

# P99 HAKO_CAPI_PURE Alias Warning

## Goal

After P98, `HAKO_CAPI_PURE=1` has one active behavior owner:

```text
tools/dev/phase29ck_boundary_historical_alias_probe.sh
```

P99 keeps that behavior working, but makes any live alias use visible through a
stable warn-once signal:

```text
[deprecate/env] 'HAKO_CAPI_PURE' is deprecated; use 'HAKO_BACKEND_COMPILE_RECIPE=pure-first'
```

## Decision

- Emit the warning from the C shim when the generic FFI export observes the
  alias.
- Emit the same warning from Rust env SSOT when caller-side recipe selection
  observes the alias.
- Emit the same warning from the `ny-llvmc` boundary driver when it observes the
  alias during FFI symbol selection.
- Do not change route selection in this card.
- Do not fail-fast yet; P100 decides fail-fast/no-op deletion after this warning
  has a locked behavior probe.

## Non-goals

- no alias branch deletion
- no compat replay policy change
- no backend acceptance widening
- no new env var or debug toggle

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q -p nyash-rust --lib config::env::llvm_provider_flags
cargo test -q -p nyash-llvm-compiler boundary_driver_ffi
bash tools/dev/phase29ck_boundary_historical_alias_probe.sh
bash tools/dev/phase29ck_boundary_explicit_compat_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
