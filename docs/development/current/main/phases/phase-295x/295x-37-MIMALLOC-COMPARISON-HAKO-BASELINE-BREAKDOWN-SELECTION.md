---
Status: Landed
Date: 2026-05-25
Scope: select a narrow `.hako` exact-EXE baseline breakdown seam.
Blocker: MIMALLOC-COMPARISON-HAKO-BASELINE-BREAKDOWN-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-36-MIMALLOC-COMPARISON-MEMORY-GAP-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - tools/checks/k2_wide_phase295x_hako_baseline_breakdown_selection_guard.sh
---

# 295x-37 Mimalloc Comparison Hako Baseline Breakdown Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-HAKO-BASELINE-BREAKDOWN-SELECTION-295X-001
```

Select an empty exact-EXE footprint diagnostic as the next seam:

```text
MIMALLOC-COMPARISON-HAKO-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-001
```

The next row should observe the fixed `.hako` exact-EXE baseline before any
shrink work. The selected diagnostic combines:

```text
existing empty evidence exact-EXE RSS
empty no-output exact-EXE RSS control
exact-EXE file / PT_LOAD / section footprint
exact-EXE dynamic dependency inventory
C empty runner reference footprint
```

The main split to expose is:

```text
hako_empty_evidence_rss - hako_empty_noio_rss
```

If that delta is large, evidence output / print / stdout path is likely part
of the fixed cost. If it is small, linked runtime, loadable footprint, or
runtime initialization become stronger candidates.

## Stop Line

This row does not implement the diagnostic tool, reduce baseline RSS, change
compiler/linker/runtime behavior, open runtime-init instrumentation, compute
performance or memory winners, require RSS parity, enable provider/DLL or host
replacement seams, install hooks, or open worker/TLS, atomics, remote-free
stress, abandoned heap stress, OSVM page-source parity, or native allocator
replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_hako_baseline_breakdown_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
