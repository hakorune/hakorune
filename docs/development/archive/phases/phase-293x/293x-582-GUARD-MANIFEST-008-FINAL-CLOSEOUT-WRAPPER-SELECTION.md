# 293x-582 GUARD-MANIFEST-008 Final Closeout Wrapper Selection

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-008` is the planning row selected by
`GUARD-MANIFEST-007`.

The remaining public hako_alloc closeout wrappers are:

```text
tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_reuse_proof_closeout_guard.sh
```

This row should decide whether to migrate both as the final small closeout
wrapper batch or keep one for a separate helper-extraction row.

Selected target:

```text
final hako_alloc closeout wrapper batch:
  tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
  tools/checks/k2_wide_hako_alloc_reuse_proof_closeout_guard.sh
```

Next selected row:

```text
GUARD-MANIFEST-009
```

`GUARD-MANIFEST-009` should migrate these final two public hako_alloc closeout
wrappers behind manifest-owned implementation commands.

## Stop Lines

- No allocator `.hako` behavior.
- No compiler acceptance change.
- No dev-gate / allocator-wide manifest pilot wiring.
- No all-family batch outside the remaining hako_alloc closeout wrappers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM008.1` | Review the final two closeout wrappers. | implementation row is selected or helper extraction is selected. | no implementation bundle |
| `GM008.2` | Create next card. | next card exists and is selected current. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Selected the final two public hako_alloc closeout wrappers for manifest-backed
  wrapper migration.
- Kept helper extraction deferred; after this row, public closeout wrapper
  count is small enough to evaluate helpers from the implementation side.

