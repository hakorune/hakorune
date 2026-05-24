---
Status: Landed
Date: 2026-05-24
Scope: select the next comparison-quality seam after the same-workload refresh.
Blocker: MIMALLOC-COMPARISON-NEXT-PORT-SEAM-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-08-MIMALLOC-COMPARISON-SAME-WORKLOAD-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - tools/checks/k2_wide_phase295x_next_seam_selection_guard.sh
---

# 295x-09 Mimalloc Comparison Next Seam Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-NEXT-PORT-SEAM-SELECTION-295X-001
```

The next useful seam is not a new `.hako` allocator feature yet. The
same-workload execution path is now green, but the comparison still has only a
single-run RSS sample. Before adding a wider port seam, refresh the existing
repeated-run evidence path in phase-295x.

Select:

```text
MIMALLOC-COMPARISON-REPEATED-RUN-295X-REFRESH-001
```

This improves comparison quality while keeping winner claims closed. It also
keeps the provider/DLL/replacement surfaces parked.

## Stop Line

This row does not:

- make performance or memory winner claims;
- add a new workload family;
- change either runner output schema;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_next_seam_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
