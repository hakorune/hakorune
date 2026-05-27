---
Status: Landed
Date: 2026-05-27
Scope: define the workload matrix and subject ids for `.hako` mimalloc, C mimalloc, hakozuna reference, and provider package evidence.
Blocker: HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/phases/phase-296x/296x-42-HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION.md
---

# 296x-43 Hako Mimalloc Performance Parity Workload Matrix

## Decision

Close:

```text
HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001
```

Define the stable subject ids:

```text
hako_mimalloc_exact_exe
c_mimalloc_explicit_runner
hakozuna_reference
provider_package_hako_mimalloc_explicit
```

Define the workload ladder:

```text
small_block_alloc_free
realloc_aligned
remote_free_publish_collect
mixed_small
large_huge_backing
osvm_page_source
hakmem_selected_family
```

The first baseline pack uses the active comparison pair:

```text
hako_mimalloc_exact_exe
c_mimalloc_explicit_runner
```

The other subjects remain reference/parked until a later row opens them
explicitly.

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001
```

The next row should run the first same-workload baseline pack with the accepted
repeated-measurement policy.

## Stop Line

This row does not run benchmarks, activate providers, replace the process
allocator, install hooks, or claim a winner.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_parity_workload_matrix_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
