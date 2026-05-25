---
Status: Landed
Date: 2026-05-25
Scope: select baseline attribution for the external `malloc-large` evidence family.
Blocker: MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-189-MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-295x/295x-191-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE.md
  - docs/development/current/main/phases/phase-295x/295x-34-MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE.md
  - docs/development/current/main/phases/phase-295x/295x-35-MIMALLOC-COMPARISON-MEMORY-GAP-INCREMENTAL.md
  - tools/checks/k2_wide_phase295x_malloc_large_memory_gap_attribution_selection_guard.sh
---

# 295x-190 Malloc-Large Memory Gap Attribution Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001
```

Select baseline attribution as the next comparison seam for the external
`malloc-large` alignment family:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001
```

This follows the normalized `malloc-large` evidence closeout and keeps the
comparison lane at the attribution layer before any winner claim.

The selected attribution plan keeps the standard repeated-measurement
baseline vocabulary:

```text
workload=representative-empty-v0
operation_family=empty-baseline
operation_sequence_id=representative-empty-v0-seq
free_order_id=no-release-v0
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=5
canonical_rss_collector=external-time
winner_claim=0
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001
```

The next row should add empty-baseline evidence for the external
`malloc-large` alignment family under the repeated measurement policy before
any winner claim.

## Stop Line

This row does not compute baseline-subtracted deltas, make winner claims,
require RSS parity, enable provider/DLL/replacement seams, install hooks, or
open worker/TLS, atomics, remote-free stress, abandoned heap stress, OSVM
page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_malloc_large_memory_gap_attribution_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
