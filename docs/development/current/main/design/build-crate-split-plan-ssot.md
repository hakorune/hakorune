---
Status: SSOT
Decision: accepted
Date: 2026-06-18
Scope: Build-time reduction through crate split planning.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1076-BUILD-CRATE-SPLIT-PLAN-001.md
---

# Build Crate Split Plan SSOT

## Problem

The main `nyash-rust` crate is too large to compile efficiently.

Observed audit:

```text
main_crate_lines=469k
main_crate_files=2370
total_build_time_sec=41.6
main_crate_compile_time_sec=33.8
main_crate_compile_time_percent=81
src_mir_lines=278k
src_mir_percent_of_main_crate=59
```

One giant crate limits parallelism and forces unrelated compiler/runtime edits
through the same compile unit.

## Decision

Adopt a staged crate split, but do not start with the deepest lowering code.

The first goal is build-time leverage with low architectural risk:

```text
stage_0=mir_core_growth
stage_1=hakorune_mir_plans
stage_2=hakorune_backend
stage_3=hakorune_frontend
stage_4=box_core_config
stage_5=hakorune_lowering
stage_6=runtime_boxes
```

## Ranking

| Rank | Crate | Approx Size | Effect | Effort | Risk | Decision |
|---:|---|---:|---|---|---|---|
| 1 | `hakorune-mir-plans` | 40-45k lines | high | medium | low | first real split |
| 2 | grow `mir_core` | 1.2k -> larger | medium | small | low | prerequisite |
| 3 | `hakorune-backend` | 18k lines | medium | medium | low | after plans |
| 4 | `hakorune-frontend` | 17.5k lines | medium | medium | medium | after backend |
| 5 | `box-core + config` | 6k lines | medium | medium | medium | only after boundary audit |
| 6 | `hakorune-lowering` | 82k lines | high | large | high | last compiler split |
| 7 | `runtime + boxes` | 46k lines | medium | large | high | last overall split |

## Stage 0: mir_core Growth

Purpose:

```text
move_stable_mir_data_types=1
move_plan_independent_value_types=1
move_report_contract_types_when_dependency_free=1
```

Allowed:

```text
MirType / ValueId / BlockId style shared primitives
small plan-neutral enums
serde-compatible metadata structs with no builder/backend dependency
```

Forbidden:

```text
builder control-flow logic
lowering logic
runtime boxes
backend emitters
policy decisions with active owners
```

## Stage 1: hakorune-mir-plans

Purpose:

```text
extract plan vocabularies and passive plan data from src/mir
keep lowering/building behavior in main crate at first
```

Candidate families:

```text
object_storage_plan
local_fastpath_fact
map_repr_plan passive data
route plan data models after dependency audit
plan report vocabularies
```

Non-goals:

```text
do not move control_flow lowering yet
do not move MIRBuilder yet
do not move runtime Box implementations
do not change behavior while splitting
```

## Guardrail

Each split row must be BoxShape-only:

```text
behavior_changed=0
public_api_changed_only_for_crate_boundary=1
cargo_build_release_bin_hakorune_green=1
quick_smoke_green_when_slice_ready=1
```

No language acceptance shape, optimizer rule, or runtime behavior change may be
mixed into the crate split commits.

## Next Task

```text
latest_done=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001
next_task=BUILD-VM-REPL-REFERENCE-GATE-001
purpose=gate the isolated REPL VM direct import with a vm-reference fail-fast path
implementation_allowed=repl_only
```

## Stage 0 Result

```text
mir_core_growth_first_slice=control_flow_id_newtypes
moved_types=LoopId,ExitEdgeId,ContinueEdgeId
compat_reexport=src/mir/control_form.rs
behavior_changed=0
```

## Stage 1 First Slice Result

```text
hakorune_mir_plans_created=1
first_family=object_storage_plan
main_crate_compat_facade=src/object_storage_plan.rs
behavior_changed=0
```

## Baseline Result

```text
baseline_card=BUILD-TIME-BASELINE-MEASURE-001
cold_build_real_sec=157.37
cold_build_user_sec=208.27
cold_build_sys_sec=9.49
large_file_count=0
```

## Stage 1 Second Slice Result

```text
second_family=aggregate_storage_plan
owner=crates/hakorune_mir_plans/src/aggregate_storage_plan.rs
main_crate_compat_facade=src/aggregate_storage_plan.rs
behavior_changed=0
```

## Stage 1 Third Slice Result

