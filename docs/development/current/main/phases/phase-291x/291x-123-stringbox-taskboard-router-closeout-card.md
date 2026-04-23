---
Status: Landed
Date: 2026-04-24
Scope: StringBox taskboard router follow-up wording closeout.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# 291x-123 StringBox Taskboard Router Closeout Card

## Decision

Close stale router follow-up wording in the StringBox taskboard after the current
StringBox, ArrayBox, and MapBox stable rows have landed.

This is docs-only BoxShape cleanup. It does not change router policy or CoreBox
behavior.

## Cleanup Scope

- mark `MapBox.clear` as the landed final mutating reset row, not remaining work
- replace the old `next safe cleanup` sentence with the current future-work
  rule: new route work needs a new one-family card
- keep deferred StringBox feature rows explicitly separate from router closeout

## Non-Goals

- no new StringBox rows
- no whole-CoreBox route flip
- no MapBox feature rows
- no code changes

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
