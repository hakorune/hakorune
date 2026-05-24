---
Status: Landed
Date: 2026-05-24
Scope: select the phase-294x mimalloc comparison closeout row.
Blocker: MIMALLOC-COMPARISON-PHASE-CLOSEOUT-SELECTION-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-242-MIMALLOC-COMPARISON-PRESENTATION-CHAIN-CONSOLIDATION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-243 Mimalloc Comparison Phase Closeout Selection

## Decision

Close `MIMALLOC-COMPARISON-PHASE-CLOSEOUT-SELECTION-001`.

Select `PHASE-294X-MIMALLOC-COMPARISON-CLOSEOUT-001` as the next blocker.

The comparison-quality vertical slice and presentation chain have enough
refreshed evidence to close the mimalloc-facing detour inside phase-294x. Use
one final closeout card to summarize validated evidence, parked rows, and the
next return path to the usize semantic foundation backlog.

## Closeout Scope

The next row should record:

- latest validated V5 `.hako` / C mimalloc schema alignment;
- explicit C mimalloc runner evidence status;
- result ledger, summary, reporting, first-conclusion, and presentation chain
  status;
- parked deeper presentation-only extension rows;
- parked provider/DLL/host replacement/native allocator rows.

## Stop Line

The next row must remain docs/closeout-only. It must not:

- rerun repeated or heavy benchmark packs;
- enable provider activation, host allocator replacement, hooks, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`;
- open worker/TLS, atomics, remote-free stress, abandoned-heap stress, or native
  allocator replacement claims;
- drain broad `usize` field groups outside the comparison slice.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
