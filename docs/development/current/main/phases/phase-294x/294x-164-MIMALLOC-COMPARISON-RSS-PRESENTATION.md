---
Status: Landed
Date: 2026-05-24
Scope: format same-workload single-run RSS evidence without winner claims.
Blocker: MIMALLOC-COMPARISON-RSS-PRESENTATION-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-163-MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT.md
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/allocator/mimalloc_comparison_rss_presentation.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_guard.sh
---

# 294x-164 Mimalloc Comparison RSS Presentation

## Decision

Add a presentation-only RSS report over the existing same-workload memory
evidence.

The row labels the evidence as single-run and keeps the raw RSS values visible:

```text
mimalloc-comparison-rss-presentation-v0
measurement_scope=single-run
rss_unit=bytes
```

It also emits MiB-scaled integer display helpers (`*_mib_x100`) so later docs or
reports can show the same evidence without ad hoc formatting.

## Stop Line

This row does not open:

- repeated-run aggregation;
- performance or memory-use winner claims;
- provider activation;
- host allocator replacement;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution.

## Follow-On

```text
MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT-001:
  close out the presentation-only RSS row, then choose repeated-run evidence or
  return to the next explicit `usize` field-group row.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
