# LOOP-COMMON-RECURSIVE-AFTER-R3

Status: next executable row after R2
Decision: bounded continuation/After physicalization; R2 closeout is required
Date: 2026-08-08

## Purpose

Use the closed R1 segment layout and the R2 Callable segment-block receipt to
issue one neutral recursive After/edge receipt. The row must preserve the
existing Canonical CFG/Binding SSA/PhiTxn owners and must not open Generic G0
physical emission.

## Required boundary

```text
PreparedLoopPhysicalLayoutV1
  -> exact segment/block receipt
  -> neutral recursive transfer writer
  -> one After continuation receipt
```

Recipe/JoinSig remain the only logical authorities. The R3 product may consume
the R1 `Jump`, `Predicate`, and `OpenNestedLoop` transfer facts, but it may not
re-interpret Recipe, rediscover segments, or introduce a second CFG/SSA/PHI
owner. Callable parity is the only physical canary.

## Explicit non-goals

```text
Generic G0 physical emission
G0 carrier/parameter lowering
Tail or Completion redesign
production selector/caller switch
retry/fallback
collector/publication changes
M8/M9 coverage
legacy deletion
```

## Acceptance gates

```text
Callable segment After/edge receipt is owner-branded and exact.
Every R2 segment transfer is consumed once; missing/foreign/duplicate rows reject.
The existing Canonical CFG/Binding SSA/PhiTxn owners remain the only physical owners.
Failure discards the unpublished function session as one transaction.
All changed source/check files remain below 800 lines.
The same implementation commit updates docs/reference/**, the affected
README, CURRENT_STATE/workstream/task mirrors, and this task's closeout.
```

R3 must stop again before G0 physicalization. The next design boundary is the
G0 I1/R0 canary only after R3 closes.
