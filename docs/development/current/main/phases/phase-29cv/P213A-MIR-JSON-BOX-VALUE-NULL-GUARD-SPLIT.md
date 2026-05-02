---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P213a, MirJsonEmitBox box-value null guard split
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P211A-MIR-JSON-CONST-VALUE-FIELD-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
---

# P213a: MIR JSON Box-Value Null Guard Split

## Problem

P212a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_box_value/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`_emit_box_value/1` is a string emitter, but it also performs nullable schema
field guards inline:

```hako
if val == null { ... }
if ty_box != null { ... }
if inner_box != null { ... }
if inner == true || ("" + inner) == "true" { ... }
```

Those guards expose void/null sentinel constants directly to the generic string
body classifier. Teaching `generic_string_body.rs` another void-sentinel variant
would grow Stage0 around a source-owner guard shape.

## Decision

Keep `_emit_box_value/1` as the string emitter and move nullable scalar checks
behind small i64 helpers:

```text
_is_present_compat(value) -> 0/1
_is_true_compat(value)    -> 0/1
```

The string body then sees only scalar guard calls and string assembly. The
helpers are ordinary generic i64 bodies; they do not add a new
`GlobalCallTargetShape`, generic map-get acceptance, or C body-specific emitter.

## Non-Goals

- no `generic_string_body.rs` void-sentinel expansion
- no generic `RuntimeDataBox.get` or `MapBox.get` acceptance
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to MIR const-value schema field proof

## Acceptance

Probe result should move past `_emit_box_value/1`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p213a_box_value_null_guard.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
