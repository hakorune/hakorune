# DYNAMIC-FAULT-CATALOG-EXHAUSTIVE-R0

Status: classification half closed; reject/visibility hardening parked P1
Date: 2026-08-10
Depends on: `DYNAMIC-FAULT-CUTPOINT-CATALOG-I0` closed

## Change

Replace the wildcard non-Fault classification in the bounded Dynamic Fault
catalog with one exhaustive operation projection owned beside the V2 operation
schema.

```text
LoopOperationExecutionClassV2
  NonFaulting
  FaultBeforeNormalResult { family, result }
  ExternallyBoundOutcome { result }
```

The Fault catalog consumes this projection. It does not infer faultability
from a runtime tag, provider, selector name, `MirType`, or physical emitter.
Adding a new `LoopOperationV2` variant must force a compile-time update of the
projection; `_ => continue` is forbidden.

## Same-row hardening

- preserve the typed Fault-catalog reject through the semantic-program issuer
  as `FaultCutPoints(reject)` or stable categories
  `MissingRelation | IdentityConflict | ShapeMismatch | CoverageMismatch`;
- keep catalog rows and borrow views private to the semantic-program subtree
  until a real exit-transaction consumer exists;
- retain the exact six-row golden and all current source/Recipe relations;
- add a structural guard proving no wildcard classification and no facade
  visibility widening.

## Non-claims

```text
no new Fault-capable operation
no Fault value or JoinSig edge
no Home or cleanup obligation
no runtime/provider dispatch
no CFG/MIR/Completion/physicalization
```

This BoxShape row changes classification ownership only. It must not be folded
into a new Dynamic operator semantic Decision or source-family expansion.

## Landed subset

The exhaustive `LoopOperationExecutionClassV2` projection and Fault-catalog
consumer are closed with the operator-contract I0. The remaining row is only:

```text
typed reject category preservation
caller-zero Fault view visibility narrowing
```

Do not reopen the already-closed operation classification or create a second
faultability table when finishing those two items.
