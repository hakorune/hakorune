---
Status: Landed
Date: 2026-05-24
Scope: close out the no-winner mimalloc comparison summary.
Blocker: MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-CLOSEOUT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-168-MIMALLOC-COMPARISON-SUMMARY-NO-WINNER.md
  - tools/allocator/mimalloc_comparison_summary_no_winner.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_closeout_guard.sh
---

# 294x-169 Mimalloc Comparison Summary No-Winner Closeout

## Decision

Close out the no-winner mimalloc comparison summary row.

The comparison vertical slice now has:

- same-workload hako and C mimalloc evidence;
- normalized requested-byte parity;
- single-run RSS presentation;
- repeated-run RSS min/max evidence;
- no-winner range-only summary;
- explicit closed seams for provider activation, host replacement, hooks,
  global allocator install, worker/TLS, atomics, and provider package / DLL
  generation.

This is enough comparison-quality evidence for the current phase. Further
comparison work should name a narrower evidence gap before reopening this lane.

## Follow-On Selection

Select:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-169
```

Return to explicit `usize` field-group selection for the next production
`hako_alloc` non-negative stored-field migration row.

## Stop Line

This closeout does not open:

- performance or memory-use winner claims;
- mean/median/statistical significance claims;
- provider activation;
- host allocator replacement;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution;
- provider package / DLL generation.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
