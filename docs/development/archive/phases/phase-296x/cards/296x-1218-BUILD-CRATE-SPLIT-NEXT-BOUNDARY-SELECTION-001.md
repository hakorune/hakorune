---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Select next build crate split boundary after frontend parser measurement.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1217-BUILD-FRONTEND-PARSER-POST-SPLIT-MEASUREMENT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-001

## Inventory

```text
current_blocker=BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-001
frontend_parser_split_series_closed=1
frontend_parser_post_split_cold_build_real_sec=157.63
build_time_winner_claim=0

box_trait_lines=307
box_factory_rs_file_count=6
box_factory_total_lines=1372
config_rs_file_count=4
config_total_lines=5002

src_backend_total_lines=19345
src_backend_crate_ref_count=654
src_mir_builder_total_lines=111032
src_mir_builder_crate_ref_count=5783
src_runtime_total_lines=22480
src_boxes_total_lines=23505
```

## Decision

```text
selected_next_boundary=box_core_config
selected_next_task=BUILD-BOX-CORE-CONFIG-BOUNDARY-AUDIT-001
reason=next_ranked_boundary_after_frontend_and_requires_audit_before_move

rejected_boundary=frontend_parser_active_owner_bundle
rejected_reason=parser_passive_series_closed_active_parser_moves_need_new_design_row

rejected_boundary=backend_aot_full
rejected_reason=blocked_by_MirModule_and_WasmBackend_and_not_best_default_build_boundary

rejected_boundary=wholesale_backend
rejected_reason=previously_rejected_and_still_mixed_with_VM_WASM_AOT_runtime_refs

rejected_boundary=mir_builder_lowering
rejected_reason=high_risk_late_stage_boundary_with_deep_AST_runtime_config_MIR_coupling

rejected_boundary=runtime_boxes
rejected_reason=last_overall_split_runtime_and_boxes_are_dependency_hubs
```

`box-core + config` is not selected for direct code movement. It is selected for
an audit row because the build split SSOT ranks it after frontend work but marks
it as valid only after boundary audit.

## Contract

```text
output_contract=build-crate-split-next-boundary-selection-v0

selection_only=1
behavior_changed=0
code_moved=0
implementation_allowed=0
selected_next_task=BUILD-BOX-CORE-CONFIG-BOUNDARY-AUDIT-001

summary=ok
```

## Next

```text
next_task=BUILD-BOX-CORE-CONFIG-BOUNDARY-AUDIT-001
purpose=audit box trait / box factory / config dependencies before selecting any passive split seam
implementation_allowed=0
audit_allowed=1
```
