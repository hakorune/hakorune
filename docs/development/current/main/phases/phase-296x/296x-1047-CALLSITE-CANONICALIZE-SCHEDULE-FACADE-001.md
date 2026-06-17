# 296x-1047 CALLSITE-CANONICALIZE-SCHEDULE-FACADE-001

Status: Landed
Date: 2026-06-17
Scope: callsite canonicalize schedule facade implementation

## Contract

```text
output_contract=hako-callsite-canonicalize-schedule-facade-v0
row_kind=boxshape_implementation

selected_option=B_thin_schedule_facade
production_entry_count=4
centralized_schedule_owner=1
entry_removal_enabled=0
schedule_reorder_enabled=0
semantic_refresh_timing_changed=0
program_json_and_mir_json_policy_merged=0

transform_owner=src/mir/passes/callsite_canonicalize
schedule_owner=src/mir/passes/callsite_canonicalize/schedule.rs

behavior_changed=0
summary=ok
```

## Implementation

Added:

```text
src/mir/passes/callsite_canonicalize/schedule.rs
```

with:

```rust
CallsiteCanonicalizeScheduleSite
canonicalize_for_site(module, site)
```

The facade delegates to the existing `canonicalize_callsites` transform. The
`site` enum names the scheduling seam; it does not change transform semantics.

Updated production entries:

```text
src/mir/compiler/mod.rs
src/mir/optimizer/core.rs
src/runner/json_v0_bridge/core.rs
src/runner/mir_json_v0.rs
```

All four entries remain in place. No entry is removed, reordered, or merged.

## Inventory Result

```text
production_entry_count=4
known_entry_count=4
unknown_entry_count=0
centralized_schedule_owner=1
entry_0_call_kind=schedule_facade
entry_1_call_kind=schedule_facade
entry_2_call_kind=schedule_facade
entry_3_call_kind=schedule_facade
```

## Stop Line

```text
do not remove callsite canonicalize entries
do not change schedule order
do not change semantic refresh timing
do not merge Program(JSON v0) bridge and MIR JSON loader behavior
do not change callsite canonicalization transform semantics
do not use this row to retire legacy MIR instructions
```

## Verification

```bash
cargo test -q callsite_canonicalize --lib
python3 -m unittest tools.hako_check.tests.test_callsite_canonicalize_entry_inventory
python3 tools/hako_check/callsite_canonicalize_entry_inventory.py --repo-root .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
