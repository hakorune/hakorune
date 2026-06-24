Status: Done
Date: 2026-06-18
Scope: close hakorune-mir-json-emit crate split and select post-split measurement
Related:
  - docs/development/current/main/phases/phase-296x/296x-1116-BUILD-MIR-JSON-EMIT-CRATE-FACADE-WIRING-001.md
  - crates/hakorune_mir_json_emit
  - src/runner/mir_json_export_model.rs
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-MIR-JSON-EMIT-CRATE-CLOSEOUT-001

## Purpose

Close the JSON-ready MIR export serializer crate split.

## Result

```text
new_crate=hakorune-mir-json-emit
new_crate_scope=json_ready_dto_serializer_only
main_crate_facade=src/runner/mir_json_export_model.rs
serialization_owner=hakorune_mir_json_emit
projection_owner=main_crate
json_output_changed=0
new_crate_reads_mir_directly=0
direct_mir_json_emit_crate_extraction_selected=0
```

The split intentionally did not move `src/runner/mir_json_emit/**` projection
logic. That code still owns MIR walking and remains in the main crate until a
separate input-view boundary exists.

## Decision

```text
selected_next_task=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
reason=crate_split_landed_and_should_be_measured_before_next_boundary
next_boundary_selection_blocked_until_measurement=1
```

## Contract

```text
output_contract=build-mir-json-emit-crate-closeout-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
new_crate_reads_mir_directly=0
next_task=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001

summary=ok
```
