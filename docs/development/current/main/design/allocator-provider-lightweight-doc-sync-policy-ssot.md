---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: allocator provider lane docs sync policy after M86.
Related:
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# Allocator Provider Lightweight Docs Sync Policy (SSOT)

## Problem

M76-M86 kept the allocator provider lane safe, but each row started requiring
too many human-written mirrors:

- phase README landed bullet;
- phase real-app taskboard checkbox;
- global mimalloc capability taskboard row and order number;
- task-breakdown completed row;
- task-breakdown immediate ladder;
- task-breakdown post-M75 ladder;
- task-breakdown dependency order;
- task-breakdown next-step prose;
- current-state latest-card pointer;
- card;
- check script index and gate wiring.

That makes small docs-first rows feel like ledger maintenance. It also violates
the existing current-docs update policy, which says per-card mirror updates
should stay compact.

## Decision

For M87 and later allocator provider rows, the mandatory per-row docs set is:

1. the row SSOT or fixture contract;
2. the row card;
3. `docs/development/current/main/CURRENT_STATE.toml`;
4. the dedicated guard, when the row needs one;
5. `docs/tools/check-scripts-index.md` plus gate wiring, only when a new public
   guard script is added;
6. code/test docs only when the row changes their contract.

These mirrors are no longer per-row mandatory:

- `docs/development/current/main/phases/phase-293x/README.md`;
- `docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md`;
- `docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md`;
- full progress tables in
  `docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md`.

Update those mirrors only at a closeout row, when the lane/blocker changes, or
when the mirror's own stable contract changes.

## Guard Contract

M87+ row guards must not require phase README, phase taskboard, or the global
mimalloc taskboard to repeat the row. A row guard should prove:

- its own SSOT/card/fixture exists;
- any new guard is indexed and wired into the allocator gate;
- forbidden activation behavior has not leaked in;
- current-card pointer only when the guard belongs to the current card.

Past guards must not pin `CURRENT_STATE.latest_card` or
`CURRENT_STATE.latest_card_path`.

## Closeout Pattern

When several rows have landed, a closeout row may update the heavy mirrors in
one batch:

- phase README summary;
- phase real-app taskboard;
- global mimalloc taskboard;
- allocator provider task-breakdown progress tables.

That closeout should be coverage-only unless a separate implementation row
explicitly changes behavior.

## Non-Goals

- no behavior change;
- no allocator activation;
- no generated docs helper in this card;
- no physical archive/move of old phase history.
