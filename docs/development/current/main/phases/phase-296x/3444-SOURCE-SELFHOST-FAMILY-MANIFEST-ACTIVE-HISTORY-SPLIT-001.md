# 3444 - SOURCE-SELFHOST-FAMILY-MANIFEST-ACTIVE-HISTORY-SPLIT-001

## Token

```text
SOURCE-SELFHOST-FAMILY-MANIFEST-ACTIVE-HISTORY-SPLIT-001
```

## Purpose

Separate the Source Selfhost family guard's active index from its historical
traceability ledger without dropping or renaming any row.

## Canonical Files

```text
active:
  docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-active-v1.json

history:
  docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-history-v1.jsonl

compatibility snapshot (frozen):
  docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json
```

Active contains only `current_semantic` and `current_maintenance` rows. The
history ledger contains `current_queue` and `historical_traceability` rows,
one JSON object per line. The v0 manifest remains callable for legacy tools but
must not receive new rows.

## Required Invariants

1. Active and history token sets are disjoint.
2. Their union exactly equals the frozen v0 token set, plus explicitly added
   post-split active rows.
3. Every row still points to an existing card, fixture, and legacy guard when
   those fields are present.
4. CURRENT_STATE's blocker exists in active.
5. New rows enter active or history explicitly; no implicit role promotion.
6. The family guard reads active for current validation and validates the
   split ledger as traceability evidence.

## Non-Claims

```text
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Result

The baseline split is 27 active rows and 397 history rows. The active index
now accepts explicit post-split active rows, while the history ledger remains
unchanged and the v0 manifest remains a compatibility snapshot.
