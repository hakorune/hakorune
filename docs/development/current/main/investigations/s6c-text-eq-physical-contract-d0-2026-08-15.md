---
Status: design stop; no physical implementation opened
Date: 2026-08-15
Work mode: design_stop
Classification: T2 BoxShape
---

# S6C-TEXT-EQ-PHYSICAL-CONTRACT-D0

## Six-line brief

```text
Decision: name one physical TextEq contract for the S6C session, or stop as NoSafeSlice::MissingS6CTextEqPhysicalOwner; do not infer CompareEq or a generic StringBox helper.
Source authority + canonical issuer: the retained resolver Equal(Text,Text)->Bool relation, the V2 Recipe TextEq row, and the existing execution-class seal; one new contract owner must co-seal them.
Non-authority: selector/name lookup, MIR Compare, Recipe ordinal, generic runtime equality, Dynamic/llvmlite lanes, physical IDs, fallback, and retry.
Fail-fast boundary: missing target, receiver/argument/result ABI, Home, effect, or placement parity rejects before ReadyEntry/session/Builder effect.
Smallest next slice: read-only census of existing TextEq emitters/contracts and one accepted owner decision with a focused negative matrix; no new semantic receipt until the owner is named.
Non-claims: no TextEq implementation, ReadyEntry, host/session, CFG/SSA/PHI, callable Tail emission, production selector, cutover, or legacy retirement.
```

## Required decision

The S6C prephysical ingress intentionally carries TextEq as source/typed
logical evidence only. This row must decide whether an existing neutral
runtime/ABI contract can be borrowed, or whether the cohort has no safe
physical slice. A generic `CompareEq` or a selector-derived StringBox target
does not satisfy the boundary.

The accepted owner must define, without re-reading source or MIR:

```text
receiver relation: Subject substring result
argument relation: Needle
result relation: Bool normal result
placement: Loop body / TextEq If condition
effect: existing semantic Core/operation effect, not MIR EffectMask
execution: non-faulting or an explicitly named outcome contract
Home/ABI/wire: exact physical contract and revision
```

## Stop conditions

Return to `NoSafeSlice::MissingS6CTextEqPhysicalOwner` if the census finds no
single issuer for the physical target, if the target requires selector/name
reselection, or if Home/ABI/effect/wire must be invented from MIR or runtime
implementation details. Do not open ReadyEntry or a physical session in that
case.

## Evidence and non-claims

The ingress I0 receipt is complete and caller-zero. This design row is the
only current execution authority after that receipt. It may consume no code,
fixture, or production route; it records only the named owner decision and
the negative matrix for missing, swapped, foreign, wrong-ABI, wrong-placement,
and non-Bool TextEq contracts.
