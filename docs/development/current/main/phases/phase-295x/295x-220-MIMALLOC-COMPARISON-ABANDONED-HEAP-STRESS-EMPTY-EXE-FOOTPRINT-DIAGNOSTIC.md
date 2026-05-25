---
Status: Current
Date: 2026-05-25
Scope: diagnose the fixed abandoned-heap stress exact-EXE empty footprint.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-219-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-BASELINE-BREAKDOWN-SELECTION.md
  - tools/allocator/mimalloc_hako_empty_exe_footprint.py
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_empty_exe_footprint_diagnostic_guard.sh
---

# 295x-220 Abandoned Heap Stress Empty EXE Footprint Diagnostic

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-002
```

Add a diagnostic-only report:

```text
output_contract=mimalloc-comparison-hako-empty-exe-footprint-diagnostic-v0
baseline_workload=representative-empty-v0
diagnostic_workload=representative-empty-noio-v0
```

The report observes:

```text
hako_empty_evidence_external_rss_median_bytes
hako_empty_noio_external_rss_median_bytes
hako_empty_evidence_minus_noio_rss_bytes
hako_evidence_exe_file_bytes / PT_LOAD / section / NEEDED fields
hako_noio_exe_file_bytes / PT_LOAD / section / NEEDED fields
c_runner_file_bytes / PT_LOAD / section / NEEDED fields
c_mimalloc_library_file_bytes
```

Static ELF footprint is diagnostic evidence, not an RSS claim:

```text
static_footprint_evidence=1
static_footprint_is_rss_claim=0
baseline_shrink_action=0
winner_claim=0
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-002
```

The follow-on should classify the observed baseline into likely next seams:

```text
evidence output path dominated
static/loadable footprint dominated
runtime init dominated
measurement unresolved
```

It should select exactly one next diagnostic or shrink seam.

## Stop Line

This row does not reduce baseline RSS, change compiler/linker/runtime behavior,
open runtime-init instrumentation, compute memory/performance winners, require
RSS parity, enable provider/DLL or host replacement seams, install hooks, or
open worker/TLS, atomics, remote-free stress, abandoned heap stress, OSVM
page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_empty_exe_footprint_diagnostic_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
