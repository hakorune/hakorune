---
Status: Landed
Date: 2026-05-25
Scope: add presentation-only reporting for repeated measurement evidence.
Blocker: MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-31-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
  - tools/allocator/mimalloc_repeated_measurement_presentation.py
  - tools/checks/k2_wide_phase295x_repeated_measurement_presentation_guard.sh
---

# 295x-32 Mimalloc Comparison Repeated Measurement Presentation

## Decision

Close:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION-295X-001
```

`tools/allocator/mimalloc_repeated_measurement_presentation.py` now formats the
repeated measurement report into compact presentation-only evidence:

```text
output_contract=mimalloc-comparison-repeated-measurement-presentation-v0
presentation_only=1
winner_claim=0
```

The presentation preserves per-workload `.hako` and C mimalloc external RSS
min/median/max values and the median delta. It does not rank allocators or
claim a winner.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001
```

Reason: repeated evidence shows a stable RSS gap. The next row should select a
memory-gap attribution plan that separates fixed process/runtime baseline from
workload-incremental RSS before any winner claim.

## Stop Line

This row does not compute winners, require RSS parity, enable provider/DLL/
replacement seams, install hooks, or open worker/TLS, atomics, remote-free
stress, abandoned heap stress, OSVM page-source parity, or native allocator
replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_repeated_measurement_presentation_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
