Status: Done
Date: 2026-06-18
Scope: scaffold function-level MIR JSON export model summary vocabulary
Related:
  - docs/development/current/main/phases/phase-296x/296x-1102-BUILD-MIR-JSON-EXPORT-MODEL-ROOT-SUMMARY-WIRING-001.md
  - src/runner/mir_json_export_model.rs

# BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-SCAFFOLD-001

## Purpose

Add function-level export model vocabulary before moving any MIR JSON emitter
logic toward a future crate. The main crate still owns projection from
`MirFunction` and emitted JSON remains unchanged.

## Change

```text
new_vocabulary=MirJsonFunctionExportSummary
summary_fields=name,param_count,block_count,instruction_count,metadata_entry_count
function_summary_wired_to_root=0
json_output_changed=0
future_crate_created=0
```

The scaffold is intentionally passive. It names the function-level export seam
without reading MIR, changing `build_mir_json_root`, or modifying payload
schema.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-export-model-function-summary-scaffold-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
mir_json_emit_payload_changed=0
future_crate_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-WIRING-001
purpose=wire function summary construction without changing emitted JSON
```
