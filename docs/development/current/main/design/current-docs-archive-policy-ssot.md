---
Status: SSOT
Date: 2026-05-15
Scope: current docs archive and slimming policy.
Related:
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-293x/README.md
---

# Current Docs Archive Policy

## Decision

Current docs are restart/navigation surfaces, not landed-history storage.

Use this split:

```text
current entry:
  CURRENT_STATE.toml
  CURRENT_TASK.md
  05-Restart-Quick-Resume.md
  10-Now.md
  active phase README

active execution:
  active card
  latest-card pointer
  taskboard only when its stable contract changes

durable design:
  design/*-ssot.md

historical execution:
  phase archive cards
  landed ledger
  old cards with optional forwarding stubs
```

## What Stays Live

Keep these in their current paths:

- `CURRENT_STATE.toml` and thin restart mirrors.
- `CURRENT_STATE.phase_status`.
- `CURRENT_STATE.latest_card_path`.
- active phase README.
- active taskboards named by `CURRENT_STATE.taskboard`.
- design SSOTs that remain current policy owners.
- check scripts and fixtures used by active or recent guards.

## What Moves To Archive

Landed phase cards can move when all are true:

- Status is landed / historical / superseded.
- The card is not `CURRENT_STATE.phase_status`.
- The card is not `CURRENT_STATE.latest_card_path`.
- The card is not the active row for a current taskboard.
- The card has no current guard that requires the old path.

Archive buckets for phase card directories:

```text
docs/development/current/main/phases/phase-293x/archive/cards/293x-000-099/
docs/development/current/main/phases/phase-293x/archive/cards/293x-100-199/
docs/development/current/main/phases/phase-293x/archive/cards/293x-200-299/
docs/development/current/main/phases/phase-293x/archive/cards/293x-300-399/
```

Keep a forwarding stub at the old path only when a current doc, guard, or script
still references the old path. If no tracked current reference exists, the
archive ledger is enough.

## Ledger Rule

Long landed history belongs in a ledger, not in current mirrors.

Recommended shape:

```text
Card | Status | Summary | Guard | Commit
```

`CURRENT_STATE.toml` keeps only a short `landed_tail`.

```text
target maximum:
  12 rows
```

## Guard Reference Rule

Implementation guards should not force taskboards to become landed-history
ledgers.

Prefer guard inputs in this order:

1. active card
2. durable SSOT
3. check-scripts index
4. code/test fixture
5. taskboard only when the taskboard's own contract changed

Do not add a taskboard assertion just to prove a card landed.

## First Slimming Phase

`DOCS-SLIM-001` owns policy and inventory only:

- add this SSOT
- trim `CURRENT_STATE.landed_tail`
- add guardrails to prevent regrowth
- produce archive bucket counts
- do not physically move old cards yet

Physical archive moves are `DOCS-SLIM-002+`.
