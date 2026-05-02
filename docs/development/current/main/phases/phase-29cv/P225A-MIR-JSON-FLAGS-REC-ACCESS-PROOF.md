---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P225a, MirJsonEmitBox flags recursion access proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P224A-MIR-JSON-PARAMS-ARRAY-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P225a: MIR JSON Flags Recursion Access Proof

## Problem

P224a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_flags_rec/4
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_flags_rec/4` has two schema-local accesses:

```hako
local k = keys.get(idx)
local v = flags.get(k)
```

These are one MIR function-flags recursion shape:

- `keys.get(idx)` projects a string key from the explicit key array.
- `flags.get(k)` projects the flag value for that already-projected key.

This is not general `RuntimeDataBox.get` support.

## Decision

Add one exact MIR-owned proof vocabulary for the two access sites:

```text
proof = mir_json_flags_rec_access
function = MirJsonEmitBox._emit_flags_rec/4
method = RuntimeDataBox.get/1
```

Route kind remains site-specific:

```text
keys.get(idx) -> array_slot_load_any -> string
flags.get(k) -> runtime_data_load_any -> string-or-void
```

The second site is recognized only when the key operand originates from a
method `get` result in the same function, which keeps the dynamic map read tied
to this flags-recursion shape instead of widening map semantics.

## Non-Goals

- no generic array-get widening for `RuntimeDataBox`
- no generic map-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no MIR function flags schema change

## Acceptance

Probe result should move past `_emit_flags_rec/4`; a later blocker may remain:

```bash
cargo test -q proves_mir_json_flags_rec_access_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p225a_flags_rec.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
