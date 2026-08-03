# JOINIR-LOOP-NESTED-PREDICATE-CLOSURE0-D2-A-DESIGN-STOP

Status: design stop accepted; no production implementation authorized yet.
Date: 2026-08-03

## Why this stop exists

The real `NestedLoopMinimal` source shape is not the synthetic nested-`Always`
M6 witness.  The checked fixture
`apps/tests/phase1883_nested_minimal.hako` is:

```text
outer:   loop (i < 3) { local j; j = 0; inner; i = i + 1 }
inner:   loop (j < 3) { sum = sum + 1; j = j + 1 }
```

It has two predicate loops, a child-owned `j` carrier, and an update to the
ancestor-owned `sum` carrier.  It has no explicit `Break` or `Continue`.
The legacy extractor confirms this route only accepts Local/Assignment/inner
Loop/outer-step statements; `If` and explicit exits are different families.

The portable schema can already encode a nested `Predicate` condition, but
`LoopJoinSigElaboratorV1` intentionally rejects every non-root predicate at
`join_sig.rs` (`UnsupportedNestedPredicate`).  Removing that guard alone is
not a proof: the logical contract must explicitly close child false-path
completion, inherited ancestor carrier payloads, child `j` carrier updates,
the ancestor `sum` update crossing the child, and parent-body continuation.

## Authority boundary

```text
source projector (future, resolver-owned, AST-free output)
  -> verified recursive Recipe
  -> shared JoinSig logical closure
  -> later CanonicalCfgSession
  -> BindingSsaBuilderV1 -> PhiTxn
```

This card owns only the shared logical JoinSig contract and caller-zero tests.
It does not add a route, producer, source projector, physicalizer, PHI/SSA
writer, or scheduler behavior.  The PHI/SSA SSOT remains
`CanonicalCfgSessionV1 -> BindingSsaBuilderV1 -> PhiTxn`.

## Required decisions

1. **Route classification:** the no-explicit-exit two-predicate shape belongs
   to `NestedLoopMinimal`; outer `Break`/`Continue`, conditional exits, and
   branch merges belong to the LoopCond/LoopTrue cohorts and must not be
   smuggled into this row.
2. **Child false path:** a nested predicate's `PredicateFalse` edge must close
   to the child `After` port and return a normal flow to the parent body; it is
   not an implicit parent exit.
3. **Carrier visibility:** child edges expose ancestor carriers through the
   existing lineage payload rule, while child-owned carriers are declared and
   closed locally.  The contract must state how an ancestor carrier written in
   the child is returned to the parent flow.
4. **Closure evidence:** use the existing typed errors (`MissingCarrierClosure`,
   `BindingNotAvailable`, `ValueNotAvailable`, `UnsupportedExit`, and
   `BranchMergeMismatch`) where sufficient.  Add a new logical reject only if
   one of these cannot name the violated obligation; never use `Option`/Retry
   as a semantic escape hatch.
5. **Recipe schema:** first prove the existing `LoopRecipeV1` fields are
   sufficient.  A schema extension is a separate design stop, not an
   implementation convenience in this row.

## Caller-zero fixture and gates

The first fixture is source-free test data, not a production producer:

- root and child are both `Predicate`;
- child condition block computes `j < literal`;
- child body writes `sum` and `j` and falls through naturally;
- outer body continues with the outer `i` update;
- root/child parent relation and carrier ownership are explicit.

Acceptance requires:

1. Recipe verification succeeds without any route input.
2. JoinSig is deterministic and contains two rows.
3. Child `Enter`, `PredicateTrue`, `PredicateFalse`, and normal completion are
   explicit; no false-path is silently converted to `Break` or `Retry`.
4. Child `j` and ancestor `sum` obligations are visible on the appropriate
   logical edges and the parent flow receives the updated `sum`.
5. Typed negative fixtures cover missing child carrier, unavailable condition
   value, missing ancestor payload, and malformed/unreachable continuation.
6. No Builder/CorePlan/PlanLowerer/physical ID/PHI/SSA/Retry/route caller is
   introduced; all touched files remain below 800 lines.

## Ordering and independent legacy lane

The known V0 nested-carrier scope bug is separate legacy policy work: V0's
nested arm fails to propagate final carrier values while V1 already does.  A
new card may suppress V0 before effects and select V1, with digest and `.hako`
scope fixtures, without changing this JoinSig design.  That lane must not be
folded into D2-A or treated as proof that the new Recipe path is live.

After D2-A is accepted and the logical fixture is green, the order is:

```text
D2-B shared JoinSig nested-predicate closure (caller-zero)
  -> D2-C resolver-owned AST-free Nested shape/projector
  -> D2-D real NestedLoopMinimal Recipe producer (caller-zero)
  -> later physical pilot through canonical CFG/Binding SSA/PhiTxn
```
