---
Status: Landed
Date: 2026-05-24
Scope: close out the presentation-only RSS report over same-workload evidence.
Blocker: MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-164-MIMALLOC-COMPARISON-RSS-PRESENTATION.md
  - tools/allocator/mimalloc_comparison_rss_presentation.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_closeout_guard.sh
---

# 294x-165 Mimalloc Comparison RSS Presentation Closeout

## Decision

Close out the presentation-only same-workload RSS report.

The landed report now gives a stable display contract for the single-run memory
evidence:

```text
mimalloc-comparison-rss-presentation-v0
measurement_scope=single-run
rss_unit=bytes
workload_match=1
requested_bytes_delta=0
winner_claim=0
repeated_runs=0
```

The report keeps raw byte values and MiB-scaled integer helpers together, so
docs and future report rows do not need ad hoc RSS formatting.

## Follow-On Selection

Select:

```text
MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-001
```

The next row should add a narrow repeated-run evidence contract over the same
workload pair. It should keep the initial scope small: collect repeated samples
and publish counts/min/max or similarly simple aggregates, without making a
winner claim.

## Stop Line

This closeout does not open:

- performance or memory-use winner claims;
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
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
