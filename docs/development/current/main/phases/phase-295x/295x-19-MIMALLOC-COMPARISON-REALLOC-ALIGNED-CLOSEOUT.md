---
Status: Landed
Date: 2026-05-24
Scope: close the realloc/aligned comparison workload family.
Blocker: MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-18-MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-RUN.md
  - tools/checks/k2_wide_phase295x_realloc_aligned_evidence_run_guard.sh
  - tools/checks/k2_wide_phase295x_realloc_aligned_closeout_guard.sh
---

# 295x-19 Mimalloc Comparison Realloc/Aligned Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT-295X-001
```

The `representative-realloc-aligned-v0` workload family is closed with:

- explicit C mimalloc workload dispatch;
- `.hako` exact-EXE memory evidence;
- normalized same-workload report;
- workload / operation family / sequence / free-order parity;
- allocation/free/requested/realloc/aligned structural parity;
- moved/copy/RSS preserved as evidence-only fields;
- winner claims still closed.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION-295X-001
```

Reason: small-block and realloc/aligned same-workload families now have
executable comparison evidence. The next useful comparison seam is a mixed-size
workload family selection. Repeated benchmark policy remains parked until a
later row that is explicitly preparing a winner claim.

## Stop Line

This row does not:

- introduce a mixed-size runner implementation;
- make moved/copy/RSS parity claims;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_realloc_aligned_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
