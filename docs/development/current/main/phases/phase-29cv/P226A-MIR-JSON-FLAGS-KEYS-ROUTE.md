---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P226a, Map keys helper route for MirJsonEmitBox flags
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P225A-MIR-JSON-FLAGS-REC-ACCESS-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
  - crates/nyash_kernel/src/plugin/map_aliases.rs
---

# P226a: MIR JSON Flags Keys Route

## Problem

P225a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_flags/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The first unsupported method in `_emit_flags/1` is the flags key projection:

```hako
local keys = flags.keys()
```

This is not a new body shape. It is a small missing runtime helper route for an
existing map surface used by canonical MIR JSON emission.

## Decision

Add a narrow `generic_method.keys` route:

```text
proof = mir_json_flags_keys
function = MirJsonEmitBox._emit_flags/1
method = RuntimeDataBox.keys/0
route_kind = map_keys_array
helper = nyash.map.keys_h
return_shape = mixed_runtime_i64_or_handle
```

`generic_string_body` consumes only that proof as:

```text
flags.keys() -> Array
```

The C shim reads the route metadata and emits `nyash.map.keys_h(recv)`. The
runtime helper returns the existing `MapBox.keys()` ArrayBox as a handle.

## Non-Goals

- no generic map iteration semantics in `generic_string_body`
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no rewrite of MIR JSON flags schema

## Acceptance

Probe result should move the first `_emit_flags/1` blocker from method-call
handling to the next unsupported shape:

```bash
cargo test -q flags_keys --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p226a_flags_keys.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed next blocker:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_flags/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```
