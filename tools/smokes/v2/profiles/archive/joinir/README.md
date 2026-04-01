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
- `phase143_legacy_pack.sh`
- `phase286_pattern9_legacy_pack.sh`

## Runner

Use:

```bash
./tools/smokes/v2/run.sh --profile archive --filter "joinir/<basename>"
```

These scripts are intentionally skipped and retained only for historical replay/reference.
