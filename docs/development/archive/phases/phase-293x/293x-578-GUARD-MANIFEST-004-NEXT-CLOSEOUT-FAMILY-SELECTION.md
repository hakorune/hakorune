# 293x-578 GUARD-MANIFEST-004 Next Closeout Family Selection

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-004` is the planning row selected by
`GUARD-MANIFEST-003`.

The first public `k2_wide_*` wrapper family now delegates through
`run_row_guard.sh`, with thick bodies behind `tools/checks/impl/`. The next row
should select one more closeout family or a shared helper extraction based on
the remaining duplication.

Selected family:

```text
reclaim scheduler closeout guards:
  tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh
  tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh
  tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_closeout_guard.sh
  tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_closeout_guard.sh
  tools/checks/k2_wide_hako_alloc_reclaim_scheduler_scalar_lane_closeout_guard.sh
```

Next selected row:

```text
GUARD-MANIFEST-005
```

`GUARD-MANIFEST-005` should use the same wrapper migration pattern as
`GUARD-MANIFEST-003`: public wrapper stays stable, thick body moves to
`tools/checks/impl/`, and `guard_rows.toml` owns the command.

## Scope

- Review remaining hako_alloc closeout guards.
- Decide whether the next slice should migrate another family or extract a
  shared closeout helper.
- Keep the next slice narrow and behavior-free.

## Stop Lines

- No allocator `.hako` behavior.
- No compiler acceptance change.
- No dev-gate / allocator-wide manifest pilot wiring.
- No all-family batch migration.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM004.1` | Inventory remaining closeout duplication. | one next family/helper row is selected. | no implementation bundle |
| `GM004.2` | Create next implementation card. | next card exists and is selected current. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Selected the reclaim scheduler closeout family as the next manifest-backed
  public-wrapper migration.
- Chose wrapper migration over helper extraction because the bodies are similar
  but not uniform enough for low-risk helper extraction in the same row.

