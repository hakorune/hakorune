---
Status: Landed
Date: 2026-05-24
Scope: close the mixed-size comparison workload family.
Blocker: MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-22-MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-RUN.md
  - tools/checks/k2_wide_phase295x_mixed_size_evidence_run_guard.sh
  - tools/checks/k2_wide_phase295x_mixed_size_closeout_guard.sh
---

# 295x-23 Mimalloc Comparison Mixed-Size Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT-295X-001
```

The `representative-mixed-small-v0` workload family is closed with explicit C
mimalloc evidence, `.hako` exact-EXE evidence, normalized same-workload report,
structural count/requested parity, RSS evidence-only handling, and winner claims still closed.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION-295X-001
```

Reason: small-block, realloc/aligned, and mixed-small workload families now
have executable comparison evidence. The next comparison seam should select a
huge-ish workload family while keeping OSVM/page-source parity claims,
provider activation, process allocator replacement, and winner claims parked.

## Stop Line

This row does not implement a huge-ish runner path, require RSS parity, add
benchmark summary policy, make winner claims, enable provider/DLL/replacement
seams, or open worker/TLS, atomics, remote-free, abandoned heap, OSVM
page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mixed_size_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
