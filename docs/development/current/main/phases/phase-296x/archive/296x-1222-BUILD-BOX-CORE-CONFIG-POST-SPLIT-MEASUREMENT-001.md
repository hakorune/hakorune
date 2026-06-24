---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Cold release build measurement after first box-core passive split.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1221-BUILD-BOX-CORE-CONFIG-PASSIVE-SPLIT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-POST-SPLIT-MEASUREMENT-001

## Command

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune
```

## Result

```text
release_build_status=green
release_build_target=hakorune
cargo_reported_release_time=2m30s
cold_build_real_sec=150.16
cold_build_user_sec=203.50
cold_build_sys_sec=8.77
```

Baseline comparison:

```text
baseline_card=BUILD-TIME-BASELINE-MEASURE-001
baseline_cold_build_real_sec=157.37
frontend_parser_post_split_card=BUILD-FRONTEND-PARSER-POST-SPLIT-MEASUREMENT-001
frontend_parser_post_split_cold_build_real_sec=157.63
box_core_config_post_split_cold_build_real_sec=150.16
single_run_delta_vs_frontend_parser_sec=-7.47
```

Interpretation:

```text
box_core_passive_split_reached_build=1
new_crate_compiled=hakorune-box-core
measurement_recorded=1
single_run_visible_improvement=1
build_time_winner_claim=0
winner_claim_reason=single_run_and_split_is_too_small_to_attribute_without_repeat
```

## Verification

```text
cargo_test_hakorune_box_core=green
cargo_check_default=green
current_state_pointer_guard=green
diff_check=green
release_build_green=1
```

## Contract

```text
output_contract=build-box-core-config-post-split-measurement-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
measurement_recorded=1
release_build_green=1
build_time_winner_claim=0

summary=ok
```

## Next

```text
next_task=BUILD-BOX-CORE-CONFIG-NEXT-SEAM-SELECTION-001
purpose=decide whether to continue box-core/config passive seams or return to global crate-boundary selection
implementation_allowed=0
selection_allowed=1
```
