Status: Done
Date: 2026-06-18
Scope: design serializer boundary from MIR JSON DTO to payload
Related:
  - docs/development/current/main/phases/phase-296x/296x-1109-BUILD-MIR-JSON-DTO-CLOSEOUT-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-DTO-SERIALIZER-DESIGN-001

## Purpose

Define the serializer seam that a future `hakorune-mir-json-emit` crate can
own. The serializer must consume `MirJsonExportDocument` and produce the same
payload shape as the current root builder, without reading MIR.

## Decision

```text
serializer_owner=src/runner/mir_json_export_model.rs
serializer_input=MirJsonExportDocument
serializer_output=serde_json::Value
serializer_reads_mir_directly=0
projection_owner=main_crate
json_schema_changed=0
```

`build_mir_json_root` remains the projection owner for now. The next
implementation row may build the DTO, serialize it through the new serializer,
and debug-assert parity with the existing payload. Returning the serializer
payload can happen only after parity is fixed.

## Serializer Shape

```text
serialize_document(document):
  serialize functions from DTO fields
  create root according to document.root_kind
  insert root_metadata surfaces
  return serde_json::Value
```

Surface ordering is preserved by the DTO vectors. The first serializer keeps
instruction and metadata payloads as `serde_json::Value`; typed instruction DTOs
remain out of scope.

## Stop Lines

```text
do_not_read_mir_in_serializer=1
do_not_create_future_crate_yet=1
do_not_change_payload_shape=1
do_not_return_serializer_payload_until_parity=1
do_not_move_cfg_extractor=1
```

## Next

```text
next_task=BUILD-MIR-JSON-DTO-SERIALIZER-SCAFFOLD-001
purpose=add pure serializer from MirJsonExportDocument to serde_json::Value
```

## Contract

```text
output_contract=build-mir-json-dto-serializer-design-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
future_crate_created=0
mir_json_payload_changed=0

summary=ok
```
