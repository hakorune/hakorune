---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P214a, MirJsonEmitBox phi incoming array proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P213A-MIR-JSON-BOX-VALUE-NULL-GUARD-SPLIT.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/runner/mir_json_emit/emitters/phi.rs
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P214a: MIR JSON PHI Incoming Array Proof

## Problem

P213a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_phi_incoming_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_phi_incoming_rec/3` emits canonical MIR JSON PHI incoming pairs. The
Rust MIR JSON emitter writes this schema as array pairs:

```json
"incoming": [[value_id, block_id], ...]
```

The Hako emitter still carries a map fallback:

```hako
if BoxHelpers.is_array(item) {
  value_id = item.get(0)
  block_id = item.get(1)
} else {
  block_id = item.get("block")
  value_id = item.get("value")
}
```

Keeping both shapes forces Stage0 to understand general dynamic collection
reads inside a string body.

## Decision

Keep the canonical array-pair schema and remove the map fallback from this
helper. Then add exact MIR-owned generic method proofs for the remaining reads:

```text
proof = mir_json_phi_incoming_array_item
  values.get(idx) -> Array item

proof = mir_json_phi_incoming_pair_scalar
  item.get(0|1) -> scalar i64
```

`generic_string_body` consumes only those exact proofs by site. This avoids
generic `RuntimeDataBox.get` acceptance and does not add a new
`GlobalCallTargetShape`.

## Non-Goals

- no generic map-get acceptance
- no generic array-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to Rust canonical PHI incoming JSON schema

## Acceptance

Probe result should move past `_emit_phi_incoming_rec/3`; a later blocker may
remain:

```bash
cargo test -q proves_mir_json_phi_incoming_array_get_routes --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p214a_phi_incoming.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
