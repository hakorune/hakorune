---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P229a, generic string collection-or-void PHI refinement
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P228A-MIR-JSON-FUNCTION-FIELD-PROOF.md
  - src/mir/global_call_route_plan/generic_string_facts.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P229a: Generic String Collection-Or-Void PHI

## Problem

P228a proves `_emit_function/1` schema fields, but the source-exe probe still
reports:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_function/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The remaining method blocker is downstream of the schema field proof:

```hako
local blocks = func.get("blocks")
if me._is_map_missing_sentinel(blocks) == 1 { blocks = null }
if blocks == null { ... }
local n = blocks.length()
```

The field proof says `blocks -> Array`, but the missing-sentinel rewrite creates
a PHI of `Array | VoidSentinel`. Without an explicit collection-or-void class,
the generic string dataflow loses the Array fact before `blocks.length()`.

## Decision

Add value-level collection-or-void facts:

```text
ArrayOrVoid
MapOrVoid
```

These are dataflow classes only:

- `Array + VoidSentinel` PHI becomes `ArrayOrVoid`.
- `Map + VoidSentinel` PHI becomes `MapOrVoid`.
- explicit non-void guard PHIs refine `ArrayOrVoid -> Array` and
  `MapOrVoid -> Map`.
- `Eq`/`Ne` comparisons against the void sentinel are allowed.

## Non-Goals

- no generic collection normalizer
- no generic iteration support
- no new route proof vocabulary
- no new `GlobalCallTargetShape`
- no C body-specific emitter

## Acceptance

```bash
cargo test -q collection_or_void --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p229a_collection_or_void.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed next blocker:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_module_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```
