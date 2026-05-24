---
Status: Landed
Date: 2026-05-24
Scope: implement repeated measurement runner without winner claims.
Blocker: MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-28-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/checks/k2_wide_phase295x_repeated_measurement_runner_guard.sh
---

# 295x-29 Mimalloc Comparison Repeated Measurement Runner

## Decision

Close:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER-295X-001
```

`tools/allocator/mimalloc_repeated_measurement_runner.py` now executes repeated
`.hako` and C mimalloc evidence samples under:

```text
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=5
summary=min,median,max
canonical_rss_collector=external-time
winner_claim=0
```

The runner consumes existing single-run evidence scripts rather than opening a
new allocator integration seam. The C runner and `.hako` runner both publish
`external_peak_rss_bytes`; runner-internal RSS evidence remains preserved.

The legacy small-block `.hako` proof app now emits the same operation-family,
operation-sequence, and even/odd free-order contract as the C runner.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN-295X-001
```

Reason: the runner exists and has been validated on one representative
workload. The next row should run the selected workload pack with the repeated
measurement profile and still keep winner claims closed.

## Stop Line

This row does not compute winners, require RSS parity, enable provider/DLL/
replacement seams, install hooks, or open worker/TLS, atomics, remote-free
stress, abandoned heap stress, OSVM page-source parity, or native allocator
replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_repeated_measurement_runner_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
