Status: Done
Date: 2026-06-18
Scope: wire passive MIR JSON export model summary construction
Related:
  - docs/development/current/main/phases/phase-296x/296x-1101-BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-EXPORT-MODEL-ROOT-SUMMARY-WIRING-001

## Purpose

Connect the passive export-model vocabulary to the MIR JSON root builder without
changing emitted JSON. This keeps the future crate split seam visible while the
main crate still owns MIR-to-JSON projection.

## Change

```text
summary_helper=mir_json_export_model::summarize_root
summary_inputs=schema_v1_enabled,function_count,root_metadata_entry_count
summary_consumer=src/runner/mir_json_emit/root.rs
json_output_changed=0
future_crate_created=0
```

`build_mir_json_root` now constructs a `MirJsonExportModelSummary` from the
selected schema mode and the already-built root surfaces. The summary is not
inserted into the JSON payload; it is only a structural seam for the future
export model.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-export-model-root-summary-wiring-v0

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
next_task=BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-SCAFFOLD-001
purpose=add function-level export model summary vocabulary without changing emitted JSON
```
