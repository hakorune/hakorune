---
Status: accepted; bounded BoxShape implementation row
Date: 2026-08-15
Decision: expose one canonical role-wise source-call view from the retained output product
Scope: S6C logical output call parity only; no consumer result dialect
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-LOGICAL-CALL-VIEW-R0

## Six-line brief

```text
Decision: Length and Substring are borrowed only as one canonical role-wise call view.
Source authority + canonical issuer: retained S6C output product; its existing input façade and fixed call rows are co-checked once.
Non-authority: raw Recipe/JoinSig, independent input() access, positional/name re-pairing, JoinModule/MIR, IDs, Artifact, fallback.
Fail-fast boundary: role, receiver, ordered args, result, owner/frame, placement, and source target drift reject before callback effect.
Smallest next slice: add private S6CLogicalCallWithSourceRefV1 pairs and remove the raw source-input escape from output consumer view.
Non-claims: no logical consumer result, JoinModule/MIR lowering, physical selection, selector, production, retry, or legacy retirement.
```

## Contract

`VerifiedS6CScanWithInitLogicalOutputV1::with_output` remains the only input
boundary. Its view exposes `calls()` returning exactly two role-wise pairs:

```text
Length    = fixed CallSlot row + retained Length source contract
Substring = fixed CallSlot row + retained Substring source contract
```

The raw `input()` accessor is removed from the output view. A future consumer
cannot obtain a source-call relation separately and pair it with another row.
The existing row arrays remain logical inspection data; no new key or source
authority is issued.

## Acceptance

```text
pair count = 2
roles = Length, Substring in fixed order
row role == source role
receiver/args/result == existing fixed Recipe-local rows
owner/frame/placement/operation/arity == retained source contracts
```

Focused negatives cover role swap, receiver/argument/result drift, and foreign
source pairing. The consumer result dialect remains a separate design stop;
this row returns no new semantic product and opens no physical effect.

## Implementation receipt (2026-08-15)

The output façade now exposes only a private role-wise pair view for Length
and Substring. Each pair carries the fixed logical CallSlot row together with
the retained source contract; the raw `input()` accessor is removed. Six
focused S6C tests, cargo check, format, diff, pointer, and Loop pre-cutover
guards are green. The consumer result dialect remains the next design stop.
