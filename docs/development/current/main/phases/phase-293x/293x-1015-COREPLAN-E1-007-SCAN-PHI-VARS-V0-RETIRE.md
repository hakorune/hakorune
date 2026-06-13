# COREPLAN-E1-007: scan_phi_vars v0 retire

Status: Landed
Date: 2026-06-14
Scope: BoxShape-only retire of the last routed legacy v0 family.

## Target

```text
loop_scan_phi_vars_v0
```

## Decision

Retire the dedicated `loop_scan_phi_vars_v0` route/facts/recipe/lowering
family. The focused PhiInjector / `_collect_phi_vars` fixtures are now covered
by existing owners:

```text
LoopSimpleWhile
LoopCondBreak
```

This card does not add accepted source shapes and does not change the
planner_required policy. It removes the final active `loop_*_v0` compatibility
box after focused smoke fixtures prove the replacement routes.

## Acceptance

```text
one_v0_box_retired=1
active_v0_box_count=0
route_wiring_removed_for_one_box=1
facts_field_removed_for_one_box=1
accepted_shape_added=0
focused_fixture_gate_green=1
```

## Proof

```bash
bash tools/checks/coreplan_scan_phi_vars_v0_retire_guard.sh
bash tools/checks/coreplan_active_v0_inventory_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only phi_injector_len_loop
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only phi_injector_var_step_len_loop
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_phi_injector_k_loop_no_exit_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_phi_collect_outer_loop_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only phi_injector_nested_loop_no_exit_var_step_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_phi_injector_nested_loop_count_min
```

## Next

```text
COREPLAN-E1-CLOSEOUT
```