```text
third_family=map_repr_plan_pure_data_subset
owner=crates/hakorune_mir_plans/src/map_repr_plan
main_crate_builder_facade=src/mir/map_repr_plan/plans.rs
refresh_logic_owner=src/mir/map_repr_plan/refresh.rs
candidate_detection_owner=src/mir/map_repr_plan/candidates.rs
behavior_changed=0
```

## Stage 1 Fourth Slice Result

```text
fourth_family=local_fastpath_fact_pure_aggregator
owner=crates/hakorune_mir_plans/src/local_fastpath_fact.rs
main_crate_assignment_facade=src/mir/local_fastpath_fact.rs
moved_function=build_local_fastpath_facts_from_map_repr_plans
mirfunction_assignment_owner_preserved=1
behavior_changed=0
```

## Stage 1 Fifth Slice Result

```text
fifth_family=typed_field_storage_vocabulary
owner=crates/hakorune_mir_plans/src/typed_field_storage.rs
main_crate_compat_reexport=crate::mir::function::TypedObjectFieldStorage
storage_inference_moved=0
behavior_changed=0
```

## Stage 1 Sixth Slice Result

```text
sixth_family=array_record_passive_bundle
owner=crates/hakorune_mir_plans/src/array_record_plan.rs
main_crate_compat_reexport=crate::mir::function::*
producer_logic_moved=0
behavior_changed=0
```

## Stage 1 Seventh Slice Result

```text
seventh_family=object_state_passive_bundle
owner=crates/hakorune_mir_plans/src/object_state_plan.rs
main_crate_compat_reexport=crate::mir::function::*
declaration_inventory_moved=0
producer_logic_moved=0
behavior_changed=0
```

## Stage 1 Eighth Slice Result

```text
eighth_family=function_fact_passive_bundle
owner=crates/hakorune_mir_plans/src/function_fact_plan.rs
main_crate_compat_reexport=crate::mir::function::*
producer_logic_moved=0
refresh_logic_moved=0
behavior_changed=0
```

## Stage 1 Closeout Result

```text
closed_stage=hakorune_mir_plans_stage_1
remaining_low_risk_passive_bundle_count=0
next_task=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
behavior_changed=0
```

## Post Stage 1 Measurement Result

```text
post_stage1_card=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
cold_build_real_sec=158.95
cold_build_user_sec=212.73
cold_build_sys_sec=11.59
baseline_cold_build_real_sec=157.37
build_time_winner_claim=0
main_crate_still_dominant=1
recommended_next_stage=hakorune_backend_preflight
```

## Backend Split Preflight Result

```text
preflight_card=BUILD-BACKEND-CRATE-PREFLIGHT-001
src_backend_wholesale_split_selected=0
selected_next_boundary=runner_mir_json_emit
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001
reason=product_exe_route_uses_mir_json_emit_before_ny_llvmc
behavior_changed=0
```

## MIR JSON Emit Preflight Result

```text
preflight_card=BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001
src_runner_mir_json_emit_rs_total_lines=10033
crate_mir_reference_count=372
direct_crate_extraction_selected=0
selected_next_task=BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001
reason=emitter_input_view_boundary_required
behavior_changed=0
```

## MIR JSON Emit Boundary SSOT Result

```text
boundary_card=BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001
projection_owner=main_crate
serialization_owner=future_hakorune_mir_json_emit_crate
future_crate_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001
behavior_changed=0
```

## MIR JSON Export Model Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001
new_owner=src/runner/mir_json_export_model.rs
new_vocabulary=MirJsonExportSchema,MirJsonExportRootKind,MirJsonExportModelSummary
mir_json_emit_behavior_changed=0
future_crate_created=0
```

## MIR JSON DTO Closeout Result

```text
closeout_card=BUILD-MIR-JSON-DTO-CLOSEOUT-001
dto_document_constructed=1
mir_json_emit_direct_mir_reference_count=378
direct_crate_extraction_selected=0
selected_next_task=BUILD-MIR-JSON-DTO-SERIALIZER-DESIGN-001
```

## MIR JSON DTO Serializer Design Result

```text
design_card=BUILD-MIR-JSON-DTO-SERIALIZER-DESIGN-001
serializer_input=MirJsonExportDocument
serializer_output=serde_json::Value
serializer_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-DTO-SERIALIZER-SCAFFOLD-001
```

## MIR JSON DTO Serializer Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-DTO-SERIALIZER-SCAFFOLD-001
serializer_function=mir_json_export_model::serialize_document
serializer_reads_mir_directly=0
root_builder_wired_to_serializer=0
json_output_changed=0
```

