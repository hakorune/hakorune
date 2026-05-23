---
Status: Landed
Date: 2026-05-24
Scope: summarize repeated-run mimalloc comparison evidence without winner claims.
Blocker: MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-167-MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT.md
  - tools/allocator/mimalloc_comparison_summary_no_winner.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_guard.sh
---

# 294x-168 Mimalloc Comparison Summary No Winner

## Decision

Add a small summary formatter over the repeated-run RSS evidence.

The row emits:

```text
mimalloc-comparison-summary-no-winner-v0
comparison_claim=range-only
winner_claim=0
```

The summary is intentionally descriptive. It names the workload, sample count,
RSS byte ranges, and closed seams. It does not rank hako vs C mimalloc.

## Stop Line

This row does not open:

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

## Follow-On

```text
MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-CLOSEOUT-001:
  close out the no-winner summary row, then return to the next explicit
  `usize` field-group row unless a later comparison card names a narrower
  evidence gap.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
