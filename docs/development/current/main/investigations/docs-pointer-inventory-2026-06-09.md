Status: Active
Date: 2026-06-09
Scope: current docs pointers and archive map for the 296x cleanup lane.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-296x/README.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
  - docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
  - docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md

# Docs Pointer Inventory

The current cleanup lane existed because the restart surface was too wide and
the pointer chain was hard to read. That cleanup is landed. This note stays as
the compact map for the archived docs cleanup lane and the current mimalloc
source-level owner lane.

## Implementation Status

```text
implementation_gap_count=0
remaining_work=docs_pointer_cleanup_only
owner_runtime_and_layout_table_lanes=implemented
```

The rows that still look active in older phase cards are historical pointer
rows, not open code gaps. Use this note to tell “what is really still missing”
from “what is only stale in the docs mirrors.”

## Read First

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `CURRENT_TASK.md`
3. `docs/development/current/main/05-Restart-Quick-Resume.md`
4. `docs/development/current/main/10-Now.md`
5. `docs/development/current/main/phases/phase-296x/README.md`
6. `docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md`
7. `docs/development/current/main/workstreams/mimalloc-current.md`

## Thin Current Pointers

- `CURRENT_STATE.toml`: machine-readable active lane / blocker / latest card
- `CURRENT_TASK.md`: root restart anchor only
- `05-Restart-Quick-Resume.md`: minimal restart checklist
- `10-Now.md`: one-screen docs dashboard
- `phase-296x/README.md`: phase front summary
- `296x-90-mimalloc-benchmark-taskboard.md`: compact queue view
- `mimalloc-current.md`: active workstream and parking lot
- `hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md`: exact-front comparison evidence and next optimization focus

## What Remains

- thin the mirrors further if the restart surface becomes noisy again
- keep stale Active labels from being mistaken for implementation gaps
- keep archive notes as the only place where long landed chronology lives

## Archive Pointers

- `docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md`
  - historical mimalloc workstream ledger
- `docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md`
  - historical taskboard ledger
- `docs/development/current/main/investigations/mimalloc-current-docs-slim-archive-2026-06-08.md`
  - docs-slim archive for the current workstream
- `docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md`
  - one-shot C vs .hako mimalloc comparison sweep and next optimization focus

## Cleanup Rule

Keep the current mirrors thin. Put long queue history in the archive notes and
point to them instead of copying the landed chronology into every restart file.
