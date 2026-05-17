# 293x-579 GUARD-MANIFEST-005 Reclaim Scheduler Closeout Thin Wrappers

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-005` is the implementation row selected by
`GUARD-MANIFEST-004`.

The selected target is the reclaim scheduler closeout family. The top-level
`k2_wide_*` names must remain stable, while the thick bodies move behind
manifest-owned implementation commands.

Next selected row:

```text
GUARD-MANIFEST-006
```

`GUARD-MANIFEST-006` should pick the next remaining hako_alloc closeout family
after the scheduler wrappers are manifest-backed.

## Selected Scripts

```text
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_scalar_lane_closeout_guard.sh
```

## Stop Lines

- No deletion or rename of public top-level guard entrypoints.
- No broad generator rewrite.
- No helper extraction in this row.
- No dev-gate / allocator-wide manifest pilot wiring.
- No allocator `.hako` behavior.
- No compiler acceptance change.
- No backend `.inc` route or matcher change.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM005.1` | Move thick scheduler closeout bodies behind implementation commands. | implementation commands run old checks. | public names stay |
| `GM005.2` | Add manifest rows. | `run_row_guard.sh --only <id>` runs each row. | no schema change |
| `GM005.3` | Thin public wrappers. | public scripts delegate to row runner only. | no body duplication |
| `GM005.4` | Extend wrapper no-growth guard. | all selected wrappers are checked. | no dev_gate wiring |

## Required Evidence

```text
tools/checks/run_row_guard.sh --only hako-alloc-reclaim-scheduler-marker-closeout
tools/checks/run_row_guard.sh --only hako-alloc-reclaim-scheduler-ledger-closeout
tools/checks/run_row_guard.sh --only hako-alloc-reclaim-scheduler-ledger-consume-closeout
tools/checks/run_row_guard.sh --only hako-alloc-reclaim-scheduler-ledger-roundtrip-closeout
tools/checks/run_row_guard.sh --only hako-alloc-reclaim-scheduler-scalar-lane-closeout
bash tools/checks/k2_wide_manifest_wrapper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added manifest rows for the five selected reclaim scheduler closeout guards.
- Moved thick command bodies to `tools/checks/impl/`.
- Kept public `tools/checks/k2_wide_*_closeout_guard.sh` paths as stable thin
  wrappers around `run_row_guard.sh --only <id>`.
- Extended `tools/checks/k2_wide_manifest_wrapper_guard.sh` to cover the
  scheduler closeout wrappers.

