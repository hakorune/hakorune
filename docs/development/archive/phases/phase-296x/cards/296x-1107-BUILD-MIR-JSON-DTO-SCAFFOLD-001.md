Status: Done
Date: 2026-06-18
Scope: scaffold passive MIR JSON DTO vocabulary
Related:
  - docs/development/current/main/phases/phase-296x/296x-1106-BUILD-MIR-JSON-DTO-BOUNDARY-DESIGN-001.md
  - src/runner/mir_json_export_model.rs

# BUILD-MIR-JSON-DTO-SCAFFOLD-001

## Purpose

Add passive JSON-ready DTO vocabulary before wiring projection code. This keeps
the future serialization crate boundary MIR-agnostic while preserving current
payload behavior.

## Change

```text
new_vocabulary=MirJsonExportDocument,MirJsonExportFunction,MirJsonExportBlock,MirJsonExportInstruction,MirJsonExportSurface
instruction_payload_type=serde_json::Value
metadata_surface_payload_type=serde_json::Value
projection_wired=0
json_output_changed=0
future_crate_created=0
```

The DTO intentionally stores JSON-ready payloads rather than MIR instructions or
MIR metadata structs. Typed instruction DTOs remain deferred.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-dto-scaffold-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
future_crate_created=0
dto_reads_mir_directly=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-DTO-ROOT-PROJECTION-WIRING-001
purpose=construct the passive DTO in build_mir_json_root without changing emitted JSON
```
