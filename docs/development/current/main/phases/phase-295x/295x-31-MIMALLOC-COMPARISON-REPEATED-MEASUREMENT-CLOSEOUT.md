---
Status: Landed
Date: 2026-05-24
Scope: close the repeated measurement pack.
Blocker: MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-30-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN.md
  - tools/checks/k2_wide_phase295x_repeated_measurement_pack_run_guard.sh
  - tools/checks/k2_wide_phase295x_repeated_measurement_closeout_guard.sh
---

# 295x-31 Mimalloc Comparison Repeated Measurement Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT-295X-001
```

The repeated measurement pack is closed for the selected phase-295x workload
families. Evidence now includes external RSS min/median/max for `.hako` and C
mimalloc across:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

Winner claims remain closed.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION-295X-001
```

Reason: the repeated measurement pack emits a stable report, but humans still
need a compact presentation that makes the result readable while preserving
`winner_claim=0`.

## Stop Line

This row does not compute winners, require RSS parity, enable provider/DLL/
replacement seams, install hooks, or open worker/TLS, atomics, remote-free
stress, abandoned heap stress, OSVM page-source parity, or native allocator
replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_repeated_measurement_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
