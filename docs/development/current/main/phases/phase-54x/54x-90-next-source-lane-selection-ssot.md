---
Status: SSOT
Date: 2026-04-04
Scope: choose the next source lane after `phase-53x` landed so the repo does not drift back into a stale default.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-53x/README.md
  - docs/development/current/main/phases/phase-53x/53x-91-task-board.md
---

# 54x-90 Next Source Lane Selection SSOT

## Intent

- keep the handoff from `phase-53x` explicit
- rank the remaining source lane candidates by leverage, not by cleanup fatigue
- decide the next active source lane before any new work starts

## Canonical Reading

- `phase-53x` is landed and handed off.
- `rust-vm` is historical / proof / compat keep, not day-to-day ownership.
- `vm-hako` stays reference/conformance and is not part of archive/delete wholesale.
- `phase-54x` exists only to select the next source lane cleanly.

## Boundaries

- do not reopen `--backend vm` as a daily default
- do not turn proof-only keeps into the next owner lane
- do not select a lane just to mirror previous cleanup sequences
- keep `cargo check --bin hakorune` and `git diff --check` green during the decision

## Success Conditions

- candidate lanes are inventoried
- a successor lane is selected
- current mirrors point at the active lane
- the handoff is concise and honest