## MIR JSON DTO Serializer Parity Wiring Result

```text
wiring_card=BUILD-MIR-JSON-DTO-SERIALIZER-PARITY-WIRING-001
serializer_called_from_root_builder=1
serializer_parity_debug_assert=1
root_builder_returns_existing_payload=1
json_output_changed=0
```

## MIR JSON DTO Serializer Return Switch Result

```text
wiring_card=BUILD-MIR-JSON-DTO-SERIALIZER-RETURN-SWITCH-001
serializer_payload_returned_from_root_builder=1
serializer_parity_debug_assert=1
legacy_root_builder_payload_kept_as_parity_oracle=1
json_output_changed=0
future_crate_created=0
```

## MIR JSON DTO Serializer Closeout Result

```text
closeout_card=BUILD-MIR-JSON-DTO-SERIALIZER-CLOSEOUT-001
serializer_seam_closed=1
mir_json_emit_direct_mir_reference_count=378
direct_mir_json_emit_crate_extraction_selected=0
future_crate_package_name=hakorune-mir-json-emit
future_crate_scope=json_ready_dto_serializer_only
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001
```

## MIR JSON Emit Crate Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001
new_crate=hakorune-mir-json-emit
new_crate_scope=json_ready_dto_serializer_only
new_crate_reads_mir_directly=0
main_crate_dependency_added=0
json_output_changed=0
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-FACADE-WIRING-001
```

## MIR JSON Emit Crate Facade Wiring Result

```text
wiring_card=BUILD-MIR-JSON-EMIT-CRATE-FACADE-WIRING-001
main_crate_dependency_added=1
compat_facade=src/runner/mir_json_export_model.rs
serialization_owner=hakorune_mir_json_emit
projection_owner=main_crate
json_output_changed=0
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-CLOSEOUT-001
```

## MIR JSON Emit Crate Closeout Result

```text
closeout_card=BUILD-MIR-JSON-EMIT-CRATE-CLOSEOUT-001
new_crate=hakorune-mir-json-emit
serialization_owner=hakorune_mir_json_emit
projection_owner=main_crate
new_crate_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
```

## MIR JSON Emit Post-Split Measurement Result

```text
measure_card=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
cold_build_real_sec=161.28
cold_build_user_sec=213.71
cold_build_sys_sec=10.49
baseline_cold_build_real_sec=157.37
post_stage1_cold_build_real_sec=158.95
build_time_winner_claim=0
selected_next_task=BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001
```

## Backend Next Boundary Selection Result

```text
selection_card=BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001
selected_next_boundary=backend_aot
backend_aot_lines=950
backend_aot_dependency_refs=4
selected_next_task=BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001
```

## Backend AOT Crate Preflight Result

```text
preflight_card=BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001
full_backend_aot_crate_split_selected=0
full_split_blocked_by=MirModule,WasmBackend
selected_first_slice=aot_passive_config_executable_error
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001
```

## Backend AOT Passive Crate Scaffold Result

```text
scaffold_card=BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001
new_crate=hakorune-backend-aot
new_crate_scope=aot_error_config_executable_builder
new_crate_reads_mir_directly=0
new_crate_depends_on_wasm_backend=0
main_crate_dependency_added=0
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-FACADE-WIRING-001
```

## Backend AOT Passive Facade Wiring Result

```text
wiring_card=BUILD-BACKEND-AOT-PASSIVE-FACADE-WIRING-001
main_crate_dependency_added=1
dependency_feature_gate=wasm-backend
passive_aot_support_owner=hakorune_backend_aot
compiler_pipeline_owner=main_crate
removed_main_crate_files=src/backend/aot/config.rs,src/backend/aot/executable.rs
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-CLOSEOUT-001
```

## Backend AOT Passive Closeout Result

```text
closeout_card=BUILD-BACKEND-AOT-PASSIVE-CLOSEOUT-001
passive_aot_support_split_closed=1
post_split_default_cold_build_measure_selected=0
reason=aot_boundary_is_optional_feature_not_default_build_owner
selected_next_task=BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001
```

## VM MIR Interpreter Compile Audit Result

```text
audit_card=BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001
mir_interpreter_default_compiled=1
mir_interpreter_file_count=66
mir_interpreter_lines=12944
vm_product_route_retired=1
vm_semantic_reference_subset_alive=1
vm_types_live_outside_interpreter=1
immediate_mir_interpreter_delete_selected=0
immediate_mir_interpreter_feature_gate_selected=0
selected_next_task=BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001
```

## VM MIR Interpreter Feature Gate Design Result

```text
design_card=BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001
feature_name=vm-reference
initial_feature_default=on
vm_types_feature_gated=0
mir_interpreter_feature_gated=planned
backend_vm_alias_feature_gated=planned
default_off_selected_now=0
selected_next_task=BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001
```

## VM Reference Feature Scaffold Result

```text
scaffold_card=BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001
feature_name=vm-reference
feature_in_default=1
vm_types_feature_gated=0
mir_interpreter_module_feature_gated=1
backend_mirinterpreter_export_feature_gated=1
backend_vm_alias_feature_gated=1
default_off_claim=0
no_default_features_check_green=0
selected_next_task=BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001
```

## VM Runner Caller Classification Result

```text
classification_card=BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001
terminal_vm_execution_owner=NyashRunner::execute_mir_module_quiet_exit
terminal_vm_execution_owner_fan_in=high
explicit_vm_repl_keep_joinir_classified_as_vm_reference=1
product_and_bridge_routes_still_use_vm_terminal=1
vm_reference_remove_from_default_allowed=0
selected_next_task=BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001
```

## VM Terminal Execution Route Design Result

```text
design_card=BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001
terminal_owner=NyashRunner::execute_mir_module_quiet_exit
terminal_owner_role=vm_reference_terminal
vm_reference_disabled_terminal_behavior=fail_fast
silent_vm_to_aot_fallback=0
silent_aot_to_vm_fallback=0
selected_next_task=BUILD-VM-TERMINAL-FAILFAST-SEAM-001
```

## VM Terminal Fail-Fast Seam Result

```text
implementation_card=BUILD-VM-TERMINAL-FAILFAST-SEAM-001
central_terminal_failfast_added=1
execute_mir_module_quiet_exit_cfg_split=1
execute_mir_module_cfg_split=1
emit_mir_json_early_exit_preserved=1
emit_exe_early_exit_preserved=1
hidden_aot_fallback_added=0
no_default_features_vm_error_count_after=6
selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001
```

## VM Direct Caller Gate Selection Result

```text
selection_card=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001
selected_family=runner_repl_vm_reference_gate
selected_next_task=BUILD-VM-REPL-REFERENCE-GATE-001
reason=single_public_entry_and_no_product_exe_aot_terminal_overlap
default_off_claim=0
```

## MIR JSON Export Model Root Summary Wiring Result

```text
wiring_card=BUILD-MIR-JSON-EXPORT-MODEL-ROOT-SUMMARY-WIRING-001
summary_helper=mir_json_export_model::summarize_root
summary_consumer=src/runner/mir_json_emit/root.rs
json_output_changed=0
future_crate_created=0
```

## MIR JSON DTO Root Projection Wiring Result

```text
wiring_card=BUILD-MIR-JSON-DTO-ROOT-PROJECTION-WIRING-001
dto_document_constructed=1
dto_source=current_json_ready_values
json_output_changed=0
future_crate_created=0
```

## MIR JSON Export Model Closeout Result

```text
closeout_card=BUILD-MIR-JSON-EXPORT-MODEL-CLOSEOUT-001
export_model_seam_closed=1
mir_json_emit_direct_mir_reference_count=378
direct_crate_extraction_selected=0
selected_next_task=BUILD-MIR-JSON-DTO-BOUNDARY-DESIGN-001
behavior_changed=0
```

## MIR JSON DTO Boundary Design Result

```text
design_card=BUILD-MIR-JSON-DTO-BOUNDARY-DESIGN-001
dto_boundary_required=1
projection_owner=main_crate
serialization_owner=future_hakorune_mir_json_emit_crate
future_crate_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-DTO-SCAFFOLD-001
```

## MIR JSON DTO Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-DTO-SCAFFOLD-001
new_vocabulary=MirJsonExportDocument,MirJsonExportFunction,MirJsonExportBlock,MirJsonExportInstruction,MirJsonExportSurface
instruction_payload_type=serde_json::Value
json_output_changed=0
future_crate_created=0
```

## MIR JSON Export Model Function Summary Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-SCAFFOLD-001
new_vocabulary=MirJsonFunctionExportSummary
function_summary_wired_to_root=0
json_output_changed=0
future_crate_created=0
```

## MIR JSON Export Model Function Summary Wiring Result

```text
wiring_card=BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-WIRING-001
summary_helper=mir_json_export_model::summarize_function
summary_consumer=src/runner/mir_json_emit/root.rs
json_output_changed=0
future_crate_created=0
```
