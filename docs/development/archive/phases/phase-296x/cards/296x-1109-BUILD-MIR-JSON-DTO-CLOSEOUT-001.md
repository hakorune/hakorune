Status: Done
Date: 2026-06-18
Scope: close out passive MIR JSON DTO construction
Related:
  - docs/development/current/main/phases/phase-296x/296x-1108-BUILD-MIR-JSON-DTO-ROOT-PROJECTION-WIRING-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-DTO-CLOSEOUT-001

## Purpose

Close the passive DTO construction slice and choose the next lowest-risk step.
The DTO now exists and is constructed, but emitted JSON still comes from the
existing root builder, and `mir_json_emit` still reads MIR directly.

## Evidence

```text
dto_vocabulary_exists=1
dto_document_constructed=1
json_output_changed=0
future_crate_created=0

mir_json_emit_direct_mir_reference_count=378
direct_crate_extraction_selected=0
```

## Decision

```text
closeout_result=dto_construction_closed
mir_json_emit_crate_extraction_allowed=0
selected_next_task=BUILD-MIR-JSON-DTO-SERIALIZER-DESIGN-001
reason=dto_serializer_required_before_future_crate_can_own_serialization
```

The future crate should eventually consume `MirJsonExportDocument` and produce
the JSON payload. That serializer boundary must be designed before another crate
extraction attempt.

## Contract

```text
output_contract=build-mir-json-dto-closeout-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
future_crate_created=0
direct_crate_extraction_selected=0

summary=ok
```
