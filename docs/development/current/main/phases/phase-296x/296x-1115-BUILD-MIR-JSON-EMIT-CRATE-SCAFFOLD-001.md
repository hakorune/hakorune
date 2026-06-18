Status: Done
Date: 2026-06-18
Scope: scaffold hakorune-mir-json-emit crate for JSON-ready DTO serialization
Related:
  - docs/development/current/main/phases/phase-296x/296x-1114-BUILD-MIR-JSON-DTO-SERIALIZER-CLOSEOUT-001.md
  - crates/hakorune_mir_json_emit
  - src/runner/mir_json_export_model.rs

# BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001

## Purpose

Create the future MIR JSON serializer crate without changing the main crate
export route.

## Change

```text
new_crate=hakorune-mir-json-emit
new_crate_scope=json_ready_dto_serializer_only
new_crate_reads_mir_directly=0
main_crate_dependency_added=0
main_crate_behavior_changed=0
json_output_changed=0
```

The scaffold duplicates the current DTO model and serializer as public crate
API. Main-crate wiring is intentionally deferred to the next row so this row
stays a pure crate-boundary scaffold.

## Verification

```text
cargo_test_hakorune_mir_json_emit=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-emit-crate-scaffold-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
main_crate_dependency_added=0
new_crate_reads_mir_directly=0
direct_mir_json_emit_crate_extraction_selected=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-EMIT-CRATE-FACADE-WIRING-001
purpose=wire main-crate mir_json_export_model facade to the new crate API
```
