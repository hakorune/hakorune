# DYNAMIC-FAULT-EXIT-TRANSACTION-D0

Status: design consultation required; implementation 0
Date: 2026-08-10
Depends on: `LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0` closed
Authority:
`loop-recipe-v2-joinsig-dynamic-d0-design-task-2026-08-10.md`

## Goal

Define one atomic Dynamic exit transaction that preserves the primary outcome
and respects already-sealed JoinSig transfers without inferring Home from the
logical `Dynamic` class.

## Required census

Before selecting an implementation row, enumerate every exact cut point in
the unchanged `skip_while/4` semantic program:

```text
I6 Dynamic CallSlot fault before V10 publication
I6 normal publication and optional future local install
I7 Dynamic CallSlot fault after V10 exists
I12 inner Return to FunctionExit
I16 normal rebind followed by JoinSig Backedge
PredicateFalse to Loop After
outer Callable Tail / Completion
```

For each cut point, identify:

```text
primary outcome owner
which values are definitely materialized
which lexical locals may have a proved Home
which cleanup obligations exist
which JoinSig transfer is authorized
whether Completion participates
failure precedence if cleanup itself fails
```

## Open design questions

1. Which existing Dynamic invocation outcome is the sole primary Fault owner?
2. Which existing Home capability can prove an owner-bearing Dynamic value,
   without runtime-tag or Recipe-class inference?
3. Does one function-session exit transaction cover Fault, inner Return,
   Backedge scope leave, and normal After, or are logical authorization and
   cleanup preparation separate products?
4. How is cleanup prepared before the first physical effect while preserving
   a fault that occurs after earlier instructions were emitted?
5. What exact terminal consumes inner Return Completion, while the outer
   Return remains Callable Tail?
6. What is the typed precedence when cleanup fails while a primary Dynamic
   Fault is already active?

## Nonclaims

```text
Dynamic implies Home
runtime tag implies Home
V10/ch is always owner-bearing
Fault becomes a Recipe value or Loop Exit
Fault becomes a JoinSig edge
Completion consumption
physical CFG/cleanup emission
DraftSeal / collector / publication
retry / fallback
```

## Consultation acceptance

- one owner table for Fault, Home capability, cleanup, JoinSig, Completion,
  and physical session;
- exact cut-point matrix for the unchanged source;
- typed primary/cleanup failure precedence;
- one proposed move-only transaction boundary with no caller-supplied pieces;
- explicit first implementation slice and still-forbidden claims;
- implementation/task/reference update order;
- every planned Rust source file remains below 800 lines.

Do not create `VerifiedDynamicHomeV1`, a test-only Home constructor, a
Fault-as-Result wrapper, or a physical cleanup edge before this Decision is
accepted.
