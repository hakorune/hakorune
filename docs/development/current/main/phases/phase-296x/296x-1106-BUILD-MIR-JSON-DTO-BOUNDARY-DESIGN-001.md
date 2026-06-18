Status: Done
Date: 2026-06-18
Scope: define MIR-agnostic DTO boundary for MIR JSON emission
Related:
  - docs/development/current/main/phases/phase-296x/296x-1105-BUILD-MIR-JSON-EXPORT-MODEL-CLOSEOUT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit

# BUILD-MIR-JSON-DTO-BOUNDARY-DESIGN-001

## Purpose

Define the DTO layer required before `src/runner/mir_json_emit` can move toward
a crate. Direct extraction is blocked because the emitter still reads
`crate::mir` and `FunctionMetadata` directly.

## Decision

```text
dto_boundary_required=1
projection_owner=main_crate
serialization_owner=future_hakorune_mir_json_emit_crate
future_crate_reads_mir_directly=0
```

The DTO boundary is JSON-ready data. It is not a MIR view and must not expose
`MirModule`, `MirFunction`, `MirInstruction`, `FunctionMetadata`, or route plan
structs to the future serialization crate.

## DTO Shape

```text
MirJsonExportDocument
  schema
  root_kind
  root_metadata_surfaces
  functions

MirJsonExportFunction
  name
  params
  blocks
  metadata_surfaces
  attrs

MirJsonExportBlock
  id
  instructions

MirJsonExportInstruction
  payload
```

`payload` is intentionally `serde_json::Value` in the first scaffold. This keeps
instruction serialization stable while the boundary is introduced. Typed
instruction DTOs are deferred until the direct MIR dependency count is lower and
the active emitter families are mapped.

## Stop Lines

```text
do_not_create_future_crate_yet=1
do_not_move_emitters_yet=1
do_not_export_mir_types_from_dto=1
do_not_change_json_schema=1
do_not_change_runner_route=1
do_not_move_cfg_extractor=1
```

## Next

```text
next_task=BUILD-MIR-JSON-DTO-SCAFFOLD-001
purpose=add passive DTO vocabulary in src/runner/mir_json_export_model.rs
implementation_scope=passive_data_only
```

## Contract

```text
output_contract=build-mir-json-dto-boundary-design-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
future_crate_created=0
mir_json_payload_changed=0
direct_crate_extraction_selected=0

summary=ok
```
