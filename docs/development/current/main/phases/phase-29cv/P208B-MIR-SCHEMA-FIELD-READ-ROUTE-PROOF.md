---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P208b, exact generic-method route proof for MirJsonEmitBox numeric value field reads
Related:
  - docs/development/current/main/phases/phase-29cv/P208A-MIR-SCHEMA-FIELD-READ-FACT-LOCK.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_plan/model.rs
  - src/runner/mir_json_emit/root.rs
---

# P208b: MIR Schema Field-Read Route Proof

## Problem

P208a locked the boundary for `MirJsonEmitBox._expect_i64/2`:

```text
MapBox.get / RuntimeDataBox.get must not become generally accepted.
The MIR JSON numeric wrapper field read must be exact-site metadata.
```

The route metadata did not yet have an exact proof for that schema-owned field
projection.

## Decision

Extend the existing `generic_method_routes` / `LoweringPlan` path instead of
adding a new body shape:

```text
route_proof = mir_json_numeric_value_field
route_kind = runtime_data_load_any
core_op = MapGet
tier = ColdRuntime
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
publication_policy = no_publication
key_const_text = "value"
```

The proof is emitted only when:

- the current function is `MirJsonEmitBox._expect_i64/2`
- the method surface is `RuntimeDataBox.get/1`
- the key literal is exactly `"value"`
- the result value flows through copy/PHI edges into `StringHelpers.to_i64/1`

## Non-Goals

- no `generic_i64_body` / `generic_string_body` widening in this card
- no C lowering behavior change in this card
- no generic static-string `MapGet` acceptance
- no new `GlobalCallTargetShape`
- no new body-specific emitter

## Result

The MIR JSON route surface now has enough MIR-owned metadata for P208c to
consume the exact proof without reclassifying method names in the backend.

## Acceptance

```bash
cargo test -q generic_method_routes
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
