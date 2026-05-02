---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P208c, consume MIR schema field-read proof in generic i64 body classification
Related:
  - docs/development/current/main/phases/phase-29cv/P208A-MIR-SCHEMA-FIELD-READ-FACT-LOCK.md
  - docs/development/current/main/phases/phase-29cv/P208B-MIR-SCHEMA-FIELD-READ-ROUTE-PROOF.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
---

# P208c: Generic I64 Schema Field Proof Consume

## Problem

P208b emits an exact `generic_method_routes` proof for the MIR JSON numeric
field read in `MirJsonEmitBox._expect_i64/2`, but the generic i64 body
classifier still rejected the raw `RuntimeDataBox.get/1` instruction as an
unsupported method call.

## Decision

Make `generic_i64_body` consume only the exact route proof:

```text
route_proof = mir_json_numeric_value_field
route_kind = runtime_data_load_any
key_const_text = "value"
site = current instruction
```

When the proof is present, the local method-call result is treated as
`StringOrVoid` inside the body classifier. This matches the source shape:

```text
inner = val.get("value")
if inner != null { return StringHelpers.to_i64(inner) }
```

The scalar result remains owned by the later `StringHelpers.to_i64/1` call.
The classifier does not infer this from `RuntimeDataBox.get` itself.

## Non-Goals

- no generic `MapBox.get` or `RuntimeDataBox.get` acceptance
- no C lowering behavior change
- no new `GlobalCallTargetShape`
- no body-specific emitter
- no backend method-name rediscovery

## Result

`MirJsonEmitBox._expect_i64/2` can be classified as `generic_i64_body` through
MIR-owned route metadata while keeping Stage0 size guard intact.

The source-exe probe advances past `_expect_i64/2` and now stops at the next
MIR JSON helper:

```text
target_shape_blocker_symbol=MirJsonEmitBox._expect_map/2
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

## Acceptance

```bash
cargo test -q global_call_routes::generic_i64
cargo test -q generic_method_routes
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p208c_schema_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
