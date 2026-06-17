Status: Done
Date: 2026-06-18
Scope: build-time baseline after first crate split slices
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1078-BUILD-MIR-CORE-GROWTH-ID-SLICE-001.md
  - docs/development/current/main/phases/phase-296x/296x-1080-BUILD-MIR-PLANS-OBJECT-STORAGE-SPLIT-001.md

# BUILD-TIME-BASELINE-MEASURE-001

## Purpose

Record the first cold build-time baseline after:

```text
mir_core_growth_first_slice=control_flow_id_newtypes
hakorune_mir_plans_first_split=object_storage_plan
```

This is a measurement row only. It does not claim build-time improvement yet.

## Measurement

Command:

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune
```

Result:

```text
release_build_status=green
release_build_target=hakorune
cold_build_real_sec=157.37
cold_build_user_sec=208.27
cold_build_sys_sec=9.49
cargo_reported_release_time=2m37s
```

Interpretation:

```text
main_crate_still_dominant=1
first_split_is_structural_baseline_not_winner=1
next_build_time_owner_requires_more_mir_plan_movement=1
```

## Line Counts

```text
src_rs_total_lines=496926
hakorune_mir_core_rs_total_lines=1237
hakorune_mir_plans_rs_total_lines=1362
large_file_threshold=800
large_file_count=0
```

## Verification

```text
cargo_test_hakorune_mir_core=green
hakorune_mir_core_unit_tests=23
hakorune_mir_core_doctests_ignored=4
cargo_test_hakorune_mir_plans=green
hakorune_mir_plans_unit_tests=13
current_state_pointer_guard=green
```

The `hakorune-mir-core` doctest examples were corrected to `ignore` / `text`
where they are explanatory snippets rather than standalone Rust programs.

## Contract

```text
output_contract=build-time-baseline-measure-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
release_build_green=1
baseline_recorded=1
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-NEXT-FAMILY-SELECTION-001
recommended_owner=more_passive_mir_plan_vocabulary
deep_lowering_split_deferred=1
runtime_boxes_split_deferred=1
```
