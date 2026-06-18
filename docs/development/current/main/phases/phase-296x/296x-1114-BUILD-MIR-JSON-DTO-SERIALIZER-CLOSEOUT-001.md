Status: Done
Date: 2026-06-18
Scope: close MIR JSON DTO serializer seam and select crate-split scaffold
Related:
  - docs/development/current/main/phases/phase-296x/296x-1113-BUILD-MIR-JSON-DTO-SERIALIZER-RETURN-SWITCH-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-DTO-SERIALIZER-CLOSEOUT-001

## Purpose

Close the serializer seam now that `build_mir_json_root` returns the DTO
serializer payload, then select the next crate-split row.

## Audit

```text
serializer_seam_closed=1
serializer_payload_returned_from_root_builder=1
serializer_reads_mir_directly=0
root_projection_still_main_crate=1
mir_json_emit_direct_mir_reference_count=378
direct_mir_json_emit_crate_extraction_selected=0
future_crate_created=0
```

The direct `src/runner/mir_json_emit` extraction remains blocked because the
projection layer still walks `MirModule`, MIR instructions, function metadata,
and root-level plan collectors. The split-safe seam is the JSON-ready DTO model
plus serializer.

## Decision

```text
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001
future_crate_package_name=hakorune-mir-json-emit
future_crate_scope=json_ready_dto_serializer_only
future_crate_reads_mir_directly=0
projection_owner=main_crate
serialization_owner=future_hakorune_mir_json_emit_crate
```

`hakorune-mir-json-emit` means "emit/serialize MIR JSON DTO payloads"; it does
not mean the crate may read MIR directly.

## Contract

```text
output_contract=build-mir-json-dto-serializer-closeout-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
direct_mir_json_emit_crate_extraction_selected=0
future_crate_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001

summary=ok
```
