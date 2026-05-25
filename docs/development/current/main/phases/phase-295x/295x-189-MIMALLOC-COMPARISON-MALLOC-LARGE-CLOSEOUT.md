---
Status: Landed
Date: 2026-05-25
Scope: close the malloc-large evidence alignment family and choose the next workload seam.
Blocker: MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-188-MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-RUN.md
  - docs/development/current/main/phases/phase-295x/295x-190-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
  - tools/checks/k2_wide_phase295x_malloc_large_evidence_run_guard.sh
  - tools/checks/k2_wide_phase295x_malloc_large_closeout_guard.sh
---

# 295x-189 Mimalloc Comparison Malloc-Large Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001
```

The `mimalloc-bench-malloc-large` alignment is closed with:

- the selected huge-ish `.hako` evidence shape;
- the external `mimalloc-bench` corpus bridge and benchres adapter;
- the comparison normalizer on the `.hako` side;
- RSS and winner claims still closed.

This closes the first external-corpus alignment slice for the `malloc-large`
family.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001
```

The next row should select baseline attribution for the external
`malloc-large` alignment family before any winner claim.

## Stop Line

This row does not:

- make RSS or winner claims;
- enable provider packages, DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_malloc_large_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
