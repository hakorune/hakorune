# COREPLAN-E1 Closeout: active v0 zero

Status: Landed
Date: 2026-06-14
Scope: close out the active routed `loop_*_v0` retirement sequence.

## Decision

`COREPLAN-E1-002` through `COREPLAN-E1-007` retired all active routed
`loop_*_v0` compatibility boxes. No active routed legacy-v0 module remains.

```text
active_v0_box_count=0
accepted_shape_added=0
route_wiring_removed_for_all_active_v0=1
```

## Proof

```bash
bash tools/checks/coreplan_active_v0_inventory_guard.sh
bash tools/checks/coreplan_scan_phi_vars_v0_retire_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

Do not add another `loop_*_v0` box from inventory alone.

Next compiler expressivity work must start from a concrete failing fixture and
choose BoxCount vs BoxShape before implementation.

## Next

```text
COREPLAN-LOOP-WIRING-001
```
