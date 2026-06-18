Status: Done
Date: 2026-06-18
Scope: close out the initial MIR JSON export model seam
Related:
  - docs/development/current/main/phases/phase-296x/296x-1100-BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1104-BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-WIRING-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit

# BUILD-MIR-JSON-EXPORT-MODEL-CLOSEOUT-001

## Purpose

Close the initial export-model seam and decide the next crate-split step. The
seam is useful as a passive projection boundary, but `mir_json_emit` is still
too coupled to MIR to extract as a crate.

## Evidence

```text
export_model_owner=src/runner/mir_json_export_model.rs
export_model_reads_mir_directly=0
root_summary_wired=1
function_summary_wired=1
json_output_changed=0

mir_json_emit_direct_mir_reference_count=378
direct_crate_extraction_selected=0
```

Remaining blockers:

```text
root_entry_takes_mir_module=1
ordering_walks_mir_module=1
emitters_read_mir_instruction_vocabulary=1
metadata_emitters_read_function_metadata_and_plan_structs=1
root_metadata_reads_module_decls_and_cfg_extractor=1
tests_are_mir_fixture_based=1
```

## Decision

```text
closeout_result=export_model_seam_closed
mir_json_emit_crate_extraction_allowed=0
selected_next_task=BUILD-MIR-JSON-DTO-BOUNDARY-DESIGN-001
reason=mir_agnostic_dto_required_before_emitter_crate_split
```

The next work should define a MIR-agnostic DTO boundary for root/function/block
/instruction summaries. The main crate remains the projection owner until that
DTO is complete.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
mir_json_root_smoke=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-export-model-closeout-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
future_crate_created=0
future_crate_reads_mir_directly=0
direct_crate_extraction_selected=0

summary=ok
```
