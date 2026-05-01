---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P133, generic pure string length method acceptance
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P132-GLOBAL-CALL-NULL-GUARD-STRING-BLOCKER.md
  - src/mir/global_call_route_plan.rs
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P133: Global Call String Length Method

## Problem

P132 moved the debug helper blocker past null guards to method calls:

```text
Stage1InputContractBox._debug_len_inline/1
Stage1InputContractBox._debug_preview_inline/1
target_shape_reason=generic_string_unsupported_method_call
```

The first method surface is `RuntimeDataBox.length()` on a value that is already
known to be a string handle. Accepting only this surface should make
`_debug_len_inline/1` lowerable while leaving `_debug_preview_inline/1` blocked
on `substring`.

## Decision

Accept `RuntimeDataBox.length()` / `StringBox.length()` in
`generic_pure_string_body` only when the receiver value is already classified as
string. The method result is an i64.

The backend may emit it only through MIR-owned `generic_method.len` metadata:

```text
core_op=StringLen
route_kind=string_len
symbol=nyash.string.len_h
```

Because `generic_method_routes` need same-module global-call target facts to
prove that a `RuntimeDataBox` receiver originated from a generic string helper,
module semantic refresh replays generic-method routing after module global-call
classification.

## Rules

Allowed:

- `RuntimeDataBox.length()` with a string-class receiver
- `StringBox.length()` with a string-class receiver
- i64 flow from length into generic i64/global/string helper calls

Forbidden:

- accepting `substring`
- accepting by raw backend method name without `generic_method.len` metadata
- treating unknown receivers as string values
- widening this card to generic method fallback or arbitrary `RuntimeDataBox`
  methods

## Expected Evidence

After this card:

```text
Stage1InputContractBox._debug_len_inline/1
  target_shape=generic_pure_string_body
  generic_method.len/StringLen -> nyash.string.len_h

Stage1InputContractBox._debug_preview_inline/1
  target_shape_reason=generic_string_unsupported_method_call
```

The next BoxCount is `substring`, not another null-guard or length blocker.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo test -q generic_method_route` succeeds.
- `cargo test -q semantic_refresh` succeeds.
- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits
  `generic_pure_string_body` for `_debug_len_inline/1` and keeps
  `_debug_preview_inline/1` on `generic_string_unsupported_method_call`.
