---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P134, generic pure string substring method acceptance
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P133-GLOBAL-CALL-STRING-LENGTH-METHOD.md
  - src/mir/global_call_route_plan.rs
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P134: Global Call String Substring Method

## Problem

P133 made `_debug_len_inline/1` lowerable. The next same helper chain still
blocked on:

```text
Stage1InputContractBox._debug_preview_inline/1
target_shape_reason=generic_string_unsupported_method_call
RuntimeDataBox.substring(0, 64)
```

The receiver is already a string-class value, but it reaches the method through
a same-module global call and a PHI.

## Decision

Accept `RuntimeDataBox.substring(i64, i64)` / `StringBox.substring(i64, i64)` in
`generic_pure_string_body` only when the receiver is already classified as a
string value and both bounds are classified as i64. The method result is a
string handle.

The backend may emit it only through MIR-owned `generic_method.substring`
metadata:

```text
core_op=StringSubstring
route_kind=string_substring
symbol=nyash.string.substring_hii
```

Generic method routing now has a conservative string-flow proof for
same-module generic pure string global calls through copy/PHI values, so a
`RuntimeDataBox.substring` receiver can be proven without backend-local method
name inference.

## Rules

Allowed:

- `RuntimeDataBox.substring(i64, i64)` with a string-class receiver
- `StringBox.substring(i64, i64)` with a string-class receiver
- copy/PHI propagation from a `generic_pure_string_body` global-call result into
  the substring receiver proof

Forbidden:

- one-argument substring in this generic pure string body capsule
- accepting unknown receiver or bound types
- accepting other string methods
- backend emission from raw `box_name.method` strings without
  `generic_method.substring` metadata

## Expected Evidence

After this card:

```text
Stage1InputContractBox._debug_preview_inline/1
  target_shape=generic_pure_string_body
  generic_method.substring/StringSubstring -> nyash.string.substring_hii
```

The remaining stage1 source-execution blockers should move past the preview
substring helper.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo test -q generic_method_route` succeeds.
- `cargo test -q semantic_refresh` succeeds.
- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits
  `generic_pure_string_body` for `_debug_preview_inline/1` and includes
  `generic_method.substring/StringSubstring` metadata for its substring call.
