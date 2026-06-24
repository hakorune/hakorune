# 296x-1043 CALLSITE-CANONICALIZE-ENTRY-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: callsite canonicalize entry inventory / residue triage

## Contract

```text
output_contract=hako-callsite-canonicalize-entry-inventory-v0
row_kind=report_only

production_entry_count=4
known_entry_count=4
unknown_entry_count=0

mir_compiler_entry=1
mir_optimizer_entry=1
program_json_v0_bridge_entry=1
mir_json_v0_loader_entry=1

transform_owner=src/mir/passes/callsite_canonicalize
single_transform_owner=1
centralized_schedule_owner=0

behavior_changed=0
canonicalize_entry_refactor_allowed=0
next_task=CALLSITE-CANONICALIZE-ENTRY-OWNER-DESIGN-001
summary=ok
```

## Purpose

The residue review flagged `canonicalize_callsites(...)` as a four-site
duplicate. This row makes that shape explicit without changing the pipeline.

The current state is:

```text
single transform owner:
  src/mir/passes/callsite_canonicalize/

production schedule entries:
  src/mir/compiler/mod.rs
  src/mir/optimizer/core.rs
  src/runner/json_v0_bridge/core.rs
  src/runner/mir_json_v0.rs
```

This is a real schedule-entry duplication, but it is not safe to collapse as a
drive-by cleanup because the entries belong to different pipeline surfaces:
AST compile, optimizer late-call/inline, Program(JSON v0) bridge, and MIR JSON
loader.

## Stop Line

```text
do not remove any canonicalize callsite entry in this row
do not reorder optimizer / compiler / bridge scheduling
do not change semantic refresh timing
do not change accepted MIR JSON or Program(JSON v0) shapes
do not treat the four entries as proof of four transform owners
```

## Verification

```bash
python3 -m unittest tools.hako_check.tests.test_callsite_canonicalize_entry_inventory
python3 tools/hako_check/callsite_canonicalize_entry_inventory.py --repo-root .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
