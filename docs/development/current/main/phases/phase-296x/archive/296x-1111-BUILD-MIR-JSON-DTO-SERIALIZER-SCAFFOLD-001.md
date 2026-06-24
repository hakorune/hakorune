Status: Done
Date: 2026-06-18
Scope: scaffold pure serializer from MIR JSON DTO to payload
Related:
  - docs/development/current/main/phases/phase-296x/296x-1110-BUILD-MIR-JSON-DTO-SERIALIZER-DESIGN-001.md
  - src/runner/mir_json_export_model.rs

# BUILD-MIR-JSON-DTO-SERIALIZER-SCAFFOLD-001

## Purpose

Add the pure serializer seam that a future crate can own. The serializer reads
only `MirJsonExportDocument` and returns `serde_json::Value`.

## Change

```text
serializer_function=mir_json_export_model::serialize_document
serializer_input=MirJsonExportDocument
serializer_output=serde_json::Value
serializer_reads_mir_directly=0
root_builder_wired_to_serializer=0
json_output_changed=0
future_crate_created=0
```

The serializer currently preserves the existing legacy and v1 root shapes using
JSON-ready payload fields already stored in the DTO.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-dto-serializer-scaffold-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
future_crate_created=0
serializer_reads_mir_directly=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-DTO-SERIALIZER-PARITY-WIRING-001
purpose=debug-assert serializer output matches existing root payload before returning it
```
