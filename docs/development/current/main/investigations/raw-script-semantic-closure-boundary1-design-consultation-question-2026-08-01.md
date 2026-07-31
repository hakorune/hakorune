# RAW-SCRIPT-SEMANTIC-CLOSURE-BOUNDARY1-D0 — design consultation question

## Ask

Please choose the next clean semantic boundary for the normal Script pipeline.
Do not propose an implementation until one finite semantic family has a named
production consumer and an exact old-edge deletion.

The current blocker is not a missing lowering function. It is the boundary
between the already-complete Script lexical closure and the remaining
Deferred Script root path.

## Current state

Latest landed commits:

```text
1f17bc93d1  refactor(mir): compose andor script lexical closure
4b8970a322  docs(mir): close census49 nosafeslice
```

The current source-only Complete closure is:

```text
Literal
Variable backed by a prior Local
Local with an admitted initializer
Print
Unary except Weak
Binary including And/Or
Await
CheckExpr with recursively admitted items
```

And/Or was admitted through the existing short-circuit owner. Its CFG/PHI,
left/right source receipts, result/type policy, diagnostics, failure discard,
and fresh-request reuse are unchanged and tested.

The remaining common Deferred edge is:

```text
src/mir/builder/program_root_lowering.rs
  Deferred Script request
  -> RawInvocationSourceTransportV1::script_root(())
```

This edge must not be removed until a complete semantic producer for one
specific family exists.

## Census49 evidence

The following candidates were reviewed read-only by workers:

```text
QMark / Match / EnumMatch / TryCatch / If / Loop
  -> control, exit, cleanup, or branch facts

Call / MethodCall / FieldAccess / New
  -> receiver/header/object authority

Array / Map / Record / RecordUpdate
  -> allocation, birth/type metadata, element or field writes

Weak
  -> WeakRef::New requires a BoxRef/type precondition absent from the
     current lexical product; accepting `weak 1` would change failure policy

Lambda
  -> owner forest, capture order, ClosureBodyId/publication

Box
  -> catalog and lifecycle ownership
```

Existing `*_with_port_v1` functions are lowering owners. Their existence alone
does not prove that the Script semantic admission product can own the source
facts required to delete the Deferred edge.

Already completed and not to be reopened:

```text
NORMAL-PROGRAM-COLLECTOR-DRAIN0-I0-R0
  landed at 67488ff283

SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001
  existing Deferred sunset/ratchet SSOT
```

## Decision choices

### Candidate A — choose one next semantic family

Select the smallest remaining family that can be represented by one complete
source/semantic product and one existing lowering owner. The answer must name
the family; “expand Script generally” is not acceptable.

### Candidate B — define a new Script semantic boundary first

If no remaining family is safe, define the boundary contract that must precede
the next I0/R0. This must be a design stop only, not a proof-only owner or a
new compatibility route.

### Candidate C — retain all remaining Deferred shapes

Acceptable only if the existing sunset/ratchet is sufficient for final
conformance and the answer explains what final-pipeline completion means while
the Deferred edge remains. Do not create another docs-only sunset row.

## Questions to answer

1. Which candidate is correct now, and why is it the smallest honest step?
2. What is the exact semantic family (source grammar and child-demand closure)?
3. Which existing production caller consumes the new product?
4. What exact old edge is deleted in the same I0/R0 commit?
5. What are the success product, rejection/deferred product, and diagnostic
   precedence rules?
6. How does the existing Deferred sunset/ratchet prove monotonic progress by
   fixture identity, not merely by a count or percentage?
7. What is the first real `.hako` or production fixture that becomes Complete?
8. What evidence proves parity, failure discard, fresh-request reuse, and exact
   source coverage?

## Mandatory constraints

```text
one semantic family per row
one resolver/admission traversal
one selected production route
one old-edge deletion in the implementation commit
existing lowering owner reused without reclassification
Deferred is a terminal selection, never fallback/retry
```

The following are hard stops:

```text
synthetic FunctionDeclaration for Program/Script
second resolver or second semantic forest
partial forest or best-effort projection
mixed selected/Deferred routing inside one request
semantic diagnostic precedence moved earlier without proof
ValueId, ABI, physical slot, or capture materialization in a lexical row
Lambda, Control, Call/Object, Box, and another family in one row
reopening collector drain or creating a docs-only sunset
new source/check file when an existing one can be extended
any touched source/check file reaching 800 lines
```

## Requested answer format

```text
Decision:
Ceremony: T1 / T2 / NoSafeSlice
Selected family:
Named production caller:
Input product:
Success product:
Deferred/rejection product:
Atomic old-edge deletion:
First real fixture:
Acceptance evidence:
Hard stops:
Next executable row or explicit design stop:
```

Do not open an I0/R0 row until this answer closes the source authority,
semantic ownership, failure precedence, and Deferred sunset boundary.
