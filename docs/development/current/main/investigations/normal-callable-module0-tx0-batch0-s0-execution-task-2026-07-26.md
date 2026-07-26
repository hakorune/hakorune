---
Status: closed executable row
Date: 2026-07-26
Decision: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1
Row: NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0
Parent: NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-S0
Scope: seal one schema/correspondence batch from completed helpers, source Main, and physical Main drafts
ceremony_tier: T1 bounded owner/evidence refactor
series_mode: BoxShape only; no accepted source/result shape grows
---

# NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0

## Outcome

```text
PreparedNormalCallableMainPhysicalV1
  -> canonical-key helper draft expectations
  + source Main expectation
  + physical-entry expectation
  -> one sealed unpublished batch
  or typed rejection retaining every prepared draft
```

`NormalModuleTransactionSchemaV1` remains the sole schema owner. This row
generalizes its Main-only adapter; it does not build a `MirModule`, verify a
candidate, drain a collector, or publish anything.

## Required laws

```text
helper order = existing consumed canonical-key schedule
helper identity = existing catalog key/symbol/arity evidence
source Main = sealed Main physical relation only
physical entry = sealed entry relation only
schema failure = helpers + source Main + physical draft retained
retry/fallback/reclassification = 0
```

## Acceptance

```text
one NormalModuleTransactionSchemaV1 producer = 1
new schema vocabulary = 0
Main-only schema adapter remains compatibility-only
candidate module/publication/process/backend authority = 0
all modified/new source/check files < 800 lines
```

## Focused evidence

```text
one helper + Main + physical produces three exact rows
helper key/symbol/arity drift rejects before batch escape
schema rejection retains every prepared draft
failure -> later success reuses the same Builder
```

## Next rows

```text
NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-COMMIT0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-G0
```

## Closeout

The adapter emits one source-Main row, canonical-key helper rows, and one
physical-entry row into the existing schema. Both success and injected schema
rejection retain the prepared drafts; no candidate module or publication owner
was added.
