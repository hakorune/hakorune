# 293x-582 GUARD-MANIFEST-008 Final Closeout Wrapper Selection

Status: selected current
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

