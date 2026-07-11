---
Status: Landed
Date: 2026-04-26
Scope: phase-291x restart/current docs and smoke-selection operating cleanup.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-smoke-index.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# 291x-274 Docs / Smoke Operating Simplification

## Problem

The current-lane docs were drifting back into ledgers:

- `CURRENT_TASK.md` repeated long landed-card history.
- `10-Now.md` repeated the same cleanup sequence.
- `05-Restart-Quick-Resume.md` was no longer a 2-5 minute restart entry.
- `phase-291x/README.md` front-loaded card lists instead of navigation.
- Smoke paths were useful, but cards had no clear rule for when to repeat them.

This made the compiler-clean cleanup lane harder to resume than necessary.

## Decision

Keep current docs thin:

- `CURRENT_STATE.toml` is the current-state SSOT.
- numbered `291x-*` cards are the landed/rejected ledger.
- `291x-smoke-index.md` is the smoke-selection SSOT.
- restart/current mirrors carry only checkpoint, next pointer, and rules.

## Changes

- Replaced long history in `CURRENT_TASK.md` with compact restart and task
  pointers.
- Replaced long history in `10-Now.md` with current / next / rules sections.
- Replaced long history in `05-Restart-Quick-Resume.md` with restart-only
  guidance.
- Reduced `phase-291x/README.md` to navigation, durable rules, current
  checkpoint, and proof bundle.
- Reorganized `291x-smoke-index.md` into daily gate, boundary smokes,
  archive/historical references, and operating rules.

## Operating Rule

New phase-291x cards should not paste long smoke lists. They should cite the
family row in `291x-smoke-index.md` unless the card introduces a new durable
boundary, in which case the index is updated first.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
