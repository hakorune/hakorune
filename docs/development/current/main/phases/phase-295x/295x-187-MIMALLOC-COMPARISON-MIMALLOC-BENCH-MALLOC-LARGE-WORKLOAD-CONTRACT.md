---
Status: Landed
Date: 2026-05-25
Scope: define the first `.hako` workload alignment contract against external `mimalloc-bench` `malloc-large`.
Blocker: MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-87-MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG.md
  - docs/development/current/main/phases/phase-295x/295x-88-MIMALLOC-COMPARISON-HAKO-RECORD-ERGONOMICS-CLEANUP.md
  - docs/development/current/main/phases/phase-295x/295x-hakmem-external-results-catalog-v0.toml
  - apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako
  - tools/allocator/hakmem_external_bench.py
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_malloc_large_workload_contract_guard.sh
---

# 295x-187 Mimalloc Comparison Malloc-Large Workload Contract

## Decision

Close:

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

Select the existing huge-ish `.hako` evidence shape as the first alignment
contract against the external `mimalloc-bench` `malloc-large` family:

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

Reason:

- the external corpus catalog already selected `mimalloc-bench-malloc-large`
  as the next workload family;
- the catalog notes that `malloc-large` is allocator-focused and aligns with
  `.hako` huge-ish/page-source evidence;
- this keeps provider / DLL / replacement seams closed and stays at the
  contract layer.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-295X-RUN-001
```

The next row should run the external `mimalloc-bench` `malloc-large` corpus
and the selected huge-ish `.hako` evidence through the comparison normalizer
while keeping RSS and winner claims closed.

## Stop Line

This row does not:

- run the external `malloc-large` evidence path;
- claim RSS or winner parity;
- add benchmark warmup or summary-statistics policy;
- enable provider packages, DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_malloc_large_workload_contract_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
