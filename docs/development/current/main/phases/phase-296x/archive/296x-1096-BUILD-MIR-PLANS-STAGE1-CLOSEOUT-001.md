Status: Done
Date: 2026-06-18
Scope: close hakorune-mir-plans Stage 1 passive split lane
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1095-BUILD-MIR-PLANS-FUNCTION-FACT-PASSIVE-BUNDLE-SPLIT-001.md

# BUILD-MIR-PLANS-STAGE1-CLOSEOUT-001

## Purpose

Close Stage 1 of the build crate split lane after the last low-risk passive
MIR plan/fact bundles have moved into `hakorune-mir-plans`.

## Closed Scope

```text
closed_stage=hakorune_mir_plans_stage_1
moved_families=object_storage_plan,aggregate_storage_plan,map_repr_plan_pure_data,local_fastpath_fact_aggregator,typed_field_storage,array_record_passive_bundle,object_state_passive_bundle,function_fact_passive_bundle
main_crate_compat_surfaces_preserved=1
behavior_changed=0
```

## Remaining Non-Stage-1 Candidates

| Candidate | Decision | Reason |
|---|---|---|
| `src/mir/function/types.rs::StaticDataPlan` | defer | Too small to justify another Stage 1 row; producer is AST-backed. |
| `ExactNumericRuntimeCheckContract` | defer | Belongs to exact-numeric semantic lane, not generic passive plan split. |
| `MirParamDecl`, `MirEnumDecl`, `UserBoxFieldDecl`, `RecordDecl` | keep | Declaration inventory, not plan vocabulary. |
| `src/mir/function/fastmem.rs` | keep | Source-span / fastmem-region dependencies need a separate boundary audit. |
| `control_flow/plan/**` | stop | Builder-private lowering/planner subsystem; reserved for deep lowering split. |
| backend / frontend / runtime crates | next-stage candidates | Require separate preflight before extraction. |

## Contract

```text
output_contract=build-mir-plans-stage1-closeout-v0

stage1_closeout=1
remaining_low_risk_passive_bundle_count=0
behavior_changed=0
boxcount_allowed=0
builder_control_flow_moved=0
producer_logic_moved=0
runtime_box_moved=0
backend_emitter_moved=0

summary=ok
```

## Verification Inherited From Final Slice

```text
cargo_test_hakorune_mir_plans=green
cargo_check=green
cargo_build_release_bin_hakorune=green
current_state_pointer_guard=green
large_file_count=0
```

## Next

```text
next_task=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
purpose=measure post-stage1 build-time baseline before selecting backend/frontend/deeper split
```
