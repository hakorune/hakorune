# 293x-580 GUARD-MANIFEST-006 Remaining Closeout Family Selection

Status: selected current
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-006` is the planning row selected by
`GUARD-MANIFEST-005`.

The segment and reclaim scheduler closeout public wrappers are now
manifest-backed. The next row should pick the next remaining hako_alloc
closeout family or decide that helper extraction is now the better cleanup.

## Remaining Candidates

```text
tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_osvm_fast_path_route_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_reuse_proof_closeout_guard.sh
```

## Stop Lines

- No allocator `.hako` behavior.
- No compiler acceptance change.
- No dev-gate / allocator-wide manifest pilot wiring.
- No all-family batch migration.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM006.1` | Inventory remaining closeout duplication. | one next family/helper row is selected. | no implementation bundle |
| `GM006.2` | Create next implementation card. | next card exists and is selected current. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

