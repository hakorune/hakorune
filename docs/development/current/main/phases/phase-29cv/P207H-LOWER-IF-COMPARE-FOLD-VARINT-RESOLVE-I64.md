---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207h, LowerIfCompareFoldVarIntBox numeric side resolver return shape
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207G-MIR-JSON-EMITTER-NAME-SPLIT.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/mir/builder/internal/lower_if_compare_fold_varint_box.hako
---

# P207h: LowerIfCompareFoldVarInt Resolve I64

## Problem

P207g removed the generic `JSON.stringify` transitive blocker. The next
source-exe probe stops at:

```text
target_shape_blocker_symbol=LowerIfCompareFoldVarIntBox._resolve_side/3
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

`LowerIfCompareFoldVarIntBox.try_lower/1` uses `_resolve_side/3` only as a
numeric side resolver for `If(Compare(...))` lowering:

```text
local lhs = me._resolve_side(...)
local rhs = me._resolve_side(...)
return IfMirEmitBox.emit_compare_ret2(lhs, rhs, op_sym, tv, ev)
```

But `_resolve_side/3` currently mixes return families:

```text
Int    -> JsonFragBox.read_int_after(...)   # numeric token string
Binary -> _fold_bin_varint(...)             # numeric token string
Var    -> PatternUtilBox.find_local_int_before(...) # i64-like value
miss   -> null
```

That makes the body look like a string-return helper even though the caller
needs scalar numeric values.

## Decision

Keep this as source-flow cleanup. Do not add a new body shape.

Make the resolver contract explicit:

```text
_resolve_side/3       -> i64 or null
_fold_bin_varint/3    -> i64 or null
```

Coerce numeric-token strings at the resolver boundary with the existing
`_coerce_i64_compat` helper.

## Boundary

This card may only change
`lang/src/mir/builder/internal/lower_if_compare_fold_varint_box.hako`.

It must not:

- change compare lowering semantics
- widen `generic_string_body`
- add a dedicated body shape for this resolver
- change `JsonFragBox.read_int_after/2`
- change `PatternUtilBox.find_local_int_before/3`

## Probe Contract

Before this card:

```text
LowerIfCompareFoldVarIntBox.try_lower/1 -> LowerIfCompareFoldVarIntBox._resolve_side/3
tier=Unsupported
target_shape_reason=generic_string_return_not_string
target_return_type=i64
```

After this card, `_resolve_side/3` should classify as an i64/null scalar helper
or stop at the next narrower blocker. The source-exe probe may still fail on a
later MIR emitter/helper seam.

## Probe Result

After this card, the source-exe probe no longer reports
`LowerIfCompareFoldVarIntBox._resolve_side/3` as the first transitive blocker.
It advances to:

```text
target_shape_blocker_symbol=BoxHelpers.array_len/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The refreshed route inventory still shows a residual local ABI limitation for
`LowerIfCompareFoldVarIntBox.try_lower/1 -> _resolve_side/3`:

```text
target_shape_reason=generic_string_return_abi_not_handle_compatible
```

That residual is not the current source-exe first blocker after this card. If
it resurfaces after the MIR JSON emitter `array_len` seam is cleaned up, handle
it as a separate boundary instead of widening Stage0 in this card.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207h_lower_if_compare_fold_varint_resolve_i64.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
