---
Status: Landed
Date: 2026-05-24
Scope: close out the repeated-run RSS evidence row.
Blocker: MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-166-MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE.md
  - tools/allocator/mimalloc_comparison_repeated_run_evidence.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_closeout_guard.sh
---

# 294x-167 Mimalloc Comparison Repeated-Run Evidence Closeout

## Decision

Close out the repeated-run RSS evidence row.

The landed contract now has:

- same-workload hako/C samples;
- repeated sample count;
- min/max RSS byte ranges for hako and C;
- min/max RSS delta ranges;
- `winner_claim=0`;
- closed provider / host replacement / hook / global allocator seams.

The evidence is still intentionally modest. It is repeated-run evidence, not a
statistical benchmark suite and not a performance or memory-use winner claim.

## Follow-On Selection

Select:

```text
MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-001
```

The next row should create a small summary formatter over the repeated-run
evidence. It may name the workload, sample count, RSS ranges, and closed seams.
It must keep winner claims and provider/host replacement behavior closed.

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
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
