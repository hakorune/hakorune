---
Status: Active
Date: 2026-04-22
Scope: app lane の `ArrayBox` surface contract / execution dispatch / exposure state を docs-first で固定する phase front。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-290x/290x-90-arraybox-surface-canonicalization-design-brief.md
  - docs/development/current/main/phases/phase-290x/290x-91-arraybox-surface-task-board.md
  - docs/development/current/main/phases/phase-290x/290x-92-arraybox-surface-inventory-ledger.md
---

# Phase 290x: ArrayBox surface canonicalization

- Status: Active
- Date: 2026-04-22
- Purpose: `ArrayBox` の truth を `surface contract / execution dispatch / exposure state` の 3 層に切り分け、app lane で触りやすい入口を先に固定する。
- Active proving ground:
  - `apps/kilo_nyash/enhanced_kilo_editor.hako`
- Sibling guardrail:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - phase-137x is observe-only unless app work produces a real blocker

## Decision

`ArrayBox.insert()` が landed したあとも、repo にはまだ 1 つの surface SSOT がない。

今の問題は「機能が足りない」ことより、

```text
implemented
exposed
documented
smoke-pinned
```

が別々に進んでいることだよ。

phase-290x は、この drift を止めるための docs-first phase。
最初のゴールは `ArrayBox` を次の 3 層で読むことを固定すること。

1. `surface contract`
2. `execution dispatch`
3. `exposure state`

## Reading Order

1. `docs/development/current/main/phases/phase-290x/290x-90-arraybox-surface-canonicalization-design-brief.md`
2. `docs/development/current/main/phases/phase-290x/290x-91-arraybox-surface-task-board.md`
3. `docs/development/current/main/phases/phase-290x/290x-92-arraybox-surface-inventory-ledger.md`

## Current Rule

- docs-first: phase-290x starts with pointer cleanup, vocabulary lock, and inventory
- no broad runtime refactor beyond the catalog / invoke seam without a new small card
- `length()` is the canonical user-facing name; `size()` is compatibility alias unless a later decision explicitly changes that
- `apps/std/array.hako` is sugar, not the semantic owner
- phase-137x perf work stays observe-only while phase-290x is active

## Implementation State

The first implementation card has landed:

```text
Array surface catalog
  -> ArrayMethodId
  -> ArrayBox::invoke_surface(...)
  -> thin VM / registry / method-resolution / effects consumers
```

Stable Array surface smoke is landed:

```text
tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
```

Next follow-up is return to app-lane slices unless another ArrayBox drift is found.
