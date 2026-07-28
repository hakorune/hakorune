Status: Done
Date: 2026-06-18
Scope: wire function-level MIR JSON export model summary construction
Related:
  - docs/development/current/main/phases/phase-296x/296x-1103-BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-SCAFFOLD-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-WIRING-001

## Purpose

Wire function-level export summaries in the MIR JSON root builder without
changing emitted JSON. This continues to move toward a JSON-ready export model
while keeping the main crate as the only MIR projection owner.

## Change

```text
summary_helper=mir_json_export_model::summarize_function
summary_inputs=name,param_count,block_count,instruction_count,metadata_entry_count
summary_consumer=src/runner/mir_json_emit/root.rs
json_output_changed=0
future_crate_created=0
```

`build_mir_json_root` now constructs a passive `MirJsonFunctionExportSummary`
per emitted function. The summary is validated through debug assertions and is
not inserted into the payload.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
mir_json_root_smoke=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-export-model-function-summary-wiring-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
mir_json_emit_payload_changed=0
future_crate_created=0
future_crate_reads_mir_directly=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-EXPORT-MODEL-CLOSEOUT-001
purpose=decide whether the export-model seam is sufficient before the next crate split attempt
```
