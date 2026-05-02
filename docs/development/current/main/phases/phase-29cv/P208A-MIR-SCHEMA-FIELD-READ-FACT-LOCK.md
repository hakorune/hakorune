---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P208a, MIR schema field-read fact boundary for MirJsonEmitBox numeric unwrap
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207J-MIR-JSON-EMITTER-LOCAL-I64-UNWRAP.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_plan/model.rs
---

# P208a: MIR Schema Field-Read Fact Lock

## Problem

P207j moved MIR JSON numeric coercion out of the broad
`BoxHelpers.value_i64/1` helper and into `MirJsonEmitBox._expect_i64/2`.
The source-exe probe now stops on the owner-local helper:

```text
target_shape_blocker_symbol=MirJsonEmitBox._expect_i64/2
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The remaining unsupported instruction is the schema-owned field projection:

```hako
local inner = val.get("value")
if inner != null { return StringHelpers.to_i64(inner) }
```

In MIR this is not a clean direct `MapBox.get` surface. The observed callee is
`RuntimeDataBox.get` with union certainty, so accepting it by box/method name
would widen Stage0 into generic runtime-data/map semantics.

## Decision

Do not add a new body shape for `MirJsonEmitBox._expect_i64/2`.
Do not add generic `MapBox.get` or `RuntimeDataBox.get` acceptance to
`generic_string_body` / `generic_i64_body`.

Instead, represent this as an exact MIR-owned field-read route fact carried by
the existing generic-method route and LoweringPlan pipeline:

```text
RuntimeDataBox.get(key="value")
  + schema proof = mir_json_numeric_value_field
  + return_shape = scalar_i64_or_missing_zero
  + value_demand = scalar_i64
  -> explicit LoweringPlan/CoreMethod MapGet entry
```

This keeps the owner boundary:

```text
MIR/generic_method_routes prove the exact schema field read.
LoweringPlan carries the backend contract.
ny-llvmc emits the plan.
generic body classifiers do not rediscover map/runtime-data semantics.
```

## Acceptance Boundary

The fact is valid only when all of these are true:

- the call site is a `get/1` method call
- the receiver surface is `RuntimeDataBox`
- the call belongs to `MirJsonEmitBox._expect_i64/2`
- the key is the static string literal `"value"`
- the result is consumed by `StringHelpers.to_i64/1` in the same helper
- the route proof is `mir_json_numeric_value_field`

The fact must not accept:

- arbitrary `RuntimeDataBox.get`
- arbitrary `MapBox.get`
- arbitrary static string keys
- source-box-name based lowering in ny-llvmc
- a new `GlobalCallTargetShape`
- a new body-specific C emitter

## Implementation Order

```text
P208a:
  document this boundary and update CURRENT_STATE

P208b:
  extend generic_method_routes with the exact proof
  add route/JSON fixture coverage
  keep backend behavior unchanged if possible

P208c:
  consume the exact proof in global-call body classification/lowering
  keep C-side lowering plan-driven, not method-name driven
```

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
