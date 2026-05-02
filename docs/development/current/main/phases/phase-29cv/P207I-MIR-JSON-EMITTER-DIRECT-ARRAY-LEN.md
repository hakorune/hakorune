---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207i, MirJsonEmitBox direct ArrayBox length flow
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207H-LOWER-IF-COMPARE-FOLD-VARINT-RESOLVE-I64.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/shared/mir/json_emit_box.hako
---

# P207i: MIR JSON Emitter Direct Array Length

## Problem

P207h advances the source-exe probe to:

```text
target_shape_blocker_symbol=BoxHelpers.array_len/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The blocker is in `MirJsonEmitBox`: MIR arrays such as `functions`, `blocks`,
`instructions`, `params`, and `effects` are passed through the generic
`BoxHelpers.array_len/1` wrapper. That wrapper is intentionally broad and must
handle MapBox-wrapped length values, but this makes the MIR string emitter
depend on an unknown-receiver method-call wrapper.

## Decision

Do not add unknown `.length()` semantics to Stage0 and do not widen the common
`BoxHelpers.array_len/1` helper.

Inside `MirJsonEmitBox`, use direct `.length()` on MIR arrays that are already
guarded or schema-owned by the emitter:

```text
local n = funcs.length()
local n = blocks.length()
local n = insts.length()
```

This keeps the MIR emitter on its own schema facts and avoids routing through
the broad common helper.

## Boundary

This card may only replace `BoxHelpers.array_len(...)` calls in
`lang/src/shared/mir/json_emit_box.hako` with direct array length reads.

It must not:

- change `BoxHelpers.array_len/1`
- add generic unknown `.length()` lowering
- widen `generic_i64_body` or `generic_string_body`
- change MIR(JSON) output schema intentionally
- alter generic JSON stringify

## Probe Contract

Before this card:

```text
MirJsonEmitBox.to_json/1 -> BoxHelpers.array_len/1
tier=Unsupported
target_shape_reason=generic_string_unsupported_method_call
```

After this card, the source-exe probe should no longer stop on
`BoxHelpers.array_len/1`. It may advance to the next MIR emitter wrapper seam.

## Probe Result

After this card, the source-exe probe advances to:

```text
target_shape_blocker_symbol=BoxHelpers.value_i64/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

That is the next broad common-helper wrapper inside `MirJsonEmitBox._int_str/1`.
It should be handled separately as a MIR emitter numeric-field contract cleanup,
not by widening Stage0 in this card.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207i_mir_json_direct_array_len.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
