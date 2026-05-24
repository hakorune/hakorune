---
Status: Landed
Date: 2026-05-24
Scope: define repeated measurement policy before winner claims.
Blocker: MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY-295X-001
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-27-MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT.md
  - tools/checks/k2_wide_phase295x_repeated_measurement_policy_guard.sh
---

# 295x-28 Mimalloc Comparison Repeated Measurement Policy

## Decision

Close:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY-295X-001
```

Phase-295x now has a repeated measurement policy before any performance or
memory winner claim.

The policy is:

```text
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=5
summary=min,median,max
canonical_rss_collector=external-time
internal_rss_evidence=preserved
winner_claim=0
```

The initial workload pack is:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER-295X-001
```

Reason: the policy is now fixed. The next row should implement a small runner
that executes the selected workload pack using the policy and emits repeated
evidence without winner claims.

## Stop Line

This row does not implement the repeated measurement runner, compute winners,
require RSS parity, replace allocators, install hooks, enable provider/DLL
seams, or open worker/TLS, atomics, remote-free stress, abandoned heap stress,
OSVM page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_repeated_measurement_policy_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
