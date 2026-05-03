---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P227a, MirJsonEmitBox flags keys null guard
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P226A-MIR-JSON-FLAGS-KEYS-ROUTE.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/generic_string_facts.rs
---

# P227a: MIR JSON Flags Keys Void Guard

## Problem

P226a routes the `_emit_flags/1` key projection through an exact helper route.
The source-exe probe now reaches the next shape in the same function:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_flags/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

The unsupported site is the null guard after the exact keys projection:

```hako
local keys = flags.keys()
if keys == null { return "{}" }
```

This is a handle null-guard compare. It is not collection iteration semantics
and should not introduce a new body shape.

## Decision

Allow `Eq`/`Ne` comparison between collection handle classes and the explicit
void/null sentinel inside the existing generic string value facts:

```text
Array <=> VoidSentinel
Map   <=> VoidSentinel
```

This keeps the boundary at value classification:

- `generic_method.keys` remains the only new helper route from P226a.
- `generic_string_body` only accepts the null guard as a boolean compare.
- no ArrayBox/MapBox iteration policy is added.
- no new `GlobalCallTargetShape` is added.
- no C body-specific emitter is added.

## Acceptance

The unit fixture must prove that `_emit_flags/1` accepts a `keys == null`
guard after the exact keys route:

```bash
cargo test -q flags_keys --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p227a_flags_keys_void_guard.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed next blocker:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_function/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```
