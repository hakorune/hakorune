---
Status: Landed
Date: 2026-05-24
Scope: run the selected workload pack with repeated measurement policy.
Blocker: MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-29-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
  - tools/checks/k2_wide_phase295x_repeated_measurement_pack_run_guard.sh
---

# 295x-30 Mimalloc Comparison Repeated Measurement Pack Run

## Decision

Close:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN-295X-001
```

The selected workload pack now runs under:

```text
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=5
canonical_rss_collector=external-time
winner_claim=0
```

The pack covers:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

Each workload emits external RSS min/median/max evidence for `.hako` and C
mimalloc. The report still does not rank or declare a winner.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT-295X-001
```

Reason: the repeated measurement pack now executes. The next row should close
the measurement pack and select whether to add presentation-only reporting,
open a next workload family, or pause phase-295x.

## Stop Line

This row does not compute winners, require RSS parity, enable provider/DLL/
replacement seams, install hooks, or open worker/TLS, atomics, remote-free
stress, abandoned heap stress, OSVM page-source parity, or native allocator
replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_repeated_measurement_pack_run_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
