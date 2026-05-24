---
Status: Landed
Date: 2026-05-24
Scope: select the next same-workload family after small-block count evidence.
Blocker: MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-14-MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - tools/checks/k2_wide_phase295x_next_workload_seam_selection_guard.sh
---

# 295x-15 Mimalloc Comparison Next Workload Seam Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION-295X-001
```

Select:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-295X-REFRESH-001
```

The next same-workload family is:

```text
representative-realloc-aligned-v0
```

This follows the completed `representative-small-block-v0` evidence pack:

- workload ids match;
- requested bytes match;
- allocation/free counts match;
- repeated RSS evidence exists;
- winner claims remain closed.

## Contract Requirements

Before comparing realloc/aligned evidence, the next row must fix workload shape
metadata:

```text
operation_family=realloc-aligned
operation_sequence_id=representative-realloc-aligned-v0-seq
free_order_id=ascending-release-v0
```

Why: the current small-block C runner frees even indexes before odd indexes,
while the `.hako` representative app releases sequentially. Count/requested/RSS
comparison is still valid, but reuse/realloc/moved/copy evidence needs explicit
operation-sequence and free-order contracts before deltas are trusted.

## Evidence Policy

The next runner/normalizer row may gate on:

- `workload_match=1`;
- `operation_family_match=1`;
- `allocation_count_delta=0`;
- `free_count_delta=0`;
- `requested_bytes_delta=0`;
- `realloc_count_delta=0`;
- `aligned_alloc_count_delta=0`.

It must keep these as side-by-side evidence only:

- realloc moved count;
- copied bytes;
- RSS deltas.

Those values may differ between model and C mimalloc behavior and must not be
used as winner or parity claims in this row family.

## Stop Line

This row does not:

- implement the realloc/aligned runner changes;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_next_workload_seam_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
