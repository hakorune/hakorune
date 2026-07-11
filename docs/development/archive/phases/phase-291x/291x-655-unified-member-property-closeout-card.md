---
Status: Landed
Date: 2026-04-28
Scope: close out the unified-member property compiler-cleanliness sub-lane
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
---

# 291x-655: Unified Member Property Closeout

## Goal

Cut the phase checkpoint after the unified-member property cleanup burst.

This is a docs/checkpoint card. It does not change parser or MIR behavior.

## Inventory

The sub-lane from `291x-636` through `291x-654` closed these surfaces:

- canonical `get` property syntax and compatibility with stored `get` names;
- parser-side property emission SSOT for `computed` / `once` / `birth_once`;
- `birth_once` constructor prologue emission and cycle validation;
- method/member postfix parsing and gate ownership;
- weak field stored-field contract;
- MIR property getter registration and property-read lowering surfaces;
- MIR property read coverage for computed, once, and birth_once;
- stale parser classifier and local stringly property kind cleanup;
- unified-member test ENV guard cleanup.

## Decision

Treat unified-member property cleanup as closed for this checkpoint.

The next work item returns to lane selection:

```text
phase-291x next compiler-cleanliness lane selection pending
```

If property work reopens, it should start from a concrete blocker or from the
remaining parser header/body parsing deduplication question, not from the
closed registry/emission/read-lowering surfaces.

## Remaining Candidates

These are not blockers for the phase cut:

- parser header/body parsing still has some local duplication across stored,
  computed, once, and birth_once member forms;
- exact `once` original exception preservation remains out of scope until catch
  binding is represented through MIR lowering;
- broader `plan/facts` and `lower::planner_compat` work still require a new
  family-sized BoxShape lane if reopened.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune -q
git diff --check
```

## Result

- Updated the current checkpoint docs from `291x-635` to this closeout.
- Kept the active blocker token as lane selection pending.
- Left detailed history in numbered phase cards, not in current mirrors.
