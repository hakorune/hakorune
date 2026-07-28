---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Cold release build measurement after frontend parser passive split series.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1216-BUILD-FRONTEND-PARSER-SPLIT-SERIES-CLOSEOUT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-FRONTEND-PARSER-POST-SPLIT-MEASUREMENT-001

## Purpose

Record the cold release build-time result after closing the frontend parser
passive split series.

This row is measurement-only. It does not claim a build-time win.

## Command

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune
```

## Result

```text
release_build_status=green
release_build_target=hakorune
cargo_reported_release_time=2m37s
cold_build_real_sec=157.63
cold_build_user_sec=214.27
cold_build_sys_sec=12.15
```

Baseline comparison:

```text
baseline_card=BUILD-TIME-BASELINE-MEASURE-001
baseline_cold_build_real_sec=157.37
post_stage1_card=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
post_stage1_cold_build_real_sec=158.95
mir_json_emit_post_split_card=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
mir_json_emit_post_split_cold_build_real_sec=161.28
frontend_parser_post_split_cold_build_real_sec=157.63
build_time_winner_claim=0
```

Interpretation:

```text
frontend_parser_split_series_closed=1
measurement_only=1
main_crate_still_dominant=1
frontend_parser_split_structural_not_build_time_winner=1
next_boundary_requires_selection=1
```

## Contract

```text
output_contract=build-frontend-parser-post-split-measurement-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
release_build_green=1
measurement_recorded=1
build_time_winner_claim=0
implementation_allowed=0

summary=ok
```

## Next

```text
next_task=BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-001
purpose=select the next build-time crate boundary after the frontend parser passive split series measurement
implementation_allowed=0
measurement_allowed=0
selection_allowed=1
```
