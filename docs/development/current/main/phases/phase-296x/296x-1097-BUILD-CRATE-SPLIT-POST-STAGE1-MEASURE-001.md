Status: Done
Date: 2026-06-18
Scope: post-stage1 cold build-time measurement
Related:
  - docs/development/current/main/phases/phase-296x/296x-1096-BUILD-MIR-PLANS-STAGE1-CLOSEOUT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001

## Purpose

Record the cold build-time baseline after Stage 1 of `hakorune-mir-plans` is
closed.

This row is measurement-only. It does not claim a build-time win.

## Measurement

Command:

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune
```

Result:

```text
release_build_status=green
release_build_target=hakorune
cold_build_real_sec=158.95
cold_build_user_sec=212.73
cold_build_sys_sec=11.59
cargo_reported_release_time=2m38s
```

Baseline comparison:

```text
baseline_card=BUILD-TIME-BASELINE-MEASURE-001
baseline_cold_build_real_sec=157.37
post_stage1_cold_build_real_sec=158.95
cold_build_real_delta_sec=1.58
build_time_winner_claim=0
```

Interpretation:

```text
main_crate_still_dominant=1
stage1_was_structural_split_not_build_time_winner=1
next_build_time_owner_requires_larger_crate_boundary=1
recommended_next_stage=hakorune_backend_preflight
```

## Line Counts

```text
src_rs_total_lines=495887
hakorune_mir_core_rs_total_lines=1237
hakorune_mir_plans_rs_total_lines=2817
large_file_threshold=800
large_file_count=0
```

## Contract

```text
output_contract=build-crate-split-post-stage1-measure-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
release_build_green=1
measurement_recorded=1
build_time_winner_claim=0
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-BACKEND-CRATE-PREFLIGHT-001
purpose=preflight the next ranked crate split after Stage 1 did not move cold build time
```
