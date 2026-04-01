# Archive: JoinIR Legacy Pack Stems

This directory holds historical JoinIR pack stems that were retired from the active
`integration/joinir` lane.

## Contents

- `phase29bq_mir_preflight_lowered_away_reject_vm.sh`
- `phase29bq_continue_target_header_planner_required_vm.sh`
- `phase29bq_hako_mirbuilder_phase5_min_vm.sh`
- `phase29bq_hako_mirbuilder_phase7_min_vm.sh`
- `phase29bq_hako_mirbuilder_phase9_min_vm.sh`
- `phase1883_nested_minimal_vm.sh`
- `phase29ao_pattern2_release_adopt_vm.sh`
- `phase29ao_pattern3_release_adopt_vm.sh`
- `phase29ao_pattern5_release_adopt_vm.sh`
- `phase29ao_pattern6_release_adopt_vm.sh`
- `phase29ao_pattern7_release_adopt_vm.sh`
- `phase29ao_pattern1_strict_shadow_vm.sh`
- `phase29ap_pattern6_nested_strict_shadow_vm.sh`
- `phase29ap_pattern4_continue_min_vm.sh`
- `phase29ap_stringutils_join_vm.sh`
- `phase29ap_stringutils_tolower_vm.sh`
- `phase29bn_planner_required_dev_gate_v2_vm.sh`
- `phase29bi_planner_required_pattern2_pack_vm.sh`
- `phase29bl_planner_required_pattern1_4_5_pack_vm.sh`
- `phase29bo_planner_required_pattern8_9_pack_vm.sh`
- `phase143_legacy_pack.sh`
- `phase286_pattern9_legacy_pack.sh`

## Runner

Use:

```bash
./tools/smokes/v2/run.sh --profile archive --filter "joinir/<basename>"
```

These scripts are intentionally skipped and retained only for historical replay/reference.
