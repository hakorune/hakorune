---
Status: Landed
Date: 2026-05-24
Scope: close the huge-ish comparison workload family.
Blocker: MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-26-MIMALLOC-COMPARISON-HUGE-ISH-EVIDENCE-RUN.md
  - tools/checks/k2_wide_phase295x_huge_ish_evidence_run_guard.sh
  - tools/checks/k2_wide_phase295x_huge_ish_closeout_guard.sh
---

# 295x-27 Mimalloc Comparison Huge-Ish Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT-295X-001
```

The `representative-huge-ish-v0` workload family is closed with explicit C
mimalloc evidence, `.hako` exact-EXE evidence, normalized same-workload report,
structural count/requested/large-request parity, RSS evidence-only handling,
and winner claims still closed.

This closes the initial phase-295x same-workload family set:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY-295X-001
```

Reason: the comparison lane now has multiple executable same-workload evidence
families. Before any performance or memory winner claim, the next row should
define the repeated measurement policy: sample count, warmup count, summary
statistic, environment capture, binary/library identity, and RSS collector
rules.

## Stop Line

This row does not implement the repeated measurement runner, require RSS
parity, make performance or memory winner claims, enable provider/DLL/
replacement seams, or open worker/TLS, atomics, remote-free, abandoned heap,
OSVM page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_huge_ish_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
