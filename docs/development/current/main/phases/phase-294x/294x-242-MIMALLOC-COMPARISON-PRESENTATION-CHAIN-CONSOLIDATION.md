---
Status: Landed
Date: 2026-05-24
Scope: consolidate the mimalloc comparison presentation refresh chain.
Blocker: MIMALLOC-COMPARISON-PRESENTATION-CHAIN-CONSOLIDATION-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-228-MIMALLOC-COMPARISON-VSLICE-REFRESH.md
  - docs/development/current/main/phases/phase-294x/294x-241-MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-REFRESH.md
---

# 294x-242 Mimalloc Comparison Presentation Chain Consolidation

## Decision

Close `MIMALLOC-COMPARISON-PRESENTATION-CHAIN-CONSOLIDATION-001`.

The refreshed comparison chain now has enough evidence for the phase-294x
comparison-quality vertical slice:

- V5 `.hako` / C mimalloc schema alignment remains green.
- C mimalloc explicit runner evidence remains green.
- result ledger, summary, reporting, and first-conclusion packs remain green.
- presentation-only, presentation follow-on, extension, extension follow-on, and
  extension follow-on extension packs remain green through MIMAP-500A.

Deeper presentation-only extension rows are parked. They repeat the same closed
benchmark/provider stop lines without adding new comparison evidence.

## Next Row

Select `MIMALLOC-COMPARISON-PHASE-CLOSEOUT-SELECTION-001` as the next blocker.
It should decide whether phase-294x closes after this comparison refresh, or
whether one final closeout card is needed before returning to the usize semantic
foundation backlog.

## Parked Rows

Do not refresh deeper rows by default:

- MIMAP-504A and later presentation extension follow-on chains;
- repeated presentation-only extension rows;
- benchmark rerun or presentation expansion rows without a concrete consumer.

They may be reopened only if a later comparison consumer names the exact fields
or report shape it needs.

## Stop Line

This row does not:

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
