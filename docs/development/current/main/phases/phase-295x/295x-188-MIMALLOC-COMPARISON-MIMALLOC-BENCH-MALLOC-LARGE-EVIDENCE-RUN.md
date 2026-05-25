---
Status: Landed
Date: 2026-05-25
Scope: run the external `mimalloc-bench-malloc-large` corpus and the selected huge-ish `.hako` evidence through the comparison normalizer.
Blocker: MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-295X-RUN-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-187-MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT.md
  - docs/development/current/main/phases/phase-295x/295x-189-MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-295x/295x-87-MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG.md
  - docs/development/current/main/phases/phase-295x/295x-hakmem-external-results-catalog-v0.toml
  - apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako
  - tools/allocator/hakmem_external_bench.py
  - tools/allocator/hakmem_benchres_adapter.py
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_malloc_large_evidence_run_guard.sh
---

# 295x-188 Mimalloc Comparison Malloc-Large Evidence Run

## Decision

The current blocker is the normalized evidence run for the selected
`mimalloc-bench-malloc-large` family.

The row will compare the external corpus output against the selected huge-ish
`.hako` evidence shape through the comparison normalizer while keeping the
base output contracts unchanged.

Expected same-workload fields:

```text
workload=representative-huge-ish-v0
operation_family=huge-ish
operation_sequence_id=representative-huge-ish-v0-seq
free_order_id=ascending-release-v0
allocation_count=2
free_count=2
requested_bytes=4194321
large_request_count=1
```

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001
```

The next row closes the external corpus alignment family and chooses the next
workload seam.

## Stop Line

This row does not:

- claim RSS parity as a winner;
- add benchmark warmup or final summary statistics;
- open provider packages, DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_malloc_large_evidence_run_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
