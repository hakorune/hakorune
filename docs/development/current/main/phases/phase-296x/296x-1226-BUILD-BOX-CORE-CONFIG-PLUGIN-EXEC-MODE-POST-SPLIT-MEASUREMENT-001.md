---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Cold release build measurement after PluginExecMode passive split.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1225-BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-PASSIVE-SPLIT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-POST-SPLIT-MEASUREMENT-001

## Command

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune
```

## Result

```text
release_build_status=green
release_build_target=hakorune
cargo_reported_release_time=2m39s
cold_build_real_sec=159.44
cold_build_user_sec=221.35
cold_build_sys_sec=10.85
```

Baseline comparison:

```text
baseline_card=BUILD-TIME-BASELINE-MEASURE-001
baseline_cold_build_real_sec=157.37
box_core_config_post_split_card=BUILD-BOX-CORE-CONFIG-POST-SPLIT-MEASUREMENT-001
box_core_config_post_split_cold_build_real_sec=150.16
plugin_exec_mode_post_split_cold_build_real_sec=159.44
build_time_winner_claim=0
build_time_loss_claim=0
```

Interpretation:

```text
plugin_exec_mode_split_reached_build=1
new_owner_type=PluginExecMode
measurement_recorded=1
single_run_variability_visible=1
build_time_winner_claim=0
loss_claim_reason=single_run_not_sufficient_for_attribution
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
output_contract=build-box-core-config-plugin-exec-mode-post-split-measurement-v0

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
next_task=BUILD-BOX-CORE-CONFIG-CLOSEOUT-001
purpose=close the current box-core/config passive seam burst and return to next boundary selection
implementation_allowed=0
closeout_allowed=1
```
