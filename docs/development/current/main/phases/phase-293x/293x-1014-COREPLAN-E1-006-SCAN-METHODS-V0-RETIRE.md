# COREPLAN-E1-006: scan_methods v0 retire

Status: Landed
Date: 2026-06-14
Scope: BoxShape-only retire of one routed legacy v0 family.

## Target

```text
loop_scan_methods_v0
```

## Decision

Retire the dedicated `loop_scan_methods_v0` route/facts/recipe/lowering
family. The focused scan-methods fixtures are now covered by existing owners:

```text
LoopSimpleWhile
LoopCondBreak
flowbox/adopt
```

This card does not add accepted source shapes and does not change the
planner_required policy. It removes a one-shape compatibility box after the
replacement routes are already covered by focused smoke fixtures.

## Acceptance

```text
one_v0_box_retired=1
active_v0_box_count=1
route_wiring_removed_for_one_box=1
facts_field_removed_for_one_box=1
accepted_shape_added=0
focused_fixture_gate_green=1
```

## Proof

```bash
bash tools/checks/coreplan_scan_methods_v0_retire_guard.sh
bash tools/checks/coreplan_active_v0_inventory_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_blocker_scan_methods_loop_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_scan_methods_program_block_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_scan_methods_nested_loop_depth1_methodcall_min
```

Note:

```text
selfhost_blocker_scan_methods_loop_min is a full selfhost import fixture. Its
row in phase29bq_fast_gate_cases.tsv carries timeout=60; the global fast-gate
default remains 10s for ordinary cases.
```

## Next

```text
COREPLAN-E1-007-SCAN-PHI-VARS-V0-RETIRE
```
