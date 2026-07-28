Status: Done
Date: 2026-06-18
Scope: cold release build measurement after hakorune-mir-json-emit split
Related:
  - docs/development/current/main/phases/phase-296x/296x-1117-BUILD-MIR-JSON-EMIT-CRATE-CLOSEOUT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001

## Command

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune
```

## Result

```text
cold_build_real_sec=161.28
cold_build_user_sec=213.71
cold_build_sys_sec=10.49

baseline_card=BUILD-TIME-BASELINE-MEASURE-001
baseline_cold_build_real_sec=157.37
post_stage1_card=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
post_stage1_cold_build_real_sec=158.95

build_time_winner_claim=0
main_crate_still_dominant=1
```

This split is structural. It moves serializer ownership out of the main crate
but does not reduce cold build time yet.

## Decision

```text
selected_next_task=BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001
reason=measurement_done_and_next_boundary_should_be_selected_from_dependency_evidence
```

## Contract

```text
output_contract=build-mir-json-emit-post-split-measure-v0

behavior_changed=0
json_schema_changed=0
measurement_only=1
build_time_winner_claim=0
next_task=BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001

summary=ok
```
