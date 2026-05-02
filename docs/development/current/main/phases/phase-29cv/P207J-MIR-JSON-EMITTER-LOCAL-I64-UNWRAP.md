---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207j, MirJsonEmitBox owner-local i64 field unwrap
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207I-MIR-JSON-EMITTER-DIRECT-ARRAY-LEN.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/shared/mir/json_emit_box.hako
---

# P207j: MIR JSON Emitter Local I64 Unwrap

## Problem

P207i advances the source-exe probe to:

```text
target_shape_blocker_symbol=BoxHelpers.value_i64/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The call is from `MirJsonEmitBox._expect_i64/2`. MIR JSON numeric fields are a
schema-owned emitter concern, but `_expect_i64/2` delegates to the broad common
`BoxHelpers.value_i64/1` wrapper.

## Decision

Keep numeric-field coercion owner-local in `MirJsonEmitBox`:

```text
MapBox { value } -> StringHelpers.to_i64(value)
raw numeric      -> StringHelpers.to_i64(raw)
missing/null     -> 0 with existing dev assert behavior
```

Do not add backend semantics for the common `BoxHelpers.value_i64/1` wrapper in
this card.

## Boundary

This card may only change `MirJsonEmitBox._expect_i64/2`.

It must not:

- change `BoxHelpers.value_i64/1`
- widen `generic_i64_body` or `generic_string_body`
- change MIR(JSON) output schema intentionally
- change generic JSON stringify

## Probe Contract

Before this card:

```text
MirJsonEmitBox._int_str/1 -> MirJsonEmitBox._expect_i64/2 -> BoxHelpers.value_i64/1
tier=Unsupported
target_shape_reason=generic_string_unsupported_method_call
```

After this card, the source-exe probe should no longer stop on
`BoxHelpers.value_i64/1`. It may advance to the next MIR emitter/helper seam.

## Probe Result

After this card, `MirJsonEmitBox._expect_i64/2` no longer calls the broad
`BoxHelpers.value_i64/1` helper. The source-exe first blocker advances to the
owner-local helper itself:

```text
target_shape_blocker_symbol=MirJsonEmitBox._expect_i64/2
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

Route inventory for `_expect_i64/2` now only shows direct scalar callees:

```text
BoxHelpers.is_map/1 -> DirectAbi generic_i64_body
StringHelpers.to_i64/1 -> DirectAbi generic_i64_body
```

The remaining issue is `_expect_i64/2`'s own internal MapBox field read and
should be handled as a separate owner-local shape/source cleanup if it remains
the first blocker.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207j_mir_json_local_i64_unwrap.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
