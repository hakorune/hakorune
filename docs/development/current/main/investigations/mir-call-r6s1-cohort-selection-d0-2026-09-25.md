# MIR-CALL-R6S1-COHORT-SELECTION-D0 — select the R6-S1 producer cohort

Status: open
Date: 2026-09-25
Parent: MIR-CALL-R6S0-TRANSPORT-EMITTER-SPLIT-S0 (landed; R6-S0 complete)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" (~line 127) row R6-S1.

## Decision

Bounded design row — design only. Select the exact R6-S1 cohort:

```text
R6-S1  one canonical producer cohort:
       existing source authority -> mandatory MirCall -> one typed consumer;
       delete that cohort's old writer/reissuer in the same series.
```

Acceptance requires a complete Promote/Stop/Delete tuple for the
selected cohort: the existing source authority (issuer), the mandatory
`MirCall` edge it already produces or must produce, the single typed
consumer, the cohort's old writer/reissuer to delete in-series, the
finite affected caller set, and the verification method — all named
from already-landed owners, no new authority.

This card selects the cohort; it does not implement the migration.

## Scope

### Inventory

Design-stop census within the call-producer surface only:
`src/mir/builder/calls/` writers (`emit_unified_call`,
`emit_finalized_generic_call_v1`, `emit_legacy_call`,
`create_legacy_call`), `MirJsonV0Loader` repair/canonicalize sites,
JoinIR lowering call producers, and their typed consumers. Do not
repeat the whole-repository Call/R7 census.

### Refusal

No production code, fixtures, fallback, guard, or receipt. No
`CallV2`. Do not reopen parked lanes. Do not select a cohort whose
consumer set is not finite or whose old writer has live production
callers outside the series.

### Reopening

If no cohort has a complete tuple, record `NoSafeSlice` per cohort
with the missing element and an observable reopen trigger.

## Exit

An accepted Decision naming the R6-S1 cohort (issuer, MirCall edge,
typed consumer, delete-set, finite callers, verification) or
`NoSafeSlice` with per-cohort missing elements and reopen triggers.
On acceptance, work_mode returns to `fast` for the bounded cohort
row and `next_execution_card` names it.
