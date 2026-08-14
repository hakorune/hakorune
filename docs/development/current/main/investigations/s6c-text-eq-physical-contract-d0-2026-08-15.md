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

## Census result at HEAD 830fcfc0f8

The logical owner is present, but no neutral physical owner is present. The
source relation is issued by `s6c_typed_input` as
`Equal(Text, Text) -> Bool`, and the V2 Recipe owns only the typed `TextEq`
row plus its `NonFaulting` execution classification. Neither is a physical
target or ABI issuer. The existing CoreMethod target issuer covers
`StringLen` and `StringSubstring`, but has no string-equality row. The
runtime `StringBox::equals`, generic MIR `CompareOp::Eq`, JoinIR comparison,
name-based `equals` rewrite, and the Dynamic evidence taxonomy are all
non-authority paths for this cohort.

Therefore the current decision is explicitly:

```text
NoSafeSlice::MissingS6CTextEqPhysicalOwner
```

No `ReadyEntry`, host/session, Builder, MIR, CFG/SSA/PHI, or physical caller
may be opened until one owner co-seals the target, receiver/argument/result
lanes, Home, semantic effect, execution/outcome policy, ABI, and wire
revision.

## Ingress census correction

The completed prephysical ingress is a source/logical boundary, not an effect
ledger or physical session. Its fixed placement census is:

```text
15 item placements = 13 operation rows + 1 If + 1 Exit
source-bound calls = 2 (Length, Substring)
```

The following projections must remain distinct and must not be promoted to a
new semantic effect authority:

```text
Recipe operation-family census: ReadBinding 4, WriteBinding 1,
  CallSlot 2, pure expression operations 6
resolver BodyEffect census:    Call 2, Write 1
CoreMethod semantic effect:     PureRead 2
V2 execution census:            NonFaulting 11, FaultBeforeNormalResult 0,
  ExternallyBoundOutcome 2
```

Operation evidence also retains anchor multiplicity. In particular, the
single Recipe `BodyIndexRead` feeds two distinct source occurrences (the
substring argument and the slice-end `Add` lhs), so a future physical view
must not collapse it to one `source_anchor`.

The callable Tail `return -1` remains a separate Completion/exit subview of
the same retained product. It is excluded from Loop operation/control rows,
but must remain borrowable together with the exact Loop Return; hiding it
would force a forbidden second pairing later.

## Stop conditions

Return to `NoSafeSlice::MissingS6CTextEqPhysicalOwner` if the census finds no
single issuer for the physical target, if the target requires selector/name
reselection, or if Home/ABI/effect/wire must be invented from MIR or runtime
implementation details. Do not open ReadyEntry or a physical session in that
case.

The stop is lifted only by a named existing owner or an accepted new owner
decision. A generic equality helper, a selector-derived symbol, or a MIR
`Compare` lowering does not satisfy this condition.

## Evidence and non-claims

The ingress I0 receipt is complete and caller-zero. This design row is the
only current execution authority after that receipt. It may consume no code,
fixture, or production route; it records only the named owner decision and
the negative matrix for missing, swapped, foreign, wrong-ABI, wrong-placement,
and non-Bool TextEq contracts.
