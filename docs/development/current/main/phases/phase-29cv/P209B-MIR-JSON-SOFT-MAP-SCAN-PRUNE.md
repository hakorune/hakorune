---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P209b, prune MirJsonEmitBox soft map key-scan fallback
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P209A-MIR-JSON-MAP-ASSERT-SPLIT.md
  - lang/src/shared/mir/json_emit_box.hako
---

# P209b: MIR JSON Soft Map Scan Prune

## Problem

P209a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._map_get_soft_rec/6
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_map_get_soft_rec/6` is a compatibility key-scan fallback for a missing direct
map lookup:

```text
direct = obj.get(key)
if direct is not [map/missing] return direct
scan obj.keys(), compare stringified keys, and retry obj.get(k)
```

Teaching Stage0 this recursive scan would add RuntimeDataBox `keys`/`get`
semantics and a body-specific recursive helper shape. That violates the P207a
size guard: Stage0 should lower explicit MIR routes, not reimplement
MirJsonEmitBox compatibility policy.

## Decision

Keep `_map_get_soft/2` as a thin schema-field lookup helper:

```text
if obj == null return null
return obj.get(key)
```

The runtime missing-key sentinel remains visible to callers, which already
check `_is_map_missing_sentinel(...)` at the schema fallback sites. Exact MIR
schema fields are expected to be present under canonical string keys; if they
are not, the existing sentinel checks choose the local default or fallback key.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic `RuntimeDataBox.keys` or recursive map-scan support
- no generic `MapBox.get` / `RuntimeDataBox.get` acceptance in
  `generic_string_body`
- no C body-specific emitter
- no change to canonical MIR JSON schema

## Acceptance

Probe result should move past `_map_get_soft_rec/6`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p209b_map_soft.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

