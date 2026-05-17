# 293x-578 GUARD-MANIFEST-004 Next Closeout Family Selection

Status: selected current
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-004` is the planning row selected by
`GUARD-MANIFEST-003`.

The first public `k2_wide_*` wrapper family now delegates through
`run_row_guard.sh`, with thick bodies behind `tools/checks/impl/`. The next row
should select one more closeout family or a shared helper extraction based on
the remaining duplication.

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

