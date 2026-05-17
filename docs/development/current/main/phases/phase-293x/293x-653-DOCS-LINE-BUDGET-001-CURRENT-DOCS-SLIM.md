# 293x-653 DOCS-LINE-BUDGET-001 Current Docs Slim

Status: landed
Date: 2026-05-18

## Decision

`DOCS-LINE-BUDGET-001` is a BoxShape cleanup sidecar before `MIMAP-143A`.

Current restart docs should not carry 2000+ line landed-history ledgers. The
active path should stay compact and point to archived full ledgers when exact
historical evidence is still useful.

## Scope

- Slim phase-137x current docs into compact observe-only entries.
- Move old phase-29bq / phase-29ao long history logs behind forwarding stubs.
- Split `mimalloc-allocator-first-task-granularity-ssot.md` into:
  - compact active SSOT with current decision, stop lines, row table, and guard
    anchors;
  - archived full historical ledger snapshot.
- Add a line-budget rule to the current docs archive policy.
- Update one stale historical closeout guard string after the MIMAP-142A owner
  row widened the memory README wording.

## Stop Lines

- No allocator behavior.
- No compiler route behavior.
- No source syntax change.
- No current blocker change.
- No guard semantics change beyond the stale exact README string.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_backing_set_helper_guard.sh
bash tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_closeout_guard.sh
bash tools/checks/impl/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh
bash tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh
git diff --check
```

## Landed Result

No non-archive file under `docs/development/current/main` remains over 2000
lines. `MIMAP-143A` remains the active allocator blocker.
