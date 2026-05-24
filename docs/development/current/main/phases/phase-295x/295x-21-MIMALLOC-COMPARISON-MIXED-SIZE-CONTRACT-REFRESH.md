---
Status: Landed
Date: 2026-05-24
Scope: add runner/evidence contract support for representative-mixed-small-v0.
Blocker: MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-295X-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-20-MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION.md
  - tools/allocator/c_mimalloc_explicit_runner.c
  - apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/main.hako
  - tools/checks/k2_wide_phase295x_mixed_size_contract_refresh_guard.sh
---

# 295x-21 Mimalloc Comparison Mixed-Size Contract Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-295X-REFRESH-001
```

The explicit C mimalloc runner and a narrow `.hako` exact-EXE evidence app now
publish the selected mixed-size workload contract:

```text
workload=representative-mixed-small-v0
operation_family=mixed-small
operation_sequence_id=representative-mixed-small-v0-seq
free_order_id=ascending-release-v0
allocation_count=16
free_count=16
requested_bytes=3096
```

The base output contracts remain unchanged:

- C: `allocator-comparison-c-mimalloc-explicit-runner-v0`;
- `.hako`: `hako-exe-memory-evidence-v0`.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-295X-RUN-001
```

The next row should run the C and `.hako` mixed-small evidence through the
normalizer and require structural count/requested parity while keeping RSS as
side-by-side evidence only.

## Stop Line

This row does not:

- run the normalized mixed-size comparison report;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, huge/OSVM execution, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mixed_size_contract_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
