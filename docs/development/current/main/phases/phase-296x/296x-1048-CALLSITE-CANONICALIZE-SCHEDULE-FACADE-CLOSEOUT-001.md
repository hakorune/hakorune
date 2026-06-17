# 296x-1048 CALLSITE-CANONICALIZE-SCHEDULE-FACADE-CLOSEOUT-001

Status: Landed
Date: 2026-06-17
Scope: callsite canonicalize schedule facade closeout

## Contract

```text
output_contract=hako-callsite-canonicalize-schedule-facade-closeout-v0
row_kind=closeout

production_entry_count=4
known_entry_count=4
unknown_entry_count=0
centralized_schedule_owner=1
single_transform_owner=1

entry_removal_enabled=0
schedule_reorder_enabled=0
semantic_refresh_timing_changed=0
program_json_and_mir_json_policy_merged=0
behavior_changed=0

callsite_canonicalize_entry_residue_closed=1
next_design_queue=EXACT-STACK-OBJECT-RETIRE-DESIGN-001_or_FASTPATH-REACHABILITY-RETIRE-OR-ENABLE-DESIGN-001
summary=ok
```

## Result

The callsite canonicalize residue is closed as a BoxShape cleanup:

```text
before:
  four production entries directly called canonicalize_callsites(...)
  centralized_schedule_owner=0

after:
  four production entries delegate through canonicalize_for_site(...)
  centralized_schedule_owner=1
```

No entry was removed and no pipeline timing was changed.

## Evidence

```text
entry_0_path=src/mir/compiler/mod.rs
entry_0_call_kind=schedule_facade

entry_1_path=src/mir/optimizer/core.rs
entry_1_call_kind=schedule_facade

entry_2_path=src/runner/json_v0_bridge/core.rs
entry_2_call_kind=schedule_facade

entry_3_path=src/runner/mir_json_v0.rs
entry_3_call_kind=schedule_facade
```

## Stop Line

```text
do not reopen callsite canonicalize schedule cleanup without new evidence
do not remove entries as a drive-by follow-up
do not use this closeout to justify legacy MIR instruction retirement
```

## Verification

```bash
cargo test -q callsite_canonicalize --lib
python3 -m unittest tools.hako_check.tests.test_callsite_canonicalize_entry_inventory
python3 tools/hako_check/callsite_canonicalize_entry_inventory.py --repo-root .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
