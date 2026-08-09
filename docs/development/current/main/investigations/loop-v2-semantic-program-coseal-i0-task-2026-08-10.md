# LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0

Status: active after `LOOP-RECIPE-V2-JOINSIG-DYNAMIC-I0` closeout
Date: 2026-08-10
Design authority:
`loop-recipe-v2-joinsig-dynamic-d0-design-task-2026-08-10.md`

## Goal

Consume the exact existing Dynamic source/Recipe/envelope aggregate, derive
its JoinSig internally, require After from that same JoinSig, and issue one
move-only semantic program without a split/re-pair API.

## Boundary

```text
VerifiedDynamicFullLoopSourceRecipeEnvelopeV2
  -> internal common JoinSig elaboration
  -> internal require_after(L0, B0, Dynamic)
  -> VerifiedLoopSemanticProgramV2
```

The caller supplies none of:

```text
owner
root Loop key
JoinSig
After binding
Continuation
```

The existing two-site Completion remains a sibling. The co-seal proves only
the exact partition: inner source Return is the Recipe Return path; outer
source Return is the Callable Tail path after Loop After.

## Acceptance

- exact source/Recipe/JoinSig/After origin is sealed atomically;
- mixed/foreign Recipe, JoinSig, source product, After, or Completion partition
  rejects;
- After is exactly `L0/B0/Dynamic` and is borrow-only from the semantic
  program;
- no external `from_after`, raw owner, or split issue API is reachable from the
  V2 path;
- V10/ch remains a local relation and is absent from carrier/After identity;
- implementation, focused tests, owner README, public MIR reference, and task
  receipt update land together;
- all touched source files remain below 800 lines.

## Nonclaims

```text
Fault exit transaction
Home capability/install/cleanup
physical transfer/layout/CFG
Tail operand physicalization
Completion consumption
DraftSeal / collector / publication
```
