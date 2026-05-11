---
Status: Complete
Date: 2026-05-12
Scope: phase lock for exact `usize` semantics before mimalloc migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# 294x-00 Usize Semantic Phase Lock

## Goal

Cut a new phase for Hakorune `usize` completeness before the mimalloc `.hako`
port starts using `usize` in live allocator state.

## Decision

Phase 294x owns exact pointer-sized unsigned integer semantics.

Phase 293x remains the parent real-app/mimalloc lane, but the active next work
is now the `usize` semantic foundation because mimalloc is being used to raise
Hakorune's low-level expressivity.

## Stop Line

Until the relevant 294x rows land:

- hako_alloc live numeric fields stay `i64`;
- `usize` remains annotation metadata only;
- negative sentinel indexes stay signed;
- no backend may silently treat exact `usize` as the old `Integer(i64)` lane.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
